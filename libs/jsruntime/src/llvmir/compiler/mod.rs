mod control_flow;
mod peer;

use std::ffi::CStr;
use std::io::Write;
use std::ops::Deref;
use std::ops::DerefMut;

use indexmap::IndexMap;

use jsparser::syntax::LoopFlags;
use jsparser::Symbol;

use crate::function::FunctionId;
use crate::function::FunctionRegistry;
use crate::logger;
use crate::semantics::CompileCommand;
use crate::semantics::Locator;
use crate::semantics::ScopeRef;
use crate::semantics::ScopeTree;
use crate::Program;
use crate::Runtime;

use super::bridge;
use super::Module;

use control_flow::ControlFlowStack;
use peer::BasicBlock;
use peer::LambdaIr;

impl<X> Runtime<X> {
    pub fn compile(&mut self, program: &Program, optimize: bool) -> Result<Module, CompileError> {
        logger::debug!(event = "compile");
        // TODO: Deferring the compilation until it's actually called improves the performance.
        // Because the program may contain unused functions.
        let mut compiler = Compiler::new(&self.function_registry, &program.scope_tree);
        compiler.start_compile(self.pref.enable_llvmir_labels);
        compiler.set_data_layout(self.executor.get_data_layout());
        compiler.set_target_triple(self.executor.get_target_triple());
        for func in program.functions.iter() {
            compiler.start_function(func.symbol, func.id);
            for command in func.commands.iter() {
                compiler.process_command(command);
            }
            compiler.end_function(optimize);
        }
        Ok(compiler.end_compile())
    }
}

/// A Compiler targeting LLVM IR.
struct Compiler<'r, 's> {
    /// The pointer to the compiler peer.
    peer: peer::Compiler,

    /// The function registry of the JavaScript program to compile.
    function_registry: &'r FunctionRegistry,

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

    // The following variables must be reset in the end of compilation for each function.
    locals: Vec<bridge::ValueIr>,
    captures: IndexMap<Locator, bridge::ValueIr>,

    dump_buffer: Option<Vec<std::ffi::c_char>>,
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

macro_rules! dump_enabled {
    () => {
        cfg!(debug_assertions)
            && std::env::var_os("BEE_DEBUG_JSRUNTIME_COMPILER_DUMP").is_some()
    };
}

impl<'r, 's> Compiler<'r, 's> {
    pub fn new(function_registry: &'r FunctionRegistry, scope_tree: &'s ScopeTree) -> Self {
        const DUMP_BUFFER_SIZE: usize = 512;
        Self {
            peer: Default::default(),
            function_registry,
            scope_tree,
            operand_stack: Default::default(),
            control_flow_stack: Default::default(),
            basic_block_name_stack: None,
            locals: Default::default(),
            captures: Default::default(),
            dump_buffer: if dump_enabled!() {
                Some(Vec::with_capacity(DUMP_BUFFER_SIZE))
            } else {
                None
            },
        }
    }

    fn start_compile(&mut self, enable_labels: bool) {
        logger::debug!(event = "start_compile", enable_labels);
        if enable_labels {
            self.basic_block_name_stack = Some(Default::default());
        }
        self.peer.start_compile(enable_labels);
    }

    fn end_compile(&self) -> Module {
        logger::debug!(event = "end_compile");
        self.peer.end_compile()
    }

    fn set_data_layout(&self, data_layout: &CStr) {
        logger::debug!(event = "set_data_layout", ?data_layout);
        self.peer.set_data_layout(data_layout);
    }

    fn set_target_triple(&self, triple: &CStr) {
        logger::debug!(event = "set_target_triple", ?triple);
        self.peer.set_target_triple(triple);
    }

    fn start_function(&mut self, symbol: Symbol, func_id: FunctionId) {
        logger::debug!(event = "start_function", ?symbol, ?func_id);

        let native = self.function_registry.get_native(func_id);
        self.peer.start_function(&native.name);

        let locals_block = self.create_basic_block("locals");
        let args_block = self.create_basic_block("args");
        let body_block = self.create_basic_block("body");
        let return_block = self.create_basic_block("return");

        self.control_flow_stack.push_function_flow(
            locals_block,
            args_block,
            body_block,
            return_block,
        );

        self.peer.set_locals_block(locals_block);

        self.peer.set_basic_block(body_block);
        self.peer.create_store_undefined_to_retv();
        self.peer.create_alloc_status();
    }

