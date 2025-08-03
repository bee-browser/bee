macro_rules! runtime_debug {
    ($block:block) => {
        if cfg!(debug_assertions) $block
    };
}

mod control_flow;
mod editor;

use std::ffi::c_void;
use std::ops::Deref;
use std::ops::DerefMut;

use cranelift::codegen;
use cranelift::codegen::ir;
use cranelift::codegen::isa;
use cranelift::frontend::FunctionBuilder;
use cranelift::frontend::FunctionBuilderContext;
use jsparser::SymbolRegistry;
use rustc_hash::FxHashMap;

use jsparser::Symbol;
use jsparser::syntax::LoopFlags;

use crate::ProgramId;
use crate::Runtime;
use crate::lambda::LambdaInfo;
use crate::lambda::LambdaKind;
use crate::lambda::LambdaRegistry;
use crate::logger;
use crate::semantics::CompileCommand;
use crate::semantics::Function;
use crate::semantics::Locator;
use crate::semantics::Program;
use crate::semantics::ScopeRef;
use crate::semantics::ScopeTree;
use crate::semantics::ThisBinding;
use crate::semantics::VariableRef;
use crate::types::U16Chunk;
use crate::types::Value;

use super::CompileError;
use super::CompilerSupport;
use super::EditorSupport;
use super::Executor;
use super::LambdaId;
use super::RuntimeFunctionCache;

use control_flow::ControlFlowStack;
use editor::Editor;

pub struct Session<'r> {
    program: &'r Program,
    symbol_registry: &'r mut SymbolRegistry,
    lambda_registry: &'r mut LambdaRegistry,
    pub executor: &'r mut Executor,
    scope_cleanup_checker_enabled: bool,
    global_object: *mut c_void,
    object_prototype: *mut c_void,
    function_prototype: *mut c_void,
}

impl CompilerSupport for Session<'_> {
    fn is_scope_cleanup_checker_enabled(&self) -> bool {
        self.scope_cleanup_checker_enabled
    }

    fn get_symbol_name(&self, symbol: Symbol) -> &[u16] {
        self.symbol_registry.resolve(symbol).unwrap()
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

    fn get_function(&self, lambda_id: LambdaId) -> &Function {
        let index = self.lambda_registry.get(lambda_id).function_index as usize;
        &self.program.functions[index]
    }

    fn target_config(&self) -> isa::TargetFrontendConfig {
        self.executor.target_config()
    }

    fn global_object(&mut self) -> *mut c_void {
        self.global_object
    }

    fn object_prototype(&self) -> *mut c_void {
        self.object_prototype
    }

    fn function_prototype(&self) -> *mut c_void {
        self.function_prototype
    }
}

// TODO: Deferring the compilation until it's actually called improves the performance.
// Because the program may contain unused functions.
pub fn compile<X>(
    runtime: &mut Runtime<X>,
    program_id: ProgramId,
    _optimize: bool,
) -> Result<(), CompileError> {
    // Allocate large data on the heap memory.
    //
    // Many existing programs including examples in bytecodealliance/wasmtime allocate
    // `FunctionBuilderContext` and `codegen::Context` on the stack.  However, it's not good to
    // allocate large data on the stack in a function that is called deep in the stack.  At this
    // point, the size of `FunctionBuilderContext` is nearly 1KB and the size of `codegen::Context`
    // is more than 4KB.
    let mut context = Box::new(CraneliftContext::new());

    // Compile the functions in reverse order in order to compile a coroutine function before its
    // ramp function so that the size of the scratch buffer for the coroutine function is available
    // when the ramp function is compiled.
    //
    // NOTE: The functions are stored in post-order traversal on the scope tree.  So, we don't need
    // to use `Iterator::rev()`.
    //
    // TODO: We should manage dependencies between functions in a more general way.
    let program = &runtime.programs[program_id.index()];
    for func in program.functions.iter() {
        let mut session = {
            let scope_cleanup_checker_enabled = runtime.is_scope_cleanup_checker_enabled();
            let global_object = runtime.global_object.as_ptr();
            Session {
                program,
                symbol_registry: &mut runtime.symbol_registry,
                lambda_registry: &mut runtime.lambda_registry,
                executor: &mut runtime.executor,
                scope_cleanup_checker_enabled,
                global_object,
                object_prototype: runtime.object_prototype,
                function_prototype: runtime.function_prototype,
            }
        };
        context.compile_function(func, &mut session, &program.scope_tree);
        if let Some(ref mut monitor) = runtime.monitor {
            monitor.print_function_ir(func.id, &context.context.func);
        }

        runtime.executor.codegen(func, &mut context.context);
    }

    Ok(())
}

pub fn compile_function<X>(
    runtime: &mut Runtime<X>,
    program_id: ProgramId,
    function_index: usize,
    _optimize: bool,
) -> Result<(), CompileError> {
    logger::debug!(event = "compile_function", ?program_id, function_index);

    // Allocate large data on the heap memory.
    //
    // Many existing programs including examples in bytecodealliance/wasmtime allocate
    // `FunctionBuilderContext` and `codegen::Context` on the stack.  However, it's not good to
    // allocate large data on the stack in a function that is called deep in the stack.  At this
    // point, the size of `FunctionBuilderContext` is nearly 1KB and the size of `codegen::Context`
    // is more than 4KB.
    let mut context = Box::new(CraneliftContext::new());

    let program = &runtime.programs[program_id.index()];
    let func = &program.functions[function_index];

    let mut session = {
        let scope_cleanup_checker_enabled = runtime.is_scope_cleanup_checker_enabled();
        let global_object = runtime.global_object.as_ptr();
        Session {
            program,
            symbol_registry: &mut runtime.symbol_registry,
            lambda_registry: &mut runtime.lambda_registry,
            executor: &mut runtime.executor,
            scope_cleanup_checker_enabled,
            global_object,
            object_prototype: runtime.object_prototype,
            function_prototype: runtime.function_prototype,
        }
    };

    context.compile_function(func, &mut session, &program.scope_tree);
    if let Some(ref mut monitor) = runtime.monitor {
        monitor.print_function_ir(func.id, &context.context.func);
    }

    runtime.executor.codegen(func, &mut context.context);

    Ok(())
}

struct CraneliftContext {
    builder_context: FunctionBuilderContext,
    context: codegen::Context,
}

impl CraneliftContext {
    fn new() -> Self {
        Self {
            builder_context: FunctionBuilderContext::new(),
            context: codegen::Context::new(),
        }
    }

    fn compile_function<R>(&mut self, func: &Function, support: &mut R, scope_tree: &ScopeTree)
    where
        R: CompilerSupport + EditorSupport,
    {
        let compiler = Compiler::new(
            func,
            support,
            scope_tree,
            // At this point, the size of ir::Function is 896 bytes.  Copy operation is not fast.
            // It's better to use codegen::Context::func as the sample code does.
            &mut self.context.func,
            &mut self.builder_context,
        );
        compiler.compile(func);
    }
}

struct Compiler<'a, R> {
    support: &'a mut R,

    /// The scope tree of the JavaScript program to compile.
    scope_tree: &'a ScopeTree,

    /// A stack to hold sets of basic blocks which construct of a region in the control flow graph
    /// (CFG) finally built.
    control_flow_stack: ControlFlowStack,

    pending_labels: Vec<Symbol>,

    editor: Editor<'a>,

    /// A stack for operands.
    operand_stack: OperandStack,

    // The following values must be reset in the end of compilation for each function.
    this: Option<AnyIr>,
    this_capture: Option<CaptureIr>,
    arguments: Vec<AnyIr>,
    locals: Vec<AnyIr>,
    captures: FxHashMap<Locator, CaptureIr>,

    max_scratch_buffer_len: u32,

    skip_count: u16,
}

