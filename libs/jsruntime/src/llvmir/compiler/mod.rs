mod bridge;
mod control_flow;

use std::ffi::CStr;
use std::io::Write;
use std::ops::Deref;
use std::ops::DerefMut;

use indexmap::IndexMap;

use jsparser::syntax::LoopFlags;
use jsparser::Symbol;

use crate::lambda::LambdaId;
use crate::lambda::LambdaInfo;
use crate::logger;
use crate::semantics::CompileCommand;
use crate::semantics::Locator;
use crate::semantics::ScopeRef;
use crate::semantics::ScopeTree;
use crate::semantics::VariableRef;
use crate::types::Char16Seq;
use crate::Program;
use crate::Runtime;
use crate::Value;

use super::Module;

use bridge::BasicBlock;
use bridge::BooleanIr;
use bridge::CaptureIr;
use bridge::Char16SeqIr;
use bridge::ClosureIr;
use bridge::CompilerBridge;
use bridge::CoroutineIr;
use bridge::LambdaIr;
use bridge::NumberIr;
use bridge::ObjectIr;
use bridge::PromiseIr;
use bridge::StatusIr;
use bridge::SwitchIr;
use bridge::ValueIr;
use control_flow::ControlFlowStack;

const VALUE_SIZE: u32 = size_of::<Value>() as u32;
const VALUE_HOLDER_SIZE: u32 = size_of::<u64>() as u32;

impl<X> Runtime<X> {
    pub fn compile(&mut self, program: &Program, optimize: bool) -> Result<Module, CompileError> {
        logger::debug!(event = "compile");
        // TODO: Deferring the compilation until it's actually called improves the performance.
        // Because the program may contain unused functions.
        let mut compiler = Compiler::new(self, &program.scope_tree);
        compiler.start_compile();
        // Compile JavaScript functions in reverse order in order to compile a coroutine function
        // before its ramp function so that the size of the scratch buffer for the coroutine
        // function is available when the ramp function is compiled.
        //
        // NOTE: The functions are stored in post-order traversal on the function tree.  So, we
        // don't need to use `Iterator::rev()`.
        //
        // TODO: We should manage dependencies between functions in a more general way.
        for func in program.functions.iter() {
            compiler.start_function(func.name, func.id);
            for command in func.commands.iter() {
                compiler.process_command(command);
            }
            compiler.end_function(func.id, optimize);
        }
        Ok(compiler.end_compile(program.entry_lambda_id()))
    }
}

/// A Compiler targeting LLVM IR.
struct Compiler<'r, 's, R> {
    support: &'r mut R,

    /// The pointer to the compiler peer.
    bridge: CompilerBridge,

    /// The scope tree of the JavaScript program to compile.
    scope_tree: &'s ScopeTree,

    /// A stack for operands.
    operand_stack: OperandStack,

    /// A stack to hold sets of basic blocks which construct of a region in the control flow graph
    /// (CFG) finally built.
    control_flow_stack: ControlFlowStack,

    /// A stack to hold names of basic blocks representing the current nested structure of a
    /// JavaScript program currently compiled.
    ///
    /// A CFG in LLVM IR cannot represent nested structures in a program but information about
    /// nested structures of a program is useful in general when we debug the program.  A nested
    /// structure at some point during compilation can be represented by using a stack.
    basic_block_name_stack: Option<BasicBlockNameStack>,

    pending_labels: Vec<Symbol>,

    // The following values must be reset in the end of compilation for each function.
    locals: Vec<ValueIr>,
    captures: IndexMap<Locator, CaptureIr>,

    max_scratch_buffer_len: u32,

    dump_buffer: Option<Vec<std::ffi::c_char>>,

    enable_scope_cleanup_checker: bool,
}

trait CompilerSupport {
    // RuntimePref
    fn is_scope_cleanup_checker_enabled(&self) -> bool;
    fn is_llvmir_labels_enabled(&self) -> bool;

    // SymbolRegistry
    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol;

    // LambdaRegistry
    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo;
    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo;

    // Executor
    fn get_data_layout(&self) -> &CStr;
    fn get_target_triple(&self) -> &CStr;
}

impl<X> CompilerSupport for Runtime<X> {
    fn is_scope_cleanup_checker_enabled(&self) -> bool {
        self.pref.enable_scope_cleanup_checker
    }

    fn is_llvmir_labels_enabled(&self) -> bool {
        self.pref.enable_llvmir_labels
    }

    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol {
        self.symbol_registry.intern_utf16(name)
    }

    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo {
        self.lambda_registry.get(lambda_id)
    }

    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo {
        self.lambda_registry.get_mut(lambda_id)
    }

    fn get_data_layout(&self) -> &CStr {
        self.executor.get_data_layout()
    }

    fn get_target_triple(&self) -> &CStr {
        self.executor.get_target_triple()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    // TODO: define errors
}

macro_rules! push_bb_name {
    ($compiler:expr, $name:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.push($name);
        }
    };
    ($compiler:expr, $name:expr, $id:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.push_with_id($name, $id);
        }
    };
}

macro_rules! pop_bb_name {
    ($compiler:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.pop();
        }
    };
}

macro_rules! bb_name {
    ($compiler:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.as_name()
        } else {
            (c"".as_ptr(), 0)
        }
    };
}

macro_rules! runtime_debug {
    ($block:block) => {
        if cfg!(debug_assertions) $block
    };
}