    fn end_function(&mut self, optimize: bool) {
        logger::debug!(event = "end_function", optimize);
        let flow = self.control_flow_stack.pop_function_flow();

        self.peer.create_br(flow.return_block);
        self.peer.move_basic_block_after(flow.return_block);

        self.peer.set_basic_block(flow.locals_block);
        self.peer.create_br(flow.args_block);
        self.peer.move_basic_block_after(flow.args_block);

        self.peer.set_basic_block(flow.args_block);
        self.peer.create_br(flow.body_block);
        self.peer.move_basic_block_after(flow.body_block);

        self.peer.set_basic_block(flow.return_block);

        self.peer.end_function(optimize);

        self.locals.clear();

        debug_assert!(self.captures.is_empty());
        self.captures.clear();

        debug_assert!(self.control_flow_stack.is_empty());
        self.control_flow_stack.clear();
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
            CompileCommand::Function(func_id) => self.process_function(*func_id),
            CompileCommand::Closure(prologue, num_captures) => {
                self.process_closure(*prologue, *num_captures)
            }
            CompileCommand::Reference(symbol, locator) => self.process_reference(*symbol, *locator),
            CompileCommand::Exception => self.process_exception(),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::MutableBinding => self.process_mutable_binding(),
            CompileCommand::ImmutableBinding => self.process_immutable_binding(),
            CompileCommand::DeclareFunction => self.process_declare_function(),
            CompileCommand::DeclareClosure => self.process_declare_closure(),
            CompileCommand::Arguments(nargs) => self.process_arguments(*nargs),
            CompileCommand::Argument(index) => self.process_argument(*index),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(*scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
            CompileCommand::CaptureVariable(declaration) => {
                self.process_capture_variable(*declaration)
            }
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
            CompileCommand::ExponentiationAssignment => self.process_exponentiation_assignment(),
            CompileCommand::MultiplicationAssignment => self.process_multiplication_assignment(),
            CompileCommand::DivisionAssignment => self.process_division_assignment(),
            CompileCommand::RemainderAssignment => self.process_remainder_assignment(),
            CompileCommand::AdditionAssignment => self.process_addition_assignment(),
            CompileCommand::SubtractionAssignment => self.process_subtraction_assignment(),
            CompileCommand::LeftShiftAssignment => self.process_left_shift_assignment(),
            CompileCommand::SignedRightShiftAssignment => {
                self.process_signed_right_shift_assignment()
            }
            CompileCommand::UnsignedRightShiftAssignment => {
                self.process_unsigned_right_shift_assignment()
            }
            CompileCommand::BitwiseAndAssignment => self.process_bitwise_and_assignment(),
            CompileCommand::BitwiseXorAssignment => self.process_bitwise_xor_assignment(),
            CompileCommand::BitwiseOrAssignment => self.process_bitwise_or_assignment(),
            CompileCommand::Truthy => self.process_truthy(),
            CompileCommand::FalsyShortCircuit => self.process_falsy_short_circuit(),
            CompileCommand::TruthyShortCircuit => self.process_truthy_short_circuit(),
            CompileCommand::NullishShortCircuit => self.process_nullish_short_circuit(),
            CompileCommand::FalsyShortCircuitAssignment => {
                self.process_falsy_short_circuit_assignment()
            }
            CompileCommand::TruthyShortCircuitAssignment => {
                self.process_truthy_short_circuit_assignment()
            }
            CompileCommand::NullishShortCircuitAssignment => {
                self.process_nullish_short_circuit_assignment()
            }
            CompileCommand::Then => self.process_then(),
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
            CompileCommand::CaseClause(has_statement) => self.process_case_clause(*has_statement),
            CompileCommand::DefaultClause(has_statement) => {
                self.process_default_clause(*has_statement)
            }
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
            CompileCommand::Discard => self.process_discard(),
            CompileCommand::Swap => self.process_swap(),
            CompileCommand::PrepareScopeCleanupChecker(stack_size) => {
                self.peer.process_prepare_scope_cleanup_checker(*stack_size)
            }
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
        let boolean = self.peer.get_boolean(value);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    fn process_number(&mut self, value: f64) {
        let number = self.peer.get_number(value);
        self.operand_stack.push(Operand::Number(number));
    }

    fn process_string(&mut self, _value: &[u16]) {
        unimplemented!("string literal");
    }

    fn process_function(&mut self, func_id: FunctionId) {
        let name = if func_id.is_native() {
            &self.function_registry.get_native(func_id).name
        } else {
            &self.function_registry.get_host(func_id).name
        };
        let lambda = self.peer.get_function(func_id, name);
        self.operand_stack.push(Operand::Function(lambda));
    }

    fn pop_lambda(&mut self) -> LambdaIr {
        match self.operand_stack.pop() {
            Some(Operand::Function(lambda)) => lambda,
            _ => unreachable!(),
        }
    }

    fn pop_capture(&mut self) -> bridge::ValueIr {
        match self.operand_stack.pop() {
            Some(Operand::Capture(capture)) => capture,
            _ => unreachable!(),
        }
    }

    fn process_closure(&mut self, prologue: bool, num_captures: u16) {
        debug_assert!(self.operand_stack.len() >= 1 + num_captures as usize);

        let backup = self.peer.get_basic_block();
        if prologue {
            let block = self.control_flow_stack.scope_flow().hoisted_block;
            self.peer.set_basic_block(block);
        }

        let lambda = self.pop_lambda();
        let closure = self.peer.create_call_runtime_create_closure(lambda, num_captures);

        let captures = self.peer.create_load_captures_from_closure(closure);
        for i in 0..num_captures {
            let capture = self.pop_capture();
            self.peer.create_store_capture_ptr_to_captures(capture, captures, i);
        }

        self.operand_stack.push(Operand::Closure(closure));

        if prologue {
            self.peer.set_basic_block(backup);
        }
    }

    fn process_reference(&mut self, symbol: Symbol, locator: Locator) {
        debug_assert!(!matches!(locator, Locator::None));
        self.operand_stack.push(Operand::Reference(symbol, locator));
    }

    fn process_exception(&mut self) {
        // TODO: Should we check status_ at runtime?
        self.operand_stack.push(Operand::Any(self.peer.get_exception()));
    }

    fn process_allocate_locals(&mut self, num_locals: u16) {
        debug_assert!(num_locals > 0);

        for i in 0..num_locals {
            let local = self.peer.create_local_variable(i);
            self.locals.push(local);
        }
    }

    fn process_mutable_binding(&mut self) {
        const FLAGS: u8 = (bridge::VARIABLE_INITIALIZED | bridge::VARIABLE_MUTABLE) as u8;

        let (operand, _) = self.dereference();
        let (symbol, locator) = self.pop_reference();

        let variable = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.peer.create_store_flags_to_variable(FLAGS, variable);
        self.peer.create_store_symbol_to_variable(symbol, variable);
        self.create_store_operand_to_variable(operand, variable);
    }

    fn dereference(&mut self) -> (Operand, Option<(Symbol, Locator)>) {
        logger::debug!(event = "dereference");

        let operand = self.operand_stack.pop().unwrap();
        match operand {
            Operand::Reference(symbol, locator) => {
                let value = self.create_get_value_ptr(locator);
                (Operand::Any(value), Some((symbol, locator)))
            }
            _ => (operand, None),
        }
    }

    fn create_get_value_ptr(&mut self, locator: Locator) -> bridge::ValueIr {
        match locator {
            Locator::Argument(index) => self.peer.create_get_argument_variable_ptr(index),
            Locator::Local(index) => self.locals[index as usize],
            Locator::Capture(index) => self.peer.create_get_capture_variable_ptr(index),
            _ => unreachable!(),
        }
    }

    fn pop_reference(&mut self) -> (Symbol, Locator) {
        match self.operand_stack.pop().unwrap() {
            Operand::Reference(symbol, locator) => (symbol, locator),
            _ => unreachable!(),
        }
    }

    fn create_store_operand_to_variable(&mut self, operand: Operand, variable: bridge::ValueIr) {
        match operand {
            Operand::Undefined => self.peer.create_store_undefined_to_variable(variable),
            Operand::Null => self.peer.create_store_null_to_variable(variable),
            Operand::Boolean(value) => self.peer.create_store_boolean_to_variable(value, variable),
            Operand::Number(value) => self.peer.create_store_number_to_variable(value, variable),
            Operand::Closure(value) => self.peer.create_store_closure_to_variable(value, variable),
            Operand::Any(value) => self.peer.create_store_value_to_variable(value, variable),
            _ => unreachable!(),
        }
    }

    fn process_immutable_binding(&mut self) {
        const FLAGS: u8 = bridge::VARIABLE_INITIALIZED as u8;

        let (operand, _) = self.dereference();
        let (symbol, locator) = self.pop_reference();

        let variable = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.peer.create_store_flags_to_variable(FLAGS, variable);
        self.peer.create_store_symbol_to_variable(symbol, variable);
        self.create_store_operand_to_variable(operand, variable);
    }

    fn process_declare_function(&mut self) {
        const FLAGS: u8 = (bridge::VARIABLE_INITIALIZED | bridge::VARIABLE_MUTABLE) as u8;

        let block = self.control_flow_stack.scope_flow().hoisted_block;

        let backup = self.peer.get_basic_block();
        self.peer.set_basic_block(block);

        let (operand, _) = self.dereference();
        // TODO: operand must hold a lambda.
        let (symbol, locator) = self.pop_reference();

        let variable = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.peer.create_store_flags_to_variable(FLAGS, variable);
        self.peer.create_store_symbol_to_variable(symbol, variable);
        self.create_store_operand_to_variable(operand, variable);

        self.peer.set_basic_block(backup);
    }

    fn process_declare_closure(&mut self) {
        const FLAGS: u8 = (bridge::VARIABLE_INITIALIZED | bridge::VARIABLE_MUTABLE) as u8;

        let block = self.control_flow_stack.scope_flow().hoisted_block;

        let backup = self.peer.get_basic_block();
        self.peer.set_basic_block(block);

        let (operand, _) = self.dereference();
        // TODO: operand must hold a closure.
        let (symbol, locator) = self.pop_reference();

        let variable = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.peer.create_store_flags_to_variable(FLAGS, variable);
        self.peer.create_store_symbol_to_variable(symbol, variable);
        self.create_store_operand_to_variable(operand, variable);

        self.peer.set_basic_block(backup);
    }

    fn process_arguments(&mut self, nargs: u16) {
        if nargs > 0 {
            let argv = self.peer.create_argv(nargs);
            self.operand_stack.push(Operand::Argv(argv));
            self.swap();
        }
    }

    fn swap(&mut self) {
        logger::debug!(event = "swap");
        debug_assert!(self.operand_stack.len() > 1);
        let last_index = self.operand_stack.len() - 1;
        self.operand_stack.swap(last_index - 1, last_index);
    }

    fn process_argument(&mut self, index: u16) {
        let (operand, _) = self.dereference();
        let argv = self.peek_argv();
        let arg = self.peer.create_get_arg_in_argv(argv, index);
        self.create_store_operand_to_variable(operand, arg);
    }

    fn peek_argv(&self) -> bridge::ValueIr {
        match self.operand_stack.last().unwrap() {
            Operand::Argv(value) => *value,
            _ => unreachable!(),
        }
    }

    fn process_call(&mut self, argc: u16) {
        let argv = if argc > 0 {
            self.pop_argv()
        } else {
            self.peer.get_nullptr()
        };

        let (operand, _) = self.dereference();
        let closure = match operand {
            Operand::Closure(closure) => closure, // IIFE
            Operand::Any(value) => self.create_load_closure_from_value_or_throw_type_error(value),
            _ => {
                self.process_number(1.);
                self.process_throw();
                return;
            }
        };

        let retv = self.peer.create_retv();

        let status = self.peer.create_call_on_closure(closure, argc, argv, retv);

        self.create_check_status_for_exception(status, retv);

        // The function may throw an exception.
        self.control_flow_stack.set_thrown();

        self.operand_stack.push(Operand::Any(retv));
    }

    fn pop_argv(&mut self) -> bridge::ValueIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Argv(value) => value,
            _ => unreachable!(),
        }
    }

