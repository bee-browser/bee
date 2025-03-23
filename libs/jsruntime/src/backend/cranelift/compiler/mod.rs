mod control_flow;
mod runtime;

use std::ops::Deref;
use std::ops::DerefMut;

use codegen::ir::FuncRef;
use codegen::ir::SigRef;
use codegen::ir::StackSlot;
use cranelift::prelude::*;
use cranelift_jit::JITBuilder;
use cranelift_jit::JITModule;
use cranelift_module::DataDescription;
use cranelift_module::FuncId;
use cranelift_module::Linkage;
use cranelift_module::Module as _;

use base::static_assert_eq;
use jsparser::Symbol;

use super::CompileError;
use super::Module;
use super::Program;
use crate::backend::CompilerSupport;
use crate::backend::RuntimeFunctions;
use crate::logger;
use crate::semantics::CompileCommand;
use crate::semantics::Function;
use crate::semantics::Locator;
use crate::semantics::ScopeRef;
use crate::semantics::ScopeTree;
use crate::semantics::VariableRef;

use control_flow::ControlFlowStack;
use runtime::RuntimeFunctionCache;
use runtime::RuntimeFunctionIds;

pub fn compile<R>(
    support: &mut R,
    program: &Program,
    optimize: bool,
) -> Result<Module, CompileError>
where
    R: CompilerSupport,
{
    let runtime_functions = support.get_runtime_functions();

    // TODO: Deferring the compilation until it's actually called improves the performance.
    // Because the program may contain unused functions.
    let mut context = CraneliftContext::new(&runtime_functions);

    // Compile JavaScript functions in reverse order in order to compile a coroutine function
    // before its ramp function so that the size of the scratch buffer for the coroutine
    // function is available when the ramp function is compiled.
    //
    // NOTE: The functions are stored in post-order traversal on the function tree.  So, we
    // don't need to use `Iterator::rev()`.
    //
    // TODO: We should manage dependencies between functions in a more general way.
    for func in program.functions.iter() {
        let mut compiler = context.create_compiler(support, &program.scope_tree);
        compiler.compile(func, optimize);
    }

    Ok(Module {
        inner: context.module,
        context: context.context,
    })
}

struct CraneliftContext {
    builder_context: FunctionBuilderContext,
    context: codegen::Context,
    _data_description: DataDescription,
    module: JITModule,
    id_fmod: FuncId,
    id_pow: FuncId,
    runtime_func_ids: RuntimeFunctionIds,
}

impl CraneliftContext {
    fn new(runtime_functions: &RuntimeFunctions) -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        runtime::register_symbols(&mut builder, runtime_functions);

        let mut module = JITModule::new(builder);
        let runtime_func_ids = runtime::declare_functions(&mut module);

        // TODO(feat): if cfg!(feature = "libm")
        let name = "fmod";
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(types::F64));
        sig.params.push(AbiParam::new(types::F64));
        sig.returns.push(AbiParam::new(types::F64));
        let id_fmod = module
            .declare_function(name, Linkage::Import, &sig)
            .unwrap();

        let name = "pow";
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(types::F64));
        sig.params.push(AbiParam::new(types::F64));
        sig.returns.push(AbiParam::new(types::F64));
        let id_pow = module
            .declare_function(name, Linkage::Import, &sig)
            .unwrap();

        Self {
            builder_context: FunctionBuilderContext::new(),
            context: module.make_context(),
            _data_description: DataDescription::new(),
            module,
            id_fmod,
            id_pow,
            runtime_func_ids,
        }
    }

    fn create_compiler<'r, 's, R>(
        &mut self,
        runtime: &'r mut R,
        scope_tree: &'s ScopeTree,
    ) -> Compiler<'r, 's, '_, R>
    where
        R: CompilerSupport,
    {
        Compiler::new(runtime, scope_tree, self)
    }
}