impl<'r, 's, R> Compiler<'r, 's, R>
where
    R: CompilerSupport,
{
    pub fn new(support: &'r mut R, scope_tree: &'s ScopeTree) -> Self {
        const DUMP_BUFFER_SIZE: usize = 512;

        macro_rules! dump_enabled {
            () => {
                cfg!(debug_assertions) && matches!(
                    std::env::var_os("BEE_DEBUG_JSRUNTIME_COMPILER_DUMP"),
                    Some(v) if v == "1",
                )
            };
        }

        macro_rules! dump_buffer {
            () => {
                if dump_enabled!() {
                    Some(Vec::with_capacity(DUMP_BUFFER_SIZE))
                } else {
                    None
                }
            };
        }

        let enable_scope_cleanup_checker = support.is_scope_cleanup_checker_enabled();

        Self {
            support,
            bridge: Default::default(),
            scope_tree,
            operand_stack: Default::default(),
            control_flow_stack: Default::default(),
            basic_block_name_stack: None,
            pending_labels: Default::default(),
            locals: Default::default(),
            captures: Default::default(),
            max_scratch_buffer_len: 0,
            dump_buffer: dump_buffer!(),
            enable_scope_cleanup_checker,
        }
    }

    fn start_compile(&mut self) {
        let data_layout = self.support.get_data_layout();
        let target_triple = self.support.get_target_triple();
        let enable_labels = self.support.is_llvmir_labels_enabled();
        logger::debug!(
            event = "start_compile",
            ?data_layout,
            ?target_triple,
            enable_labels
        );
        self.bridge.start_compile();
        self.bridge.set_data_layout(data_layout);
        self.bridge.set_target_triple(target_triple);
        if enable_labels {
            self.bridge.enable_labels();
            self.basic_block_name_stack = Some(Default::default());
        }
    }

    fn end_compile(&self, entry_lambda_id: LambdaId) -> Module {
        logger::debug!(event = "end_compile");
        let module_peer = self.bridge.end_compile();
        Module::new(module_peer, entry_lambda_id)
    }

    fn start_function(&mut self, symbol: Symbol, lambda_id: LambdaId) {
        logger::debug!(event = "start_function", ?symbol, ?lambda_id);

        self.bridge.start_function(lambda_id);

        let locals_block = self.create_basic_block("locals");
        let init_block = self.create_basic_block("init");
        let args_block = self.create_basic_block("args");
        let body_block = self.create_basic_block("body");
        let return_block = self.create_basic_block("return");

        self.control_flow_stack.push_function_flow(
            locals_block,
            init_block,
            args_block,
            body_block,
            return_block,
        );

        assert!(self.pending_labels.is_empty());
        self.control_flow_stack
            .push_exit_target(return_block, false);

        self.bridge.set_locals_block(locals_block);

        self.bridge.set_basic_block(init_block);
        self.bridge.create_store_undefined_to_retv();
        self.bridge.create_alloc_status();
        self.bridge.create_alloc_flow_selector();
        if self.enable_scope_cleanup_checker {
            let is_coroutine = self.support.get_lambda_info(lambda_id).is_coroutine;
            self.bridge.enable_scope_cleanup_checker(is_coroutine);
        }

        self.bridge.set_basic_block(body_block);
    }

    fn end_function(&mut self, lambda_id: LambdaId, optimize: bool) {
        logger::debug!(event = "end_function", ?lambda_id, optimize);

        let dormant_block = self
            .support
            .get_lambda_info(lambda_id)
            .is_coroutine
            .then(|| self.control_flow_stack.pop_coroutine_flow().dormant_block);

        self.control_flow_stack.pop_exit_target();
        let flow = self.control_flow_stack.pop_function_flow();

        self.bridge.create_br(flow.return_block);
        self.bridge.move_basic_block_after(flow.return_block);

        self.bridge.set_basic_block(flow.locals_block);
        self.bridge.create_br(flow.init_block);
        self.bridge.move_basic_block_after(flow.init_block);

        self.bridge.set_basic_block(flow.init_block);
        self.bridge.create_br(flow.args_block);
        self.bridge.move_basic_block_after(flow.args_block);

        self.bridge.set_basic_block(flow.args_block);
        self.bridge.create_br(flow.body_block);
        self.bridge.move_basic_block_after(flow.body_block);

        self.bridge.set_basic_block(flow.return_block);
        if let Some(block) = dormant_block {
            self.bridge.move_basic_block_after(block);
        }

        if self.enable_scope_cleanup_checker {
            self.bridge.assert_scope_id(ScopeRef::NONE);
        }

        self.bridge.end_function(optimize);

        self.locals.clear();

        debug_assert!(self.captures.is_empty());
        self.captures.clear();

        debug_assert!(self.control_flow_stack.is_empty());
        self.control_flow_stack.clear();

        let info = self.support.get_lambda_info_mut(lambda_id);
        if info.is_coroutine {
            info.scratch_buffer_len = self.max_scratch_buffer_len;
        }
        self.max_scratch_buffer_len = 0;
    }

    fn process_command(&mut self, command: &CompileCommand) {
        logger::debug!(event = "process_command", ?command);
        match command {
            CompileCommand::Nop => (),
            CompileCommand::Undefined => self.process_undefined(),
            CompileCommand::Null => self.process_null(),
            CompileCommand::Boolean(value) => self.process_boolean(*value),
            CompileCommand::Number(value) => self.process_number(*value),
            CompileCommand::String(value) => self.process_string(value),
            CompileCommand::Object => self.process_object(),
            CompileCommand::Lambda(lambda_id) => self.process_lambda(*lambda_id),
            CompileCommand::Closure(prologue, func_scope_ref) => {
                self.process_closure(*prologue, *func_scope_ref)
            }
            CompileCommand::Coroutine(lambda_id, num_locals) => {
                self.process_coroutine(*lambda_id, *num_locals)
            }
            CompileCommand::Promise => self.process_promise(),
            CompileCommand::Exception => self.process_exception(),
            CompileCommand::VariableReference(symbol) => self.process_variable_reference(*symbol),
            CompileCommand::PropertyReference(symbol) => self.process_property_reference(*symbol),
            CompileCommand::ToPropertyKey => self.process_to_property_key(),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::MutableVariable => self.process_mutable_variable(),
            CompileCommand::ImmutableVariable => self.process_immutable_variable(),
            CompileCommand::DeclareVars(scope_ref) => self.process_declare_vars(*scope_ref),
            CompileCommand::DeclareClosure => self.process_declare_closure(),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(*scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
            CompileCommand::CreateDataProperty => self.process_create_data_property(),
            CompileCommand::CopyDataProperties => self.process_copy_data_properties(),
            CompileCommand::PushArrayElement => self.process_push_array_element(),
            CompileCommand::PostfixIncrement => self.process_postfix_increment(),
            CompileCommand::PostfixDecrement => self.process_postfix_decrement(),
            CompileCommand::PrefixIncrement => self.process_prefix_increment(),
            CompileCommand::PrefixDecrement => self.process_prefix_decrement(),
            CompileCommand::Delete => self.process_delete(),
            CompileCommand::Void => self.process_void(),
            CompileCommand::Typeof => self.process_typeof(),
            CompileCommand::UnaryPlus => self.process_unary_plus(),
            CompileCommand::UnaryMinus => self.process_unary_minus(),
            CompileCommand::BitwiseNot => self.process_bitwise_not(),
            CompileCommand::LogicalNot => self.process_logical_not(),
            CompileCommand::Exponentiation => self.process_exponentiation(),
            CompileCommand::Multiplication => self.process_multiplication(),
            CompileCommand::Division => self.process_division(),
            CompileCommand::Remainder => self.process_remainder(),
            CompileCommand::Addition => self.process_addition(),
            CompileCommand::Subtraction => self.process_subtraction(),
            CompileCommand::LeftShift => self.process_left_shift(),
            CompileCommand::SignedRightShift => self.process_signed_right_shift(),
            CompileCommand::UnsignedRightShift => self.process_unsigned_right_shift(),
            CompileCommand::LessThan => self.process_less_than(),
            CompileCommand::GreaterThan => self.process_greater_than(),
            CompileCommand::LessThanOrEqual => self.process_less_than_or_equal(),
            CompileCommand::GreaterThanOrEqual => self.process_greater_than_or_equal(),
            CompileCommand::Instanceof => self.process_instanceof(),
            CompileCommand::In => self.process_in(),
            CompileCommand::Equality => self.process_equality(),
            CompileCommand::Inequality => self.process_inequality(),
            CompileCommand::StrictEquality => self.process_strict_equality(),
            CompileCommand::StrictInequality => self.process_strict_inequality(),
            CompileCommand::BitwiseAnd => self.process_bitwise_and(),
            CompileCommand::BitwiseXor => self.process_bitwise_xor(),
            CompileCommand::BitwiseOr => self.process_bitwise_or(),
            CompileCommand::Ternary => self.process_ternary(),
            CompileCommand::Assignment => self.process_assignment(),
            CompileCommand::FalsyShortCircuit => self.process_falsy_short_circuit(),
            CompileCommand::TruthyShortCircuit => self.process_truthy_short_circuit(),
            CompileCommand::NullishShortCircuit => self.process_nullish_short_circuit(),
            CompileCommand::Truthy => self.process_truthy(),
            CompileCommand::NonNullish => self.process_non_nullish(),
            CompileCommand::IfThen => self.process_if_then(),
            CompileCommand::Else => self.process_else(),
            CompileCommand::IfElseStatement => self.process_if_else_statement(),
            CompileCommand::IfStatement => self.process_if_statement(),
            CompileCommand::DoWhileLoop(id) => self.process_do_while_loop(*id),
            CompileCommand::WhileLoop(id) => self.process_while_loop(*id),
            CompileCommand::ForLoop(id, flags) => self.process_for_loop(*id, *flags),
            CompileCommand::LoopInit => self.process_loop_init(),
            CompileCommand::LoopTest => self.process_loop_test(),
            CompileCommand::LoopNext => self.process_loop_next(),
            CompileCommand::LoopBody => self.process_loop_body(),
            CompileCommand::LoopEnd => self.process_loop_end(),
            CompileCommand::CaseBlock(id, num_cases) => self.process_case_block(*id, *num_cases),
            CompileCommand::Case => self.process_case(),
            CompileCommand::Default => self.process_default(),
            CompileCommand::CaseClause(has_statement) => self.process_case_clause(*has_statement),
            CompileCommand::Switch(id, num_cases, default_index) => {
                self.process_switch(*id, *num_cases, *default_index)
            }
            CompileCommand::Try => self.process_try(),
            CompileCommand::Catch(nominal) => self.process_catch(*nominal),
            CompileCommand::Finally(nominal) => self.process_finally(*nominal),
            CompileCommand::TryEnd => self.process_try_end(),
            CompileCommand::LabelStart(symbol, is_iteration_statement) => {
                self.process_label_start(*symbol, *is_iteration_statement)
            }
            CompileCommand::LabelEnd(symbol, is_iteration_statement) => {
                self.process_label_end(*symbol, *is_iteration_statement)
            }
            CompileCommand::Continue(symbol) => self.process_continue(*symbol),
            CompileCommand::Break(symbol) => self.process_break(*symbol),
            CompileCommand::Return(n) => self.process_return(*n),
            CompileCommand::Throw => self.process_throw(),
            CompileCommand::Environment(num_locals) => self.process_environment(*num_locals),
            CompileCommand::JumpTable(num_states) => self.process_jump_table(*num_states),
            CompileCommand::Await(next_state) => self.process_await(*next_state),
            CompileCommand::Resume => self.process_resume(),
            CompileCommand::Discard => self.process_discard(),
            CompileCommand::Swap => self.process_swap(),
            CompileCommand::Duplicate(offset) => self.process_duplicate(*offset),
            CompileCommand::Dereference => self.process_dereference(),
            CompileCommand::Debugger => self.process_debugger(),
            CompileCommand::PlaceHolder => unreachable!(),
        }

        if let Some(ref mut buf) = self.dump_buffer {
            let size = buf.capacity();
            let buf = buf.as_mut_ptr();

            eprintln!("### operand-stack");
            self.operand_stack.dump(buf, size);
            eprintln!();

            self.control_flow_stack.dump(buf, size);
        }
    }

    fn process_undefined(&mut self) {
        self.operand_stack.push(Operand::Undefined);
    }

    fn process_null(&mut self) {
        self.operand_stack.push(Operand::Null);
    }

    fn process_boolean(&mut self, value: bool) {
        let boolean = self.bridge.get_boolean(value);
        self.operand_stack
            .push(Operand::Boolean(boolean, Some(value)));
    }

    fn process_number(&mut self, value: f64) {
        let number = self.bridge.get_number(value);
        self.operand_stack
            .push(Operand::Number(number, Some(value)));
    }

    fn process_string(&mut self, value: &[u16]) {
        // Theoretically, the heap memory pointed by `value` can be freed after the IR built by the
        // compiler is freed.
        let seq = self.bridge.create_char16_seq(value);
        self.operand_stack
            .push(Operand::String(seq, Some(Char16Seq::new_stack(value))));
    }

    fn process_object(&mut self) {
        let object = self.bridge.create_object();
        self.operand_stack.push(Operand::Object(object));
    }

    fn process_lambda(&mut self, lambda_id: LambdaId) {
        let lambda = self.bridge.get_function(lambda_id);
        self.operand_stack.push(Operand::Lambda(lambda));
    }

    fn pop_lambda(&mut self) -> LambdaIr {
        match self.operand_stack.pop() {
            Some(Operand::Lambda(lambda)) => lambda,
            _ => unreachable!(),
        }
    }

    fn process_closure(&mut self, prologue: bool, func_scope_ref: ScopeRef) {
        let backup = self.bridge.get_basic_block();
        if prologue {
            let block = self.control_flow_stack.scope_flow().hoisted_block;
            self.bridge.set_basic_block(block);
        }

        let scope = self.scope_tree.scope(func_scope_ref);
        debug_assert!(scope.is_function());

        let lambda = self.pop_lambda();
        // TODO(perf): use `Function::num_captures` instead of `Scope::count_captures()`.
        let closure = self.bridge.create_closure(lambda, scope.count_captures());

        let scope_ref = self.control_flow_stack.scope_flow().scope_ref;
        for variable in scope
            .variables
            .iter()
            .filter(|variable| variable.is_capture())
        {
            // TODO(perf): improve if `find_variable()` is the primary case of performance
            // bottleneck.
            let variable_ref = self.scope_tree.find_variable(scope_ref, variable.symbol);
            debug_assert_ne!(variable_ref, VariableRef::NONE);
            let locator = self.scope_tree.compute_locator(variable_ref);
            let capture = match locator {
                Locator::Argument(_) | Locator::Local(_) => {
                    debug_assert!(self.captures.contains_key(&locator));
                    *self.captures.get(&locator).unwrap()
                }
                Locator::Capture(i) => self.bridge.create_load_capture(i),
                _ => unreachable!(),
            };
            self.bridge
                .create_store_capture_to_closure(capture, closure, variable.index);
        }

        self.operand_stack.push(Operand::Closure(closure));

        if prologue {
            self.bridge.set_basic_block(backup);
        }
    }

    fn pop_closure(&mut self) -> ClosureIr {
        match self.operand_stack.pop() {
            Some(Operand::Closure(closure)) => closure,
            _ => unreachable!(),
        }
    }

    fn process_coroutine(&mut self, lambda_id: LambdaId, num_locals: u16) {
        let scrach_buffer_len = self.support.get_lambda_info(lambda_id).scratch_buffer_len;
        debug_assert!(scrach_buffer_len <= u16::MAX as u32);
        let closure = self.pop_closure();
        let coroutine = self
            .bridge
            .create_coroutine(closure, num_locals, scrach_buffer_len as u16);
        self.operand_stack.push(Operand::Coroutine(coroutine));
    }

    fn process_promise(&mut self) {
        let coroutine = self.pop_coroutine();
        let promise = self.bridge.create_register_promise(coroutine);
        self.operand_stack.push(Operand::Promise(promise));
    }

    fn process_exception(&mut self) {
        // TODO: Should we check status_ at runtime?
        self.operand_stack
            // TODO(perf): compile-time evaluation
            .push(Operand::Any(self.bridge.get_exception(), None));
    }

    fn process_variable_reference(&mut self, symbol: Symbol) {
        let scope_ref = self.control_flow_stack.scope_flow().scope_ref;
        // TODO(perf): improve if `find_variable()` is the primary case of performance bottleneck.
        let variable_ref = self.scope_tree.find_variable(scope_ref, symbol);
        debug_assert_ne!(variable_ref, VariableRef::NONE);
        let locator = self.scope_tree.compute_locator(variable_ref);
        self.operand_stack
            .push(Operand::VariableReference(symbol, locator));
    }

    fn process_property_reference(&mut self, symbol: Symbol) {
        self.operand_stack
            .push(Operand::PropertyReference(symbol.into()));
    }

    fn process_to_property_key(&mut self) {
        let (operand, _) = self.dereference();
        let key = match operand {
            Operand::Undefined => Symbol::UNDEFINED.into(),
            Operand::Null => Symbol::NULL.into(),
            Operand::Boolean(_, Some(false)) => Symbol::FALSE.into(),
            Operand::Boolean(_, Some(true)) => Symbol::TRUE.into(),
            Operand::Boolean(value, None) => self.bridge.create_boolean_to_any(value).into(),
            Operand::Number(_, Some(value)) => value.into(),
            Operand::Number(value, None) => self.bridge.create_number_to_any(value).into(),
            Operand::String(_, Some(ref value)) => self
                .support
                .make_symbol_from_name(value.make_utf16())
                .into(),
            Operand::String(value, None) => self.bridge.create_string_to_any(value).into(),
            Operand::Lambda(_) => todo!(),
            Operand::Closure(value) => self.bridge.create_closure_to_any(value).into(),
            Operand::Coroutine(_) => todo!(),
            Operand::Object(value) => self.bridge.create_object_to_any(value).into(),
            Operand::Promise(_) => todo!(),
            Operand::Any(_, Some(Value::Undefined)) => Symbol::UNDEFINED.into(),
            Operand::Any(_, Some(Value::Null)) => Symbol::NULL.into(),
            Operand::Any(_, Some(Value::Boolean(false))) => Symbol::FALSE.into(),
            Operand::Any(_, Some(Value::Boolean(true))) => Symbol::FALSE.into(),
            Operand::Any(_, Some(Value::String(value))) => self
                .support
                .make_symbol_from_name(value.make_utf16())
                .into(),
            Operand::Any(value, None) => value.into(),
            Operand::Any(..) => todo!(),
            Operand::PropertyReference(_) | Operand::VariableReference(..) => {
                unreachable!("{operand:?}")
            }
        };
        self.operand_stack.push(Operand::PropertyReference(key));
    }

    fn process_allocate_locals(&mut self, num_locals: u16) {
        for i in 0..num_locals {
            let local = self.bridge.create_local_value(i);
            self.locals.push(local);
        }
    }

    fn process_mutable_variable(&mut self) {
        let (_symbol, locator) = self.pop_reference();
        let (operand, _) = self.dereference();

        let value = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.create_store_operand_to_value(&operand, value);
    }

    fn dereference(&mut self) -> (Operand, Option<(Symbol, Locator)>) {
        logger::debug!(event = "dereference", operand_stack.top=?self.operand_stack.last());

        let operand = self.operand_stack.pop().unwrap();
        match operand {
            // Shortcut for frequently used reference to `undefined`.
            Operand::VariableReference(Symbol::UNDEFINED, Locator::Global) => (
                Operand::Undefined,
                Some((Symbol::UNDEFINED, Locator::Global)),
            ),
            Operand::VariableReference(symbol, locator) => {
                let value = self.create_get_value_ptr(symbol, locator);
                // TODO(pref): compile-time evaluation
                (Operand::Any(value, None), Some((symbol, locator)))
            }
            Operand::PropertyReference(key) => {
                self.perform_to_object();
                let object = self.pop_object();
                let value = match key {
                    PropertyKey::Symbol(key) => {
                        self.bridge.create_get_value_by_symbol(object, key, false)
                    }
                    PropertyKey::Number(key) => {
                        self.bridge.create_get_value_by_number(object, key, false)
                    }
                    PropertyKey::Value(key) => {
                        self.bridge.create_get_value_by_value(object, key, false)
                    }
                };
                runtime_debug! {{
                    let is_nullptr = self.bridge.create_value_is_nullptr(value);
                    let non_nullptr = self.bridge.create_logical_not(is_nullptr);
                    self.bridge.create_assert(
                        non_nullptr,
                        c"runtime.get_value() should return a non-null pointer",
                    );
                }}
                // TODO(pref): compile-time evaluation
                (Operand::Any(value, None), None)
            }
            _ => (operand, None),
        }
    }

    fn create_get_value_ptr(&mut self, symbol: Symbol, locator: Locator) -> ValueIr {
        match locator {
            Locator::None => unreachable!(),
            Locator::Argument(index) => self.bridge.create_get_argument_value_ptr(index),
            Locator::Local(index) => self.locals[index as usize],
            Locator::Capture(index) => self.bridge.create_get_capture_value_ptr(index),
            Locator::Global => self.create_get_global_value_ptr(symbol),
        }
    }

    // TODO(perf): return the value directly if it's a read-only global property.
    fn create_get_global_value_ptr(&mut self, key: Symbol) -> ValueIr {
        // TODO: strict mode
        let value =
            self.bridge
                .create_get_value_by_symbol(self.bridge.get_object_nullptr(), key, true);

        let then_block = self.create_basic_block("global_object.get_value.is_nullptr");
        let end_block = self.create_basic_block("global_object.get_value");

        // if value.is_nullptr()
        let is_nullptr = self.bridge.create_value_is_nullptr(value);
        self.bridge
            .create_cond_br(is_nullptr, then_block, end_block);
        // {
        self.bridge.set_basic_block(then_block);
        // TODO(feat): ReferenceError
        self.process_number(1000.);
        self.process_throw();
        self.bridge.create_br(end_block);
        // }
        self.bridge.set_basic_block(end_block);

        value
    }

    fn pop_reference(&mut self) -> (Symbol, Locator) {
        match self.operand_stack.pop().unwrap() {
            Operand::VariableReference(symbol, locator) => (symbol, locator),
            operand => unreachable!("{operand:?}"),
        }
    }

    fn create_store_operand_to_value(&mut self, operand: &Operand, dest: ValueIr) {
        match operand {
            Operand::Undefined => self.bridge.create_store_undefined_to_value(dest),
            Operand::Null => self.bridge.create_store_null_to_value(dest),
            Operand::Boolean(value, ..) => self.bridge.create_store_boolean_to_value(*value, dest),
            Operand::Number(value, ..) => self.bridge.create_store_number_to_value(*value, dest),
            Operand::String(value, ..) => self.bridge.create_store_string_to_value(*value, dest),
            Operand::Closure(value) => self.bridge.create_store_closure_to_value(*value, dest),
            Operand::Object(value) => self.bridge.create_store_object_to_value(*value, dest),
            Operand::Promise(value) => self.bridge.create_store_promise_to_value(*value, dest),
            Operand::Any(value, ..) => self.bridge.create_store_value_to_value(*value, dest),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    fn process_immutable_variable(&mut self) {
        let (_symbol, locator) = self.pop_reference();
        let (operand, _) = self.dereference();

        let value = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.create_store_operand_to_value(&operand, value);
    }

    fn process_declare_vars(&mut self, scope_ref: ScopeRef) {
        debug_assert!(self.scope_tree.scope(scope_ref).is_function());

        // In the specification, function-scoped variables defined by "VariableStatement"s are
        // created in "10.2.11 FunctionDeclarationInstantiation ( func, argumentsList )".  We
        // create them here for simplicity but this still works properly for well-formed JavaScript
        // programs.
        //
        // Function-scoped variables are created in the `init` basic block of the current scope.
        // The `init` basic block is performed before the `hoisted` basic block on which inner
        // functions defined by "FunctionDeclaration"s are created.
        let block = self.control_flow_stack.scope_flow().init_block;

        let backup = self.bridge.get_basic_block();
        self.bridge.set_basic_block(block);

        // TODO(refactor): inefficient
        for (variable_ref, variable) in self.scope_tree.iter_variables(scope_ref) {
            if !variable.is_function_scoped() {
                continue;
            }
            let value = match self.scope_tree.compute_locator(variable_ref) {
                Locator::Local(index) => self.locals[index as usize],
                locator => unreachable!("{locator:?}"),
            };
            self.bridge.create_store_undefined_to_value(value);
        }

        self.bridge.set_basic_block(backup);
    }

    fn process_declare_closure(&mut self) {
        let block = self.control_flow_stack.scope_flow().hoisted_block;

        let backup = self.bridge.get_basic_block();
        self.bridge.set_basic_block(block);

        let (symbol, locator) = self.pop_reference();
        let (operand, _) = self.dereference();
        // TODO: operand must hold a closure.

        match locator {
            Locator::Local(index) => {
                let value = self.locals[index as usize];
                self.create_store_operand_to_value(&operand, value);
            }
            Locator::Global => {
                let value = self.create_to_any(&operand);
                self.bridge.create_set_value_by_symbol(
                    self.bridge.get_object_nullptr(),
                    symbol,
                    value,
                );
            }
            _ => unreachable!("{locator:?}"),
        };

        self.bridge.set_basic_block(backup);
    }

    fn swap(&mut self) {
        logger::debug!(event = "swap");
        debug_assert!(self.operand_stack.len() > 1);
        let last_index = self.operand_stack.len() - 1;
        self.operand_stack.swap(last_index - 1, last_index);
    }

    fn duplicate(&mut self, offset: u8) {
        logger::debug!(event = "duplicate", offset);
        debug_assert!(self.operand_stack.len() > offset as usize);
        let index = self.operand_stack.len() - 1 - offset as usize;
        self.operand_stack.duplicate(index);
    }

    fn process_call(&mut self, argc: u16) {
        let argv = if argc > 0 {
            let argv = self.bridge.create_argv(argc);
            for i in (0..argc).rev() {
                let (operand, _) = self.dereference();
                let ptr = self.bridge.create_get_arg_in_argv(argv, i);
                self.create_store_operand_to_value(&operand, ptr);
            }
            argv
        } else {
            self.bridge.get_argv_nullptr()
        };

        let (operand, _) = self.dereference();
        let closure = match operand {
            Operand::Closure(closure) => closure, // IIFE
            Operand::Any(value, ..) => {
                self.create_load_closure_from_value_or_throw_type_error(value)
            }
            _ => {
                self.process_number(1.);
                self.process_throw();
                return;
            }
        };

        let retv = self.bridge.create_retv();

        let status = self
            .bridge
            .create_call_on_closure(closure, argc, argv, retv);

        self.create_check_status_for_exception(status, retv);

        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(retv, None));
    }

    fn create_load_closure_from_value_or_throw_type_error(&mut self, value: ValueIr) -> ClosureIr {
        let then_block = self.create_basic_block("is_closure.then");
        let else_block = self.create_basic_block("is_closure.else");
        let end_block = self.create_basic_block("closure");

        // if value.is_closure()
        let is_closure = self.bridge.create_is_closure(value);
        self.bridge
            .create_cond_br(is_closure, then_block, else_block);
        // then
        let (then_value, then_block) = {
            self.bridge.set_basic_block(then_block);
            let closure = self.bridge.create_load_closure_from_value(value);
            self.bridge.create_br(end_block);
            (closure, self.bridge.get_basic_block())
        };
        // else
        let (else_value, else_block) = {
            self.bridge.set_basic_block(else_block);
            // TODO(feat): TypeError
            self.process_number(1001.);
            self.process_throw();
            self.bridge.create_br(end_block);
            (
                self.bridge.get_closure_nullptr(),
                self.bridge.get_basic_block(),
            )
        };

        self.bridge.set_basic_block(end_block);
        self.bridge
            .create_closure_phi(then_value, then_block, else_value, else_block)
    }

    // Handle an exception if it's thrown.
    fn create_check_status_for_exception(&mut self, status: StatusIr, retv: ValueIr) {
        let exception_block = self.control_flow_stack.exception_block();

        let then_block = self.create_basic_block("status.exception");
        let else_block = self.create_basic_block("status.normal");

        // if status.is_exception()
        let is_exception = self.bridge.create_is_exception_status(status);
        self.bridge
            .create_cond_br(is_exception, then_block, else_block);
        // then
        {
            self.bridge.set_basic_block(then_block);
            self.bridge.create_store_exception_status();
            self.bridge.create_set_flow_selector_throw();
            self.bridge.create_store_value_to_retv(retv);
            self.bridge.create_br(exception_block);
        }

        self.bridge.set_basic_block(else_block);
    }

    fn process_push_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        push_bb_name!(self, "scope", scope_ref.id());

        let init_block = self.create_basic_block("init");
        let hoisted_block = self.create_basic_block("hoisted");
        let body_block = self.create_basic_block("body");
        let cleanup_block = self.create_basic_block("cleanup");

        self.control_flow_stack.push_scope_flow(
            scope_ref,
            init_block,
            hoisted_block,
            body_block,
            cleanup_block,
        );

        self.control_flow_stack
            .push_exit_target(cleanup_block, false);

        self.bridge.create_br(init_block);
        self.bridge.move_basic_block_after(init_block);

        self.bridge.set_basic_block(init_block);

        let scope = self.scope_tree.scope(scope_ref);
        for variable in scope.variables.iter() {
            if variable.is_function_scoped() {
                continue;
            }
            let locator = variable.locator();
            if variable.is_captured() {
                let value = match locator {
                    Locator::Argument(index) => self.bridge.create_get_argument_value_ptr(index),
                    Locator::Local(index) => self.locals[index as usize],
                    _ => unreachable!(),
                };
                let capture = self.bridge.create_capture(value);
                debug_assert!(!self.captures.contains_key(&locator));
                self.captures.insert(locator, capture);
            }
            if let Locator::Local(index) = locator {
                let value = self.locals[index as usize];
                self.bridge.create_store_none_to_value(value);
            }
        }

        self.bridge.set_basic_block(body_block);
    }

    fn process_pop_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        // Create additional blocks of the scope region before pop_bb_name!().
        // Because these constitute the scope region.
        let precheck_block = self.create_basic_block("precheck");
        let postcheck_block = self.create_basic_block("postcheck");
        let ctrl_block = self.create_basic_block("ctrl");
        let exit_block = self.create_basic_block("exit");

        self.control_flow_stack.pop_exit_target();
        let parent_exit_block = self.control_flow_stack.exit_block();

        let flow = self.control_flow_stack.pop_scope_flow();
        debug_assert_eq!(flow.scope_ref, scope_ref);

        self.bridge.create_br(flow.cleanup_block);
        self.bridge.move_basic_block_after(flow.cleanup_block);

        self.bridge.set_basic_block(flow.init_block);
        self.bridge.create_br(precheck_block);
        self.bridge.move_basic_block_after(precheck_block);

        self.bridge.set_basic_block(precheck_block);
        if self.enable_scope_cleanup_checker {
            self.bridge.set_scope_id_for_checker(scope_ref);
        }
        self.bridge.create_br(flow.hoisted_block);
        self.bridge.move_basic_block_after(flow.hoisted_block);

        self.bridge.set_basic_block(flow.hoisted_block);
        self.bridge.create_br(flow.body_block);
        self.bridge.move_basic_block_after(flow.body_block);

        self.bridge.set_basic_block(flow.cleanup_block);
        let scope = self.scope_tree.scope(scope_ref);
        for variable in scope.variables.iter() {
            if variable.is_captured() {
                self.escape_value(variable.locator());
            }
            if variable.is_local() {
                // tidy local value
                // TODO: GC
            }
        }
        self.bridge.create_br(postcheck_block);
        self.bridge.move_basic_block_after(postcheck_block);

        self.bridge.set_basic_block(postcheck_block);
        if self.enable_scope_cleanup_checker {
            self.bridge.assert_scope_id(scope_ref);
            if self.control_flow_stack.has_scope_flow() {
                let outer_scope_ref = self.control_flow_stack.scope_flow().scope_ref;
                self.bridge.set_scope_id_for_checker(outer_scope_ref);
            } else {
                self.bridge.set_scope_id_for_checker(ScopeRef::NONE);
            }
        }
        self.bridge.create_br(ctrl_block);
        self.bridge.move_basic_block_after(ctrl_block);

        self.bridge.set_basic_block(ctrl_block);
        let is_normal = self.bridge.create_is_flow_selector_normal();
        self.bridge
            .create_cond_br(is_normal, exit_block, parent_exit_block);

        self.bridge.move_basic_block_after(exit_block);
        self.bridge.set_basic_block(exit_block);

        pop_bb_name!(self);
    }

    fn escape_value(&mut self, locator: Locator) {
        debug_assert!(!locator.is_capture());
        debug_assert!(self.captures.contains_key(&locator));
        let capture = self.captures.swap_remove(&locator).unwrap();
        let value = self.create_get_value_ptr(Symbol::NONE, locator);
        self.bridge.create_escape_value(capture, value);
    }

    // 13.2.5.5 Runtime Semantics: PropertyDefinitionEvaluation
    fn process_create_data_property(&mut self) {
        let (operand, _) = self.dereference();
        let key = self.pop_property_reference();
        let object = self.peek_object();
        let value = self.create_to_any(&operand);
        let retv = self.bridge.create_retv();

        // 7.3.6 CreateDataPropertyOrThrow ( O, P, V )

        // 1. Let success be ? CreateDataProperty(O, P, V).
        let status = match key {
            PropertyKey::Symbol(key) => self
                .bridge
                .create_create_data_property_by_symbol(object, key, value, retv),
            PropertyKey::Number(key) => self
                .bridge
                .create_create_data_property_by_number(object, key, value, retv),
            PropertyKey::Value(key) => self
                .bridge
                .create_create_data_property_by_value(object, key, value, retv),
        };
        self.create_check_status_for_exception(status, retv);
        // `retv` holds a boolean value.
        runtime_debug! {{
            let is_boolean = self.bridge.create_is_boolean(retv);
            self.bridge.create_assert(
                is_boolean,
                c"runtime.create_data_property() returns a boolan value",
            );
        }}
        let success = self.bridge.create_load_boolean_from_value(retv);

        // 2. If success is false, throw a TypeError exception.
        let then_block = self.create_basic_block("success.then");
        let else_block = self.create_basic_block("success.else");
        let merge_block = self.create_basic_block("success.merge");
        // if success
        self.bridge.create_cond_br(success, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        self.bridge.create_br(merge_block);
        // } else {
        self.bridge.set_basic_block(else_block);
        // TODO(feat): TypeError
        self.process_number(1001.);
        self.process_throw();
        self.bridge.create_br(merge_block);
        // }
        self.bridge.set_basic_block(merge_block);
    }

    // 13.2.5.5 Runtime Semantics: PropertyDefinitionEvaluation
    // PropertyDefinition : ... AssignmentExpression
    fn process_copy_data_properties(&mut self) {
        // 1. Let exprValue be ? Evaluation of AssignmentExpression.
        let (operand, _) = self.dereference();

        // 2. Let fromValue be ? GetValue(exprValue).
        let from_value = self.create_to_any(&operand);

        // TODO: 3. Let excludedNames be a new empty List.

        // 4. Perform ? CopyDataProperties(object, fromValue, excludedNames).

        let object = self.peek_object();
        let retv = self.bridge.create_retv();

        let status = self
            .bridge
            .create_copy_data_properties(object, from_value, retv);
        self.create_check_status_for_exception(status, retv);
    }

    fn process_push_array_element(&mut self) {
        // 1. Let exprValue be ? Evaluation of AssignmentExpression.
        let (operand, _) = self.dereference();

        // 2. Let fromValue be ? GetValue(exprValue).
        let from_value = self.create_to_any(&operand);

        let object = self.peek_object();
        let retv = self.bridge.create_retv();

        let status = self
            .bridge
            .create_push_array_element(object, from_value, retv);
        self.create_check_status_for_exception(status, retv);
    }

    // 7.1.18 ToObject ( argument )
    fn perform_to_object(&mut self) {
        let (operand, _) = self.dereference();
        match operand {
            Operand::Undefined | Operand::Null => {
                // TODO(feat): TypeError
                self.process_number(1001.);
                self.process_throw();
            }
            Operand::Boolean(..) => todo!(),
            Operand::Number(..) => todo!(),
            Operand::String(..) => todo!(),
            Operand::Closure(_value) => todo!(),
            Operand::Object(value) => self.operand_stack.push(Operand::Object(value)),
            Operand::Promise(_value) => todo!(),
            Operand::Any(value, ..) => {
                let object = self.bridge.create_to_object(value);
                self.operand_stack.push(Operand::Object(object));
            }
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    fn peek_object(&mut self) -> ObjectIr {
        match self.operand_stack.last().unwrap() {
            Operand::Object(value) => *value,
            _ => unreachable!(),
        }
    }

    fn pop_object(&mut self) -> ObjectIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Object(value) => value,
            operand => unreachable!("{operand:?}"),
        }
    }

    fn pop_property_reference(&mut self) -> PropertyKey {
        match self.operand_stack.pop().unwrap() {
            Operand::PropertyReference(key) => key,
            _ => unreachable!(),
        }
    }

    // 13.4.2.1 Runtime Semantics: Evaluation
    // 13.4.3.1 Runtime Semantics: Evaluation
    // 13.4.4.1 Runtime Semantics: Evaluation
    // 13.4.5.1 Runtime Semantics: Evaluation
    fn process_incr_decr(&mut self, pos: char, op: char) {
        let (operand, reference) = self.dereference();
        let old_value = self.to_numeric(operand);
        // TODO: BigInt
        let one = self.bridge.get_number(1.0);
        let new_value = if op == '+' {
            self.bridge.create_fadd(old_value, one)
        } else {
            self.bridge.create_fsub(old_value, one)
        };
        match reference {
            Some((symbol, locator)) if symbol != Symbol::NONE => {
                debug_assert!(!locator.is_none());
                self.operand_stack
                    .push(Operand::VariableReference(symbol, locator));
                // TODO(perf): compile-time evaluation
                self.operand_stack.push(Operand::Number(new_value, None));
                self.process_assignment();
                self.process_discard();
            }
            _ => {
                // TODO(feat): throw a ReferenceError at runtime
            }
        }
        let value = if pos == '^' { new_value } else { old_value };
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 7.1.4 ToNumber ( argument )
    fn to_numeric(&self, operand: Operand) -> NumberIr {
        match operand {
            Operand::Undefined => self.bridge.get_nan(),
            Operand::Null => self.bridge.get_zero(),
            Operand::Boolean(value, ..) => self.bridge.create_boolean_to_number(value),
            Operand::Number(value, ..) => value,
            Operand::String(..) => unimplemented!("string.to_numeric"),
            Operand::Closure(_) => self.bridge.get_nan(),
            Operand::Object(_) => unimplemented!("object.to_numeric"),
            Operand::Any(value, ..) => self.bridge.to_numeric(value),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::Promise(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    // 13.4.2.1 Runtime Semantics: Evaluation
    fn process_postfix_increment(&mut self) {
        self.process_incr_decr('$', '+');
    }

    // 13.4.3.1 Runtime Semantics: Evaluation
    fn process_postfix_decrement(&mut self) {
        self.process_incr_decr('$', '-');
    }

    // 13.4.4.1 Runtime Semantics: Evaluation
    fn process_prefix_increment(&mut self) {
        self.process_incr_decr('^', '+');
    }

    // 13.4.5.1 Runtime Semantics: Evaluation
    fn process_prefix_decrement(&mut self) {
        self.process_incr_decr('^', '-');
    }

    // 13.5.1.2 Runtime Semantics: Evaluation
    fn process_delete(&mut self) {
        unimplemented!("delete operator");
    }

    // 13.5.2.1 Runtime Semantics: Evaluation
    fn process_void(&mut self) {
        self.operand_stack.pop();
        self.operand_stack.push(Operand::Undefined);
    }

    // 13.5.3.1 Runtime Semantics: Evaluation
    fn process_typeof(&mut self) {
        use jsparser::symbol::builtin::names;

        let (operand, _) = self.dereference();
        match operand {
            Operand::Undefined => self.process_string(names::UNDEFINED),
            Operand::Null => self.process_string(names::OBJECT),
            Operand::Boolean(..) => self.process_string(names::BOOLEAN),
            Operand::Number(..) => self.process_string(names::NUMBER),
            Operand::String(..) => self.process_string(names::STRING),
            Operand::Closure(..) | Operand::Coroutine(..) => self.process_string(names::FUNCTION),
            Operand::Object(..) | Operand::Promise(..) => self.process_string(names::OBJECT),
            Operand::Any(_, Some(ref value)) => match value {
                Value::Undefined => self.process_string(names::UNDEFINED),
                Value::Null => self.process_string(names::OBJECT),
                Value::Boolean(_) => self.process_string(names::BOOLEAN),
                Value::Number(_) => self.process_string(names::NUMBER),
                Value::String(_) => self.process_string(names::STRING),
                Value::Closure(_) => self.process_string(names::FUNCTION),
                Value::Object(_) | Value::Promise(_) => self.process_string(names::OBJECT),
                Value::None => unreachable!("{value:?}"),
            },
            Operand::Any(value, None) => {
                let string = self.bridge.create_typeof(value);
                self.operand_stack.push(Operand::String(string, None));
            }
            Operand::Lambda(..)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(..) => unreachable!("{operand:?}"),
        }
    }

    // 13.5.4.1 Runtime Semantics: Evaluation
    fn process_unary_plus(&mut self) {
        let (operand, _) = self.dereference();
        let value = self.to_numeric(operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 13.5.5.1 Runtime Semantics: Evaluation
    fn process_unary_minus(&mut self) {
        let (operand, _) = self.dereference();
        let value = self.to_numeric(operand);
        // TODO: BigInt
        // 6.1.6.1.1 Number::unaryMinus ( x )
        let value = self.bridge.create_fneg(value);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 13.5.6.1 Runtime Semantics: Evaluation
    fn process_bitwise_not(&mut self) {
        let (operand, _) = self.dereference();
        let number = self.to_numeric(operand);
        // TODO: BigInt
        let number = self.bridge.create_bitwise_not(number);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.5.7.1 Runtime Semantics: Evaluation
    fn process_logical_not(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand);
        let boolean = self.bridge.create_logical_not(boolean);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    fn create_to_boolean(&mut self, operand: Operand) -> BooleanIr {
        match operand {
            Operand::Undefined | Operand::Null => self.bridge.get_boolean(false),
            Operand::Boolean(value, ..) => value,
            Operand::Number(value, ..) => self.bridge.create_number_to_boolean(value),
            Operand::String(..) => todo!(),
            Operand::Closure(_) | Operand::Object(_) | Operand::Promise(_) => {
                self.bridge.get_boolean(true)
            }
            Operand::Any(value, ..) => self.bridge.create_to_boolean(value),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    // 13.6.1 Runtime Semantics: Evaluation
    fn process_exponentiation(&mut self) {
        unimplemented!("** operator");
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_multiplication(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.bridge.create_fmul(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_division(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.bridge.create_fdiv(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_remainder(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.bridge.create_frem(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.8.1.1 Runtime Semantics: Evaluation
    fn process_addition(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.bridge.create_fadd(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.8.2.1 Runtime Semantics: Evaluation
    fn process_subtraction(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.bridge.create_fsub(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.1.1 Runtime Semantics: Evaluation
    fn process_left_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.bridge.create_left_shift(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.2.1 Runtime Semantics: Evaluation
    fn process_signed_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.bridge.create_signed_right_shift(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.3.1 Runtime Semantics: Evaluation
    fn process_unsigned_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.bridge.create_unsigned_right_shift(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.bridge.create_less_than(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.bridge.create_greater_than(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than_or_equal(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.bridge.create_less_than_or_equal(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than_or_equal(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.bridge.create_greater_than_or_equal(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_instanceof(&mut self) {
        unimplemented!("instanceof operator");
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_in(&mut self) {
        unimplemented!("in operator");
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_equality(&mut self) {
        logger::debug!(event = "process_equality");

        // TODO: comparing the references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let boolean = self.create_is_loosely_equal(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 7.2.13 IsLooselyEqual ( x, y )
    fn create_is_loosely_equal(&mut self, lhs: Operand, rhs: Operand) -> BooleanIr {
        logger::debug!(event = "create_is_loosely_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs, ..) = lhs {
            // TODO: compile-time evaluation
            let rhs = self.create_to_any(&rhs);
            return self.bridge.create_is_loosely_equal(lhs, rhs);
        }
        if let Operand::Any(rhs, ..) = rhs {
            // TODO: compile-time evaluation
            let lhs = self.create_to_any(&lhs);
            return self.bridge.create_is_loosely_equal(lhs, rhs);
        }

        // 1. If Type(x) is Type(y), then Return IsStrictlyEqual(x, y).
        if std::mem::discriminant(&lhs) == std::mem::discriminant(&rhs) {
            return self.create_is_strictly_equal(lhs, rhs);
        }

        // 2. If x is null and y is undefined, return true.
        if matches!(lhs, Operand::Null) && matches!(rhs, Operand::Undefined) {
            return self.bridge.get_boolean(true);
        }

        // 3. If x is undefined and y is null, return true.
        if matches!(lhs, Operand::Undefined) && matches!(rhs, Operand::Null) {
            return self.bridge.get_boolean(true);
        }

        // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 7. If x is a BigInt and y is a String, then
        // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
        // TODO
        // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: ...
        let lhs = self.create_to_any(&lhs);
        let rhs = self.create_to_any(&rhs);
        self.bridge.create_is_loosely_equal(lhs, rhs)
    }

    fn create_to_any(&mut self, operand: &Operand) -> ValueIr {
        logger::debug!(event = "create_to_any", ?operand);
        match operand {
            Operand::Any(value, ..) => *value,
            Operand::Undefined => self.bridge.create_undefined_to_any(),
            Operand::Null => self.bridge.create_null_to_any(),
            Operand::Boolean(value, ..) => self.bridge.create_boolean_to_any(*value),
            Operand::Number(value, ..) => self.bridge.create_number_to_any(*value),
            Operand::String(value, ..) => self.bridge.create_string_to_any(*value),
            Operand::Closure(value) => self.bridge.create_closure_to_any(*value),
            Operand::Object(value) => self.bridge.create_object_to_any(*value),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_)
            | Operand::Promise(_) => unreachable!("{operand:?}"),
        }
    }

    // 7.2.14 IsStrictlyEqual ( x, y )
    fn create_is_strictly_equal(&mut self, lhs: Operand, rhs: Operand) -> BooleanIr {
        logger::debug!(event = "create_is_strictly_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs, ..) = lhs {
            return self.create_any_is_strictly_equal(lhs, rhs);
        }
        if let Operand::Any(rhs, ..) = rhs {
            return self.create_any_is_strictly_equal(rhs, lhs);
        }
        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return self.bridge.get_boolean(false);
        }
        // TODO: BigInt
        match (lhs, rhs) {
            (Operand::Undefined, Operand::Undefined) => self.bridge.get_boolean(true),
            (Operand::Null, Operand::Null) => self.bridge.get_boolean(true),
            (Operand::Boolean(lhs, ..), Operand::Boolean(rhs, ..)) => {
                self.bridge.create_is_same_boolean(lhs, rhs)
            }
            (Operand::Number(lhs, ..), Operand::Number(rhs, ..)) => {
                self.bridge.create_is_same_number(lhs, rhs)
            }
            (Operand::String(_lhs, ..), Operand::String(_rhs, ..)) => {
                todo!();
            }
            (Operand::Closure(lhs), Operand::Closure(rhs)) => {
                self.bridge.create_is_same_closure(lhs, rhs)
            }
            (Operand::Promise(lhs), Operand::Promise(rhs)) => {
                self.bridge.create_is_same_promise(lhs, rhs)
            }
            (Operand::Object(lhs), Operand::Object(rhs)) => {
                self.bridge.create_is_same_object(lhs, rhs)
            }
            (lhs, rhs) => unreachable!("({lhs:?}, {rhs:?})"),
        }
    }

    fn create_any_is_strictly_equal(&mut self, lhs: ValueIr, rhs: Operand) -> BooleanIr {
        logger::debug!(event = "create_any_is_strictly_equal", ?lhs, ?rhs);
        match rhs {
            Operand::Undefined => self.bridge.create_is_undefined(lhs),
            Operand::Null => self.bridge.create_is_null(lhs),
            Operand::Boolean(rhs, ..) => self.create_is_same_boolean_value(lhs, rhs),
            Operand::Number(rhs, ..) => self.create_is_same_number_value(lhs, rhs),
            Operand::String(_rhs, ..) => todo!(),
            Operand::Closure(rhs) => self.create_is_same_closure_value(lhs, rhs),
            Operand::Object(rhs) => self.create_is_same_object_value(lhs, rhs),
            Operand::Promise(rhs) => self.create_is_same_promise_value(lhs, rhs),
            Operand::Any(rhs, ..) => self.bridge.create_is_strictly_equal(lhs, rhs),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{rhs:?}"),
        }
    }

    fn create_is_same_boolean_value(&mut self, value: ValueIr, boolean: BooleanIr) -> BooleanIr {
        let then_block = self.create_basic_block("is_boolean.then");
        let else_block = self.create_basic_block("is_boolean.else");
        let merge_block = self.create_basic_block("is_boolean");

        // if value.kind == ValueKind::Boolean
        let cond = self.bridge.create_is_boolean(value);
        self.bridge.create_cond_br(cond, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        let then_value = self.bridge.create_is_same_boolean_value(value, boolean);
        self.bridge.create_br(merge_block);
        // } else {
        self.bridge.set_basic_block(else_block);
        let else_value = self.bridge.get_boolean(false);
        self.bridge.create_br(merge_block);
        // }
        self.bridge.set_basic_block(merge_block);
        self.bridge
            .create_boolean_phi(then_value, then_block, else_value, else_block)
    }

    fn create_is_same_number_value(&mut self, value: ValueIr, number: NumberIr) -> BooleanIr {
        logger::debug!(event = "create_is_same_number", ?value, ?number);

        let then_block = self.create_basic_block("is_number.then");
        let else_block = self.create_basic_block("is_number.else");
        let merge_block = self.create_basic_block("is_number");

        // if value.kind == ValueKind::Number
        let cond = self.bridge.create_is_number(value);
        self.bridge.create_cond_br(cond, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        let then_value = self.bridge.create_is_same_number_value(value, number);
        self.bridge.create_br(merge_block);
        // } else {
        self.bridge.set_basic_block(else_block);
        let else_value = self.bridge.get_boolean(false);
        self.bridge.create_br(merge_block);
        // }
        self.bridge.set_basic_block(merge_block);
        self.bridge
            .create_boolean_phi(then_value, then_block, else_value, else_block)
    }

    fn create_is_same_closure_value(&mut self, value: ValueIr, closure: ClosureIr) -> BooleanIr {
        let then_block = self.create_basic_block("is_closure.then");
        let else_block = self.create_basic_block("is_closure.else");
        let merge_block = self.create_basic_block("is_closure");

        // if value.kind == ValueKind::Number
        let cond = self.bridge.create_is_closure(value);
        self.bridge.create_cond_br(cond, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        let then_value = self.bridge.create_is_same_closure_value(value, closure);
        self.bridge.create_br(merge_block);
        // } else {
        self.bridge.set_basic_block(else_block);
        let else_value = self.bridge.get_boolean(false);
        self.bridge.create_br(merge_block);
        // }
        self.bridge.set_basic_block(merge_block);
        self.bridge
            .create_boolean_phi(then_value, then_block, else_value, else_block)
    }

    fn create_is_same_promise_value(&mut self, value: ValueIr, promise: PromiseIr) -> BooleanIr {
        let then_block = self.create_basic_block("is_promise.then");
        let else_block = self.create_basic_block("is_promise.else");
        let merge_block = self.create_basic_block("is_promise");

        // if value.kind == ValueKind::Promise
        let cond = self.bridge.create_is_promise(value);
        self.bridge.create_cond_br(cond, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        let then_value = self.bridge.create_is_same_promise_value(value, promise);
        self.bridge.create_br(merge_block);
        // } else {
        self.bridge.set_basic_block(else_block);
        let else_value = self.bridge.get_boolean(false);
        self.bridge.create_br(merge_block);
        // }
        self.bridge.set_basic_block(merge_block);
        self.bridge
            .create_boolean_phi(then_value, then_block, else_value, else_block)
    }

    fn create_is_same_object_value(&mut self, value: ValueIr, object: ObjectIr) -> BooleanIr {
        let then_block = self.create_basic_block("is_object.then");
        let else_block = self.create_basic_block("is_object.else");
        let merge_block = self.create_basic_block("is_object");

        // if value.kind == ValueKind::Object
        let cond = self.bridge.create_is_object(value);
        self.bridge.create_cond_br(cond, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        let then_value = self.bridge.create_is_same_object_value(value, object);
        self.bridge.create_br(merge_block);
        // } else {
        self.bridge.set_basic_block(else_block);
        let else_value = self.bridge.get_boolean(false);
        self.bridge.create_br(merge_block);
        // }
        self.bridge.set_basic_block(merge_block);
        self.bridge
            .create_boolean_phi(then_value, then_block, else_value, else_block)
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_inequality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let eq = self.create_is_loosely_equal(lhs, rhs);
        let boolean = self.bridge.create_logical_not(eq);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_equality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let boolean = self.create_is_strictly_equal(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_inequality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let eq = self.create_is_strictly_equal(lhs, rhs);
        let boolean = self.bridge.create_logical_not(eq);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_and(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.to_numeric(lval);
        let rnum = self.to_numeric(rval);
        // TODO: BigInt

        let number = self.bridge.create_bitwise_and(lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_xor(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.to_numeric(lval);
        let rnum = self.to_numeric(rval);
        // TODO: BigInt

        let number = self.bridge.create_bitwise_xor(lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_or(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.to_numeric(lval);
        let rnum = self.to_numeric(rval);
        // TODO: BigInt

        let number = self.bridge.create_bitwise_or(lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    fn process_ternary(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();
        let then_block = flow.then_block;
        let else_block = self.bridge.get_basic_block();

        let (else_operand, _) = self.dereference();

        self.bridge.set_basic_block(then_block);
        let (then_operand, _) = self.dereference();

        let block = self.create_basic_block("ternary");

        if std::mem::discriminant(&then_operand) == std::mem::discriminant(&else_operand) {
            self.bridge.set_basic_block(then_block);
            self.bridge.create_br(block);

            self.bridge.set_basic_block(else_block);
            self.bridge.create_br(block);

            self.bridge.set_basic_block(block);

            // In this case, we can use the value of each item as is.
            match (then_operand, else_operand) {
                (Operand::Undefined, Operand::Undefined) => {
                    self.process_undefined();
                    return;
                }
                (Operand::Null, Operand::Null) => {
                    self.process_null();
                    return;
                }
                (Operand::Boolean(then_value, ..), Operand::Boolean(else_value, ..)) => {
                    let boolean = self
                        .bridge
                        .create_boolean_phi(then_value, then_block, else_value, else_block);
                    // TODO(perf): compile-time evaluation
                    self.operand_stack.push(Operand::Boolean(boolean, None));
                    return;
                }
                (Operand::Number(then_value, ..), Operand::Number(else_value, ..)) => {
                    let number = self
                        .bridge
                        .create_number_phi(then_value, then_block, else_value, else_block);
                    // TODO(perf): compile-time evaluation
                    self.operand_stack.push(Operand::Number(number, None));
                    return;
                }
                (Operand::String(_then_value, ..), Operand::String(_else_value, ..)) => {
                    todo!();
                }
                (Operand::Any(then_value, ..), Operand::Any(else_value, ..)) => {
                    let any = self
                        .bridge
                        .create_value_phi(then_value, then_block, else_value, else_block);
                    // TODO(pref): compile-time evaluation
                    self.operand_stack.push(Operand::Any(any, None));
                    return;
                }
                _ => unreachable!(),
            }
        }

        // We have to convert the value before the branch in each block.

        self.bridge.set_basic_block(then_block);
        let then_value = self.create_to_any(&then_operand);
        self.bridge.create_br(block);

        self.bridge.set_basic_block(else_block);
        let else_value = self.create_to_any(&else_operand);
        self.bridge.create_br(block);

        self.bridge.set_basic_block(block);
        let any = self
            .bridge
            .create_value_phi(then_value, then_block, else_value, else_block);
        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(any, None));
    }

    fn pop_boolean(&mut self) -> BooleanIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Boolean(value, ..) => value,
            _ => unreachable!(),
        }
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_assignment(&mut self) {
        let (rhs, _) = self.dereference();

        match self.operand_stack.pop().unwrap() {
            Operand::VariableReference(symbol, Locator::Global) => {
                let object = self.bridge.get_object_nullptr();
                let value = self.create_to_any(&rhs);
                // TODO(feat): ReferenceError, TypeError
                self.bridge
                    .create_set_value_by_symbol(object, symbol, value);
            }
            Operand::VariableReference(symbol, locator) => {
                let value = self.create_get_value_ptr(symbol, locator);
                // TODO: throw a TypeError in the strict mode.
                // auto* flags_ptr = CreateGetFlagsPtr(value_ptr);
                self.create_store_operand_to_value(&rhs, value);
            }
            Operand::PropertyReference(key) => {
                // TODO(refactor): reduce code clone
                self.perform_to_object();
                let object = self.pop_object();
                let value = self.create_to_any(&rhs);
                match key {
                    PropertyKey::Symbol(key) => {
                        self.bridge.create_set_value_by_symbol(object, key, value);
                    }
                    PropertyKey::Number(key) => {
                        self.bridge.create_set_value_by_number(object, key, value);
                    }
                    PropertyKey::Value(key) => {
                        self.bridge.create_set_value_by_value(object, key, value);
                    }
                }
            }
            operand => unreachable!("{operand:?}"),
        }

        self.operand_stack.push(rhs);
    }

    fn process_falsy_short_circuit(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand.clone());
        let boolean = self.bridge.create_logical_not(boolean);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
        self.process_if_then();
        self.operand_stack.push(operand);
        self.process_else();
    }

    fn process_truthy_short_circuit(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand.clone());
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
        self.process_if_then();
        self.operand_stack.push(operand);
        self.process_else();
    }

    fn process_nullish_short_circuit(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_is_non_nullish(operand.clone());
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
        self.process_if_then();
        self.operand_stack.push(operand);
        self.process_else();
    }

    fn create_is_non_nullish(&mut self, operand: Operand) -> BooleanIr {
        match operand {
            Operand::Undefined | Operand::Null => self.bridge.get_boolean(false),
            Operand::Boolean(..)
            | Operand::Number(..)
            | Operand::Closure(_)
            | Operand::Object(_)
            | Operand::Promise(_) => self.bridge.get_boolean(true),
            Operand::String(..) => todo!(),
            Operand::Any(value, ..) => self.bridge.create_is_non_nullish(value),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    fn process_truthy(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    fn process_non_nullish(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_is_non_nullish(operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    fn process_if_then(&mut self) {
        let cond_value = self.pop_boolean();
        let then_block = self.create_basic_block("then");
        let else_block = self.create_basic_block("else");
        self.bridge
            .create_cond_br(cond_value, then_block, else_block);
        self.bridge.set_basic_block(then_block);
        self.control_flow_stack
            .push_if_then_else_flow(then_block, else_block);
    }

    fn process_else(&mut self) {
        let then_block = self.bridge.get_basic_block();
        let else_block = self.control_flow_stack.update_then_block(then_block);
        self.bridge.move_basic_block_after(else_block);
        self.bridge.set_basic_block(else_block);
    }

    fn process_if_else_statement(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();
        let else_block = self.bridge.get_basic_block();

        let mut block = BasicBlock::NONE;

        if self.bridge.is_basic_block_terminated(else_block) {
            // We should not append any instructions after a terminator instruction such as `ret`.
        } else {
            block = self.create_basic_block("merge");
            self.bridge.create_br(block);
        }

        if self.bridge.is_basic_block_terminated(flow.then_block) {
            // We should not append any instructions after a terminator instruction such as `ret`.
        } else {
            if block == BasicBlock::NONE {
                block = self.create_basic_block("merge");
            }
            self.bridge.set_basic_block(flow.then_block);
            self.bridge.create_br(block);
        }

        if block != BasicBlock::NONE {
            self.bridge.set_basic_block(block);
        }
    }

    fn process_if_statement(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();
        let then_block = self.bridge.get_basic_block();

        let block = self.create_basic_block("merge");

        if self.bridge.is_basic_block_terminated(then_block) {
            // We should not append any instructions after a terminator instruction such as `ret`.
        } else {
            self.bridge.create_br(block);
        }

        self.bridge.move_basic_block_after(flow.else_block);
        self.bridge.set_basic_block(flow.else_block);
        self.bridge.create_br(block);

        self.bridge.set_basic_block(block);
    }

    fn process_do_while_loop(&mut self, id: u16) {
        push_bb_name!(self, "do-while", id);

        let loop_body = self.create_basic_block("loop-body");
        let loop_ctrl = self.create_basic_block("loop-ctrl");
        let loop_test = self.create_basic_block("loop-test");
        let loop_exit = self.create_basic_block("loop-exit");

        let loop_start = loop_body;
        let loop_continue = loop_test;
        let loop_break = loop_exit;

        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_exit, loop_exit);
        self.control_flow_stack
            .push_loop_body_flow(loop_ctrl, loop_test);

        self.bridge.create_br(loop_start);

        self.build_loop_ctrl_block(loop_ctrl, loop_continue, loop_break);

        self.bridge.set_basic_block(loop_start);
    }

    fn process_while_loop(&mut self, id: u16) {
        push_bb_name!(self, "while", id);

        let loop_test = self.create_basic_block("loop-test");
        let loop_body = self.create_basic_block("loop-body");
        let loop_ctrl = self.create_basic_block("loop-ctrl");
        let loop_exit = self.create_basic_block("loop-exit");

        let loop_start = loop_test;
        let loop_continue = loop_test;
        let loop_break = loop_exit;

        self.control_flow_stack
            .push_loop_body_flow(loop_ctrl, loop_exit);
        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_exit, loop_body);

        self.bridge.create_br(loop_start);

        self.build_loop_ctrl_block(loop_ctrl, loop_continue, loop_break);

        self.bridge.set_basic_block(loop_start);
    }

    // TODO: rewrite using if and break
    fn process_for_loop(&mut self, id: u16, flags: LoopFlags) {
        push_bb_name!(self, "for", id);

        let has_init = flags.contains(LoopFlags::HAS_INIT);
        let has_test = flags.contains(LoopFlags::HAS_TEST);
        let has_next = flags.contains(LoopFlags::HAS_NEXT);

        let loop_init = if has_init {
            self.create_basic_block("loop-init")
        } else {
            BasicBlock::NONE
        };
        let loop_test = if has_test {
            self.create_basic_block("loop-test")
        } else {
            BasicBlock::NONE
        };
        let loop_body = self.create_basic_block("loop-body");
        let loop_ctrl = self.create_basic_block("loop-ctrl");
        let loop_next = if has_next {
            self.create_basic_block("loop-next")
        } else {
            BasicBlock::NONE
        };
        let loop_exit = self.create_basic_block("loop-exit");

        let mut loop_start = loop_body;
        let mut loop_continue = loop_body;
        let loop_break = loop_exit;
        let mut insert_point = loop_body;

        self.control_flow_stack
            .push_loop_body_flow(loop_ctrl, loop_exit);

        if has_next {
            if has_test {
                self.control_flow_stack
                    .push_loop_next_flow(loop_test, loop_body);
            } else {
                self.control_flow_stack
                    .push_loop_next_flow(loop_body, loop_body);
            }
            loop_continue = loop_next;
            insert_point = loop_next;
        }

        if has_test {
            if has_next {
                self.control_flow_stack
                    .push_loop_test_flow(loop_body, loop_exit, loop_next);
            } else {
                self.control_flow_stack
                    .push_loop_test_flow(loop_body, loop_exit, loop_body);
            }
            loop_start = loop_test;
            if !has_next {
                loop_continue = loop_test;
            }
            insert_point = loop_test;
        }

        if has_init {
            if has_test {
                self.control_flow_stack
                    .push_loop_init_flow(loop_test, loop_test);
            } else if has_next {
                self.control_flow_stack
                    .push_loop_init_flow(loop_body, loop_next);
            } else {
                self.control_flow_stack
                    .push_loop_init_flow(loop_body, loop_body);
            }
            loop_start = loop_init;
            insert_point = loop_init;
        }

        self.bridge.create_br(loop_start);

        self.build_loop_ctrl_block(loop_ctrl, loop_continue, loop_break);

        self.bridge.set_basic_block(insert_point);
    }

    fn build_loop_ctrl_block(
        &mut self,
        loop_ctrl: BasicBlock,
        loop_continue: BasicBlock,
        loop_break: BasicBlock,
    ) {
        let set_normal_block = self.create_basic_block("loop-ctrl.set_normal");
        let break_or_continue_block = self.create_basic_block("loop-ctrl.break_or_continue");

        self.control_flow_stack.push_exit_target(loop_ctrl, true);
        for label in std::mem::take(&mut self.pending_labels).into_iter() {
            self.control_flow_stack.set_exit_label(label);
        }
        let exit_id = self.control_flow_stack.exit_id();

        self.bridge.set_basic_block(loop_ctrl);
        let is_normal_or_continue = self
            .bridge
            .create_is_flow_selector_normal_or_continue(exit_id.depth());
        let is_break_or_continue = self
            .bridge
            .create_is_flow_selector_break_or_continue(exit_id.depth());
        self.bridge.create_cond_br(
            is_break_or_continue,
            set_normal_block,
            break_or_continue_block,
        );

        self.bridge.set_basic_block(set_normal_block);
        self.bridge.create_set_flow_selector_normal();
        self.bridge.create_br(break_or_continue_block);

        self.bridge.set_basic_block(break_or_continue_block);
        self.bridge
            .create_cond_br(is_normal_or_continue, loop_continue, loop_break);
    }

    fn process_loop_init(&mut self) {
        let loop_init = self.control_flow_stack.pop_loop_init_flow();
        self.bridge.create_br(loop_init.branch_block);
        self.bridge.set_basic_block(loop_init.insert_point);
    }

    fn process_loop_test(&mut self) {
        let loop_test = self.control_flow_stack.pop_loop_test_flow();
        let (operand, _) = self.dereference();
        let cond = self.create_to_boolean(operand);
        self.bridge
            .create_cond_br(cond, loop_test.then_block, loop_test.else_block);
        self.bridge.set_basic_block(loop_test.insert_point);
    }

    fn process_loop_next(&mut self) {
        let loop_next = self.control_flow_stack.pop_loop_next_flow();
        // Discard the evaluation result.
        self.process_discard();
        self.bridge.create_br(loop_next.branch_block);
        self.bridge.set_basic_block(loop_next.insert_point);
    }

    fn process_loop_body(&mut self) {
        let loop_body = self.control_flow_stack.pop_loop_body_flow();
        self.bridge.create_br(loop_body.branch_block);
        self.bridge.move_basic_block_after(loop_body.insert_point);
        self.bridge.set_basic_block(loop_body.insert_point);
    }

    fn process_loop_end(&mut self) {
        pop_bb_name!(self);
        self.control_flow_stack.pop_exit_target();
    }

    fn process_case_block(&mut self, id: u16, num_cases: u16) {
        debug_assert!(num_cases > 0);

        push_bb_name!(self, "switch", id);

        let case_block = self.create_basic_block("case");
        let ctrl_block = self.create_basic_block("ctrl");
        let set_normal_block = self.create_basic_block("ctrl.set_normal");
        let end_block = self.create_basic_block("end");

        self.control_flow_stack.push_switch_flow(end_block);
        self.control_flow_stack.push_exit_target(ctrl_block, true);
        debug_assert!(self.pending_labels.is_empty());
        let exit_id = self.control_flow_stack.exit_id();

        self.bridge.create_br(case_block);

        self.bridge.set_basic_block(ctrl_block);
        let is_break = self.bridge.create_is_flow_selector_break(exit_id.depth());
        self.bridge
            .create_cond_br(is_break, set_normal_block, end_block);

        self.bridge.set_basic_block(set_normal_block);
        self.bridge.create_set_flow_selector_normal();
        self.bridge.create_br(end_block);

        self.bridge.set_basic_block(case_block);
    }

    fn process_case(&mut self) {
        let clause_start_block = self.create_basic_block("case.clause");
        let next_case_block = self.create_basic_block("case");
        let cond_value = self.pop_boolean();
        self.bridge
            .create_cond_br(cond_value, clause_start_block, next_case_block);
        self.bridge.set_basic_block(clause_start_block);
        self.control_flow_stack
            .push_case_flow(next_case_block, clause_start_block);
    }

    fn process_default(&mut self) {
        let next_case_block = self.bridge.get_basic_block();
        let clause_start_block = self.create_basic_block("default.clause");
        self.bridge.set_basic_block(clause_start_block);
        self.control_flow_stack
            .push_case_flow(next_case_block, clause_start_block);
        self.control_flow_stack
            .set_default_case_block(clause_start_block)
    }

    fn process_case_clause(&mut self, has_statement: bool) {
        let clause_end_block = self.bridge.get_basic_block();
        let next_case_block = self
            .control_flow_stack
            .update_case_flow(clause_end_block, has_statement);
        self.bridge.set_basic_block(next_case_block);
    }

    fn process_switch(&mut self, _id: u16, num_cases: u16, _default_index: Option<u16>) {
        pop_bb_name!(self);

        let last_case_block = self.bridge.get_basic_block();

        // Connect the last basic blocks of each case/default clause to the first basic block of
        // the statement lists of the next case/default clause if it's not terminated.
        //
        // The last basic blocks has been stored in the control flow stack in reverse order.
        let mut fall_through_block = self.control_flow_stack.switch_flow().end_block;
        debug_assert_ne!(fall_through_block, BasicBlock::NONE);
        for _ in 0..num_cases {
            let flow = self.control_flow_stack.pop_case_flow();
            let terminated = self.bridge.is_basic_block_terminated(flow.clause_end_block);
            if !terminated {
                self.bridge.set_basic_block(flow.clause_end_block);
                self.bridge.create_br(fall_through_block);
                self.bridge.move_basic_block_after(fall_through_block);
            }
            fall_through_block = flow.clause_start_block;
            debug_assert_ne!(fall_through_block, BasicBlock::NONE);
        }

        self.control_flow_stack.pop_exit_target();
        let switch = self.control_flow_stack.pop_switch_flow();

        // Create an unconditional jump to the statement of the default clause if it exists.
        // Otherwise, jump to the end block.
        self.bridge.set_basic_block(last_case_block);
        self.bridge
            .create_br(if switch.default_block != BasicBlock::NONE {
                switch.default_block
            } else {
                switch.end_block
            });

        self.bridge.move_basic_block_after(switch.end_block);
        self.bridge.set_basic_block(switch.end_block);
    }

    fn process_try(&mut self) {
        let try_block = self.create_basic_block("try");
        let catch_block = self.create_basic_block("catch");
        let finally_block = self.create_basic_block("finally");
        let end_block = self.create_basic_block("try-end");

        self.control_flow_stack.push_exception_flow(
            try_block,
            catch_block,
            finally_block,
            end_block,
        );
        self.control_flow_stack.push_exit_target(catch_block, false);

        // Jump from the end of previous block to the beginning of the try block.
        self.bridge.create_br(try_block);

        self.bridge.set_basic_block(try_block);

        push_bb_name!(self, "try");
    }

    fn process_catch(&mut self, nominal: bool) {
        pop_bb_name!(self);

        self.control_flow_stack.set_in_catch(nominal);

        let flow = self.control_flow_stack.exception_flow();
        let finally_block = flow.finally_block;
        let catch_block = flow.catch_block;

        self.control_flow_stack.pop_exit_target();
        self.control_flow_stack
            .push_exit_target(finally_block, false);

        // Jump from the end of the try block to the beginning of the finally block.
        self.bridge.create_br(finally_block);
        self.bridge.move_basic_block_after(catch_block);
        self.bridge.set_basic_block(catch_block);

        if !nominal {
            self.bridge.create_store_normal_status();
            self.bridge.create_set_flow_selector_normal();
        }

        push_bb_name!(self, "catch");
    }

    fn process_finally(&mut self, _nominal: bool) {
        pop_bb_name!(self);

        self.control_flow_stack.set_in_finally();

        let flow = self.control_flow_stack.exception_flow();
        let finally_block = flow.finally_block;

        self.control_flow_stack.pop_exit_target();

        // Jump from the end of the catch block to the beginning of the finally block.
        self.bridge.create_br(finally_block);
        self.bridge.move_basic_block_after(finally_block);
        self.bridge.set_basic_block(finally_block);

        push_bb_name!(self, "finally");
    }

    fn process_try_end(&mut self) {
        pop_bb_name!(self);

        let flow = self.control_flow_stack.pop_exception_flow();
        let parent_exit_block = self.control_flow_stack.exit_block();

        // Jump from the end of the finally block to the beginning of the outer catch block if
        // there is an uncaught exception.  Otherwise, jump to the beginning of the try-end block.
        let is_normal = self.bridge.create_is_flow_selector_normal();
        self.bridge
            .create_cond_br(is_normal, flow.end_block, parent_exit_block);

        self.bridge.move_basic_block_after(flow.end_block);
        self.bridge.set_basic_block(flow.end_block);
    }

    fn process_label_start(&mut self, label: Symbol, is_iteration_statement: bool) {
        debug_assert_ne!(label, Symbol::NONE);

        if is_iteration_statement {
            // Special treatments are needed for iteration statements.
            // See `build_loop_ctrl_block()` for details.
            if is_iteration_statement {
                debug_assert!(!self.pending_labels.contains(&label));
                self.pending_labels.push(label);
            }
        } else {
            let start_block = self.create_basic_block("start");
            let end_block = self.create_basic_block("end");

            self.bridge.create_br(start_block);
            self.bridge.move_basic_block_after(end_block);
            self.bridge.set_basic_block(start_block);

            self.control_flow_stack.push_exit_target(end_block, false);
            self.control_flow_stack.set_exit_label(label);
        }
    }

    fn process_label_end(&mut self, label: Symbol, is_iteration_statement: bool) {
        debug_assert_ne!(label, Symbol::NONE);

        if is_iteration_statement {
            debug_assert!(self.pending_labels.is_empty());
        } else {
            let end_block = self.control_flow_stack.pop_exit_target();
            self.bridge.create_br(end_block);
            self.bridge.move_basic_block_after(end_block);
            self.bridge.set_basic_block(end_block);
        }
    }

    fn process_continue(&mut self, label: Symbol) {
        let exit_id = self.control_flow_stack.exit_id_for_label(label);
        self.bridge
            .create_set_flow_selector_continue(exit_id.depth());

        let block = self.control_flow_stack.exit_block();
        self.bridge.create_br(block);
        self.create_basic_block_for_deadcode();
    }

    fn process_break(&mut self, label: Symbol) {
        let exit_id = self.control_flow_stack.exit_id_for_label(label);
        self.bridge.create_set_flow_selector_break(exit_id.depth());

        let block = self.control_flow_stack.exit_block();
        self.bridge.create_br(block);
        self.create_basic_block_for_deadcode();
    }

    fn process_return(&mut self, n: u32) {
        if n > 0 {
            debug_assert_eq!(n, 1);
            let (operand, _) = self.dereference();
            self.create_store_operand_to_retv(&operand);
        }

        self.bridge.create_store_normal_status();
        self.bridge.create_set_flow_selector_return();

        let next_block = self.control_flow_stack.cleanup_block();

        self.bridge.create_br(next_block);

        self.create_basic_block_for_deadcode();
    }

    fn create_store_operand_to_retv(&mut self, operand: &Operand) {
        match operand {
            Operand::Undefined => self.bridge.create_store_undefined_to_retv(),
            Operand::Null => self.bridge.create_store_null_to_retv(),
            Operand::Boolean(value, ..) => self.bridge.create_store_boolean_to_retv(*value),
            Operand::Number(value, ..) => self.bridge.create_store_number_to_retv(*value),
            Operand::String(..) => todo!(),
            Operand::Closure(value) => self.bridge.create_store_closure_to_retv(*value),
            Operand::Object(value) => self.bridge.create_store_object_to_retv(*value),
            Operand::Promise(value) => self.bridge.create_store_promise_to_retv(*value),
            Operand::Any(value, ..) => self.bridge.create_store_value_to_retv(*value),
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    fn process_throw(&mut self) {
        let (operand, _) = self.dereference();
        self.create_store_operand_to_retv(&operand);
        self.bridge.create_store_exception_status();
        self.bridge.create_set_flow_selector_throw();

        let next_block = self.control_flow_stack.exception_block();

        self.bridge.create_br(next_block);
        self.bridge.move_basic_block_after(next_block);

        self.create_basic_block_for_deadcode();
    }

    fn process_environment(&mut self, num_locals: u16) {
        let flow = self.control_flow_stack.function_flow();
        let backup = self.bridge.get_basic_block();

        // Local variables and captured variables living outer scopes are loaded here from the
        // `Coroutine` data passed via the `env` argument of the coroutine lambda function to be
        // generated by the compiler.
        self.bridge.set_basic_block(flow.init_block);
        self.bridge.create_set_captures_for_coroutine();
        for i in 0..num_locals {
            let local = self.bridge.create_get_local_ptr_from_coroutine(i);
            self.locals.push(local);
        }

        self.bridge.set_basic_block(backup);
    }

    fn process_jump_table(&mut self, num_states: u32) {
        debug_assert!(num_states >= 2);
        let initial_block = self.create_basic_block("co.initial");
        let done_block = self.create_basic_block("co.done");
        let inst = self
            .bridge
            .create_switch_for_coroutine(done_block, num_states);
        self.bridge
            .create_add_state_for_coroutine(inst, 0, initial_block);

        self.bridge.set_basic_block(done_block);
        self.bridge
            .create_unreachable(c"the coroutine has already done");

        self.bridge.set_basic_block(initial_block);

        self.control_flow_stack
            .push_coroutine_flow(inst, done_block, num_states);
    }

    fn process_await(&mut self, next_state: u32) {
        self.resolve_promise();
        self.save_operands_to_scratch_buffer();
        self.bridge.create_set_coroutine_state(next_state);
        self.bridge.create_suspend();

        // resume block
        let block = self.create_basic_block("resume");
        let inst = self.control_flow_stack.coroutine_switch_inst();
        let state = self.control_flow_stack.coroutine_next_state();
        self.bridge
            .create_add_state_for_coroutine(inst, state, block);
        self.bridge.set_basic_block(block);

        self.load_operands_from_scratch_buffer();

        let has_error_block = self.create_basic_block("has_error");
        let result_block = self.create_basic_block("result");

        // if ##error.has_value()
        let error = self.bridge.create_get_argument_value_ptr(2); // ##error
        let has_error = self.bridge.create_has_value(error);
        self.bridge
            .create_cond_br(has_error, has_error_block, result_block);
        {
            // throw ##error;
            self.bridge.set_basic_block(has_error_block);
            // TODO(pref): compile-time evaluation
            self.operand_stack.push(Operand::Any(error, None));
            self.process_throw();
            self.bridge.create_br(result_block);
        }

        self.bridge.set_basic_block(result_block);
        let result = self.bridge.create_get_argument_value_ptr(1); // ##result

        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(result, None));
    }

    fn resolve_promise(&mut self) {
        let promise = self.bridge.create_get_argument_value_ptr(0); // ##promise
        let promise = self.bridge.create_load_promise_from_value(promise);

        let (operand, _) = self.dereference();
        match operand {
            Operand::Undefined => {
                let result = self.bridge.create_undefined_to_any();
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::Null => {
                let result = self.bridge.create_null_to_any();
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::Boolean(value, ..) => {
                let result = self.bridge.create_boolean_to_any(value);
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::Number(value, ..) => {
                let result = self.bridge.create_number_to_any(value);
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::String(value, ..) => {
                let value = self.ensure_heap_string(value);
                let result = self.bridge.create_string_to_any(value);
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::Closure(value) => {
                let result = self.bridge.create_closure_to_any(value);
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::Object(value) => {
                let result = self.bridge.create_object_to_any(value);
                self.bridge.create_emit_promise_resolved(promise, result);
            }
            Operand::Promise(value) => {
                self.bridge.create_await_promise(value, promise);
            }
            Operand::Any(value, ..) => {
                let then_block = self.create_basic_block("is_promise.then");
                let else_block = self.create_basic_block("is_promise.else");
                let block = self.create_basic_block("block");
                // if value.is_promise()
                let is_promise = self.bridge.create_is_promise(value);
                self.bridge
                    .create_cond_br(is_promise, then_block, else_block);
                // {
                self.bridge.set_basic_block(then_block);
                let target = self.bridge.create_load_promise_from_value(value);
                self.bridge.create_await_promise(target, promise);
                self.bridge.create_br(block);
                // } else {
                self.bridge.set_basic_block(else_block);
                self.bridge.create_emit_promise_resolved(promise, value);
                self.bridge.create_br(block);
                // }
                self.bridge.set_basic_block(block);
            }
            Operand::Lambda(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => {
                unreachable!("{operand:?}")
            }
        }
    }

    fn ensure_heap_string(&mut self, value: Char16SeqIr) -> Char16SeqIr {
        let then_block = self.create_basic_block("is_stack_string");
        let else_block = self.create_basic_block("is_not_stack_string");
        let block = self.create_basic_block("merge_block");

        // if value.on_stack()
        let on_stack = self.bridge.create_string_on_stack(value);
        self.bridge.create_cond_br(on_stack, then_block, else_block);
        // {
        self.bridge.set_basic_block(then_block);
        let then_value = self.bridge.create_migrate_string_to_heap(value);
        self.bridge.create_br(block);
        // } else {
        self.bridge.set_basic_block(else_block);
        let else_value = value;
        self.bridge.create_br(block);
        // }
        self.bridge.set_basic_block(block);
        self.bridge
            .create_string_phi(then_value, then_block, else_value, else_block)
    }

    // TODO(perf): Currently, we have to save all values (except for special cases) on the operand
    // stack into the scratch buffer before the execution of the coroutine suspends.  However, we
    // don't need to save some of them.  For example, there may be constant values on the operand
    // stack.  Additionally, there may be values which will be computed to constant values after
    // the LLVM IR compiler performs constant folding in optimization passes.
    //
    // NOTE: It's best to implement a special pass to determine which value on the operand stack
    // has to be saved, but we don't like to tightly depend on LLVM.  Because we plan to replace it
    // with another library written in Rust such as Cranelift in the future.
    fn save_operands_to_scratch_buffer(&mut self) {
        let mut offset = 0u32;
        for operand in self.operand_stack.iter() {
            match operand {
                Operand::Boolean(value, ..) => {
                    self.bridge
                        .create_write_boolean_to_scratch_buffer(offset, *value);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Number(value, ..) => {
                    self.bridge
                        .create_write_number_to_scratch_buffer(offset, *value);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::String(value, ..) => {
                    // TODO(issue#237): GcCellRef
                    self.bridge
                        .create_write_string_to_scratch_buffer(offset, *value);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Closure(value) => {
                    // TODO(issue#237): GcCellRef
                    self.bridge
                        .create_write_closure_to_scratch_buffer(offset, *value);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Object(value) => {
                    // TODO(issue#237): GcCellRef
                    self.bridge
                        .create_write_object_to_scratch_buffer(offset, *value);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Promise(value) => {
                    self.bridge
                        .create_write_promise_to_scratch_buffer(offset, *value);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Any(value, ..) => {
                    self.bridge
                        .create_write_value_to_scratch_buffer(offset, *value);
                    offset += VALUE_SIZE;
                }
                Operand::Undefined
                | Operand::Null
                | Operand::VariableReference(..)
                | Operand::PropertyReference(_) => (),
                Operand::Lambda(_) | Operand::Coroutine(_) => unreachable!("{operand:?}"),
            }
        }

        // TODO: Should return a compile error.
        assert!(offset <= u16::MAX as u32);
        self.max_scratch_buffer_len = self.max_scratch_buffer_len.max(offset);
    }

    fn load_operands_from_scratch_buffer(&mut self) {
        let mut offset = 0u32;
        for operand in self.operand_stack.iter_mut() {
            match operand {
                Operand::Boolean(value, ..) => {
                    *value = self.bridge.create_read_boolean_from_scratch_buffer(offset);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Number(value, ..) => {
                    *value = self.bridge.create_read_number_from_scratch_buffer(offset);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::String(value, ..) => {
                    // TODO(issue#237): GcCellRef
                    *value = self.bridge.create_read_string_from_scratch_buffer(offset);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Closure(value) => {
                    // TODO(issue#237): GcCellRef
                    *value = self.bridge.create_read_closure_from_scratch_buffer(offset);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Object(value) => {
                    // TODO(issue#237): GcCellRef
                    *value = self.bridge.create_read_object_from_scratch_buffer(offset);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Promise(value) => {
                    *value = self.bridge.create_read_promise_from_scratch_buffer(offset);
                    offset += VALUE_HOLDER_SIZE;
                }
                Operand::Any(value, ..) => {
                    *value = self.bridge.create_read_value_from_scratch_buffer(offset);
                    offset += VALUE_SIZE;
                }
                Operand::Undefined
                | Operand::Null
                | Operand::VariableReference(..)
                | Operand::PropertyReference(_) => (),
                Operand::Lambda(_) | Operand::Coroutine(_) => unreachable!("{operand:?}"),
            }
        }
    }

    fn process_resume(&mut self) {
        let promise = self.pop_promise();
        self.bridge.create_resume(promise);
    }

    fn pop_coroutine(&mut self) -> CoroutineIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Coroutine(value) => value,
            _ => unreachable!(),
        }
    }

    fn pop_promise(&mut self) -> PromiseIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Promise(value) => value,
            _ => unreachable!(),
        }
    }

    fn process_discard(&mut self) {
        debug_assert!(!self.operand_stack.is_empty());
        self.operand_stack.pop();
    }

    fn process_swap(&mut self) {
        self.swap();
    }

    fn process_duplicate(&mut self, offset: u8) {
        self.duplicate(offset);
    }

    fn process_dereference(&mut self) {
        let (operand, _) = self.dereference();
        self.operand_stack.push(operand);
    }

    fn process_debugger(&mut self) {
        self.bridge.create_debugger();
    }

    fn create_basic_block(&mut self, name: &str) -> BasicBlock {
        push_bb_name!(self, name);
        let (name, name_len) = bb_name!(self);
        let block = self.bridge.create_basic_block(name, name_len);
        pop_bb_name!(self);
        block
    }

    // FIXME: Handle dead code in the proper way.
    //
    // We insert a **unreachable** basic block for dead code in order to avoid the following
    // validation error: "Terminator found in the middle of a basic block!"
    //
    // IRBuilder accepts inserting instructions after a terminator instruction in a basic block.
    // It's our responsibility to avoid a malformed basic block.  We think that it's not a good
    // direction to check the existence of a terminator instruction in a basic block before
    // insertion in efficiency and maintainability points of view.  Instead, we create an
    // **unreachable** basic block for dead code.  Eventually, this basic block was removed in the
    // optimization passes.
    //
    // At this point, we don't know whether this is a common method or not...
    fn create_basic_block_for_deadcode(&mut self) {
        let block = self.create_basic_block("deadcode");
        self.bridge.set_basic_block(block);
    }
}

struct OperandStack(Vec<Operand>);

impl OperandStack {
    fn new() -> Self {
        Self(vec![])
    }

    fn duplicate(&mut self, index: usize) {
        let dup = self.0[index].clone();
        self.push(dup);
    }
}

impl Default for OperandStack {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for OperandStack {
    type Target = Vec<Operand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OperandStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Dump for OperandStack {
    fn dump(&self, buf: *mut std::ffi::c_char, len: usize) {
        for operand in self.0.iter().rev() {
            operand.dump(buf, len);
        }
    }
}

/// Values pushed on to the operand stack.
// TODO(feat): add variant for BigInt
#[derive(Clone, Debug)]
enum Operand {
    /// Compile-time constant value of `undefined`.
    Undefined,

    /// Compile-time constant value of `null`.
    Null,

    /// Runtime value and optional compile-time constant value of boolean type.
    // TODO(perf): compile-time evaluation
    Boolean(BooleanIr, Option<bool>),

    /// Runtime value and optional compile-time constant value of number type.
    // TODO(perf): compile-time evaluation
    Number(NumberIr, Option<f64>),

    /// Runtime value and optional compile-time constant value of string type.
    // TODO(perf): compile-time evaluation
    String(Char16SeqIr, Option<Char16Seq>),

    /// Runtime value of lambda function type.
    Lambda(LambdaIr),

    /// Runtime value of closure type.
    Closure(ClosureIr),

    /// Runtime value of coroutine type.
    Coroutine(CoroutineIr),

    /// Runtime value of object type.
    Object(ObjectIr),

    /// Runtime value of promise type.
    Promise(PromiseIr),

    /// Runtime value and optional compile-time constant value of any type.
    // TODO(perf): compile-time evaluation
    Any(ValueIr, #[allow(unused)] Option<Value>),

    // Compile-time constant value types.
    VariableReference(Symbol, Locator),
    PropertyReference(PropertyKey),
}

impl Dump for Operand {
    fn dump(&self, buf: *mut std::ffi::c_char, len: usize) {
        macro_rules! ir2cstr {
            ($value:expr) => {
                $value.get_name_or_as_operand(buf, len)
            };
        }

        match self {
            Self::Undefined => eprintln!("Undefined"),
            Self::Null => eprintln!("Null"),
            Self::Boolean(value, ..) => eprintln!("Boolean({:?})", ir2cstr!(value)),
            Self::Number(value, ..) => eprintln!("Number({:?})", ir2cstr!(value)),
            Self::String(value, ..) => eprintln!("String({:?})", ir2cstr!(value)),
            Self::Lambda(value) => eprintln!("Lambda({:?})", ir2cstr!(value)),
            Self::Closure(value) => eprintln!("Closure({:?})", ir2cstr!(value)),
            Self::Coroutine(value) => eprintln!("Coroutine({:?})", ir2cstr!(value)),
            Self::Promise(value) => eprintln!("Promise({:?})", ir2cstr!(value)),
            Self::Object(value) => eprintln!("Object({:?})", ir2cstr!(value)),
            Self::Any(value, ..) => eprintln!("Any({:?})", ir2cstr!(value)),
            Self::VariableReference(symbol, locator) => {
                eprintln!("VariableReference({symbol}, {locator:?})")
            }
            Self::PropertyReference(key) => eprintln!("PropertyReference({key:?})"),
        }
    }
}

#[derive(Clone, Debug)]
enum PropertyKey {
    Symbol(Symbol),
    Number(f64),
    Value(ValueIr),
}

impl From<Symbol> for PropertyKey {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl From<f64> for PropertyKey {
    fn from(value: f64) -> Self {
        if value.is_nan() {
            Symbol::NAN.into()
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                Symbol::INFINITY.into()
            } else {
                Symbol::NEG_INFINITY.into()
            }
        } else if value == 0. {
            Self::Number(0.) // convert `-0.` to `0.`
        } else {
            Self::Number(value)
        }
    }
}

impl From<ValueIr> for PropertyKey {
    fn from(value: ValueIr) -> Self {
        Self::Value(value)
    }
}

struct BasicBlockNameStack {
    buffer: Vec<u8>,
    index_stack: Vec<usize>,
}

impl BasicBlockNameStack {
    fn new() -> Self {
        let mut buffer = Vec::with_capacity(1024);
        write!(&mut buffer, "bb").unwrap();
        Self {
            buffer,
            index_stack: Vec::with_capacity(64),
        }
    }

    fn as_name(&self) -> (*const std::ffi::c_char, usize) {
        (
            self.buffer.as_ptr() as *const std::ffi::c_char,
            self.buffer.len(),
        )
    }

    fn push(&mut self, name: &str) {
        let index = self.buffer.len();
        write!(&mut self.buffer, ".{}", name).unwrap();
        self.index_stack.push(index);
    }

    fn push_with_id<T: std::fmt::Display>(&mut self, name: &str, id: T) {
        let index = self.buffer.len();
        write!(&mut self.buffer, ".{}", name).unwrap();
        write!(&mut self.buffer, ".{}", id).unwrap();
        self.index_stack.push(index);
    }

    fn pop(&mut self) {
        let index = self.index_stack.pop().unwrap();
        self.buffer.truncate(index);
    }
}

impl Default for BasicBlockNameStack {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Dump {
    fn dump(&self, buf: *mut std::ffi::c_char, len: usize);
}