    fn create_load_closure_from_value_or_throw_type_error(&mut self, value: bridge::ValueIr) -> bridge::ValueIr {
        let closure_ptr = self.peer.create_closure_ptr();

        let then_block = self.create_basic_block("is_closure.then");
        let else_block = self.create_basic_block("is_closure.else");
        let end_block = self.create_basic_block("closure");

        // if value.is_closure()
        let is_closure = self.peer.create_is_closure(value);
        self.peer.create_cond_br(is_closure, then_block, else_block);
        // then
        {
            self.peer.set_basic_block(then_block);
            let closure = self.peer.create_load_closure_from_value(value);
            self.peer.create_store(closure, closure_ptr);
            self.peer.create_br(end_block);
        }
        // else
        {
            self.peer.set_basic_block(else_block);
            // TODO: TypeError
            self.process_number(1.);
            self.process_throw();
            self.peer.create_br(end_block);
        }

        self.peer.set_basic_block(end_block);
        self.peer.create_load_closure(closure_ptr)
    }

    // Handle an exception if it's thrown.
    fn create_check_status_for_exception(&mut self, status: bridge::ValueIr, retv: bridge::ValueIr) {
        let exception_block = self.control_flow_stack.exception_block();

        let then_block = self.create_basic_block("status.exception");
        let else_block = self.create_basic_block("status.normal");

        // if status.is_exception()
        let is_exception = self.peer.create_is_exception_status(status);
        self.peer.create_cond_br(is_exception, then_block, else_block);
        // then
        {
            self.peer.set_basic_block(then_block);
            self.peer.create_store_exception_status();
            self.peer.create_store_value_to_retv(retv);
            self.peer.create_br(exception_block);
        }

        self.peer.set_basic_block(else_block);
    }

    fn process_push_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        push_bb_name!(self, "scope", scope_ref.id());

        let init_block = self.create_basic_block("init");
        let hoisted_block = self.create_basic_block("hoisted");
        let body_block = self.create_basic_block("body");
        let cleanup_block = self.create_basic_block("cleanup");

        self.control_flow_stack.push_scope_flow(
            init_block,
            hoisted_block,
            body_block,
            cleanup_block,
        );

        self.peer.create_br(init_block);
        self.peer.move_basic_block_after(init_block);
        self.peer.set_basic_block(body_block);

        self.peer.start_scope_cleanup_checker(scope_ref);

        let backup = self.peer.get_basic_block();
        self.peer.set_basic_block(init_block);
        let scope = self.scope_tree.scope(scope_ref);
        for binding in scope.bindings.iter() {
            let locator = binding.locator();
            if binding.captured {
                let variable = match locator {
                    Locator::Argument(index) => self.peer.create_get_argument_variable_ptr(index),
                    Locator::Local(index) => self.locals[index as usize],
                    _ => unreachable!(),
                };
                let capture = self.peer.create_capture(variable);
                debug_assert!(!self.captures.contains_key(&locator));
                self.captures.insert(locator, capture);
            }
            if let Locator::Local(index) = locator {
                let variable = self.locals[index as usize];
                self.peer.create_store_flags_to_variable(0, variable);
            }
        }
        self.peer.set_basic_block(backup);
    }

    fn escape_variable(&mut self, locator: Locator) {
        debug_assert!(!locator.is_capture());
        debug_assert!(self.captures.contains_key(&locator));

        let block = self.control_flow_stack.scope_flow().cleanup_block;

        let backup = self.peer.get_basic_block();
        self.peer.set_basic_block(block);

        let capture = self.captures.swap_remove(&locator).unwrap();
        let variable = self.create_get_variable_ptr(locator);
        self.peer.create_escape_variable(capture, variable);

        self.peer.set_basic_block(backup);
    }

    fn create_get_variable_ptr(&mut self, locator: Locator) -> bridge::ValueIr {
        match locator {
            Locator::Argument(i) => self.peer.create_get_argument_variable_ptr(i),
            Locator::Local(i) => self.locals[i as usize],
            Locator::Capture(i) => self.peer.create_get_capture_variable_ptr(i),
            _ => unreachable!(),
        }
    }

    fn process_pop_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        let scope = self.scope_tree.scope(scope_ref);
        for binding in scope.bindings.iter() {
            if binding.captured {
                self.escape_variable(binding.locator());
            }
            if binding.is_local() {
                // tidy local variable
                // TODO: GC
            }
        }

        pop_bb_name!(self);

        let flow = self.control_flow_stack.pop_scope_flow();

        self.peer.create_br(flow.cleanup_block);
        self.peer.move_basic_block_after(flow.cleanup_block);

        self.peer.set_basic_block(flow.init_block);
        self.peer.create_br(flow.hoisted_block);
        self.peer.move_basic_block_after(flow.hoisted_block);

        self.peer.set_basic_block(flow.hoisted_block);
        self.peer.create_br(flow.body_block);
        self.peer.move_basic_block_after(flow.body_block);

        self.peer.set_basic_block(flow.cleanup_block);
        self.peer.end_scope_cleanup_checker(scope_ref);

        let block = self.create_basic_block("block");