struct Compiler<'r, 's, 'c, R> {
    support: &'r mut R,

    /// The scope tree of the JavaScript program to compile.
    scope_tree: &'s ScopeTree,

    /// A stack to hold sets of basic blocks which construct of a region in the control flow graph
    /// (CFG) finally built.
    control_flow_stack: ControlFlowStack,

    pending_labels: Vec<Symbol>,

    builder: FunctionBuilder<'c>,
    module: &'c mut JITModule,
    ptr_type: Type,
    lambda_sig: SigRef,
    ref_fmod: FuncRef,
    ref_pow: FuncRef,
    runtime_func_cache: RuntimeFunctionCache<'c>,

    /// A stack for operands.
    operand_stack: OperandStack,

    // The following values must be reset in the end of compilation for each function.
    locals: Vec<AnyIr>,
}

impl<'r, 's, 'c, R> Compiler<'r, 's, 'c, R>
where
    R: CompilerSupport,
{
    fn new(
        support: &'r mut R,
        scope_tree: &'s ScopeTree,
        context: &'c mut CraneliftContext,
    ) -> Self {
        let ptr_type = context.module.target_config().pointer_type();

        // formal parameters
        let params = &mut context.context.func.signature.params;
        // runtime: *mut c_void
        params.push(AbiParam::new(ptr_type));
        // context: *mut c_void
        params.push(AbiParam::new(ptr_type));
        // args: u16
        params.push(AbiParam::new(types::I16));
        // argv: *mut Value
        params.push(AbiParam::new(ptr_type));
        // retv: *mut Value
        params.push(AbiParam::new(ptr_type));

        // #[repr(u32)] Status
        context
            .context
            .func
            .signature
            .returns
            .push(AbiParam::new(types::I32));

        let lambda_sig = context.context.func.signature.clone();

        let ref_fmod = context
            .module
            .declare_func_in_func(context.id_fmod, &mut context.context.func);

        let ref_pow = context
            .module
            .declare_func_in_func(context.id_pow, &mut context.context.func);

        let mut builder =
            FunctionBuilder::new(&mut context.context.func, &mut context.builder_context);

        let lambda_sig = builder.import_signature(lambda_sig);

        Self {
            support,
            scope_tree,
            control_flow_stack: Default::default(),
            pending_labels: Default::default(),
            builder,
            module: &mut context.module,
            ptr_type,
            lambda_sig,
            ref_fmod,
            ref_pow,
            runtime_func_cache: RuntimeFunctionCache::new(&context.runtime_func_ids),
            operand_stack: Default::default(),
            locals: Default::default(),
        }
    }

    fn compile(&mut self, func: &Function, optimize: bool) {
        self.start_compile(func);

        for command in func.commands.iter() {
            self.process_command(command);
        }

        self.end_compile(func, optimize);
    }

    fn start_compile(&mut self, func: &Function) {
        logger::debug!(event = "start_compile", ?func.name, ?func.id);

        // Unlike LLVM IR, we cannot specify a label for each basic block.  This is bad from a
        // debugging and readability perspective...
        let entry_block = self.create_entry_block();
        let body_block = self.create_block();
        let exit_block = self.create_block();

        self.control_flow_stack
            .push_function_flow(entry_block, body_block, exit_block);

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
        self.switch_to_block(entry_block);
        self.emit_jump(body_block);

        // Immediately call `seal_block()` with the `body_block`.  This block is always inserted
        // just after the `entry_block`.  Blocks may be inserted between the `body_block` and the
        // `exit_block`.
        self.seal_block(body_block);

        // The `entry_block` is already in the layout.  We can insert empty blocks after it.
        self.insert_block_after(body_block, entry_block);
        self.insert_block_after(exit_block, body_block);

        //self.bridge.create_store_undefined_to_retv();
        // TODO: self.bridge.create_alloc_status();
        // TODO: self.bridge.create_alloc_flow_selector();
        if self.support.is_scope_cleanup_checker_enabled() {
            let _is_coroutine = self.support.get_lambda_info(func.id).is_coroutine;
            // TODO: self.bridge.enable_scope_cleanup_checker(is_coroutine);
        }

        self.switch_to_block(body_block);
    }

    fn end_compile(&mut self, func: &Function, optimize: bool) {
        logger::debug!(event = "end_compile", ?func.id, optimize);

        debug_assert!(self.operand_stack.is_empty());

        let dormant_block = self
            .support
            .get_lambda_info(func.id)
            .is_coroutine
            .then(|| self.control_flow_stack.pop_coroutine_flow().dormant_block);

        self.control_flow_stack.pop_exit_target();
        let flow = self.control_flow_stack.pop_function_flow();

        self.emit_jump(flow.exit_block);

        self.seal_block(flow.exit_block);
        self.switch_to_block(flow.exit_block);
        if let Some(_block) = dormant_block {
            //self.move_block_after(block);
        }

        if self.support.is_scope_cleanup_checker_enabled() {
            // TODO: self.bridge.assert_scope_id(ScopeRef::NONE);
        }

        // TODO: self.bridge.end_function(optimize);
        let retv = self.builder.ins().iconst(types::I32, 0);
        self.builder.ins().return_(&[retv]);

        // TODO: self.locals.clear();

        // TODO: debug_assert!(self.captures.is_empty());
        // TODO: self.captures.clear();

        debug_assert!(self.control_flow_stack.is_empty());
        self.control_flow_stack.clear();

        let info = self.support.get_lambda_info_mut(func.id);
        if info.is_coroutine {
            // TODO: info.scratch_buffer_len = self.max_scratch_buffer_len;
        }
        // TODO: self.max_scratch_buffer_len = 0;
    }

    fn process_command(&mut self, command: &CompileCommand) {
        logger::debug!(event = "process_command", ?command);
        match command {
            CompileCommand::Nop => (),
            CompileCommand::Undefined => self.process_undefined(),
            CompileCommand::Null => self.process_null(),
            CompileCommand::Boolean(value) => self.process_boolean(*value),
            CompileCommand::Number(value) => self.process_number(*value),
            CompileCommand::VariableReference(symbol) => self.process_variable_reference(*symbol),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::DeclareVars(scope_ref) => self.process_declare_vars(*scope_ref),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(*scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
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
            CompileCommand::Discard => self.process_discard(),
            CompileCommand::Swap => self.process_swap(),
            _ => todo!("{command:?}"),
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

    fn process_undefined(&mut self) {
        self.operand_stack.push(Operand::Undefined);
    }

    fn process_null(&mut self) {
        self.operand_stack.push(Operand::Null);
    }

    fn process_boolean(&mut self, value: bool) {
        let value_ir = self.emit_boolean(value);
        self.operand_stack
            .push(Operand::Boolean(value_ir, Some(value)));
    }

    fn process_number(&mut self, value: f64) {
        let value_ir = self.emit_number(value);
        self.operand_stack
            .push(Operand::Number(value_ir, Some(value)));
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

    fn process_allocate_locals(&mut self, _num_locals: u16) {
        // TODO
    }

    fn process_declare_vars(&mut self, scope_ref: ScopeRef) {
        debug_assert!(self.scope_tree.scope(scope_ref).is_function());
        // TODO
    }

    fn process_call(&mut self, argc: u16) {
        let argv = self.emit_create_argv(argc);
        let (operand, _) = self.dereference();
        let closure = match operand {
            // TODO: Operand::Closure(closure) => closure, // IIFE
            Operand::Any(value, ..) => self.emit_load_closure_or_throw_type_error(value),
            _ => {
                self.process_number(1001.); // TODO: TypeError
                self.process_throw();
                return;
            }
        };

        let retv = self.emit_create_any();
        let status = self.emit_call(closure, argc, argv, retv);

        self.emit_check_status_for_exception(status, retv);

        // TODO(pref): compile-time evaluation
        self.operand_stack.push(Operand::Any(retv, None));
    }

    fn get_runtime_ptr(&self) -> Value {
        let flow = self.control_flow_stack.function_flow();
        self.builder.block_params(flow.entry_block)[0]
    }

    fn emit_call(&mut self, closure: ClosureIr, argc: u16, argv: ArgvIr, retv: AnyIr) -> StatusIr {
        let lambda = self.emit_load_lambda_from_closure(closure);
        let context = self.emit_load_captures_from_closure(closure);
        let args = &[
            self.get_runtime_ptr(),
            context,
            self.builder.ins().iconst(types::I16, argc as i64),
            argv.0,
            retv.0,
        ];
        let call = self
            .builder
            .ins()
            .call_indirect(self.lambda_sig, lambda, args);
        StatusIr(self.builder.inst_results(call)[0])
    }

    fn process_push_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        let body_block = self.create_block();
        self.insert_block_after_current(body_block);

        self.control_flow_stack
            .push_scope_flow(scope_ref, body_block);

        // TODO

        self.emit_jump(body_block);
        self.switch_to_block(body_block);
    }

    fn process_pop_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        let flow = self.control_flow_stack.pop_scope_flow();
        debug_assert_eq!(flow.scope_ref, scope_ref);

        // TODO
    }

    // 13.5.4.1 Runtime Semantics: Evaluation
    fn process_unary_plus(&mut self) {
        let (operand, _) = self.dereference();
        let value = self.apply_to_numeric(operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 13.5.5.1 Runtime Semantics: Evaluation
    fn process_unary_minus(&mut self) {
        let (operand, _) = self.dereference();
        let value = self.apply_to_numeric(operand);
        // TODO: BigInt
        // 6.1.6.1.1 Number::unaryMinus ( x )
        let value = self.emit_neg(value);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(value, None));
    }

    // 13.5.6.1 Runtime Semantics: Evaluation
    fn process_bitwise_not(&mut self) {
        let (operand, _) = self.dereference();
        let number = self.apply_to_numeric(operand);
        // TODO: BigInt
        let number = self.emit_bitwise_not(number);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.5.7.1 Runtime Semantics: Evaluation
    fn process_logical_not(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.perform_to_boolean(operand);
        let boolean = self.emit_logical_not(boolean);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.6.1 Runtime Semantics: Evaluation
    fn process_exponentiation(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let number = self.emit_exp(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_multiplication(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let number = self.emit_mul(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_division(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let number = self.emit_div(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.7.1 Runtime Semantics: Evaluation
    fn process_remainder(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let number = self.emit_rem(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.8.1.1 Runtime Semantics: Evaluation
    fn process_addition(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let number = self.emit_add(lhs, rhs);

        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.8.2.1 Runtime Semantics: Evaluation
    fn process_subtraction(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let number = self.emit_sub(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.1.1 Runtime Semantics: Evaluation
    fn process_left_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.emit_left_shift(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.2.1 Runtime Semantics: Evaluation
    fn process_signed_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.emit_signed_right_shift(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.9.3.1 Runtime Semantics: Evaluation
    fn process_unsigned_right_shift(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        // TODO: BigInt
        let number = self.emit_unsigned_right_shift(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    fn process_discard(&mut self) {
        debug_assert!(!self.operand_stack.is_empty());
        self.operand_stack.pop();
    }

    fn process_throw(&mut self) {
        let (_operand, _) = self.dereference();
        // TODO
    }

    fn process_swap(&mut self) {
        self.swap();
    }

    // commonly used functions

    fn perform_to_boolean(&mut self, operand: Operand) -> BooleanIr {
        match operand {
            Operand::Undefined | Operand::Null => self.emit_boolean(false),
            Operand::Boolean(value, ..) => value,
            Operand::Number(value, ..) => self.emit_number_to_boolean(value),
            // Operand::String(..) => todo!(),
            // Operand::Closure(_) | Operand::Object(_) | Operand::Promise(_) => {
            //     self.bridge.get_boolean(true)
            // }
            Operand::Any(value, ..) => self.emit_to_boolean(value),
            Operand::VariableReference(..) => unreachable!(),
            // Operand::Lambda(_)
            // | Operand::Coroutine(_)
            // | Operand::VariableReference(..)
            // | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    // 7.1.4 ToNumber ( argument )
    fn apply_to_numeric(&mut self, operand: Operand) -> NumberIr {
        logger::debug!(event = "to_numeric", ?operand);
        match operand {
            Operand::Undefined => self.emit_number(f64::NAN),
            Operand::Null => self.emit_number(0.0),
            Operand::Boolean(value, ..) => self.emit_boolean_to_number(value),
            Operand::Number(value, ..) => value,
            // Operand::String(..) => unimplemented!("string.to_numeric"),
            // Operand::Closure(_) => self.bridge.get_nan(),
            // Operand::Object(_) => unimplemented!("object.to_numeric"),
            Operand::Any(value, ..) => self.emit_to_numeric(value),
            Operand::VariableReference(..) => unreachable!(),
            // Operand::Lambda(_)
            // | Operand::Coroutine(_)
            // | Operand::Promise(_)
            // | Operand::PropertyReference(_) => unreachable!(),
        }
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
                let value = self.emit_get_variable(symbol, locator);
                // TODO(pref): compile-time evaluation
                (Operand::Any(value, None), Some((symbol, locator)))
            }
            /*
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
            */
            _ => (operand, None),
        }
    }

    fn swap(&mut self) {
        logger::debug!(event = "swap");
        debug_assert!(self.operand_stack.len() > 1);
        let last_index = self.operand_stack.len() - 1;
        self.operand_stack.swap(last_index - 1, last_index);
    }

    // operations on blocks

    fn current_block(&self) -> Block {
        self.builder.current_block().unwrap()
    }

    fn create_entry_block(&mut self) -> Block {
        let block = self.builder.create_block();

        // As described in the following document, the incoming function arguments must be passed
        // to the entry block as block parameters:
        // //cranelift/docs/ir.md#static-single-assignment-form in bytecodealliance/wasmtime
        self.builder.append_block_params_for_function_params(block);

        // Immediately call `seal_block()` because this block is the first block and there is no
        // predecessor of the entry block.
        self.builder.seal_block(block);

        block
    }

    fn create_block(&mut self) -> Block {
        self.builder.create_block()
    }

    fn insert_block_after(&mut self, block: Block, after: Block) {
        self.builder.insert_block_after(block, after);
    }

    fn insert_block_after_current(&mut self, block: Block) {
        self.builder.insert_block_after(block, self.current_block());
    }

    fn switch_to_block(&mut self, block: Block) {
        self.builder.switch_to_block(block);
    }

    fn seal_block(&mut self, block: Block) {
        self.builder.seal_block(block);
    }

    // stack allocation

    const VALUE_SIZE: u16 = size_of::<crate::types::Value>() as u16;
    const ALIGNMENT: u8 = align_of::<crate::types::Value>().ilog2() as u8;

    fn emit_is_nullptr(&mut self, value: AnyIr) -> BooleanIr {
        BooleanIr(self.builder.ins().icmp_imm(IntCC::Equal, value.0, 0))
    }

    fn emit_nullptr(&mut self) -> Value {
        self.builder.ins().iconst(self.ptr_type, 0)
    }

    fn emit_create_any(&mut self) -> AnyIr {
        let slot = self.builder.create_sized_stack_slot(StackSlotData {
            kind: StackSlotKind::ExplicitSlot,
            size: Self::VALUE_SIZE as u32,
            align_shift: Self::ALIGNMENT,
        });

        // TODO: Value::KIND_NONE
        let kind = self.builder.ins().iconst(types::I8, 0);
        self.builder.ins().stack_store(kind, slot, 0);

        let addr = self.builder.ins().stack_addr(self.ptr_type, slot, 0);
        AnyIr(addr)
    }

    fn emit_create_argv(&mut self, argc: u16) -> ArgvIr {
        if argc == 0 {
            return ArgvIr(self.emit_nullptr());
        }

        let slot = self.builder.create_sized_stack_slot(StackSlotData {
            kind: StackSlotKind::ExplicitSlot,
            size: (Self::VALUE_SIZE * argc) as u32,
            align_shift: Self::ALIGNMENT,
        });

        // TODO: evaluation order
        for i in (0..argc).rev() {
            let (operand, _) = self.dereference();
            self.emit_store_operand_to_slot(&operand, slot, i);
        }

        let addr = self.builder.ins().stack_addr(self.ptr_type, slot, 0);
        ArgvIr(addr)
    }

    fn emit_store_operand_to_slot(&mut self, operand: &Operand, slot: StackSlot, index: u16) {
        let base_offset = (Self::VALUE_SIZE as i32) * (index as i32);
        match operand {
            Operand::Undefined => {
                // TODO: Value::KIND_UNDEFINED
                let kind = self.builder.ins().iconst(types::I8, 1);
                self.builder.ins().stack_store(kind, slot, base_offset);
            }
            Operand::Null => {
                // TODO: Value::KIND_NULL
                let kind = self.builder.ins().iconst(types::I8, 2);
                self.builder.ins().stack_store(kind, slot, base_offset);
            }
            Operand::Boolean(value, _) => {
                // TODO: Value::KIND_BOOLEAN
                let kind = self.builder.ins().iconst(types::I8, 3);
                self.builder.ins().stack_store(kind, slot, base_offset);
                self.builder
                    .ins()
                    .stack_store(value.0, slot, base_offset + 8);
            }
            Operand::Number(value, _) => {
                // TODO: Value::KIND_NUMBER
                let kind = self.builder.ins().iconst(types::I8, 4);
                self.builder.ins().stack_store(kind, slot, base_offset);
                self.builder
                    .ins()
                    .stack_store(value.0, slot, base_offset + 8);
            }
            Operand::Any(value, _) => {
                const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
                // TODO(perf): should use memcpy?
                static_assert_eq!(size_of::<crate::types::Value>() * 8, 128);
                let opaque = self.builder.ins().load(types::I128, FLAGS, value.0, 0);
                self.builder.ins().stack_store(opaque, slot, base_offset);
            }
            Operand::VariableReference(..) => unreachable!(),
        }
    }

    // instructions

    fn emit_boolean(&mut self, value: bool) -> BooleanIr {
        logger::debug!(event = "emit_boolean", value);
        BooleanIr(self.builder.ins().iconst(types::I8, value as i64))
    }

    fn emit_number_to_boolean(&mut self, value: NumberIr) -> BooleanIr {
        let zero = self.builder.ins().f64const(0.0);
        BooleanIr(self.builder.ins().fcmp(FloatCC::NotEqual, value.0, zero))
    }

    fn emit_to_boolean(&mut self, value: AnyIr) -> BooleanIr {
        let func = self
            .runtime_func_cache
            .get_to_boolean(self.module, self.builder.func);
        let args = [self.get_runtime_ptr(), value.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    fn emit_number(&mut self, value: f64) -> NumberIr {
        logger::debug!(event = "emit_number", value);
        NumberIr(self.builder.ins().f64const(value))
    }

    fn emit_boolean_to_number(&mut self, value: BooleanIr) -> NumberIr {
        logger::debug!(event = "emit_boolean_to_number", ?value);
        NumberIr(self.builder.ins().fcvt_from_uint(types::F64, value.0))
    }

    fn emit_to_numeric(&mut self, any: AnyIr) -> NumberIr {
        logger::debug!(event = "emit_to_numeric", ?any);
        let func = self
            .runtime_func_cache
            .get_to_numeric(self.module, self.builder.func);
        let args = [self.get_runtime_ptr(), any.0];
        let call = self.builder.ins().call(func, &args);
        NumberIr(self.builder.inst_results(call)[0])
    }

    fn emit_neg(&mut self, value: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_neg", ?value);
        NumberIr(self.builder.ins().fneg(value.0))
    }

    fn emit_add(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_add", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fadd(lhs.0, rhs.0))
    }

    fn emit_sub(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_sub", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fsub(lhs.0, rhs.0))
    }

    fn emit_mul(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_mul", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fmul(lhs.0, rhs.0))
    }

    fn emit_div(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_div", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fdiv(lhs.0, rhs.0))
    }

    fn emit_rem(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_rem", ?lhs, ?rhs);
        let call = self.builder.ins().call(self.ref_fmod, &[lhs.0, rhs.0]);
        NumberIr(self.builder.inst_results(call)[0])
    }

    fn emit_exp(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_exp", ?lhs, ?rhs);
        let call = self.builder.ins().call(self.ref_pow, &[lhs.0, rhs.0]);
        NumberIr(self.builder.inst_results(call)[0])
    }

    // 6.1.6.1.2 Number::bitwiseNOT ( x )
    fn emit_bitwise_not(&mut self, value: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_bitwise_not", ?value);
        let int32 = self.emit_to_int32(value);
        let bnot = self.builder.ins().bnot(int32);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, bnot))
    }

    fn emit_logical_not(&mut self, value: BooleanIr) -> BooleanIr {
        logger::debug!(event = "emit_logical_not", ?value);
        BooleanIr(self.builder.ins().bxor_imm(value.0, 1))
    }

    // 6.1.6.1.9 Number::leftShift ( x, y )
    fn emit_left_shift(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_left_shift", ?x, ?y);
        let lnum = self.emit_to_int32(x);
        let rnum = self.emit_to_uint32(y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().ishl(lnum, shift_count);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, shifted))
    }

    // 6.1.6.1.10 Number::signedRightShift ( x, y )
    fn emit_signed_right_shift(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_signed_right_shift", ?x, ?y);
        let lnum = self.emit_to_int32(x);
        let rnum = self.emit_to_uint32(y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().sshr(lnum, shift_count);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, shifted))
    }

    // 6.1.6.1.11 Number::unsignedRightShift ( x, y )
    fn emit_unsigned_right_shift(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_unsigned_right_shift", ?x, ?y);
        let lnum = self.emit_to_uint32(x);
        let rnum = self.emit_to_uint32(y);
        let shift_count = self.builder.ins().urem_imm(rnum, 32);
        let shifted = self.builder.ins().ushr(lnum, shift_count);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, shifted))
    }

    // 7.1.6 ToInt32 ( argument )
    fn emit_to_int32(&mut self, value: NumberIr) -> Value {
        logger::debug!(event = "emit_to_int32", ?value);
        let func = self
            .runtime_func_cache
            .get_to_int32(self.module, self.builder.func);
        let args = [self.get_runtime_ptr(), value.0];
        let call = self.builder.ins().call(func, &args);
        self.builder.inst_results(call)[0]
    }

    fn emit_to_uint32(&mut self, value: NumberIr) -> Value {
        logger::debug!(event = "emit_to_uint32", ?value);
        let func = self
            .runtime_func_cache
            .get_to_uint32(self.module, self.builder.func);
        let args = [self.get_runtime_ptr(), value.0];
        let call = self.builder.ins().call(func, &args);
        self.builder.inst_results(call)[0]
    }

    fn emit_jump(&mut self, block: Block) {
        logger::debug!(event = "emit_jump", ?block);
        self.builder.ins().jump(block, &[]);
    }

    fn emit_get_variable(&mut self, symbol: Symbol, locator: Locator) -> AnyIr {
        logger::debug!(event = "emit_get_variable", ?symbol, ?locator);
        match locator {
            Locator::None => unreachable!(),
            Locator::Argument(index) => self.emit_get_argument(index),
            Locator::Local(index) => self.locals[index as usize],
            Locator::Capture(index) => self.emit_get_capture(index),
            Locator::Global => self.emit_get_global_variable(symbol),
        }
    }

    fn emit_get_argument(&mut self, _index: u16) -> AnyIr {
        todo!();
    }

    fn emit_get_capture(&mut self, _index: u16) -> AnyIr {
        todo!();
    }

    fn emit_call_get_global_variable(
        &mut self,
        object: ObjectIr,
        key: Symbol,
        strict: bool,
    ) -> AnyIr {
        let func = self
            .runtime_func_cache
            .get_get_value_by_symbol(self.module, self.builder.func);
        let args = [
            self.get_runtime_ptr(),
            object.0,
            self.builder.ins().iconst(types::I32, key.id() as i64),
            self.builder.ins().iconst(types::I8, strict as i64),
        ];
        let call = self.builder.ins().call(func, &args);
        AnyIr(self.builder.inst_results(call)[0])
    }

    // TODO(perf): return the value directly if it's a read-only global property.
    fn emit_get_global_variable(&mut self, key: Symbol) -> AnyIr {
        let object = ObjectIr(self.emit_nullptr());

        // TODO: strict mode
        let value = self.emit_call_get_global_variable(object, key, true);

        let then_block = self.create_block();
        let end_block = self.create_block();

        // if value.is_nullptr()
        let is_nullptr = self.emit_is_nullptr(value);
        self.builder
            .ins()
            .brif(is_nullptr.0, then_block, &[], end_block, &[]);
        // {
        self.switch_to_block(then_block);
        // TODO(feat): ReferenceError
        self.process_number(1000.);
        self.process_throw();
        self.emit_jump(end_block);
        // }
        self.switch_to_block(end_block);

        value
    }

    fn emit_load_lambda_from_closure(&mut self, closure: ClosureIr) -> Value {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = std::mem::offset_of!(crate::types::Closure, lambda) as i32;
        self.builder
            .ins()
            .load(self.ptr_type, FLAGS, closure.0, OFFSET)
    }

    fn emit_load_captures_from_closure(&mut self, closure: ClosureIr) -> Value {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = std::mem::offset_of!(crate::types::Closure, captures) as i32;
        self.builder
            .ins()
            .load(self.ptr_type, FLAGS, closure.0, OFFSET)
    }

    fn emit_is_closure(&mut self, value: AnyIr) -> BooleanIr {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = 0;
        let kind = self.builder.ins().load(types::I8, FLAGS, value.0, OFFSET);
        // TODO: Value::KIND_CLOSURE
        BooleanIr(self.builder.ins().icmp_imm(IntCC::Equal, kind, 6))
    }

    fn emit_load_closure(&mut self, value: AnyIr) -> ClosureIr {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = 8; // TODO
        ClosureIr(
            self.builder
                .ins()
                .load(self.ptr_type, FLAGS, value.0, OFFSET),
        )
    }

    fn emit_load_closure_or_throw_type_error(&mut self, value: AnyIr) -> ClosureIr {
        let then_block = self.create_block();
        let else_block = self.create_block();
        let end_block = self.create_block();

        self.builder.append_block_param(end_block, self.ptr_type);

        // if value.is_closure()
        let is_closure = self.emit_is_closure(value);
        self.builder
            .ins()
            .brif(is_closure.0, then_block, &[], else_block, &[]);
        // then
        self.switch_to_block(then_block);
        let closure = self.emit_load_closure(value);
        self.builder.ins().jump(end_block, &[closure.0]);
        // else
        self.switch_to_block(else_block);
        self.process_number(1001.); // TODO(feat): TypeError
        self.process_throw();
        let dummy = self.emit_nullptr();
        self.builder.ins().jump(end_block, &[dummy]);

        self.switch_to_block(end_block);
        ClosureIr(self.builder.block_params(end_block)[0])
    }

    fn emit_check_status_for_exception(&mut self, _status: StatusIr, _retv: AnyIr) {
        // TODO
    }
}

// operands

struct OperandStack(Vec<Operand>);

impl OperandStack {
    fn new() -> Self {
        Self(vec![])
    }

    /*
    fn duplicate(&mut self, index: usize) {
        let dup = self.0[index].clone();
        self.push(dup);
    }
    */

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

    /// Runtime value and optional compile-time constant value of any type.
    // TODO(perf): compile-time evaluation
    Any(AnyIr, #[allow(unused)] Option<Value>),

    // Compile-time constant value types.
    VariableReference(Symbol, Locator),
}

#[derive(Clone, Copy, Debug)]
struct BooleanIr(Value);

#[derive(Clone, Copy, Debug)]
struct NumberIr(Value);

#[derive(Clone, Copy, Debug)]
struct ClosureIr(Value);

#[derive(Clone, Copy, Debug)]
struct ObjectIr(Value);

#[derive(Clone, Copy, Debug)]
struct AnyIr(Value);

#[derive(Clone, Copy, Debug)]
struct ArgvIr(Value);

#[derive(Clone, Copy, Debug)]
struct StatusIr(#[allow(unused)] Value);