impl<'a, R> Compiler<'a, R>
where
    R: CompilerSupport + EditorSupport,
{
    const HIDDEN_PROMISE_INDEX: u16 = 0;
    const HIDDEN_RESULT_INDEX: u16 = 1;
    const HIDDEN_ERROR_INDEX: u16 = 2;

    fn new(
        func: &Function,
        support: &'a mut R,
        scope_tree: &'a ScopeTree,
        func_ir: &'a mut ir::Function,
        builder_context: &'a mut FunctionBuilderContext,
    ) -> Self {
        let target_config = support.target_config();
        let addr_type = target_config.pointer_type();

        func_ir.name = ir::UserFuncName::user(0, func.id.into());
        func_ir.signature.call_conv = target_config.default_call_conv;

        // formal parameters
        let params = &mut func_ir.signature.params;
        // runtime: *mut c_void
        params.push(ir::AbiParam::new(addr_type));
        // context: *mut c_void
        params.push(ir::AbiParam::new(addr_type));
        // this: *mut Value
        params.push(ir::AbiParam::new(addr_type));
        // args: u16
        params.push(ir::AbiParam::new(ir::types::I16));
        // argv: *mut Value
        params.push(ir::AbiParam::new(addr_type));
        // retv: *mut Value
        params.push(ir::AbiParam::new(addr_type));

        // #[repr(u32)] Status
        func_ir
            .signature
            .returns
            .push(ir::AbiParam::new(ir::types::I32));

        let builder = FunctionBuilder::new(func_ir, builder_context);

        Self {
            support,
            scope_tree,
            control_flow_stack: Default::default(),
            pending_labels: Default::default(),
            editor: Editor::new(builder, target_config),
            operand_stack: Default::default(),
            this: None,
            this_capture: None,
            arguments: Default::default(),
            locals: Default::default(),
            captures: Default::default(),
            max_scratch_buffer_len: 0,
            skip_count: 0,
        }
    }

    fn compile(mut self, func: &Function) {
        self.start_compile(func);
        for command in func.commands.iter() {
            if self.skip_count > 0 {
                self.skip_count -= 1;
                continue;
            }
            self.process_command(func, command);
        }
        self.end_compile(func);
    }

    fn start_compile(&mut self, func: &Function) {
        logger::debug!(event = "start_compile", ?func.name, ?func.id);

        let entry_block = self.editor.entry_block();
        self.editor.switch_to_block(entry_block);

        if matches!(
            self.support.get_lambda_info(func.id).kind,
            LambdaKind::Coroutine
        ) {
            self.editor.put_set_coroutine_mode();
        }

        // Unlike LLVM IR, we cannot specify a label for each basic block.  This is bad from a
        // debugging and readability perspective...
        let body_block = self.editor.create_block();
        let exit_block = self.editor.create_block();

        assert!(self.pending_labels.is_empty());
        self.control_flow_stack.push_exit_target(exit_block, false);

        // Unlike LLVM IR, no block can move once an instruction is inserted to the block.  So,
        // it's necessary to select one of the following ways:
        //
        //   1. Building blocks and inserting instructions in the correct program flow
        //   2. Inserting blocks to the layout in correct order before inserting any instructions
        //      to the blocks.
        //
        // In the first way, we cannot use the back-patching technique in the compilation.  So, we
        // select the second way.
        //
        // However, Cranelift denies to insert a block after a block which is not in the layout.
        // The `entry_block` has only one instruction to jump to the `locals_block`.  The
        // `entry_block` is inserted to the layout when the instruction is inserted to it.  This
        // makes it possible to insert other *empty* blocks after the `entry_block` before
        // inserting instructions to those blocks.
        self.editor.put_store_status(Status::UNSET);
        self.editor.put_store_flow_selector(FlowSelector::NORMAL);
        if self.support.is_scope_cleanup_checker_enabled() {
            self.editor.put_init_scope_cleanup_checker();
        }
        self.editor.put_jump(body_block, &[]);

        self.editor.switch_to_block(body_block);

        self.control_flow_stack
            .push_function_flow(entry_block, body_block, exit_block);

        let retv = self.editor.retv();
        self.editor.put_store_undefined_to_any(retv);
    }

    fn end_compile(mut self, func: &Function) {
        logger::debug!(event = "end_compile", ?func.id);

        debug_assert!(self.operand_stack.is_empty());

        if matches!(
            self.support.get_lambda_info(func.id).kind,
            LambdaKind::Coroutine
        ) {
            self.control_flow_stack.pop_coroutine_flow();
        }

        self.control_flow_stack.pop_exit_target();
        let flow = self.control_flow_stack.pop_function_flow();
        debug_assert!(self.control_flow_stack.is_empty());

        self.editor.put_jump(flow.exit_block, &[]);
        self.editor.switch_to_block(flow.exit_block);
        if self.support.is_scope_cleanup_checker_enabled() {
            self.editor
                .put_assert_scope_id(self.support, ScopeRef::NONE);
        }
        self.editor.put_return();

        let info = self.support.get_lambda_info_mut(func.id);
        if matches!(info.kind, LambdaKind::Coroutine) {
            info.scratch_buffer_len = self.max_scratch_buffer_len;
        }

        self.editor.end();
    }

    fn process_command(&mut self, func: &Function, command: &CompileCommand) {
        logger::debug!(event = "process_command", ?command);
        match command {
            CompileCommand::Nop => (),
            CompileCommand::Batch(n) => self.process_batch(*n),
            CompileCommand::Undefined => self.process_undefined(),
            CompileCommand::Null => self.process_null(),
            CompileCommand::Boolean(value) => self.process_boolean(*value),
            CompileCommand::Number(value) => self.process_number(*value),
            CompileCommand::String(value) => self.process_string(value),
            CompileCommand::Object => self.process_object(),
            CompileCommand::Function(name) => self.process_function(*name),
            CompileCommand::Lambda(lambda_id) => self.process_lambda(*lambda_id),
            CompileCommand::Closure(prologue, func_scope_ref) => {
                self.process_closure(func, *prologue, *func_scope_ref)
            }
            CompileCommand::Coroutine(lambda_id, num_locals) => {
                self.process_coroutine(*lambda_id, *num_locals)
            }
            CompileCommand::Promise => self.process_promise(),
            CompileCommand::Exception => self.process_exception(),
            CompileCommand::This => self.process_this(),
            CompileCommand::VariableReference(symbol) => self.process_variable_reference(*symbol),
            CompileCommand::PropertyReference(symbol) => self.process_property_reference(*symbol),
            CompileCommand::ToPropertyKey => self.process_to_property_key(),
            CompileCommand::LoadFormalParameters(n) => self.process_load_formal_parameters(*n),
            CompileCommand::AllocateLocals(n) => self.process_allocate_locals(*n),
            CompileCommand::MutableVariable => self.process_mutable_variable(),
            CompileCommand::ImmutableVariable => self.process_immutable_variable(),
            CompileCommand::DeclareVariables(scope_ref) => {
                self.process_declare_variables(func, *scope_ref)
            }
            CompileCommand::DeclareFunction => self.process_declare_function(),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::New(nargs) => self.process_new(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(func, *scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
            CompileCommand::ToString => self.process_to_string(),
            CompileCommand::ConcatStrings(n) => self.process_concat_strings(*n),
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
            CompileCommand::IfThen(expr) => self.process_if_then(*expr),
            CompileCommand::Else(expr) => self.process_else(*expr),
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
            CompileCommand::CaseClause(default, batch_index) => {
                self.process_case_clause(*default, *batch_index)
            }
            CompileCommand::Switch(id, num_cases, default_index) => {
                self.process_switch(func, *id, *num_cases, *default_index)
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

        macro_rules! dump_enabled {
            () => {
                cfg!(debug_assertions) && matches!(
                    std::env::var_os("BEE_DEBUG_JSRUNTIME_COMPILER_DUMP"),
                    Some(v) if v == "1",
                )
            };
        }

        if dump_enabled!() {
            eprintln!("### operand-stack");
            self.operand_stack.dump();
            eprintln!();

            self.control_flow_stack.dump();
        }
    }

    // commands

    fn process_batch(&mut self, n: u16) {
        debug_assert_eq!(self.skip_count, 0);
        debug_assert_ne!(n, 0);
        self.skip_count = n;
    }

    fn process_undefined(&mut self) {
        self.operand_stack.push(Operand::Undefined);
    }

    fn process_null(&mut self) {
        self.operand_stack.push(Operand::Null);
    }

    fn process_boolean(&mut self, value: bool) {
        let value_ir = self.editor.put_boolean(value);
        self.operand_stack
            .push(Operand::Boolean(value_ir, Some(value)));
    }

    fn process_number(&mut self, value: f64) {
        let value_ir = self.editor.put_number(value);
        self.operand_stack
            .push(Operand::Number(value_ir, Some(value)));
    }

    fn process_string(&mut self, value: &[u16]) {
        // Theoretically, the heap memory pointed by `value` can be freed after the IR built by the
        // compiler is freed.
        let string_ir = self.editor.put_create_string(value);
        self.operand_stack
            .push(Operand::String(string_ir, Some(U16Chunk::new_stack(value))));
    }

    fn process_object(&mut self) {
        let prototype_addr = self.support.object_prototype();
        let prototype = self.editor.put_object(prototype_addr);
        let object = self
            .editor
            .put_runtime_create_object(self.support, prototype);
        self.operand_stack.push(Operand::Object(object));
    }

    fn process_function(&mut self, name: Symbol) {
        let closure = self.pop_closure();
        // TODO(feat): Function object
        let prototype_addr = self.support.function_prototype();
        let prototype = self.editor.put_object(prototype_addr);
        let function = self
            .editor
            .put_runtime_create_object(self.support, prototype);
        self.editor.put_store_closure_to_function(closure, function);
        self.perform_set_function_name(function, name);
        self.perform_make_constructor(function);
        self.operand_stack.push(Operand::Function(function, name));
    }

    fn process_lambda(&mut self, lambda_id: LambdaId) {
        let lambda_kind = self.support.get_lambda_info(lambda_id).kind;
        // Perform lazy compilation by default.
        let lambda = self
            .editor
            .put_declare_lazy_compile(self.support, lambda_kind);
        self.operand_stack.push(Operand::Lambda(lambda, lambda_id));
    }

    fn process_closure(&mut self, func: &Function, _prologue: bool, func_scope_ref: ScopeRef) {
        let scope = self.scope_tree.scope(func_scope_ref);
        debug_assert!(scope.is_function());

        let (lambda, lambda_id) = self.pop_lambda();
        let num_captures = self.support.get_function(lambda_id).num_captures;

        // TODO(perf): use `Function::num_captures` instead of `Scope::count_captures()`.
        let closure =
            self.editor
                .put_runtime_create_closure(self.support, lambda, lambda_id, num_captures);

        if func.is_this_binding_captured() {
            let capture = self.this_capture.unwrap();
            self.editor
                .put_store_capture_to_closure(capture, closure, 0);
        }

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
                Locator::Capture(i) => self.editor.put_load_capture(i),
                _ => unreachable!(),
            };
            self.editor
                .put_store_capture_to_closure(capture, closure, variable.index);
        }

        self.operand_stack.push(Operand::Closure(closure));
    }

    fn process_coroutine(&mut self, lambda_id: LambdaId, num_locals: u16) {
        let scrach_buffer_len = self.support.get_lambda_info(lambda_id).scratch_buffer_len;
        debug_assert!(scrach_buffer_len <= u16::MAX as u32);
        let closure = self.pop_closure();
        let coroutine = self.editor.put_runtime_create_coroutine(
            self.support,
            closure,
            num_locals,
            scrach_buffer_len as u16,
        );
        self.operand_stack.push(Operand::Coroutine(coroutine));
    }

    fn process_promise(&mut self) {
        let coroutine = self.pop_coroutine();
        let promise = self
            .editor
            .put_runtime_register_promise(self.support, coroutine);
        self.operand_stack.push(Operand::Promise(promise));
    }

    fn process_exception(&mut self) {
        // TODO: Should we check status_ at runtime?
        let exception = self.editor.exception();
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Any(exception, None));
    }

    fn process_this(&mut self) {
        let this = self.this.unwrap();
        self.operand_stack.push(Operand::Any(this, None));
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
        let (operand, ..) = self.dereference();
        let key = match operand {
            Operand::Undefined => Symbol::UNDEFINED.into(),
            Operand::Null => Symbol::NULL.into(),
            Operand::Boolean(_, Some(false)) => Symbol::FALSE.into(),
            Operand::Boolean(_, Some(true)) => Symbol::TRUE.into(),
            Operand::Boolean(value, None) => {
                let any = self.editor.put_alloc_any();
                self.editor.put_store_boolean_to_any(value, any);
                any.into()
            }
            Operand::Number(_, Some(value)) => value.into(),
            Operand::Number(value, None) => {
                let any = self.editor.put_alloc_any();
                self.editor.put_store_number_to_any(value, any);
                any.into()
            }
            Operand::String(_, Some(ref value)) => self
                .support
                .make_symbol_from_name(value.make_utf16())
                .into(),
            Operand::String(value, None) => {
                let any = self.editor.put_alloc_any();
                self.editor.put_store_string_to_any(value, any);
                any.into()
            }
            Operand::Object(value) => {
                let any = self.editor.put_alloc_any();
                self.editor.put_store_object_to_any(value, any);
                any.into()
            }
            Operand::Function(value, _) => {
                let any = self.editor.put_alloc_any();
                self.editor.put_store_function_to_any(value, any);
                any.into()
            }
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
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::PropertyReference(_)
            | Operand::VariableReference(..) => {
                unreachable!("{operand:?}")
            }
        };
        self.operand_stack.push(Operand::PropertyReference(key));
    }

    fn process_load_formal_parameters(&mut self, num_params: u16) {
        if num_params == 0 {
            // Nothing to do.
            return;
        }

        // `argc` may be smaller than `num_params`.  In this case, remaining formal parameters must
        // be set to `undefined`.

        // `params` is a list of `Value`s initialized w/ `undefined`.  The `Value`s are used only
        // if `argc` is smaller than `num_params`.
        let params = self.editor.put_alloc_argv(num_params);
        for i in 0..num_params {
            let arg = self.editor.put_get_arg(params, i);
            self.editor.put_store_undefined_to_any(arg);
        }

        // Create destination blocks of the jump table and the merge block where the program flow
        // reaches eventually.
        let mut blocks = Vec::with_capacity(num_params as usize);
        for _ in 0..num_params {
            blocks.push(self.editor.create_block());
        }
        let default_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_argv(num_params);

        // Put the jump table to the instruction buffer.
        let argc = self.editor.argc_as_jump_table_index();
        self.editor.put_jump_table(argc, &blocks, default_block);

        // The `block_args` is a temporal buffer to keep `ir::Value`s which are passed to the merge
        // block.
        let mut block_args = Vec::with_capacity(num_params as usize);

        // TODO(perf): Another possible implementation is copying values from `argv` to `params`.

        for i in 0..num_params {
            self.editor.switch_to_block(blocks[i as usize]);
            for j in 0..i {
                let arg = self.editor.put_get_argument(self.support, j);
                block_args.push(arg.0.into());
            }
            for j in i..num_params {
                let arg = self.editor.put_get_arg(params, j);
                block_args.push(arg.0.into());
            }
            self.editor.put_jump(merge_block, &block_args);
            block_args.clear();
        }

        self.editor.switch_to_block(default_block);
        for i in 0..num_params {
            let arg = self.editor.put_get_argument(self.support, i);
            block_args.push(arg.0.into());
        }
        self.editor.put_jump(merge_block, &block_args);

        // Store `ir::Value`s to `self.arguments`.
        self.editor.switch_to_block(merge_block);
        for i in 0..(num_params as usize) {
            let arg = AnyIr(self.editor.get_block_param(merge_block, i));
            self.arguments.push(arg);
        }
    }

    fn process_allocate_locals(&mut self, num_locals: u16) {
        for _ in 0..num_locals {
            let local = self.editor.put_alloc_any();
            self.locals.push(local);
        }
    }

    fn process_mutable_variable(&mut self) {
        let (symbol, locator) = self.pop_reference();
        let (operand, ..) = self.dereference();

        if let Operand::Function(function, Symbol::NONE) = operand {
            // anonymous function
            self.perform_set_function_name(function, symbol);
        }

        let local = match locator {
            Locator::Local(index) => self.get_local(index),
            _ => unreachable!(),
        };

        self.emit_store_operand_to_any(&operand, local);
    }

    fn process_immutable_variable(&mut self) {
        let (symbol, locator) = self.pop_reference();
        let (operand, ..) = self.dereference();

        if let Operand::Function(function, Symbol::NONE) = operand {
            // anonymous function
            self.perform_set_function_name(function, symbol);
        }

        let local = match locator {
            Locator::Local(index) => self.get_local(index),
            _ => unreachable!(),
        };

        self.emit_store_operand_to_any(&operand, local);
    }

    // NOTE: This function may call `process_command()`.
    fn process_declare_variables(&mut self, func: &Function, scope_ref: ScopeRef) {
        debug_assert!(self.scope_tree.scope(scope_ref).is_function());

        // In the specification, function-scoped variables defined by "VariableStatement"s are
        // created in "10.2.11 FunctionDeclarationInstantiation ( func, argumentsList )".  We
        // create them here for simplicity but this still works properly for well-formed JavaScript
        // programs.
        //
        // Function-scoped variables are created in the `init` basic block of the current scope.
        // The `init` basic block is performed before the `hoisted` basic block on which inner
        // functions defined by "FunctionDeclaration"s are created.

        // TODO(refactor): inefficient
        for (variable_ref, variable) in self.scope_tree.iter_variables(scope_ref) {
            if !variable.is_function_scoped() {
                continue;
            }
            let local = match self.scope_tree.compute_locator(variable_ref) {
                Locator::Local(index) => self.get_local(index),
                locator => unreachable!("{locator:?}"),
            };
            self.editor.put_store_undefined_to_any(local);
        }

        // TODO(fix): preserve declaration order.
        for variable in self.scope_tree.scope(scope_ref).variables.iter() {
            if variable.function_declaration_batch == 0 {
                continue;
            }
            let start = variable.function_declaration_batch + 1;
            let end = start
                + match func.commands[variable.function_declaration_batch] {
                    CompileCommand::Batch(n) => n as usize,
                    _ => unreachable!(),
                };
            for command in func.commands[start..end].iter() {
                self.process_command(func, command);
            }
        }
    }

    fn process_declare_function(&mut self) {
        let (symbol, locator) = self.pop_reference();
        let (operand, ..) = self.dereference();
        let func_ir = match operand {
            Operand::Function(value, _) => value,
            _ => unreachable!("{operand:?}"),
        };

        match locator {
            Locator::Local(index) => {
                let local = self.get_local(index);
                self.editor.put_store_function_to_any(func_ir, local);
            }
            Locator::Global => {
                let object = ObjectIr(self.editor.put_nullptr());
                let value = self.editor.put_alloc_any();
                self.editor.put_store_function_to_any(func_ir, value);
                self.editor
                    .put_runtime_set_value_by_symbol(self.support, object, symbol, value);
            }
            _ => unreachable!("{locator:?}"),
        };
    }

    // 10.2.5 MakeConstructor ( F [ , writablePrototype [ , prototype ] ] )
    fn perform_make_constructor(&mut self, function: ObjectIr) {
        logger::debug!(event = "make_constructor", ?function);

        let value = self.editor.put_alloc_any();
        let retv = self.editor.put_alloc_any();

        // TODO(feat): implement others

        let object_prototype_addr = self.support.object_prototype();
        let object_prototype = self.editor.put_object(object_prototype_addr);
        let prototype = self
            .editor
            .put_runtime_create_object(self.support, object_prototype);

        self.editor.put_store_function_to_any(function, value);
        // TODO(feat): [[Writable]]: writablePrototype, [[Enumerable]]: false,
        // [[Configurable]]: true
        // TODO(feat): retv
        self.editor.put_runtime_create_data_property_by_symbol(
            self.support,
            prototype,
            Symbol::CONSTRUCTOR,
            value,
            retv,
        );

        self.editor.put_store_object_to_any(prototype, value);
        // TODO(feat): [[Writable]]: writablePrototype, [[Enumerable]]: false,
        // [[Configurable]]: false
        // TODO(feat): retv
        self.editor.put_runtime_create_data_property_by_symbol(
            self.support,
            function,
            Symbol::PROTOTYPE,
            value,
            retv,
        );
    }

    // 10.2.9 SetFunctionName ( F, name [ , prefix ] )
    fn perform_set_function_name(&mut self, function: ObjectIr, name: Symbol) {
        logger::debug!(event = "set_function_name", ?function, ?name);
        // TODO(feat): private name
        // TODO(feat): F.[[InitialName]]
        // TODO(feat): prefix
        let name = if name == Symbol::NONE {
            &[]
        } else {
            self.support.get_symbol_name(name)
        };
        let name = self.editor.put_create_string(name);
        let value = self.editor.put_alloc_any();
        self.editor.put_store_string_to_any(name, value);
        let retv = self.editor.put_alloc_any();
        // TODO(feat): [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: true
        // TODO(feat): retv
        self.editor.put_runtime_create_data_property_by_symbol(
            self.support,
            function,
            Symbol::NAME,
            value,
            retv,
        );
    }

    fn process_call(&mut self, argc: u16) {
        let argv = self.emit_create_argv(argc);
        let (operand, this, _) = self.dereference();
        let closure = match operand {
            Operand::Closure(_) => unreachable!(),
            Operand::Function(object, _) => {
                // IIFE
                self.editor.put_load_closure_from_function(object)
            }
            Operand::Any(value, ..) => self.emit_load_closure_or_throw_type_error(value),
            _ => {
                self.process_number(1001.); // TODO: TypeError
                self.process_throw();
                return;
            }
        };

        let this = if let Some(this) = this {
            let any = self.emit_create_any();
            self.editor.put_store_object_to_any(this, any);
            any
        } else {
            self.editor.this_argument()
        };
        let retv = self.emit_create_any();
        let status = self.editor.put_call(closure, this, argc, argv, retv);

        self.emit_check_status_for_exception(status, retv);

        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(retv, None));
    }

    fn process_new(&mut self, argc: u16) {
        // TODO: prototype chain
        let argv = self.emit_create_argv(argc);
        let (operand, ..) = self.dereference();
        let function = match operand {
            Operand::Closure(_) => unreachable!(),
            Operand::Function(function, _) => function,
            Operand::Any(value, ..) => self.emit_load_function_or_throw_type_error(value),
            _ => {
                self.process_number(1001.); // TODO: TypeError
                self.process_throw();
                return;
            }
        };

        let closure = self.editor.put_load_closure_from_function(function);

        let prototype = self.editor.put_runtime_get_value_by_symbol(
            self.support,
            function,
            Symbol::PROTOTYPE,
            false,
        ); // TODO: strict
        runtime_debug! {{
            let is_object = self.editor.put_is_object(prototype);
            self.editor.put_runtime_assert(self.support, is_object, c"Prototype must be an object");
        }}
        let prototype = self.editor.put_load_object(prototype);

        let this = {
            let any = self.emit_create_any();
            let object = self
                .editor
                .put_runtime_create_object(self.support, prototype);
            self.editor.put_store_object_to_any(object, any);
            any
        };
        let retv = self.emit_create_any();
        let status = self.editor.put_call(closure, this, argc, argv, retv);

        self.emit_check_status_for_exception(status, retv);

        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(this, None));
    }

    fn process_push_scope(&mut self, func: &Function, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        let init_block = self.editor.create_block();
        let body_block = self.editor.create_block();
        let cleanup_block = self.editor.create_block();

        self.control_flow_stack
            .push_scope_flow(scope_ref, body_block, cleanup_block);

        self.control_flow_stack
            .push_exit_target(cleanup_block, false);

        self.editor.put_jump(init_block, &[]);
        self.editor.switch_to_block(init_block);

        if self.support.is_scope_cleanup_checker_enabled() {
            self.editor.put_store_scope_id_for_checker(scope_ref);
        }

        let scope = self.scope_tree.scope(scope_ref);

        if scope.is_function() {
            self.resolve_this_binding(func);
        }

        for variable in scope.variables.iter() {
            if variable.is_function_scoped() {
                continue;
            }
            let locator = variable.locator();
            if variable.is_captured() {
                let target = match locator {
                    Locator::Argument(index) => self.get_argument(index),
                    Locator::Local(index) => self.get_local(index),
                    _ => unreachable!(),
                };
                let capture = self.editor.put_runtime_create_capture(self.support, target);
                debug_assert!(!self.captures.contains_key(&locator));
                self.captures.insert(locator, capture);
            }
            if let Locator::Local(index) = locator {
                let value = self.get_local(index);
                self.editor.put_store_none_to_any(value);
            }
        }

        self.editor.put_jump(body_block, &[]);
        self.editor.switch_to_block(body_block);
    }

    // Step#1..8 in "10.2.1.2 OrdinaryCallBindThis()"
    //
    // See Also:
    // 9.1.2.4 NewFunctionEnvironment()
    // 10.2 ECMAScript Function Objects
    // 10.2.3 OrdinaryFunctionCreate()
    // 10.2.11 FunctionDeclarationInstantiation()
    fn resolve_this_binding(&mut self, func: &Function) {
        logger::debug!(event = "resolve_this_binding", ?func.this_binding);
        debug_assert!(self.this.is_none());
        debug_assert!(self.this_capture.is_none());

        match func.this_binding {
            ThisBinding::None => {
                // No code is generated for the `this` binding resolution, which improves the
                // execution time in some situations.
            }
            ThisBinding::ThisArgument => {
                // const this = thisArgument;
                let this = self.editor.this_argument();
                self.this = Some(this);
            }
            ThisBinding::Capture => {
                // The capture of the outer `this` binding is always the first element in the
                // capture list.
                let this = self.editor.put_load_captured_value(0);
                self.this = Some(this);
            }
            ThisBinding::GlobalObject => {
                // const this = globalObject;
                // DO NOT USE `globalThis` HERE.
                let global_object = self.support.global_object();
                let value = self.editor.put_object(global_object);
                let this = self.perform_to_any(&Operand::Object(value));
                self.this = Some(this);
            }
            ThisBinding::Quirk => {
                // const this =
                //   thisArgument !== null && thisArgument !== undefined ?
                //   ToObject(thisArgument) : globalObject;
                self.operand_stack
                    .push(Operand::Any(self.editor.this_argument(), None));
                self.process_non_nullish();
                self.process_if_then(true);
                self.operand_stack
                    .push(Operand::Any(self.editor.this_argument(), None));
                self.perform_to_object();
                self.process_else(true);
                // DO NOT USE `globalThis` HERE.
                let global_object = self.support.global_object();
                let value = self.editor.put_object(global_object);
                self.operand_stack.push(Operand::Object(value));
                self.process_ternary();
                let this = self.pop_any();
                self.this = Some(this);
            }
        }

        if func.is_this_binding_captured() {
            let this = self.this.unwrap();
            let capture = self.editor.put_runtime_create_capture(self.support, this);
            self.this_capture = Some(capture);
        }
    }

    fn process_pop_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        // Create additional blocks of the scope region before pop_bb_name!().
        // Because these constitute the scope region.
        let postcheck_block = self.editor.create_block();
        let ctrl_block = self.editor.create_block();
        let exit_block = self.editor.create_block();

        self.control_flow_stack.pop_exit_target();
        let parent_exit_block = self.control_flow_stack.exit_block();

        let flow = self.control_flow_stack.pop_scope_flow();
        debug_assert_eq!(flow.scope_ref, scope_ref);

        self.editor.put_jump(flow.cleanup_block, &[]);

        self.editor.switch_to_block(flow.cleanup_block);
        let scope = self.scope_tree.scope(scope_ref);
        for variable in scope.variables.iter() {
            if variable.is_captured() {
                self.perform_escape_value(variable.locator());
            }
            if variable.is_local() {
                // tidy local value
                // TODO: GC
            }
        }
        if scope.is_function() {
            if let Some(this_capture) = self.this_capture {
                let this = self.this.unwrap();
                self.editor.put_escape_value(this_capture, this);
            }
            // tidy this value if it exists
            // TODO: GC
        }
        self.editor.put_jump(postcheck_block, &[]);

        self.editor.switch_to_block(postcheck_block);
        if self.support.is_scope_cleanup_checker_enabled() {
            self.editor.put_assert_scope_id(self.support, scope_ref);
            if self.control_flow_stack.has_scope_flow() {
                let outer_scope_ref = self.control_flow_stack.scope_flow().scope_ref;
                self.editor.put_store_scope_id_for_checker(outer_scope_ref);
            } else {
                self.editor.put_store_scope_id_for_checker(ScopeRef::NONE);
            }
        }
        self.editor.put_jump(ctrl_block, &[]);

        self.editor.switch_to_block(ctrl_block);
        let is_normal = self.editor.put_is_flow_selector_normal();
        self.editor
            .put_branch(is_normal, exit_block, &[], parent_exit_block, &[]);

        self.editor.switch_to_block(exit_block);
    }

    // 7.1.17 ToString ( argument )
    fn process_to_string(&mut self) {
        let (operand, ..) = self.dereference();
        let result = self.perform_to_string(&operand);
        self.operand_stack.push(Operand::String(result, None));
    }

    // TODO(perf): compile-time evaluation
    fn perform_to_string(&mut self, operand: &Operand) -> StringIr {
        use jsparser::symbol::builtin::names;

        match operand {
            Operand::Undefined => self.editor.put_create_string(names::UNDEFINED),
            Operand::Null => self.editor.put_create_string(names::NULL),
            Operand::Boolean(_, Some(true)) => self.editor.put_create_string(names::TRUE),
            Operand::Boolean(_, Some(false)) => self.editor.put_create_string(names::FALSE),
            Operand::Boolean(value, None) => self.perform_boolean_to_string(*value),
            Operand::Number(value, _) => self
                .editor
                .put_runtime_number_to_string(self.support, *value),
            Operand::String(value, _) => *value,
            Operand::Object(_) => todo!(),
            Operand::Function(..) => todo!(),
            Operand::Promise(_) => todo!(),
            Operand::Any(value, _) => self.editor.put_runtime_to_string(self.support, *value),
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    fn perform_boolean_to_string(&mut self, value: BooleanIr) -> StringIr {
        use jsparser::symbol::builtin::names;
        let string_ir = self.editor.put_alloc_string();
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block();
        self.editor
            .put_branch(value, then_block, &[], else_block, &[]);
        self.editor.switch_to_block(then_block);
        self.editor.put_set_string(names::TRUE, string_ir);
        self.editor.put_jump(merge_block, &[]);
        self.editor.switch_to_block(else_block);
        self.editor.put_set_string(names::FALSE, string_ir);
        self.editor.put_jump(merge_block, &[]);
        self.editor.switch_to_block(merge_block);
        string_ir
    }

    fn process_concat_strings(&mut self, n: u16) {
        debug_assert!(n > 1);
        // TODO(perf): compile-time concatenation
        let (mut tail, _) = self.pop_string();
        for _ in 1..n {
            let (head, _) = self.pop_string();
            tail = self
                .editor
                .put_runtime_concat_strings(self.support, head, tail)
        }
        self.operand_stack.push(Operand::String(tail, None));
    }

    fn pop_string(&mut self) -> (StringIr, Option<U16Chunk>) {
        match self.operand_stack.pop().unwrap() {
            Operand::String(value_rt, value_ct) => (value_rt, value_ct),
            operand => unreachable!("{operand:?}"),
        }
    }

    // 13.2.5.5 Runtime Semantics: PropertyDefinitionEvaluation
    fn process_create_data_property(&mut self) {
        let (operand, ..) = self.dereference();
        let key = self.pop_property_reference();

        if let Operand::Function(function, Symbol::NONE) = operand {
            // anonymous function
            let name = match key {
                PropertyKey::Symbol(name) => name,
                PropertyKey::Number(_) => todo!(),
                PropertyKey::Any(_) => todo!(),
            };
            self.perform_set_function_name(function, name);
        }

        let object = self.peek_object();
        let value = self.editor.put_alloc_any();
        self.emit_store_operand_to_any(&operand, value);
        let retv = self.emit_create_any();

        // 7.3.6 CreateDataPropertyOrThrow ( O, P, V )

        // 1. Let success be ? CreateDataProperty(O, P, V).
        let status = match key {
            PropertyKey::Symbol(key) => self.editor.put_runtime_create_data_property_by_symbol(
                self.support,
                object,
                key,
                value,
                retv,
            ),
            PropertyKey::Number(key) => self.editor.put_runtime_create_data_property_by_number(
                self.support,
                object,
                key,
                value,
                retv,
            ),
            PropertyKey::Any(key) => self.editor.put_runtime_create_data_property_by_any(
                self.support,
                object,
                key,
                value,
                retv,
            ),
        };
        self.emit_check_status_for_exception(status, retv);
        // `retv` holds a boolean value.
        runtime_debug! {{
            let is_boolean = self.editor.put_is_boolean(retv);
            self.editor.put_runtime_assert(self.support,
                is_boolean,
                c"runtime.create_data_property() returns a boolan value",
            );
        }}
        let success = self.editor.put_load_boolean(retv);

        // 2. If success is false, throw a TypeError exception.
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block();
        // if success
        self.editor
            .put_branch(success, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        self.editor.put_jump(merge_block, &[]);
        // } else {
        self.editor.switch_to_block(else_block);
        // TODO(feat): TypeError
        self.process_number(1001.);
        self.process_throw();
        self.editor.put_jump(merge_block, &[]);
        // }
        self.editor.switch_to_block(merge_block);
    }

    fn pop_property_reference(&mut self) -> PropertyKey {
        match self.operand_stack.pop().unwrap() {
            Operand::PropertyReference(key) => key,
            _ => unreachable!(),
        }
    }

    fn peek_object(&mut self) -> ObjectIr {
        match self.operand_stack.last().unwrap() {
            Operand::Object(value) => *value,
            _ => unreachable!(),
        }
    }

    // 13.2.5.5 Runtime Semantics: PropertyDefinitionEvaluation
    // PropertyDefinition : ... AssignmentExpression
    fn process_copy_data_properties(&mut self) {
        // 1. Let exprValue be ? Evaluation of AssignmentExpression.
        let (operand, ..) = self.dereference();

        // 2. Let fromValue be ? GetValue(exprValue).
        let from_value = self.editor.put_alloc_any();
        self.emit_store_operand_to_any(&operand, from_value);

        // TODO: 3. Let excludedNames be a new empty List.

        // 4. Perform ? CopyDataProperties(object, fromValue, excludedNames).

        let object = self.peek_object();
        let retv = self.emit_create_any();

        let status =
            self.editor
                .put_runtime_copy_data_properties(self.support, object, from_value, retv);
        self.emit_check_status_for_exception(status, retv);
    }

    fn process_push_array_element(&mut self) {
        // 1. Let exprValue be ? Evaluation of AssignmentExpression.
        let (operand, ..) = self.dereference();

        // 2. Let fromValue be ? GetValue(exprValue).
        let from_value = self.editor.put_alloc_any();
        self.emit_store_operand_to_any(&operand, from_value);

        let object = self.peek_object();
        let retv = self.emit_create_any();

        let status =
            self.editor
                .put_runtime_push_array_element(self.support, object, from_value, retv);
        self.emit_check_status_for_exception(status, retv);
    }

    // 13.4.2.1 Runtime Semantics: Evaluation
    fn process_postfix_increment(&mut self) {
        self.perform_incr_decr('$', '+');
    }

    // 13.4.3.1 Runtime Semantics: Evaluation
    fn process_postfix_decrement(&mut self) {
        self.perform_incr_decr('$', '-');
    }

    // 13.4.4.1 Runtime Semantics: Evaluation
    fn process_prefix_increment(&mut self) {
        self.perform_incr_decr('^', '+');
    }

    // 13.4.5.1 Runtime Semantics: Evaluation
    fn process_prefix_decrement(&mut self) {
        self.perform_incr_decr('^', '-');
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

        let (operand, ..) = self.dereference();
        match operand {
            Operand::Undefined => self.process_string(names::UNDEFINED),
            Operand::Null => self.process_string(names::OBJECT),
            Operand::Boolean(..) => self.process_string(names::BOOLEAN),
            Operand::Number(..) => self.process_string(names::NUMBER),
            Operand::String(..) => self.process_string(names::STRING),
            Operand::Object(..) | Operand::Promise(..) => self.process_string(names::OBJECT),
            Operand::Function(..) => self.process_string(names::FUNCTION),
            Operand::Any(_, Some(ref value)) => match value {
                Value::Undefined => self.process_string(names::UNDEFINED),
                Value::Null => self.process_string(names::OBJECT),
                Value::Boolean(_) => self.process_string(names::BOOLEAN),
                Value::Number(_) => self.process_string(names::NUMBER),
                Value::String(_) => self.process_string(names::STRING),
                Value::Object(_) | Value::Promise(_) => self.process_string(names::OBJECT),
                Value::Function(_) => self.process_string(names::FUNCTION),
                Value::None => unreachable!("{value:?}"),
            },
            Operand::Any(value, None) => {
                let string = self.editor.put_runtime_typeof(self.support, value);
                self.operand_stack.push(Operand::String(string, None));
            }
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(..) => unreachable!("{operand:?}"),
        }
    }

    // 13.5.4.1 Runtime Semantics: Evaluation
    fn process_unary_plus(&mut self) {
        let (operand, ..) = self.dereference();
        let value = self.perform_to_numeric(&operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 13.5.5.1 Runtime Semantics: Evaluation
    fn process_unary_minus(&mut self) {
        let (operand, ..) = self.dereference();
        let value = self.perform_to_numeric(&operand);
        // TODO: BigInt
        // 6.1.6.1.1 Number::unaryMinus ( x )
        let value = self.editor.put_negate(value);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 13.5.6.1 Runtime Semantics: Evaluation
    fn process_bitwise_not(&mut self) {
        let (operand, ..) = self.dereference();
        let number = self.perform_to_numeric(&operand);
        // TODO: BigInt
        let number = self.editor.put_bitwise_not(self.support, number);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.5.7.1 Runtime Semantics: Evaluation
    fn process_logical_not(&mut self) {
        let (operand, ..) = self.dereference();
        let boolean = self.perform_to_boolean(&operand);
        let boolean = self.editor.put_logical_not(boolean);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.6.1 Runtime Semantics: Evaluation
    fn process_exponentiation(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let number = self.editor.put_exp(self.support, lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_multiplication(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let number = self.editor.put_mul(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_division(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let number = self.editor.put_div(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_remainder(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let number = self.editor.put_rem(self.support, lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.8.1.1 Runtime Semantics: Evaluation
    // TODO(perf): compile-time evaluation
    // TODO(refactor): refactoring
    fn process_addition(&mut self) {
        let (lhs, ..) = self.dereference();
        let (rhs, ..) = self.dereference();

        match (&lhs, &rhs) {
            (Operand::String(lhs, _), Operand::String(rhs, _)) => {
                let string = self
                    .editor
                    .put_runtime_concat_strings(self.support, *lhs, *rhs);
                self.operand_stack.push(Operand::String(string, None));
            }
            (
                Operand::Any(lhs, Some(Value::String(_))),
                Operand::Any(rhs, Some(Value::String(_))),
            ) => {
                runtime_debug! {{
                    let is_string = self.editor.put_is_string(*lhs);
                    self.editor.put_runtime_assert(self.support, is_string, c"process_addition: lhs should be a string");
                }}
                let lhs = self.editor.put_load_string(*lhs);
                runtime_debug! {{
                    let is_string = self.editor.put_is_string(*rhs);
                    self.editor.put_runtime_assert(self.support, is_string, c"process_addition: rhs should be a string");
                }}
                let rhs = self.editor.put_load_string(*rhs);
                let string = self
                    .editor
                    .put_runtime_concat_strings(self.support, lhs, rhs);
                self.operand_stack.push(Operand::String(string, None));
            }
            (_, Operand::String(rhs, _)) => {
                let lhs = self.perform_to_string(&lhs);
                let string = self
                    .editor
                    .put_runtime_concat_strings(self.support, lhs, *rhs);
                self.operand_stack.push(Operand::String(string, None));
            }
            (Operand::String(lhs, _), _) => {
                let rhs = self.perform_to_string(&rhs);
                let string = self
                    .editor
                    .put_runtime_concat_strings(self.support, *lhs, rhs);
                self.operand_stack.push(Operand::String(string, None));
            }
            (Operand::Any(lhs_value, _), Operand::Any(rhs_value, _)) => {
                // TODO(feat): ToPrimitive(lhs_value)
                // TODO(feat): ToPrimitive(rhs_value)

                let then_block = self.editor.create_block();
                let else_block = self.editor.create_block();
                let merge_block = self.editor.create_block();
                let result = self.emit_create_any();

                // if lhs.value.is_string() || rhs.value.is_string()
                let lhs_is_string = self.editor.put_is_string(*lhs_value);
                let rhs_is_string = self.editor.put_is_string(*rhs_value);
                let is_string = self.editor.put_logical_or(lhs_is_string, rhs_is_string);
                self.editor
                    .put_branch(is_string, then_block, &[], else_block, &[]);
                // then
                {
                    self.editor.switch_to_block(then_block);
                    let lhs = self.perform_to_string(&lhs);
                    let rhs = self.perform_to_string(&rhs);
                    let string = self
                        .editor
                        .put_runtime_concat_strings(self.support, lhs, rhs);
                    self.editor.put_store_string_to_any(string, result);
                    self.editor.put_jump(merge_block, &[]);
                }
                // else
                {
                    self.editor.switch_to_block(else_block);
                    let lhs = self.perform_to_numeric(&lhs);
                    let rhs = self.perform_to_numeric(&rhs);
                    let number = self.editor.put_add(lhs, rhs);
                    self.editor.put_store_number_to_any(number, result);
                    self.editor.put_jump(merge_block, &[]);
                }
                // end

                self.editor.switch_to_block(merge_block);
                self.operand_stack.push(Operand::Any(result, None));
            }
            (Operand::Any(lhs_value, _), _) => {
                // TODO(feat): ToPrimitive(lhs_value)

                let then_block = self.editor.create_block();
                let else_block = self.editor.create_block();
                let merge_block = self.editor.create_block();
                let result = self.emit_create_any();

                // if lhs.value.is_string()
                let is_string = self.editor.put_is_string(*lhs_value);
                self.editor
                    .put_branch(is_string, then_block, &[], else_block, &[]);
                // then
                {
                    self.editor.switch_to_block(then_block);
                    let lhs = self.editor.put_load_string(*lhs_value);
                    let rhs = self.perform_to_string(&rhs);
                    let string = self
                        .editor
                        .put_runtime_concat_strings(self.support, lhs, rhs);
                    self.editor.put_store_string_to_any(string, result);
                    self.editor.put_jump(merge_block, &[]);
                }
                // else
                {
                    self.editor.switch_to_block(else_block);
                    let lhs = self.perform_to_numeric(&lhs);
                    let rhs = self.perform_to_numeric(&rhs);
                    let number = self.editor.put_add(lhs, rhs);
                    self.editor.put_store_number_to_any(number, result);
                    self.editor.put_jump(merge_block, &[]);
                }
                // end

                self.editor.switch_to_block(merge_block);
                self.operand_stack.push(Operand::Any(result, None));
            }
            (_, Operand::Any(rhs_value, _)) => {
                // TODO(feat): ToPrimitive(rhs_value)

                let then_block = self.editor.create_block();
                let else_block = self.editor.create_block();
                let merge_block = self.editor.create_block();
                let result = self.emit_create_any();

                // if rhs.value.is_string()
                let is_string = self.editor.put_is_string(*rhs_value);
                self.editor
                    .put_branch(is_string, then_block, &[], else_block, &[]);
                // then
                {
                    self.editor.switch_to_block(then_block);
                    let lhs = self.perform_to_string(&lhs);
                    let rhs = self.editor.put_load_string(*rhs_value);
                    let string = self
                        .editor
                        .put_runtime_concat_strings(self.support, lhs, rhs);
                    self.editor.put_store_string_to_any(string, result);
                    self.editor.put_jump(merge_block, &[]);
                }
                // else
                {
                    self.editor.switch_to_block(else_block);
                    let lhs = self.perform_to_numeric(&lhs);
                    let rhs = self.perform_to_numeric(&rhs);
                    let number = self.editor.put_add(lhs, rhs);
                    self.editor.put_store_number_to_any(number, result);
                    self.editor.put_jump(merge_block, &[]);
                }
                // end

                self.editor.switch_to_block(merge_block);
                self.operand_stack.push(Operand::Any(result, None));
            }
            _ => {
                let lhs = self.perform_to_numeric(&lhs);
                let rhs = self.perform_to_numeric(&rhs);
                let number = self.editor.put_add(lhs, rhs);
                self.operand_stack.push(Operand::Number(number, None));
            }
        }
    }

    // 13.8.2.1 Runtime Semantics: Evaluation
    fn process_subtraction(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let number = self.editor.put_sub(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.1.1 Runtime Semantics: Evaluation
    fn process_left_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.editor.put_left_shift(self.support, lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.2.1 Runtime Semantics: Evaluation
    fn process_signed_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.editor.put_signed_right_shift(self.support, lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.3.1 Runtime Semantics: Evaluation
    fn process_unsigned_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.editor.put_unsigned_right_shift(self.support, lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let boolean = self.editor.put_less_than(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let boolean = self.editor.put_greater_than(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than_or_equal(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let boolean = self.editor.put_less_than_or_equal(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than_or_equal(&mut self) {
        let (lhs, ..) = self.dereference();
        let lhs = self.perform_to_numeric(&lhs);

        let (rhs, ..) = self.dereference();
        let rhs = self.perform_to_numeric(&rhs);

        let boolean = self.editor.put_greater_than_or_equal(lhs, rhs);
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
        // TODO: comparing the references improves the performance.
        let (lhs, ..) = self.dereference();
        let (rhs, ..) = self.dereference();

        let boolean = self.perform_is_loosely_equal(&lhs, &rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_inequality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, ..) = self.dereference();
        let (rhs, ..) = self.dereference();

        let eq = self.perform_is_loosely_equal(&lhs, &rhs);
        let boolean = self.editor.put_logical_not(eq);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_equality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, ..) = self.dereference();
        let (rhs, ..) = self.dereference();

        let boolean = self.perform_is_strictly_equal(&lhs, &rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_inequality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, ..) = self.dereference();
        let (rhs, ..) = self.dereference();

        let eq = self.perform_is_strictly_equal(&lhs, &rhs);
        let boolean = self.editor.put_logical_not(eq);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_and(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, ..) = self.dereference();
        let (rval, ..) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.perform_to_numeric(&lval);
        let rnum = self.perform_to_numeric(&rval);
        // TODO: BigInt

        let number = self.editor.put_bitwise_and(self.support, lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_xor(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, ..) = self.dereference();
        let (rval, ..) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.perform_to_numeric(&lval);
        let rnum = self.perform_to_numeric(&rval);
        // TODO: BigInt

        let number = self.editor.put_bitwise_xor(self.support, lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_or(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, ..) = self.dereference();
        let (rval, ..) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.perform_to_numeric(&lval);
        let rnum = self.perform_to_numeric(&rval);
        // TODO: BigInt

        let number = self.editor.put_bitwise_or(self.support, lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    fn process_ternary(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();
        let result = flow.result.unwrap();

        let (else_operand, ..) = self.dereference();
        self.emit_store_operand_to_any(&else_operand, result);
        self.editor.put_jump(flow.merge_block, &[]);

        self.editor.switch_to_block(flow.merge_block);
        self.operand_stack.push(Operand::Any(result, None))
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_assignment(&mut self) {
        let (rhs, ..) = self.dereference();

        match self.operand_stack.pop().unwrap() {
            Operand::VariableReference(symbol, Locator::Global) => {
                let object = ObjectIr(self.editor.put_nullptr());
                let value = self.editor.put_alloc_any();
                self.emit_store_operand_to_any(&rhs, value);
                // TODO(feat): ReferenceError, TypeError
                self.editor
                    .put_runtime_set_value_by_symbol(self.support, object, symbol, value);
            }
            Operand::VariableReference(symbol, locator) => {
                let var = self.emit_get_variable(symbol, locator);
                // TODO: throw a TypeError in the strict mode.
                // auto* flags_ptr = CreateGetFlagsPtr(value_ptr);
                self.emit_store_operand_to_any(&rhs, var);
            }
            Operand::PropertyReference(key) => {
                // TODO(refactor): reduce code clone
                self.perform_to_object();
                let object = self.pop_object();
                let value = self.editor.put_alloc_any();
                self.emit_store_operand_to_any(&rhs, value);
                match key {
                    PropertyKey::Symbol(key) => {
                        self.editor.put_runtime_set_value_by_symbol(
                            self.support,
                            object,
                            key,
                            value,
                        );
                    }
                    PropertyKey::Number(key) => {
                        self.editor.put_runtime_set_value_by_number(
                            self.support,
                            object,
                            key,
                            value,
                        );
                    }
                    PropertyKey::Any(key) => {
                        self.editor
                            .put_runtime_set_value_by_any(self.support, object, key, value);
                    }
                }
            }
            operand => unreachable!("{operand:?}"),
        }

        self.operand_stack.push(rhs);
    }

    fn pop_object(&mut self) -> ObjectIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Object(value) | Operand::Function(value, _) => value,
            operand => unreachable!("{operand:?}"),
        }
    }

    // 7.1.18 ToObject ( argument )
    // TODO(perf): directly convert the value referred by the operand into an object.
    fn perform_to_object(&mut self) {
        logger::debug!(event = "perform_to_object");
        let (operand, ..) = self.dereference();
        match operand {
            // Immediate values.
            Operand::Undefined | Operand::Null => self
                .operand_stack
                .push(Operand::Object(ObjectIr(self.editor.put_nullptr()))),
            Operand::Boolean(..) => todo!(),
            Operand::Number(..) => todo!(),
            Operand::String(..) => todo!(),
            Operand::Object(_) | Operand::Function(..) => self.operand_stack.push(operand),
            Operand::Promise(_value) => todo!(),
            // Variables and properties.
            Operand::Any(value, ..) => {
                let object = self.editor.put_runtime_to_object(self.support, value);
                self.operand_stack.push(Operand::Object(object));
            }
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    fn process_falsy_short_circuit(&mut self) {
        let (operand, ..) = self.dereference();
        let boolean = self.perform_to_boolean(&operand);
        let boolean = self.editor.put_logical_not(boolean);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
        self.process_if_then(true);
        self.operand_stack.push(operand);
        self.process_else(true);
    }

    fn process_truthy_short_circuit(&mut self) {
        let (operand, ..) = self.dereference();
        let boolean = self.perform_to_boolean(&operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
        self.process_if_then(true);
        self.operand_stack.push(operand);
        self.process_else(true);
    }

    fn process_nullish_short_circuit(&mut self) {
        let (operand, ..) = self.dereference();
        let boolean = self.perform_is_non_nullish(&operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
        self.process_if_then(true);
        self.operand_stack.push(operand);
        self.process_else(true);
    }

    fn process_truthy(&mut self) {
        let (operand, ..) = self.dereference();
        let boolean = self.perform_to_boolean(&operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    fn process_non_nullish(&mut self) {
        let (operand, ..) = self.dereference();
        let boolean = self.perform_is_non_nullish(&operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    fn process_if_then(&mut self, expr: bool) {
        let cond_value = self.pop_boolean();
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block();
        let result = if expr {
            Some(self.emit_create_any())
        } else {
            None
        };
        self.editor
            .put_branch(cond_value, then_block, &[], else_block, &[]);
        self.editor.switch_to_block(then_block);
        self.control_flow_stack
            .push_if_then_else_flow(then_block, else_block, merge_block, result);
    }

    fn process_else(&mut self, expr: bool) {
        if let Some(result) = self.control_flow_stack.expr_result() {
            debug_assert!(expr);
            let (operand, ..) = self.dereference();
            self.emit_store_operand_to_any(&operand, result);
        } else {
            debug_assert!(!expr);
            self.operand_stack.pop();
        }
        let merge_block = self.control_flow_stack.merge_block();
        self.editor.put_jump_if_not_terminated(merge_block, &[]);
        let then_block = self.editor.current_block();
        let else_block = self.control_flow_stack.update_then_block(then_block);
        self.editor.switch_to_block(else_block);
    }

    fn process_if_else_statement(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();
        self.editor
            .put_jump_if_not_terminated(flow.merge_block, &[]);
        self.editor.switch_to_block(flow.merge_block);
    }

    fn process_if_statement(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();
        self.editor
            .put_jump_if_not_terminated(flow.merge_block, &[]);
        self.editor.switch_to_block(flow.else_block);
        self.editor.put_jump(flow.merge_block, &[]);
        self.editor.switch_to_block(flow.merge_block);
    }

    fn process_do_while_loop(&mut self, _id: u16) {
        let loop_body = self.editor.create_block();
        let loop_ctrl = self.editor.create_block();
        let loop_test = self.editor.create_block();
        let loop_exit = self.editor.create_block();

        let loop_start = loop_body;
        let loop_continue = loop_test;
        let loop_break = loop_exit;

        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_exit, loop_exit);
        self.control_flow_stack
            .push_loop_body_flow(loop_ctrl, loop_test);

        self.editor.put_jump(loop_start, &[]);

        self.build_loop_ctrl_block(loop_ctrl, loop_continue, loop_break);

        self.editor.switch_to_block(loop_start);
    }

    fn process_while_loop(&mut self, _id: u16) {
        let loop_test = self.editor.create_block();
        let loop_body = self.editor.create_block();
        let loop_ctrl = self.editor.create_block();
        let loop_exit = self.editor.create_block();

        let loop_start = loop_test;
        let loop_continue = loop_test;
        let loop_break = loop_exit;

        self.control_flow_stack
            .push_loop_body_flow(loop_ctrl, loop_exit);
        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_exit, loop_body);

        self.editor.put_jump(loop_start, &[]);

        self.build_loop_ctrl_block(loop_ctrl, loop_continue, loop_break);

        self.editor.switch_to_block(loop_start);
    }

    // TODO: rewrite using if and break
    fn process_for_loop(&mut self, _id: u16, flags: LoopFlags) {
        let has_init = flags.contains(LoopFlags::HAS_INIT);
        let has_test = flags.contains(LoopFlags::HAS_TEST);
        let has_next = flags.contains(LoopFlags::HAS_NEXT);

        let loop_init = self.editor.create_block();
        let loop_test = self.editor.create_block();
        let loop_body = self.editor.create_block();
        let loop_ctrl = self.editor.create_block();
        let loop_next = self.editor.create_block();
        let loop_exit = self.editor.create_block();

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

        self.editor.put_jump(loop_start, &[]);

        self.build_loop_ctrl_block(loop_ctrl, loop_continue, loop_break);

        self.editor.switch_to_block(insert_point);
    }

    fn build_loop_ctrl_block(
        &mut self,
        loop_ctrl: ir::Block,
        loop_continue: ir::Block,
        loop_break: ir::Block,
    ) {
        let set_normal_block = self.editor.create_block();
        let break_or_continue_block = self.editor.create_block();

        self.control_flow_stack.push_exit_target(loop_ctrl, true);
        for label in std::mem::take(&mut self.pending_labels).into_iter() {
            self.control_flow_stack.set_exit_label(label);
        }
        let exit_id = self.control_flow_stack.exit_id();

        self.editor.switch_to_block(loop_ctrl);
        let is_normal_or_continue = self
            .editor
            .put_is_flow_selector_normal_or_continue(exit_id.depth());
        let is_break_or_continue = self
            .editor
            .put_is_flow_selector_break_or_continue(exit_id.depth());
        self.editor.put_branch(
            is_break_or_continue,
            set_normal_block,
            &[],
            break_or_continue_block,
            &[],
        );

        self.editor.switch_to_block(set_normal_block);
        self.editor.put_store_flow_selector(FlowSelector::NORMAL);
        self.editor.put_jump(break_or_continue_block, &[]);

        self.editor.switch_to_block(break_or_continue_block);
        self.editor
            .put_branch(is_normal_or_continue, loop_continue, &[], loop_break, &[]);
    }

    fn process_loop_init(&mut self) {
        let loop_init = self.control_flow_stack.pop_loop_init_flow();
        self.editor.put_jump(loop_init.branch_block, &[]);
        self.editor.switch_to_block(loop_init.insert_point);
    }

    fn process_loop_test(&mut self) {
        let loop_test = self.control_flow_stack.pop_loop_test_flow();
        let (operand, ..) = self.dereference();
        let cond = self.perform_to_boolean(&operand);
        self.editor
            .put_branch(cond, loop_test.then_block, &[], loop_test.else_block, &[]);
        self.editor.switch_to_block(loop_test.insert_point);
    }

    fn process_loop_next(&mut self) {
        let loop_next = self.control_flow_stack.pop_loop_next_flow();
        // Discard the evaluation result.
        self.process_discard();
        self.editor.put_jump(loop_next.branch_block, &[]);
        self.editor.switch_to_block(loop_next.insert_point);
    }

    fn process_loop_body(&mut self) {
        let loop_body = self.control_flow_stack.pop_loop_body_flow();
        self.editor.put_jump(loop_body.branch_block, &[]);
        self.editor.switch_to_block(loop_body.insert_point);
    }

    fn process_loop_end(&mut self) {
        self.control_flow_stack.pop_exit_target();
    }

    fn process_case_block(&mut self, _id: u16, num_cases: u16) {
        debug_assert!(num_cases > 0);

        let ctrl_block = self.editor.create_block();
        let case_block = self.editor.create_block();

        self.control_flow_stack.push_switch_flow(ctrl_block);
        self.control_flow_stack.push_exit_target(ctrl_block, true);
        debug_assert!(self.pending_labels.is_empty());

        self.editor.put_jump(case_block, &[]);
        self.editor.switch_to_block(case_block);
    }

    fn process_case_clause(&mut self, default: bool, batch_index: Option<usize>) {
        let clause_block = self.editor.create_block();
        if default {
            // Nothing to do here.
            // A jump instruction to the default block will be added in process_switch().
        } else {
            let next_case_block = self.editor.create_block();
            let cond_value = self.pop_boolean();
            self.editor
                .put_branch(cond_value, clause_block, &[], next_case_block, &[]);
            self.editor.switch_to_block(next_case_block);
        }
        self.control_flow_stack
            .push_case_flow(clause_block, batch_index);
    }

    fn process_switch(
        &mut self,
        func: &Function,
        _id: u16,
        num_cases: u16,
        default_index: Option<u16>,
    ) {
        let ctrl_block = self.control_flow_stack.switch_flow().end_block;

        if let Some(default_index) = default_index {
            debug_assert!(default_index < num_cases);
            let default_block = self.control_flow_stack.get_default_block(default_index);
            self.editor.put_jump(default_block, &[]);
        } else {
            self.editor.put_jump(ctrl_block, &[]);
        }

        let mut fall_through_block = ctrl_block;
        for _ in 0..num_cases {
            let flow = self.control_flow_stack.pop_case_flow();
            self.editor.switch_to_block(flow.clause_block);
            if let Some(batch_index) = flow.batch_index {
                let start = batch_index + 1;
                let end = start
                    + match func.commands[batch_index] {
                        CompileCommand::Batch(n) => n as usize,
                        _ => unreachable!(),
                    };
                for command in func.commands[start..end].iter() {
                    self.process_command(func, command);
                }
            }
            self.editor
                .put_jump_if_not_terminated(fall_through_block, &[]);
            fall_through_block = flow.clause_block;
        }

        let exit_id = self.control_flow_stack.exit_id();
        self.control_flow_stack.pop_exit_target();
        self.control_flow_stack.pop_switch_flow();

        let set_normal_block = self.editor.create_block();
        let end_block = self.editor.create_block();

        self.editor.switch_to_block(ctrl_block);
        let is_break = self.editor.put_is_flow_selector_break(exit_id.depth());
        self.editor
            .put_branch(is_break, set_normal_block, &[], end_block, &[]);

        self.editor.switch_to_block(set_normal_block);
        self.editor.put_store_flow_selector(FlowSelector::NORMAL);
        self.editor.put_jump(end_block, &[]);

        self.editor.switch_to_block(end_block);
    }

    fn process_try(&mut self) {
        let try_block = self.editor.create_block();
        let catch_block = self.editor.create_block();
        let finally_block = self.editor.create_block();
        let end_block = self.editor.create_block();

        self.control_flow_stack.push_exception_flow(
            try_block,
            catch_block,
            finally_block,
            end_block,
        );
        self.control_flow_stack.push_exit_target(catch_block, false);

        // Jump from the end of previous block to the beginning of the try block.
        self.editor.put_jump(try_block, &[]);

        self.editor.switch_to_block(try_block);
    }

    fn process_catch(&mut self, nominal: bool) {
        self.control_flow_stack.set_in_catch(nominal);

        let flow = self.control_flow_stack.exception_flow();
        let finally_block = flow.finally_block;
        let catch_block = flow.catch_block;

        self.control_flow_stack.pop_exit_target();
        self.control_flow_stack
            .push_exit_target(finally_block, false);

        // Jump from the end of the try block to the beginning of the finally block.
        self.editor.put_jump(finally_block, &[]);

        self.editor.switch_to_block(catch_block);
        // Reach here when the flow selector is not NORMAL.
        // Jump to the finally block if the flow selector is not THROW.
        // TODO(perf): Directly jump to the finally block if the flow selector is not THROW.
        let block = self.editor.create_block();
        let is_throw = self.editor.put_is_flow_selector_throw();
        self.editor
            .put_branch(is_throw, block, &[], finally_block, &[]);

        self.editor.switch_to_block(block);
        if !nominal {
            self.editor.put_store_status(Status::NORMAL);
            self.editor.put_store_flow_selector(FlowSelector::NORMAL);
        }
    }

    fn process_finally(&mut self, _nominal: bool) {
        self.control_flow_stack.set_in_finally();

        let flow = self.control_flow_stack.exception_flow();
        let finally_block = flow.finally_block;

        self.control_flow_stack.pop_exit_target();

        // Jump from the end of the catch block to the beginning of the finally block.
        self.editor.put_jump(finally_block, &[]);
        self.editor.switch_to_block(finally_block);
    }

    fn process_try_end(&mut self) {
        let flow = self.control_flow_stack.pop_exception_flow();
        let parent_exit_block = self.control_flow_stack.exit_block();

        // Jump from the end of the finally block to the beginning of the outer catch block if
        // there is an uncaught exception.  Otherwise, jump to the beginning of the try-end block.
        let is_normal = self.editor.put_is_flow_selector_normal();
        self.editor
            .put_branch(is_normal, flow.end_block, &[], parent_exit_block, &[]);

        self.editor.switch_to_block(flow.end_block);
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
            let start_block = self.editor.create_block();
            let end_block = self.editor.create_block();

            self.editor.put_jump(start_block, &[]);
            self.editor.switch_to_block(start_block);

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
            self.editor.put_jump(end_block, &[]);
            self.editor.switch_to_block(end_block);
        }
    }

    fn process_continue(&mut self, label: Symbol) {
        let exit_id = self.control_flow_stack.exit_id_for_label(label);
        let flow_selector = FlowSelector::continue_at(exit_id.depth());
        self.editor.put_store_flow_selector(flow_selector);

        let block = self.control_flow_stack.exit_block();
        self.editor.put_jump(block, &[]);

        let block = self.editor.create_block_for_deadcode();
        self.editor.switch_to_block(block);
    }

    fn process_break(&mut self, label: Symbol) {
        let exit_id = self.control_flow_stack.exit_id_for_label(label);
        let flow_selector = FlowSelector::break_at(exit_id.depth());
        self.editor.put_store_flow_selector(flow_selector);

        let block = self.control_flow_stack.exit_block();
        self.editor.put_jump(block, &[]);

        let block = self.editor.create_block_for_deadcode();
        self.editor.switch_to_block(block);
    }

    fn process_return(&mut self, n: u32) {
        if n > 0 {
            debug_assert_eq!(n, 1);
            let (operand, ..) = self.dereference();
            self.store_operand_to_retv(&operand);
        }

        self.editor.put_store_status(Status::NORMAL);
        self.editor.put_store_flow_selector(FlowSelector::RETURN);

        let next_block = self.control_flow_stack.cleanup_block();
        self.editor.put_jump(next_block, &[]);

        let block = self.editor.create_block_for_deadcode();
        self.editor.switch_to_block(block);
    }

    fn process_environment(&mut self, num_locals: u16) {
        // The formal parameters of the original async functions are captured by the inner
        // coroutine closure.

        // Hidden variables are accessible only in the coroutine lambda function.  These are passed
        // to the coroutine lambda function as arguments.
        self.arguments.push(
            self.editor
                .put_get_argument(self.support, Self::HIDDEN_PROMISE_INDEX),
        );
        self.arguments.push(
            self.editor
                .put_get_argument(self.support, Self::HIDDEN_RESULT_INDEX),
        );
        self.arguments.push(
            self.editor
                .put_get_argument(self.support, Self::HIDDEN_ERROR_INDEX),
        );

        // Local variables and captured variables living outer scopes are loaded here from the
        // `Coroutine` data passed via the `env` argument of the coroutine lambda function to be
        // generated by the compiler.
        for i in 0..num_locals {
            let local = self.editor.put_get_local_from_coroutine(i);
            self.locals.push(local);
        }
    }

    fn process_jump_table(&mut self, num_states: u32) {
        debug_assert!(num_states >= 2);

        let state = self.editor.put_load_state_from_coroutine();
        let blocks = self.editor.put_switch_blocks(state, num_states);
        debug_assert_eq!(blocks.len(), num_states as usize);

        self.editor.switch_to_block(blocks[num_states as usize - 1]);
        let x = self.editor.put_boolean(false);
        self.editor
            .put_runtime_assert(self.support, x, c"the coroutine has already done");
        self.editor.put_unreachable();

        self.editor.switch_to_block(blocks[0]);

        self.control_flow_stack
            .push_coroutine_flow(blocks, num_states);
    }

    fn process_await(&mut self, next_state: u32) {
        self.perform_resolve_promise();
        self.perform_save_operands_to_scratch_buffer();
        self.editor.put_store_state_to_coroutine(next_state);
        self.editor.put_suspend();

        // resume block
        let block = self.control_flow_stack.coroutine_next_block();
        self.editor.switch_to_block(block);
        self.perform_load_operands_from_scratch_buffer();

        let has_error_block = self.editor.create_block();
        let result_block = self.editor.create_block();

        // if ##error.has_value()
        let error = self.get_hidden_error();
        let has_error = self.editor.put_has_value(error);
        self.editor
            .put_branch(has_error, has_error_block, &[], result_block, &[]);
        // {
        // throw ##error;
        self.editor.switch_to_block(has_error_block);
        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(error, None));
        self.process_throw();
        self.editor.put_jump(result_block, &[]);
        // }

        self.editor.switch_to_block(result_block);
        let result = self.get_hidden_result();

        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(result, None));
    }

    fn process_resume(&mut self) {
        let promise = self.pop_promise();
        self.editor.put_runtime_resume(self.support, promise);
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
    fn perform_save_operands_to_scratch_buffer(&mut self) {
        logger::debug!(event = "perform_save_operands_to_scratch_buffer");
        let scratch_buffer = self.editor.put_get_scratch_buffer_from_coroutine();
        // Take the whole operand stack in order to avoid the borrow checker.
        let operand_stack = std::mem::take(&mut self.operand_stack);
        let mut offset = 0;
        for operand in operand_stack.iter() {
            match operand {
                Operand::Boolean(value, ..) => {
                    self.editor
                        .put_write_boolean_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Number(value, ..) => {
                    self.editor
                        .put_write_number_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::String(value, ..) => {
                    // TODO(issue#237): GcCellRef
                    self.editor
                        .put_write_string_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Closure(value) => {
                    // TODO(issue#237): GcCellRef
                    self.editor
                        .put_write_closure_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Object(value) => {
                    // TODO(issue#237): GcCellRef
                    self.editor
                        .put_write_object_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Function(value, _) => {
                    // TODO(issue#237): GcCellRef
                    self.editor.put_write_function_to_scratch_buffer(
                        *value,
                        scratch_buffer,
                        offset,
                    );
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Promise(value) => {
                    self.editor
                        .put_write_promise_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Any(value, ..) => {
                    self.editor
                        .put_write_any_to_scratch_buffer(*value, scratch_buffer, offset);
                    offset += Value::SIZE;
                }
                Operand::Undefined
                | Operand::Null
                | Operand::VariableReference(..)
                | Operand::PropertyReference(_) => (),
                Operand::Lambda(..) | Operand::Coroutine(_) => unreachable!("{operand:?}"),
            }
        }
        self.operand_stack = operand_stack;

        // TODO: Should return a compile error.
        assert!(offset <= u16::MAX as usize);
        self.max_scratch_buffer_len = self.max_scratch_buffer_len.max(offset as u32);
    }

    fn perform_load_operands_from_scratch_buffer(&mut self) {
        logger::debug!(event = "perform_load_operands_from_scratch_buffer");
        let scratch_buffer = self.editor.put_get_scratch_buffer_from_coroutine();
        // Take the whole operand stack in order to avoid the borrow checker.
        let mut operand_stack = std::mem::take(&mut self.operand_stack);
        let mut offset = 0;
        for operand in operand_stack.iter_mut() {
            match operand {
                Operand::Boolean(value, ..) => {
                    *value = self
                        .editor
                        .put_read_boolean_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Number(value, ..) => {
                    *value = self
                        .editor
                        .put_read_number_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::String(value, ..) => {
                    // TODO(issue#237): GcCellRef
                    *value = self
                        .editor
                        .put_read_string_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Closure(value) => {
                    // TODO(issue#237): GcCellRef
                    *value = self
                        .editor
                        .put_read_closure_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Object(value) => {
                    // TODO(issue#237): GcCellRef
                    *value = self
                        .editor
                        .put_read_object_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Function(value, _) => {
                    // TODO(issue#237): GcCellRef
                    *value = self
                        .editor
                        .put_read_function_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Promise(value) => {
                    *value = self
                        .editor
                        .put_read_promise_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::HOLDER_SIZE;
                }
                Operand::Any(value, ..) => {
                    *value = self
                        .editor
                        .put_read_any_from_scratch_buffer(scratch_buffer, offset);
                    offset += Value::SIZE;
                }
                Operand::Undefined
                | Operand::Null
                | Operand::VariableReference(..)
                | Operand::PropertyReference(_) => (),
                Operand::Lambda(..) | Operand::Coroutine(_) => unreachable!("{operand:?}"),
            }
        }
        self.operand_stack = operand_stack;
    }

    fn perform_resolve_promise(&mut self) {
        logger::debug!(event = "perform_resolve_promise");

        let promise = self.get_hidden_promise();
        let promise = self.editor.put_load_promise(promise);

        let (operand, ..) = self.dereference();
        match operand {
            Operand::Undefined => {
                let result = self.editor.put_alloc_any();
                self.editor.put_store_undefined_to_any(result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::Null => {
                let result = self.editor.put_alloc_any();
                self.editor.put_store_null_to_any(result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::Boolean(value, ..) => {
                let result = self.editor.put_alloc_any();
                self.editor.put_store_boolean_to_any(value, result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::Number(value, ..) => {
                let result = self.editor.put_alloc_any();
                self.editor.put_store_number_to_any(value, result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::String(value, ..) => {
                let result = self.editor.put_alloc_any();
                let value = self.emit_ensure_heap_string(value);
                self.editor.put_store_string_to_any(value, result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::Object(value) => {
                let result = self.editor.put_alloc_any();
                self.editor.put_store_object_to_any(value, result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::Function(value, _) => {
                let result = self.editor.put_alloc_any();
                self.editor.put_store_function_to_any(value, result);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, result);
            }
            Operand::Promise(value) => {
                self.editor
                    .put_runtime_await_promise(self.support, value, promise);
            }
            Operand::Any(value, ..) => {
                let then_block = self.editor.create_block();
                let else_block = self.editor.create_block();
                let merge_block = self.editor.create_block();
                // if value.is_promise()
                let is_promise = self.editor.put_is_promise(value);
                self.editor
                    .put_branch(is_promise, then_block, &[], else_block, &[]);
                // {
                self.editor.switch_to_block(then_block);
                let target = self.editor.put_load_promise(value);
                self.editor
                    .put_runtime_await_promise(self.support, target, promise);
                self.editor.put_jump(merge_block, &[]);
                // } else {
                self.editor.switch_to_block(else_block);
                self.editor
                    .put_runtime_emit_promise_resolved(self.support, promise, value);
                self.editor.put_jump(merge_block, &[]);
                // }
                self.editor.switch_to_block(merge_block);
            }
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    fn emit_ensure_heap_string(&mut self, string: StringIr) -> StringIr {
        logger::debug!(event = "emit_ensure_heap_string", ?string);
        let then_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_addr();

        // if string.on_stack()
        let on_stack = self.editor.put_string_on_stack(string);
        self.editor
            .put_branch(on_stack, then_block, &[], merge_block, &[string.0.into()]);
        // {
        self.editor.switch_to_block(then_block);
        let heap_string = self
            .editor
            .put_runtime_migrate_string_to_heap(self.support, string);
        self.editor.put_jump(merge_block, &[heap_string.0.into()]);
        // }
        self.editor.switch_to_block(merge_block);
        StringIr(self.editor.get_block_param(merge_block, 0))
    }

    fn process_discard(&mut self) {
        debug_assert!(!self.operand_stack.is_empty());
        self.operand_stack.pop();
    }

    fn process_throw(&mut self) {
        let (operand, ..) = self.dereference();
        self.store_operand_to_retv(&operand);

        self.editor.put_store_status(Status::EXCEPTION);
        self.editor.put_store_flow_selector(FlowSelector::THROW);

        let next_block = self.control_flow_stack.exception_block();
        self.editor.put_jump(next_block, &[]);

        let block = self.editor.create_block_for_deadcode();
        self.editor.switch_to_block(block);
    }

    fn process_swap(&mut self) {
        self.swap();
    }

    fn process_duplicate(&mut self, offset: u8) {
        self.duplicate(offset);
    }

    fn process_dereference(&mut self) {
        let (operand, ..) = self.dereference();
        self.operand_stack.push(operand);
    }

    fn process_debugger(&mut self) {
        self.editor.put_runtime_launch_debugger(self.support);
    }

    // commonly used functions

    fn perform_is_non_nullish(&mut self, operand: &Operand) -> BooleanIr {
        match operand {
            Operand::Undefined | Operand::Null => self.editor.put_boolean(false),
            Operand::Boolean(..)
            | Operand::Number(..)
            | Operand::Object(_)
            | Operand::Function(..)
            | Operand::Promise(_) => self.editor.put_boolean(true),
            Operand::String(..) => todo!("string"),
            Operand::Any(value, ..) => self.editor.put_is_non_nullish(*value),
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    fn pop_boolean(&mut self) -> BooleanIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Boolean(value, ..) => value,
            _ => unreachable!(),
        }
    }

    fn pop_closure(&mut self) -> ClosureIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Closure(value) => value,
            _ => unreachable!(),
        }
    }

    fn pop_any(&mut self) -> AnyIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Any(value, _) => value,
            operand => unreachable!("{operand:?}"),
        }
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

    fn perform_to_boolean(&mut self, operand: &Operand) -> BooleanIr {
        match operand {
            Operand::Undefined | Operand::Null => self.editor.put_boolean(false),
            Operand::Boolean(value, ..) => *value,
            Operand::Number(value, ..) => self.editor.put_number_to_boolean(*value),
            Operand::String(..) => todo!(),
            Operand::Promise(_) | Operand::Object(_) | Operand::Function(..) => {
                self.editor.put_boolean(true)
            }
            Operand::Any(value, ..) => self.editor.put_runtime_to_boolean(self.support, *value),
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => {
                unreachable!("{operand:?}")
            }
        }
    }

    // 7.1.4 ToNumber ( argument )
    fn perform_to_numeric(&mut self, operand: &Operand) -> NumberIr {
        logger::debug!(event = "to_numeric", ?operand);
        match operand {
            Operand::Undefined => self.editor.put_number(f64::NAN),
            Operand::Null => self.editor.put_number(0.0),
            Operand::Boolean(value, ..) => self.editor.put_boolean_to_number(*value),
            Operand::Number(value, ..) => *value,
            Operand::String(..) => unimplemented!("string.to_numeric"),
            Operand::Object(_) | Operand::Function(..) => unimplemented!("object.to_numeric"),
            Operand::Any(value, ..) => self.editor.put_runtime_to_numeric(self.support, *value),
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::Promise(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    // 13.4.2.1 Runtime Semantics: Evaluation
    // 13.4.3.1 Runtime Semantics: Evaluation
    // 13.4.4.1 Runtime Semantics: Evaluation
    // 13.4.5.1 Runtime Semantics: Evaluation
    fn perform_incr_decr(&mut self, pos: char, op: char) {
        let (operand, _, reference) = self.dereference();
        let old_value = self.perform_to_numeric(&operand);
        // TODO: BigInt
        let one = self.editor.put_number(1.0);
        let new_value = if op == '+' {
            self.editor.put_add(old_value, one)
        } else {
            self.editor.put_sub(old_value, one)
        };
        match reference {
            Some((symbol, locator)) if symbol != Symbol::NONE => {
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

    fn perform_to_any(&mut self, operand: &Operand) -> AnyIr {
        let any = self.editor.put_alloc_any();
        self.emit_store_operand_to_any(operand, any);
        any
    }

    // 7.2.13 IsLooselyEqual ( x, y )
    fn perform_is_loosely_equal(&mut self, lhs: &Operand, rhs: &Operand) -> BooleanIr {
        logger::debug!(event = "perform_is_loosely_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs, ..) = lhs {
            // TODO: compile-time evaluation
            let rhs = self.perform_to_any(rhs);
            return self
                .editor
                .put_runtime_is_loosely_equal(self.support, *lhs, rhs);
        }
        if let Operand::Any(rhs, ..) = rhs {
            // TODO: compile-time evaluation
            let lhs = self.perform_to_any(lhs);
            return self
                .editor
                .put_runtime_is_loosely_equal(self.support, lhs, *rhs);
        }

        // 1. If Type(x) is Type(y), then Return IsStrictlyEqual(x, y).
        if std::mem::discriminant(lhs) == std::mem::discriminant(rhs) {
            return self.perform_is_strictly_equal(lhs, rhs);
        }

        // 2. If x is null and y is undefined, return true.
        if matches!(lhs, Operand::Null) && matches!(rhs, Operand::Undefined) {
            return self.editor.put_boolean(true);
        }

        // 3. If x is undefined and y is null, return true.
        if matches!(lhs, Operand::Undefined) && matches!(rhs, Operand::Null) {
            return self.editor.put_boolean(true);
        }

        // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 7. If x is a BigInt and y is a String, then
        // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
        // TODO
        // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: ...
        let lhs = self.perform_to_any(lhs);
        let rhs = self.perform_to_any(rhs);
        self.editor
            .put_runtime_is_loosely_equal(self.support, lhs, rhs)
    }

    // 7.2.14 IsStrictlyEqual ( x, y )
    fn perform_is_strictly_equal(&mut self, lhs: &Operand, rhs: &Operand) -> BooleanIr {
        logger::debug!(event = "create_is_strictly_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs, ..) = lhs {
            return self.perform_any_is_strictly_equal(*lhs, rhs);
        }
        if let Operand::Any(rhs, ..) = rhs {
            return self.perform_any_is_strictly_equal(*rhs, lhs);
        }
        if std::mem::discriminant(lhs) != std::mem::discriminant(rhs) {
            return self.editor.put_boolean(false);
        }
        // TODO: BigInt
        match (lhs, rhs) {
            (Operand::Undefined, Operand::Undefined) => self.editor.put_boolean(true),
            (Operand::Null, Operand::Null) => self.editor.put_boolean(true),
            (Operand::Boolean(lhs, ..), Operand::Boolean(rhs, ..)) => {
                self.editor.put_is_same_boolean(*lhs, *rhs)
            }
            (Operand::Number(lhs, ..), Operand::Number(rhs, ..)) => {
                self.editor.put_is_same_number(*lhs, *rhs)
            }
            (Operand::String(lhs, ..), Operand::String(rhs, ..)) => {
                self.editor.put_is_same_string(self.support, *lhs, *rhs)
            }
            (Operand::Promise(lhs), Operand::Promise(rhs)) => {
                self.editor.put_is_same_promise(*lhs, *rhs)
            }
            (Operand::Object(lhs), Operand::Object(rhs)) => {
                self.editor.put_is_same_object(*lhs, *rhs)
            }
            (Operand::Function(lhs, _), Operand::Function(rhs, _)) => {
                self.editor.put_is_same_function(*lhs, *rhs)
            }
            (lhs, rhs) => unreachable!("({lhs:?}, {rhs:?})"),
        }
    }

    fn perform_any_is_strictly_equal(&mut self, lhs: AnyIr, rhs: &Operand) -> BooleanIr {
        logger::debug!(event = "create_any_is_strictly_equal", ?lhs, ?rhs);
        match rhs {
            Operand::Undefined => self.editor.put_is_undefined(lhs),
            Operand::Null => self.editor.put_is_null(lhs),
            Operand::Boolean(rhs, ..) => self.perform_is_same_boolean(lhs, *rhs),
            Operand::Number(rhs, ..) => self.perform_is_same_number(lhs, *rhs),
            Operand::String(rhs, ..) => self.perform_is_same_string(lhs, *rhs),
            Operand::Object(rhs) => self.perform_is_same_object(lhs, *rhs),
            Operand::Function(rhs, _) => self.perform_is_same_function(lhs, *rhs),
            Operand::Promise(rhs) => self.perform_is_same_promise(lhs, *rhs),
            Operand::Any(rhs, ..) => {
                self.editor
                    .put_runtime_is_strictly_equal(self.support, lhs, *rhs)
            }
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{rhs:?}"),
        }
    }

    fn perform_is_same_boolean(&mut self, value: AnyIr, boolean: BooleanIr) -> BooleanIr {
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_i8();

        // if value.kind == ValueKind::Boolean
        let cond = self.editor.put_is_boolean(value);
        self.editor
            .put_branch(cond, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let b = self.editor.put_load_boolean(value);
        let then_value = self.editor.put_is_same_boolean(b, boolean);
        self.editor.put_jump(merge_block, &[then_value.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        let else_value = self.editor.put_boolean(false);
        self.editor.put_jump(merge_block, &[else_value.0.into()]);
        // }

        self.editor.switch_to_block(merge_block);
        BooleanIr(self.editor.get_block_param(merge_block, 0))
    }

    fn perform_is_same_number(&mut self, value: AnyIr, number: NumberIr) -> BooleanIr {
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_i8();

        // if value.kind == ValueKind::Number
        let cond = self.editor.put_is_number(value);
        self.editor
            .put_branch(cond, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let n = self.editor.put_load_number(value);
        let then_value = self.editor.put_is_same_number(n, number);
        self.editor.put_jump(merge_block, &[then_value.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        let else_value = self.editor.put_boolean(false);
        self.editor.put_jump(merge_block, &[else_value.0.into()]);
        // }

        self.editor.switch_to_block(merge_block);
        BooleanIr(self.editor.get_block_param(merge_block, 0))
    }

    fn perform_is_same_string(&mut self, value: AnyIr, string: StringIr) -> BooleanIr {
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_i8();

        // if value.kind == ValueKind::String
        let cond = self.editor.put_is_string(value);
        self.editor
            .put_branch(cond, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let s = self.editor.put_load_string(value);
        let then_value = self.editor.put_is_same_string(self.support, s, string);
        self.editor.put_jump(merge_block, &[then_value.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        let else_value = self.editor.put_boolean(false);
        self.editor.put_jump(merge_block, &[else_value.0.into()]);
        // }

        self.editor.switch_to_block(merge_block);
        BooleanIr(self.editor.get_block_param(merge_block, 0))
    }

    fn perform_is_same_object(&mut self, value: AnyIr, object: ObjectIr) -> BooleanIr {
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_i8();

        // if value.kind == ValueKind::Object
        let cond = self.editor.put_is_object(value);
        self.editor
            .put_branch(cond, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let v = self.editor.put_load_object(value);
        let then_value = self.editor.put_is_same_object(v, object);
        self.editor.put_jump(merge_block, &[then_value.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        let else_value = self.editor.put_boolean(false);
        self.editor.put_jump(merge_block, &[else_value.0.into()]);
        // }

        self.editor.switch_to_block(merge_block);
        BooleanIr(self.editor.get_block_param(merge_block, 0))
    }

    fn perform_is_same_function(&mut self, value: AnyIr, object: ObjectIr) -> BooleanIr {
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_i8();

        // if value.kind == ValueKind::Function
        let cond = self.editor.put_is_function(value);
        self.editor
            .put_branch(cond, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let v = self.editor.put_load_function(value);
        let then_value = self.editor.put_is_same_function(v, object);
        self.editor.put_jump(merge_block, &[then_value.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        let else_value = self.editor.put_boolean(false);
        self.editor.put_jump(merge_block, &[else_value.0.into()]);
        // }

        self.editor.switch_to_block(merge_block);
        BooleanIr(self.editor.get_block_param(merge_block, 0))
    }

    fn perform_is_same_promise(&mut self, value: AnyIr, promise: PromiseIr) -> BooleanIr {
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let merge_block = self.editor.create_block_with_i8();

        // if value.kind == ValueKind::Promise
        let cond = self.editor.put_is_promise(value);
        self.editor
            .put_branch(cond, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let v = self.editor.put_load_promise(value);
        let then_value = self.editor.put_is_same_promise(v, promise);
        self.editor.put_jump(merge_block, &[then_value.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        let else_value = self.editor.put_boolean(false);
        self.editor.put_jump(merge_block, &[else_value.0.into()]);
        // }

        self.editor.switch_to_block(merge_block);
        BooleanIr(self.editor.get_block_param(merge_block, 0))
    }

    fn pop_lambda(&mut self) -> (LambdaIr, LambdaId) {
        match self.operand_stack.pop().unwrap() {
            Operand::Lambda(lambda, lambda_id) => (lambda, lambda_id),
            _ => unreachable!(),
        }
    }

    fn pop_reference(&mut self) -> (Symbol, Locator) {
        match self.operand_stack.pop().unwrap() {
            Operand::VariableReference(symbol, locator) => (symbol, locator),
            operand => unreachable!("{operand:?}"),
        }
    }

    // TODO(refactor): need rethink, especially the return value.
    fn dereference(&mut self) -> (Operand, Option<ObjectIr>, Option<(Symbol, Locator)>) {
        logger::debug!(event = "dereference", operand_stack.top = ?self.operand_stack.last());

        let operand = self.operand_stack.pop().unwrap();
        match operand {
            // Shortcut for frequently used reference to `undefined`.
            Operand::VariableReference(Symbol::UNDEFINED, Locator::Global) => (
                Operand::Undefined,
                None,
                Some((Symbol::UNDEFINED, Locator::Global)),
            ),
            Operand::VariableReference(symbol, locator) => {
                let value = self.emit_get_variable(symbol, locator);
                // TODO(pref): compile-time evaluation
                (Operand::Any(value, None), None, Some((symbol, locator)))
            }
            Operand::PropertyReference(key) => {
                self.perform_to_object();
                let object = self.pop_object();
                let then_block = self.editor.create_block();
                let end_block = self.editor.create_block();
                // if object.is_nullptr()
                let is_nullptr = self.editor.put_is_nullptr(object.0);
                self.editor
                    .put_branch(is_nullptr, then_block, &[], end_block, &[]);
                // {
                self.editor.switch_to_block(then_block);
                self.process_number(1001.); // TODO: TypeError
                self.process_throw();
                // }
                self.editor.switch_to_block(end_block);
                let value = match key {
                    PropertyKey::Symbol(key) => self.editor.put_runtime_get_value_by_symbol(
                        self.support,
                        object,
                        key,
                        false,
                    ),
                    PropertyKey::Number(key) => self.editor.put_runtime_get_value_by_number(
                        self.support,
                        object,
                        key,
                        false,
                    ),
                    PropertyKey::Any(key) => {
                        self.editor
                            .put_runtime_get_value_by_any(self.support, object, key, false)
                    }
                };
                runtime_debug! {{
                    let is_nullptr = self.editor.put_is_nullptr(value.0);
                    let non_nullptr = self.editor.put_logical_not(is_nullptr);
                    self.editor.put_runtime_assert(self.support,
                        non_nullptr,
                        c"runtime.get_value() should return a non-null pointer",
                    );
                }}
                // TODO(pref): compile-time evaluation
                (Operand::Any(value, None), Some(object), None)
            }
            _ => (operand, None, None),
        }
    }

    fn perform_escape_value(&mut self, locator: Locator) {
        debug_assert!(!locator.is_capture());
        debug_assert!(self.captures.contains_key(&locator));
        logger::debug!(event = "perform_escape_value", ?locator);
        let capture = self.captures.remove(&locator).unwrap();
        let value = match locator {
            Locator::Argument(index) => self.get_argument(index),
            Locator::Local(index) => self.get_local(index),
            Locator::Capture(index) => self.editor.put_load_captured_value(index),
            Locator::Global => unreachable!(),
        };
        self.editor.put_escape_value(capture, value);
    }

    fn swap(&mut self) {
        logger::debug!(event = "swap");
        debug_assert!(self.operand_stack.len() > 1);

        let last_index = self.operand_stack.len() - 1;
        if self.operand_stack.len() == 2 {
            self.operand_stack.swap(last_index - 1, last_index);
            return;
        }

        // Operand::PropertyReference must be moved together with the previous operand that is the
        // target value of the property access.
        debug_assert!(self.operand_stack.len() > 2);
        match &self.operand_stack[(last_index - 2)..] {
            [
                Operand::PropertyReference(_),
                Operand::PropertyReference(_),
                _,
            ] => {
                // This case never happens.
                // The MemberExpression must be evaluated before subsequent terms.
                // See also: modifyMemberExpression() in //libs/jsparser/src/transpile.js.
                unreachable!();
            }
            [
                _,
                Operand::PropertyReference(_),
                Operand::PropertyReference(_),
            ] => {
                // This case never happens.
                // The MemberExpression must be evaluated before subsequent terms.
                // See also: modifyMemberExpression() in //libs/jsparser/src/transpile.js.
                unreachable!();
            }
            [
                Operand::PropertyReference(_),
                _,
                Operand::PropertyReference(_),
            ] => {
                debug_assert!(self.operand_stack.len() > 3);
                self.operand_stack.swap(last_index - 3, last_index - 1);
                self.operand_stack.swap(last_index - 2, last_index);
            }
            [_, Operand::PropertyReference(_), _] => {
                self.operand_stack[(last_index - 2)..].rotate_right(1);
            }
            [_, _, Operand::PropertyReference(_)] => {
                self.operand_stack[(last_index - 2)..].rotate_left(1);
            }
            _ => self.operand_stack.swap(last_index - 1, last_index),
        }
    }

    fn duplicate(&mut self, offset: u8) {
        logger::debug!(event = "duplicate", offset);
        debug_assert!(self.operand_stack.len() > offset as usize);
        let index = self.operand_stack.len() - 1 - offset as usize;
        self.operand_stack.duplicate(index);
    }

    // retv

    fn store_operand_to_retv(&mut self, operand: &Operand) {
        let retv = self.editor.retv();
        self.emit_store_operand_to_any(operand, retv);
    }

    fn store_any_to_retv(&mut self, any: AnyIr) {
        let retv = self.editor.retv();
        self.editor.put_store_any_to_any(any, retv);
    }

    // stack allocation

    fn emit_create_any(&mut self) -> AnyIr {
        logger::debug!(event = "emit_create_any");
        let any = self.editor.put_alloc_any();
        self.editor.put_store_none_to_any(any);
        any
    }

    fn emit_create_argv(&mut self, argc: u16) -> ArgvIr {
        logger::debug!(event = "emit_create_argv", argc);
        let argv = self.editor.put_alloc_argv(argc);
        // TODO: evaluation order
        for i in (0..argc).rev() {
            let (operand, ..) = self.dereference();
            // TODO(perf): inefficient
            let arg = self.editor.put_get_arg(argv, i);
            self.emit_store_operand_to_any(&operand, arg);
        }
        argv
    }

    fn emit_store_operand_to_any(&mut self, operand: &Operand, any: AnyIr) {
        match operand {
            Operand::Undefined => self.editor.put_store_undefined_to_any(any),
            Operand::Null => self.editor.put_store_null_to_any(any),
            Operand::Boolean(value, _) => self.editor.put_store_boolean_to_any(*value, any),
            Operand::Number(value, _) => self.editor.put_store_number_to_any(*value, any),
            Operand::String(value, _) => self.editor.put_store_string_to_any(*value, any),
            Operand::Object(value) => self.editor.put_store_object_to_any(*value, any),
            Operand::Function(value, _) => self.editor.put_store_function_to_any(*value, any),
            Operand::Any(value, _) => self.editor.put_store_any_to_any(*value, any),
            Operand::Promise(value) => self.editor.put_store_promise_to_any(*value, any),
            Operand::Lambda(..)
            | Operand::Closure(_)
            | Operand::Coroutine(_)
            | Operand::VariableReference(..)
            | Operand::PropertyReference(_) => unreachable!("{operand:?}"),
        }
    }

    // instructions

    fn emit_get_variable(&mut self, symbol: Symbol, locator: Locator) -> AnyIr {
        logger::debug!(event = "emit_get_variable", ?symbol, ?locator);
        match locator {
            Locator::Argument(index) => self.get_argument(index),
            Locator::Local(index) => self.get_local(index),
            Locator::Capture(index) => self.editor.put_load_captured_value(index),
            Locator::Global => self.emit_get_global_variable(symbol),
        }
    }

    fn get_hidden_promise(&self) -> AnyIr {
        logger::debug!(event = "get_hidden_promise");
        self.get_argument(Self::HIDDEN_PROMISE_INDEX)
    }

    fn get_hidden_result(&self) -> AnyIr {
        logger::debug!(event = "get_hidden_result");
        self.get_argument(Self::HIDDEN_RESULT_INDEX)
    }

    fn get_hidden_error(&self) -> AnyIr {
        logger::debug!(event = "get_hidden_error");
        self.get_argument(Self::HIDDEN_ERROR_INDEX)
    }

    fn get_argument(&self, index: u16) -> AnyIr {
        logger::debug!(event = "get_argument", ?index);
        self.arguments[index as usize]
    }

    fn get_local(&self, index: u16) -> AnyIr {
        logger::debug!(event = "get_local", ?index);
        self.locals[index as usize]
    }

    // TODO(perf): return the value directly if it's a read-only global property.
    fn emit_get_global_variable(&mut self, key: Symbol) -> AnyIr {
        logger::debug!(event = "emit_get_global_variable", ?key);
        let object = ObjectIr(self.editor.put_nullptr());

        // TODO: strict mode
        let value = self
            .editor
            .put_runtime_get_value_by_symbol(self.support, object, key, true);

        let then_block = self.editor.create_block();
        let end_block = self.editor.create_block();

        // if value.is_nullptr()
        let is_nullptr = self.editor.put_is_nullptr(value.0);
        self.editor
            .put_branch(is_nullptr, then_block, &[], end_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        // TODO(feat): ReferenceError
        self.process_number(1000.);
        self.process_throw();
        self.editor.put_jump(end_block, &[]);
        // }
        self.editor.switch_to_block(end_block);

        value
    }

    fn emit_load_function_or_throw_type_error(&mut self, value: AnyIr) -> ObjectIr {
        logger::debug!(event = "emit_function_closure_or_throw_type_error", ?value);
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let end_block = self.editor.create_block_with_addr();

        // if value.is_function()
        let is_function = self.editor.put_is_function(value);
        self.editor
            .put_branch(is_function, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let object = self.editor.put_load_function(value);
        self.editor.put_jump(end_block, &[object.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        self.process_number(1001.); // TODO(feat): TypeError
        self.process_throw();
        let dummy = self.editor.put_nullptr();
        self.editor.put_jump(end_block, &[dummy.into()]);
        // }

        self.editor.switch_to_block(end_block);
        ObjectIr(self.editor.get_block_param(end_block, 0))
    }

    fn emit_load_closure_or_throw_type_error(&mut self, value: AnyIr) -> ClosureIr {
        logger::debug!(event = "emit_load_closure_or_throw_type_error", ?value);
        let then_block = self.editor.create_block();
        let else_block = self.editor.create_block();
        let end_block = self.editor.create_block_with_addr();

        // if value.is_function()
        let is_function = self.editor.put_is_function(value);
        self.editor
            .put_branch(is_function, then_block, &[], else_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        let object = self.editor.put_load_function(value);
        let closure = self.editor.put_load_closure_from_function(object);
        self.editor.put_jump(end_block, &[closure.0.into()]);
        // } else {
        self.editor.switch_to_block(else_block);
        self.process_number(1001.); // TODO(feat): TypeError
        self.process_throw();
        let dummy = self.editor.put_nullptr();
        self.editor.put_jump(end_block, &[dummy.into()]);
        // }

        self.editor.switch_to_block(end_block);
        ClosureIr(self.editor.get_block_param(end_block, 0))
    }

    // captures

    fn emit_check_status_for_exception(&mut self, status: StatusIr, retv: AnyIr) {
        logger::debug!(event = "emit_check_status_for_exception", ?status, ?retv);

        let exception_block = self.control_flow_stack.exception_block();

        let then_block = self.editor.create_block();
        let merge_block = self.editor.create_block();

        // if status.is_exception()
        let is_exception = self.editor.put_is_exception_status(status);
        self.editor
            .put_branch(is_exception, then_block, &[], merge_block, &[]);
        // {
        self.editor.switch_to_block(then_block);
        self.editor.put_store_status(Status::EXCEPTION);
        self.editor.put_store_flow_selector(FlowSelector::THROW);
        self.store_any_to_retv(retv);
        self.editor.put_jump(exception_block, &[]);
        // }

        self.editor.switch_to_block(merge_block);
    }
}

// operands

struct OperandStack(Vec<Operand>);

impl OperandStack {
    fn new() -> Self {
        Self(vec![])
    }

    fn duplicate(&mut self, index: usize) {
        let dup = self.0[index].clone();
        self.push(dup);
    }

    fn dump(&self) {
        for operand in self.0.iter().rev() {
            eprintln!("{operand:?}");
        }
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

/// Values pushed on to the operand stack.
// TODO(feat): add variant for BigInt
#[derive(Clone, Debug)]
enum Operand {
    // Values that can be store into a `Value`.
    /// Compile-time constant value of `undefined`.
    Undefined,

    /// Compile-time constant value of `null`.
    Null,

    /// Runtime value and optional compile-time constant value of boolean type.
    // TODO(perf): compile-time evaluation
    Boolean(BooleanIr, #[allow(unused)] Option<bool>),

    /// Runtime value and optional compile-time constant value of number type.
    // TODO(perf): compile-time evaluation
    Number(NumberIr, #[allow(unused)] Option<f64>),

    /// Runtime value and optional compile-time constant value of number type.
    // TODO(perf): compile-time evaluation
    String(StringIr, #[allow(unused)] Option<U16Chunk>),

    /// Runtime value of closure type.
    Closure(ClosureIr),

    /// Runtime value of object type.
    Object(ObjectIr),

    /// Runtime value of function type.
    Function(ObjectIr, Symbol),

    /// Runtime value and optional compile-time constant value of any type.
    // TODO(perf): compile-time evaluation
    Any(AnyIr, Option<Value>),

    // Values that cannot be stored into a `Value`.
    /// Runtime value of lambda function type.
    Lambda(LambdaIr, LambdaId),

    /// Runtime value of coroutine type.
    Coroutine(CoroutineIr),

    /// Runtime value of promise type.
    Promise(PromiseIr),

    // Compile-time constant value types.
    VariableReference(Symbol, Locator),
    PropertyReference(PropertyKey),
}

#[derive(Clone, Debug)]
enum PropertyKey {
    // Compile-time values
    Symbol(Symbol),
    Number(f64),

    // Runtime value w/ optional compile-time value
    Any(AnyIr),
}

impl From<Symbol> for PropertyKey {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl From<f64> for PropertyKey {
    fn from(value: f64) -> Self {
        // Use objects::PropertyKey::from() in order to remove code clone.
        use crate::objects::PropertyKey;
        match PropertyKey::from(value) {
            PropertyKey::Symbol(value) => Self::Symbol(value),
            PropertyKey::Number(value) => Self::Number(value),
        }
    }
}

impl From<AnyIr> for PropertyKey {
    fn from(value: AnyIr) -> Self {
        Self::Any(value)
    }
}

/// A runtime boolean value in `ir::types::I8`.
#[derive(Clone, Copy, Debug)]
struct BooleanIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct NumberIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct StringIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct ClosureIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct ObjectIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct AnyIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct LambdaIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct CaptureIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct CoroutineIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct PromiseIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct ArgvIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct StatusIr(ir::Value);

#[derive(Clone, Copy, Debug)]
struct Status(u32);

#[allow(unused)]
impl Status {
    const UNSET_BIT: u32 = 0x10;
    const MASK: u32 = 0x0F;

    const NORMAL: Self = Self(0x00);
    const EXCEPTION: Self = Self(0x01);
    const SUSPEND: Self = Self(0x02);
    const UNSET: Self = Self(Self::UNSET_BIT | Self::NORMAL.0);

    const fn imm(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct FlowSelector(u32);

#[allow(unused)]
impl FlowSelector {
    const KIND_RETURN: u8 = 0;
    const KIND_THROW: u8 = 1;
    const KIND_BREAK: u8 = 2;
    const KIND_CONTINUE: u8 = 3;
    const KIND_NORMAL: u8 = 0xFF;

    const NORMAL: Self = Self::new(1, 0xFF, Self::KIND_NORMAL);
    const RETURN: Self = Self::new(0, 0, Self::KIND_RETURN);
    const THROW: Self = Self::new(0, 0, Self::KIND_THROW);

    const fn new(extra: u16, depth: u32, kind: u8) -> Self {
        Self(((extra as u32) << 16) | depth | (kind as u32))
    }

    const fn break_at(depth: u32) -> Self {
        Self::new(0, depth, Self::KIND_BREAK)
    }

    const fn continue_at(depth: u32) -> Self {
        Self::new(0, depth, Self::KIND_CONTINUE)
    }

    const fn imm(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Debug for FlowSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NORMAL => write!(f, "FlowSelector(Normal)"),
            Self::RETURN => write!(f, "FlowSelector(Return)"),
            Self::THROW => write!(f, "FlowSelector(Throw)"),
            _ => {
                let depth = ((self.0 >> 8) & 0xFF) as u8;
                let kind = (self.0 & 0xFF) as u8;
                if kind == Self::KIND_BREAK {
                    write!(f, "FlowSelector(Break@{depth})")
                } else {
                    write!(f, "FlowSelector(Continue@{depth})")
                }
            }
        }
    }
}