        let cleanup_block = if flow.returned {
            self.control_flow_stack.cleanup_block()
        } else {
            BasicBlock::NONE
        };
        let exception_block = if flow.thrown && !self.control_flow_stack.in_finally_block() {
            self.control_flow_stack.exception_block()
        } else {
            BasicBlock::NONE
        };
        self.peer.handle_returned_thrown(
            flow.returned,
            flow.thrown,
            block,
            cleanup_block,
            exception_block,
        );

        self.peer.move_basic_block_after(block);
        self.peer.set_basic_block(block);
    }

    fn process_capture_variable(&mut self, declaration: bool) {
        let backup = self.peer.get_basic_block();
        if declaration {
            let block = self.control_flow_stack.scope_flow().hoisted_block;
            self.peer.set_basic_block(block);
        }

        let (_, locator) = self.pop_reference();
        let capture = match locator {
            Locator::Argument(_) | Locator::Local(_) => {
                debug_assert!(self.captures.contains_key(&locator));
                self.captures.get(&locator).unwrap().clone()
            }
            Locator::Capture(i) => self.peer.create_load_capture(i),
            _ => unreachable!(),
        };

        self.operand_stack.push(Operand::Capture(capture));

        if declaration {
            self.peer.set_basic_block(backup);
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
        let new_value = if op == '+' {
            self.peer.create_incr(old_value)
        } else {
            self.peer.create_decr(old_value)
        };
        match reference {
            Some((symbol, locator)) if symbol != Symbol::NONE => {
                debug_assert!(!locator.is_none());
                self.operand_stack.push(Operand::Reference(symbol, locator));
                self.operand_stack.push(Operand::Number(new_value));
                self.process_assignment();
                self.process_discard();
            }
            _ => {
                // TODO: throw a ReferenceError at runtime
            }
        }
        self.operand_stack.push(Operand::Number(if pos == '^' {
            new_value
        } else {
            old_value
        }));
    }

    // 7.1.4 ToNumber ( argument )
    fn to_numeric(&mut self, operand: Operand) -> bridge::ValueIr {
        match operand {
            Operand::Undefined => self.peer.get_nan(),
            Operand::Null => self.peer.get_zero(),
            Operand::Boolean(value) => self.peer.create_ui_to_fp(value),
            Operand::Number(value) => value,
            Operand::Closure(_) => self.peer.get_nan(),
            Operand::Any(value) => self.peer.to_numeric(value),
            _ => unreachable!(),
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
        // TODO: implement String before this
        unimplemented!("typeof operator");
    }

    // 13.5.4.1 Runtime Semantics: Evaluation
    fn process_unary_plus(&mut self) {
        let (operand, _) = self.dereference();
        let value = self.to_numeric(operand);
        self.operand_stack.push(Operand::Number(value));
    }

    // 13.5.5.1 Runtime Semantics: Evaluation
    fn process_unary_minus(&mut self) {
        let (operand, _) = self.dereference();
        let value = self.to_numeric(operand);
        // TODO: BigInt
        // 6.1.6.1.1 Number::unaryMinus ( x )
        let value = self.peer.create_fneg(value);
        self.operand_stack.push(Operand::Number(value));
    }

    // 13.5.6.1 Runtime Semantics: Evaluation
    fn process_bitwise_not(&mut self) {
        let (operand, _) = self.dereference();
        let number = self.to_numeric(operand);
        // TODO: BigInt
        let number = self.peer.create_bitwise_not(number);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.5.7.1 Runtime Semantics: Evaluation
    fn process_logical_not(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand);
        let boolean = self.peer.create_logical_not(boolean);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    fn create_to_boolean(&mut self, operand: Operand) -> bridge::ValueIr {
        match operand {
            Operand::Undefined | Operand::Null => self.peer.get_boolean(false),
            Operand::Boolean(value) => value,
            Operand::Number(value) => self.peer.create_number_to_boolean(value),
            Operand::Closure(_) => self.peer.get_boolean(true),
            Operand::Any(value) => self.peer.create_to_boolean(value),
            _ => unreachable!(),
        }
    }

    // 13.6.1 Runtime Semantics: Evaluation
    fn process_exponentiation(&mut self) {
        unimplemented!("** operator");
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_multiplication(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.peer.create_fmul(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_division(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.peer.create_fdiv(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_remainder(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.peer.create_frem(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.8.1.1 Runtime Semantics: Evaluation
    fn process_addition(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.peer.create_fadd(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.8.2.1 Runtime Semantics: Evaluation
    fn process_subtraction(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let number = self.peer.create_fsub(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.9.1.1 Runtime Semantics: Evaluation
    fn process_left_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.peer.create_left_shift(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.9.2.1 Runtime Semantics: Evaluation
    fn process_signed_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.peer.create_signed_right_shift(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.9.3.1 Runtime Semantics: Evaluation
    fn process_unsigned_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.peer.create_unsigned_right_shift(lhs, rhs);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.peer.create_less_than(lhs, rhs);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.peer.create_greater_than(lhs, rhs);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than_or_equal(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.peer.create_less_than_or_equal(lhs, rhs);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than_or_equal(&mut self) {
        self.swap();

        let (lhs, _) = self.dereference();
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

        let boolean = self.peer.create_greater_than_or_equal(lhs, rhs);
        self.operand_stack.push(Operand::Boolean(boolean));
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

        self.swap();

        // TODO: comparing the references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let boolean = self.create_is_loosely_equal(lhs, rhs);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 7.2.13 IsLooselyEqual ( x, y )
    fn create_is_loosely_equal(&mut self, lhs: Operand, rhs: Operand) -> bridge::ValueIr {
        logger::debug!(event = "create_is_loosely_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs) = lhs {
            // TODO: compile-time evaluation
            let rhs = self.create_to_any(rhs);
            return self.peer.create_is_loosely_equal(lhs, rhs);
        }
        if let Operand::Any(rhs) = rhs {
            // TODO: compile-time evaluation
            let lhs = self.create_to_any(lhs);
            return self.peer.create_is_loosely_equal(lhs, rhs);
        }

        // 1. If Type(x) is Type(y), then Return IsStrictlyEqual(x, y).
        if std::mem::discriminant(&lhs) == std::mem::discriminant(&rhs) {
            return self.create_is_strictly_equal(lhs, rhs);
        }

        // 2. If x is null and y is undefined, return true.
        if matches!(lhs, Operand::Null) && matches!(rhs, Operand::Undefined) {
            return self.peer.get_boolean(true);
        }

        // 3. If x is undefined and y is null, return true.
        if matches!(lhs, Operand::Undefined) && matches!(rhs, Operand::Null) {
            return self.peer.get_boolean(true);
        }

        // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 7. If x is a BigInt and y is a String, then
        // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
        // TODO
        // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: ...
        let lhs = self.create_to_any(lhs);
        let rhs = self.create_to_any(rhs);
        self.peer.create_is_loosely_equal(lhs, rhs)
    }

    fn create_to_any(&mut self, operand: Operand) -> bridge::ValueIr {
        logger::debug!(event = "create_to_any", ?operand);
        match operand {
            Operand::Any(value) => value,
            Operand::Undefined => self.peer.create_undefined_to_any(),
            Operand::Null => self.peer.create_null_to_any(),
            Operand::Boolean(value) => self.peer.create_boolean_to_any(value),
            Operand::Number(value) => self.peer.create_number_to_any(value),
            Operand::Closure(value) => self.peer.create_closure_to_any(value),
            _ => unreachable!(),
        }
    }

    // 7.2.14 IsStrictlyEqual ( x, y )
    fn create_is_strictly_equal(&mut self, lhs: Operand, rhs: Operand) -> bridge::ValueIr {
        logger::debug!(event = "create_is_strictly_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs) = lhs {
            return self.create_any_is_strictly_equal(lhs, rhs);
        }
        if let Operand::Any(rhs) = rhs {
            return self.create_any_is_strictly_equal(rhs, lhs);
        }
        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return self.peer.get_boolean(false);
        }
        // TODO: BigInt
        match (lhs, rhs) {
            (Operand::Undefined, Operand::Undefined) => self.peer.get_boolean(true),
            (Operand::Null, Operand::Null) => self.peer.get_boolean(true),
            (Operand::Boolean(lhs), Operand::Boolean(rhs)) => self.peer.create_is_same_boolean(lhs, rhs),
            (Operand::Number(lhs), Operand::Number(rhs)) => self.peer.create_is_same_number(lhs, rhs),
            (Operand::Closure(lhs), Operand::Closure(rhs)) => self.peer.create_is_same_closure(lhs, rhs),
            _ => unreachable!(),
        }
    }

    fn create_any_is_strictly_equal(&mut self, lhs: bridge::ValueIr, rhs: Operand) -> bridge::ValueIr {
        logger::debug!(event = "create_any_is_strictly_equal", ?lhs, ?rhs);
        match rhs {
            Operand::Undefined => self.peer.create_is_undefined(lhs),
            Operand::Null => self.peer.create_is_null(lhs),
            Operand::Boolean(rhs) => self.create_is_same_boolean_value(lhs, rhs),
            Operand::Number(rhs) => self.create_is_same_number_value(lhs, rhs),
            Operand::Closure(rhs) => self.create_is_same_closure_value(lhs, rhs),
            Operand::Any(rhs) => self.peer.create_is_strictly_equal(lhs, rhs),
            _ => unreachable!(),
        }
    }

    fn create_is_same_boolean_value(&mut self, value: bridge::ValueIr, boolean: bridge::ValueIr) -> bridge::ValueIr {
        let then_block = self.create_basic_block("is_boolean.then");
        let else_block = self.create_basic_block("is_boolean.else");
        let merge_block = self.create_basic_block("is_boolean");

        // if value.kind == ValueKind::Boolean
        let cond = self.peer.create_is_boolean(value);
        self.peer.create_cond_br(cond, then_block, else_block);
        // {
        self.peer.set_basic_block(then_block);
        let then_value = self.peer.create_is_same_boolean_value(value, boolean);
        self.peer.create_br(merge_block);
        // } else {
        self.peer.set_basic_block(else_block);
        let else_value = self.peer.get_boolean(false);
        self.peer.create_br(merge_block);
        // }
        self.peer.set_basic_block(merge_block);
        self.peer.create_boolean_ternary(then_value, then_block, else_value, else_block)
    }

    fn create_is_same_number_value(&mut self, value: bridge::ValueIr, number: bridge::ValueIr) -> bridge::ValueIr {
        logger::debug!(event = "create_is_same_number", ?value, ?number);

        let then_block = self.create_basic_block("is_number.then");
        let else_block = self.create_basic_block("is_number.else");
        let merge_block = self.create_basic_block("is_number");

        // if value.kind == ValueKind::Number
        let cond = self.peer.create_is_number(value);
        self.peer.create_cond_br(cond, then_block, else_block);
        // {
        self.peer.set_basic_block(then_block);
        let then_value = self.peer.create_is_same_number_value(value, number);
        self.peer.create_br(merge_block);
        // } else {
        self.peer.set_basic_block(else_block);
        let else_value = self.peer.get_boolean(false);
        self.peer.create_br(merge_block);
        // }
        self.peer.set_basic_block(merge_block);
        self.peer.create_boolean_ternary(then_value, then_block, else_value, else_block)
    }

    fn create_is_same_closure_value(&mut self, value: bridge::ValueIr, closure: bridge::ValueIr) -> bridge::ValueIr {
        let then_block = self.create_basic_block("is_closure.then");
        let else_block = self.create_basic_block("is_closure.else");
        let merge_block = self.create_basic_block("is_closure");

        // if value.kind == ValueKind::Number
        let cond = self.peer.create_is_closure(value);
        self.peer.create_cond_br(cond, then_block, else_block);
        // {
        self.peer.set_basic_block(then_block);
        let then_value = self.peer.create_is_same_closure_value(value, closure);
        self.peer.create_br(merge_block);
        // } else {
        self.peer.set_basic_block(else_block);
        let else_value = self.peer.get_boolean(false);
        self.peer.create_br(merge_block);
        // }
        self.peer.set_basic_block(merge_block);
        self.peer.create_boolean_ternary(then_value, then_block, else_value, else_block)
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_inequality(&mut self) {
        self.swap();

        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let eq = self.create_is_loosely_equal(lhs, rhs);
        let boolean = self.peer.create_logical_not(eq);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_equality(&mut self) {
        self.swap();

        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let boolean = self.create_is_strictly_equal(lhs, rhs);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_inequality(&mut self) {
        self.swap();

        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let eq = self.create_is_strictly_equal(lhs, rhs);
        let boolean = self.peer.create_logical_not(eq);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_and(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        self.swap();

        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.to_numeric(lval);
        let rnum = self.to_numeric(rval);
        // TODO: BigInt

        let number = self.peer.create_bitwise_and(lnum, rnum);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_xor(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        self.swap();

        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.to_numeric(lval);
        let rnum = self.to_numeric(rval);
        // TODO: BigInt

        let number = self.peer.create_bitwise_xor(lnum, rnum);
        self.operand_stack.push(Operand::Number(number));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_or(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        self.swap();

        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.to_numeric(lval);
        let rnum = self.to_numeric(rval);
        // TODO: BigInt

        let number = self.peer.create_bitwise_or(lnum, rnum);
        self.operand_stack.push(Operand::Number(number));
    }

    fn process_ternary(&mut self) {
        let else_branch = self.control_flow_stack.pop_branch_flow();
        let then_branch = self.control_flow_stack.pop_branch_flow();

        let test_block = then_branch.before_block;
        let then_head_block = then_branch.after_block;
        let then_tail_block = else_branch.before_block;
        let else_head_block = else_branch.after_block;
        let else_tail_block = self.peer.get_basic_block();

        let (else_operand, _) = self.dereference();

        self.peer.set_basic_block(then_tail_block);
        let (then_operand, _) = self.dereference();

        self.peer.set_basic_block(test_block);
        let cond_value = self.pop_boolean();
        self.peer.create_cond_br(cond_value, then_head_block, else_head_block);

        let block = self.create_basic_block("ternary");

        if std::mem::discriminant(&then_operand) == std::mem::discriminant(&else_operand) {
            self.peer.set_basic_block(then_tail_block);
            self.peer.create_br(block);

            self.peer.set_basic_block(else_tail_block);
            self.peer.create_br(block);

            self.peer.set_basic_block(block);

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
                (Operand::Boolean(then_value), Operand::Boolean(else_value)) => {
                    let boolean = self.peer.create_boolean_ternary(then_value, then_tail_block, else_value, else_tail_block);
                    self.operand_stack.push(Operand::Boolean(boolean));
                    return;
                }
                (Operand::Number(then_value), Operand::Number(else_value)) => {
                    let number = self.peer.create_number_ternary(then_value, then_tail_block, else_value, else_tail_block);
                    self.operand_stack.push(Operand::Number(number));
                    return;
                }
                (Operand::Any(then_value), Operand::Any(else_value)) => {
                    let any = self.peer.create_any_ternary(then_value, then_tail_block, else_value, else_tail_block);
                    self.operand_stack.push(Operand::Any(any));
                    return;
                }
                _ => unreachable!(),
            }
        }

        // We have to convert the value before the branch in each block.

        self.peer.set_basic_block(then_tail_block);
        let then_value = self.create_to_any(then_operand);
        self.peer.create_br(block);

        self.peer.set_basic_block(else_tail_block);
        let else_value = self.create_to_any(else_operand);
        self.peer.create_br(block);

        self.peer.set_basic_block(block);
        let any = self.peer.create_any_ternary(then_value, then_tail_block, else_value, else_tail_block);
        self.operand_stack.push(Operand::Any(any));
    }

    fn pop_boolean(&mut self) -> bridge::ValueIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Boolean(value) => value,
            _ => unreachable!(),
        }
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_assignment(&mut self) {
        let (rhs, _) = self.dereference();
        let (_, locator) = self.pop_reference();

        let variable = self.create_get_variable_ptr(locator);
        // TODO: check the mutable flag
        // auto* flags_ptr = CreateGetFlagsPtr(variable_ptr);

        self.create_store_operand_to_variable(rhs.clone(), variable);

        self.operand_stack.push(rhs);
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_exponentiation_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_exponentiation();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_multiplication_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_multiplication();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_division_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_division();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_remainder_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_remainder();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_addition_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_addition();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_subtraction_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_subtraction();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_left_shift_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_left_shift();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_signed_right_shift_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_signed_right_shift();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_unsigned_right_shift_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_unsigned_right_shift();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_bitwise_and_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_bitwise_and();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_bitwise_xor_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_bitwise_xor();
        self.process_assignment();
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_bitwise_or_assignment(&mut self) {
        let rhs = self.operand_stack.pop().unwrap();
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        self.operand_stack.push(rhs);
        self.process_bitwise_or();
        self.process_assignment();
    }

    fn process_truthy(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand);
        self.operand_stack.push(Operand::Boolean(boolean));
    }

    fn process_falsy_short_circuit(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand.clone());
        let boolean = self.peer.create_logical_not(boolean);
        self.operand_stack.push(Operand::Boolean(boolean));
        self.branch(); // then
        self.operand_stack.push(operand);
        self.branch(); // else
    }

    fn process_truthy_short_circuit(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand.clone());
        self.operand_stack.push(Operand::Boolean(boolean));
        self.branch(); // then
        self.operand_stack.push(operand);
        self.branch(); // else
    }

    fn process_nullish_short_circuit(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.create_is_non_nullish(operand.clone());
        self.operand_stack.push(Operand::Boolean(boolean));
        self.branch(); // then
        self.operand_stack.push(operand);
        self.branch(); // else
    }

    fn create_is_non_nullish(&mut self, operand: Operand) -> bridge::ValueIr {
        match operand {
            Operand::Undefined | Operand::Null => self.peer.get_boolean(false),
            Operand::Boolean(_) | Operand::Number(_) | Operand::Closure(_) => self.peer.get_boolean(true),
            Operand::Any(value) => self.peer.create_is_non_nullish(value),
            _ => unreachable!(),
        }
    }

    fn process_falsy_short_circuit_assignment(&mut self) {
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand.clone());
        let boolean = self.peer.create_logical_not(boolean);
        self.operand_stack.push(Operand::Boolean(boolean));
        self.branch(); // then
        self.operand_stack.push(operand);
        self.branch(); // else
    }

    fn process_truthy_short_circuit_assignment(&mut self) {
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        let (operand, _) = self.dereference();
        let boolean = self.create_to_boolean(operand.clone());
        self.operand_stack.push(Operand::Boolean(boolean));
        self.branch(); // then
        self.operand_stack.push(operand);
        self.branch(); // else
    }

    fn process_nullish_short_circuit_assignment(&mut self) {
        debug_assert!(matches!(self.operand_stack.last(), Some(Operand::Reference(..))));
        self.operand_stack.duplicate();
        let (operand, _) = self.dereference();
        let boolean = self.create_is_non_nullish(operand.clone());
        self.operand_stack.push(Operand::Boolean(boolean));
        self.branch(); // then
        self.operand_stack.push(operand);
        self.branch(); // else
    }

    fn process_then(&mut self) {
        self.branch();
    }

    fn process_else(&mut self) {
        self.branch();
    }

    fn process_if_else_statement(&mut self) {
        let else_branch = self.control_flow_stack.pop_branch_flow();
        let then_branch = self.control_flow_stack.pop_branch_flow();

        let test_block = then_branch.before_block;
        let then_head_block = then_branch.after_block;
        let then_tail_block = else_branch.before_block;
        let else_head_block = else_branch.after_block;
        let else_tail_block = self.peer.get_basic_block();

        let mut block = BasicBlock::NONE;

        if self.peer.is_basic_block_terminated(else_tail_block) {
            // We should not append any instructions after a terminator instruction such as `ret`.
        } else {
            block = self.create_basic_block("block");
            self.peer.create_br(block);
        }

        if self.peer.is_basic_block_terminated(then_tail_block) {
            // We should not append any instructions after a terminator instruction such as `ret`.
        } else {
            if block == BasicBlock::NONE {
                block = self.create_basic_block("block");
            }
            self.peer.set_basic_block(then_tail_block);
            self.peer.create_br(block);
        }

        let cond_value = self.pop_boolean();

        self.peer.set_basic_block(test_block);
        self.peer.create_cond_br(cond_value, then_head_block, else_head_block);

        if block != BasicBlock::NONE {
            self.peer.set_basic_block(block);
        }
    }

    fn process_if_statement(&mut self) {
        let branch = self.control_flow_stack.pop_branch_flow();

        let test_block = branch.before_block;
        let then_head_block = branch.after_block;
        let then_tail_block = self.peer.get_basic_block();
        let block = self.create_basic_block("block");

        if self.peer.is_basic_block_terminated(then_tail_block) {
            // We should not append any instructions after a terminator instruction such as `ret`.
        } else {
            self.peer.create_br(block);
        }

        let cond_value = self.pop_boolean();

        self.peer.set_basic_block(test_block);
        self.peer.create_cond_br(cond_value, then_head_block, block);

        self.peer.set_basic_block(block);
    }

    fn process_do_while_loop(&mut self, id: u16) {
        push_bb_name!(self, "do-while", id);

        let loop_body = self.create_basic_block("loop-body");
        let loop_test = self.create_basic_block("loop-test");
        let loop_end = self.create_basic_block("loop-end");

        let loop_start = loop_body;
        let loop_continue = loop_test;
        let loop_break = loop_end;

        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_end, loop_end);
        self.control_flow_stack
            .push_loop_body_flow(loop_test, loop_test);

        self.control_flow_stack.set_continue_target(loop_continue);
        self.control_flow_stack
            .push_break_target(loop_break, Symbol::NONE);
        self.control_flow_stack
            .push_continue_target(loop_continue, Symbol::NONE);

        self.peer.create_br(loop_start);
        self.peer.set_basic_block(loop_start);
    }

    fn process_while_loop(&mut self, id: u16) {
        push_bb_name!(self, "while", id);

        let loop_test = self.create_basic_block("loop-test");
        let loop_body = self.create_basic_block("loop-body");
        let loop_end = self.create_basic_block("loop-end");

        let loop_start = loop_test;
        let loop_continue = loop_test;
        let loop_break = loop_end;

        self.control_flow_stack
            .push_loop_body_flow(loop_test, loop_end);
        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_end, loop_body);

        self.control_flow_stack.set_continue_target(loop_continue);
        self.control_flow_stack
            .push_break_target(loop_break, Symbol::NONE);
        self.control_flow_stack
            .push_continue_target(loop_continue, Symbol::NONE);

        self.peer.create_br(loop_start);
        self.peer.set_basic_block(loop_start);
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
        let loop_next = if has_next {
            self.create_basic_block("loop-next")
        } else {
            BasicBlock::NONE
        };
        let loop_end = self.create_basic_block("loop-end");

        let mut loop_start = loop_body;
        let mut loop_continue = loop_body;
        let loop_break = loop_end;
        let mut insert_point = loop_body;

        if has_next {
            self.control_flow_stack
                .push_loop_body_flow(loop_next, loop_end);
        } else if has_test {
            self.control_flow_stack
                .push_loop_body_flow(loop_test, loop_end);
        } else {
            self.control_flow_stack
                .push_loop_body_flow(loop_body, loop_end);
        }

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
                    .push_loop_test_flow(loop_body, loop_end, loop_next);
            } else {
                self.control_flow_stack
                    .push_loop_test_flow(loop_body, loop_end, loop_body);
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

        self.control_flow_stack.set_continue_target(loop_continue);
        self.control_flow_stack
            .push_break_target(loop_break, Symbol::NONE);
        self.control_flow_stack
            .push_continue_target(loop_continue, Symbol::NONE);

        self.peer.create_br(loop_start);
        self.peer.set_basic_block(insert_point);
    }

    fn process_loop_init(&mut self) {
        let loop_init = self.control_flow_stack.pop_loop_init_flow();
        self.peer.create_br(loop_init.branch_block);
        self.peer.set_basic_block(loop_init.insert_point);
    }

    fn process_loop_test(&mut self) {
        let loop_test = self.control_flow_stack.pop_loop_test_flow();
        let (operand, _) = self.dereference();
        let cond = self.create_to_boolean(operand);
        self.peer.create_cond_br(cond, loop_test.then_block, loop_test.else_block);
        self.peer.set_basic_block(loop_test.insert_point);
    }

    fn process_loop_next(&mut self) {
        let loop_next = self.control_flow_stack.pop_loop_next_flow();
        // Discard the evaluation result.
        self.process_discard();
        self.peer.create_br(loop_next.branch_block);
        self.peer.set_basic_block(loop_next.insert_point);
    }

    fn process_loop_body(&mut self) {
        let loop_body = self.control_flow_stack.pop_loop_body_flow();
        self.peer.create_br(loop_body.branch_block);
        self.peer.move_basic_block_after(loop_body.insert_point);
        self.peer.set_basic_block(loop_body.insert_point);
    }

    fn process_loop_end(&mut self) {
        pop_bb_name!(self);
        self.control_flow_stack.pop_break_target();
        self.control_flow_stack.pop_continue_target();
    }

    fn process_case_block(&mut self, id: u16, num_cases: u16) {
        debug_assert!(num_cases > 0);

        push_bb_name!(self, "switch", id);

        let (operand, _) = self.dereference();
        // TODO: item.SetLabel("switch-value");
        self.operand_stack.push(operand);
        self.operand_stack.duplicate(); // Dup for test on CaseClause

        let start_block = self.create_basic_block("start");
        self.peer.create_br(start_block);
        self.peer.set_basic_block(start_block);

        let end_block = self.create_basic_block("end");
        self.control_flow_stack.push_switch_flow(end_block);
        self.control_flow_stack
            .push_break_target(end_block, Symbol::NONE);
    }

    fn process_case_clause(&mut self, _has_statement: bool) {
        let branch = self.control_flow_stack.pop_branch_flow();

        let test_block = branch.before_block;
        let then_block = branch.after_block;
        let else_block = self.create_basic_block("else");
        let end_block = self.peer.get_basic_block();

        let cond = self.pop_boolean();

        self.peer.set_basic_block(test_block);
        self.peer.create_cond_br(cond, then_block, else_block);
        self.peer.set_basic_block(else_block);

        self.operand_stack.duplicate();

        self.control_flow_stack.push_case_banch_flow(end_block, then_block);
    }

    fn process_default_clause(&mut self, _has_statement: bool) {
        let branch = self.control_flow_stack.pop_branch_flow();

        let test_block = branch.before_block;
        let then_block = branch.after_block;
        let end_block = self.peer.get_basic_block();

        self.peer.set_basic_block(test_block);

        self.operand_stack.duplicate();

        self.control_flow_stack.push_case_banch_flow(end_block, then_block);
        self.control_flow_stack.set_default_case_block(then_block);
    }

    fn process_switch(&mut self, _id: u16, num_cases: u16, _default_index: Option<u16>) {
        pop_bb_name!(self);

        self.control_flow_stack.pop_break_target();
        let case_block = self.peer.get_basic_block();

        // Discard the switch-values
        self.process_discard();
        self.process_discard();

        // Connect the last basic blocks of each case/default clause to the first basic block of
        // the statement lists of the next case/default clause if it's not terminated.
        //
        // The last basic blocks has been stored in the control flow stack in reverse order.
        let mut fall_through_block = self.control_flow_stack.switch_flow().end_block;
        for _ in 0..num_cases {
            let case_branch = self.control_flow_stack.pop_case_branch_flow();
            let terminated = self.peer.is_basic_block_terminated(case_branch.before_block);
            if !terminated {
                self.peer.set_basic_block(case_branch.before_block);
                self.peer.create_br(fall_through_block);
                self.peer.move_basic_block_after(fall_through_block);
            }
            fall_through_block = case_branch.after_block;
        }

        let switch = self.control_flow_stack.pop_switch_flow();

        // Create an unconditional jump to the statement of the default clause if it exists.
        // Otherwise, jump to the end block.
        self.peer.set_basic_block(case_block);
        self.peer.create_br(if switch.default_block != BasicBlock::NONE {
            switch.default_block
        } else {
            switch.end_block
        });

        self.peer.move_basic_block_after(switch.end_block);
        self.peer.set_basic_block(switch.end_block);
    }

    fn branch(&mut self) {
        let before_block = self.peer.get_basic_block();

        // Push a newly created block.
        // This will be used in ConditionalExpression() in order to build a branch instruction.
        let after_block = self.create_basic_block("block");
        self.peer.set_basic_block(after_block);

        self.control_flow_stack.push_branch_flow(before_block, after_block);
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

        // Jump from the end of previous block to the beginning of the try block.
        self.peer.create_br(try_block);
        self.peer.set_basic_block(try_block);

        push_bb_name!(self, "try");
    }

    fn process_catch(&mut self, nominal: bool) {
        pop_bb_name!(self);

        self.control_flow_stack.set_in_catch(nominal);

        let flow = self.control_flow_stack.exception_flow();
        let finally_block = flow.finally_block;
        let catch_block = flow.catch_block;

        // Jump from the end of the try block to the beginning of the finally block.
        self.peer.create_br(finally_block);
        self.peer.move_basic_block_after(catch_block);
        self.peer.set_basic_block(catch_block);

        if !nominal {
            self.peer.create_store_normal_status();
        }

        push_bb_name!(self, "catch");
    }

    fn process_finally(&mut self, _nominal: bool) {
        pop_bb_name!(self);

        self.control_flow_stack.set_in_finally();

        let flow = self.control_flow_stack.exception_flow();
        let finally_block = flow.finally_block;

        // Jump from the end of the catch block to the beginning of the finally block.
        self.peer.create_br(finally_block);
        self.peer.move_basic_block_after(finally_block);
        self.peer.set_basic_block(finally_block);

        push_bb_name!(self, "finally");
    }

    fn process_try_end(&mut self) {
        pop_bb_name!(self);

        let flow = self.control_flow_stack.pop_exception_flow();
        let exception_block = self.control_flow_stack.exception_block();

        // Jump from the end of the finally block to the beginning of the outer catch block if
        // there is an uncaught exception.  Otherwise, jump to the beginning of the try-end block.
        let cond = self.peer.create_has_uncaught_exception();
        self.peer.create_cond_br(cond, exception_block, flow.end_block);

        self.peer.move_basic_block_after(flow.end_block);
        self.peer.set_basic_block(flow.end_block);
    }

    fn process_label_start(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        debug_assert_ne!(symbol, Symbol::NONE);

        let start_block = self.create_basic_block("start");
        let end_block = self.create_basic_block("end");

        self.peer.create_br(start_block);
        self.peer.move_basic_block_after(end_block);
        self.peer.set_basic_block(start_block);

        self.control_flow_stack.push_break_target(end_block, symbol);

        if is_iteration_statement {
            // The `block` member variable will be updated in the method to handle the loop start
            // of the labeled iteration statement.
            self.control_flow_stack.push_continue_target(BasicBlock::NONE, symbol);
        }
    }

    fn process_label_end(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        debug_assert_ne!(symbol, Symbol::NONE);

        if is_iteration_statement {
            self.control_flow_stack.pop_continue_target();
        }

        let break_target = self.control_flow_stack.pop_break_target();
        debug_assert_eq!(break_target.symbol, symbol);

        self.peer.create_br(break_target.block);
        self.peer.move_basic_block_after(break_target.block);
        self.peer.set_basic_block(break_target.block);
    }

    fn process_continue(&mut self, symbol: Symbol) {
        let target_block = self.control_flow_stack.continue_target(symbol);
        self.peer.create_br(target_block);
        self.create_basic_block_for_deadcode();
    }

    fn process_break(&mut self, symbol: Symbol) {
        let target_block = self.control_flow_stack.break_target(symbol);
        self.peer.create_br(target_block);
        self.create_basic_block_for_deadcode();
    }

    fn process_return(&mut self, n: u32) {
        if n > 0 {
            debug_assert_eq!(n, 1);
            let (operand, _) = self.dereference();
            self.create_store_operand_to_retv(operand);
        }

        self.peer.create_store_normal_status();

        self.control_flow_stack.set_returned();
        let next_block = self.control_flow_stack.cleanup_block();

        self.peer.create_br(next_block);

        self.create_basic_block_for_deadcode();
    }

    fn create_store_operand_to_retv(&mut self, operand: Operand) {
        match operand {
            Operand::Undefined => self.peer.create_store_undefined_to_retv(),
            Operand::Null => self.peer.create_store_null_to_retv(),
            Operand::Boolean(value) => self.peer.create_store_boolean_to_retv(value),
            Operand::Number(value) => self.peer.create_store_number_to_retv(value),
            Operand::Closure(value) => self.peer.create_store_closure_to_retv(value),
            Operand::Any(value) => self.peer.create_store_value_to_retv(value),
            _ => unreachable!(),
        }
    }

    fn process_throw(&mut self) {
        let (operand, _) = self.dereference();
        self.create_store_operand_to_retv(operand);
        self.peer.create_store_exception_status();

        self.control_flow_stack.set_thrown();
        let next_block = self.control_flow_stack.exception_block();

        self.peer.create_br(next_block);
        self.peer.move_basic_block_after(next_block);

        self.create_basic_block_for_deadcode();
    }

    fn process_discard(&mut self) {
        debug_assert!(!self.operand_stack.is_empty());
        self.operand_stack.pop();
    }

    fn process_swap(&mut self) {
        self.swap();
    }

    fn create_basic_block(&mut self, name: &str) -> BasicBlock {
        push_bb_name!(self, name);
        let (name, name_len) = bb_name!(self);
        let block = self.peer.create_basic_block(name, name_len);
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
        self.peer.set_basic_block(block);
    }
}

struct OperandStack(Vec<Operand>);

impl OperandStack {
    fn new() -> Self {
        Self(vec![])
    }

    fn duplicate(&mut self) {
        let last = self.0.pop().unwrap();
        self.push(last.clone());
        self.push(last);
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

#[derive(Clone, Debug)]
enum Operand {
    Undefined,
    Null,
    Boolean(bridge::ValueIr),
    Number(bridge::ValueIr),
    Function(LambdaIr),
    Closure(bridge::ValueIr),
    Any(bridge::ValueIr),
    Reference(Symbol, Locator),
    Argv(bridge::ValueIr),
    Capture(bridge::ValueIr),
}

impl Dump for Operand {
    fn dump(&self, buf: *mut std::ffi::c_char, len: usize) {
        macro_rules! ir2cstr {
            ($value:expr) => {
                peer::get_value_name_or_as_operand(*$value, buf, len)
            };
        }

        macro_rules! ir2cstr2 {
            ($value:expr) => {
                $value.get_name_or_as_operand(buf, len)
            };
        }

        match self {
            Self::Undefined => eprintln!("Undefined"),
            Self::Null => eprintln!("Null"),
            Self::Boolean(value) => eprintln!("Boolean({:?})", ir2cstr!(value)),
            Self::Number(value) => eprintln!("Number({:?})", ir2cstr!(value)),
            Self::Function(lambda) => eprintln!("Function({:?})", ir2cstr2!(lambda)),
            Self::Closure(value) => eprintln!("Closure({:?})", ir2cstr!(value)),
            Self::Any(value) => eprintln!("Any({:?})", ir2cstr!(value)),
            Self::Reference(symbol, locator) => eprintln!("Reference({symbol}, {locator:?})"),
            Self::Argv(value) => eprintln!("Argv({:?})", ir2cstr!(value)),
            Self::Capture(value) => eprintln!("Capture({:?})", ir2cstr!(value)),
        }
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
