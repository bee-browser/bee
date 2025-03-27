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
    locals: Vec<StackSlot>,
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
            CompileCommand::String(value) => self.process_string(value),
            CompileCommand::VariableReference(symbol) => self.process_variable_reference(*symbol),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::MutableVariable => self.process_mutable_variable(),
            CompileCommand::ImmutableVariable => self.process_immutable_variable(),
            CompileCommand::DeclareVars(scope_ref) => self.process_declare_vars(*scope_ref),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(*scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
            CompileCommand::Delete => self.process_delete(),
            CompileCommand::Void => self.process_void(),
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
            CompileCommand::Truthy => self.process_truthy(),
            CompileCommand::IfThen(expr) => self.process_if_then(*expr),
            CompileCommand::Else(expr) => self.process_else(*expr),
            CompileCommand::Discard => self.process_discard(),
            CompileCommand::Swap => self.process_swap(),
            CompileCommand::Dereference => self.process_dereference(),
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

    fn process_string(&mut self, value: &[u16]) {
        // Theoretically, the heap memory pointed by `value` can be freed after the IR built by the
        // compiler is freed.
        let string_ir = self.emit_create_string(value);
        self.operand_stack.push(Operand::String(
            string_ir,
            Some(crate::types::Char16Seq::new_stack(value)),
        ));
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

    fn process_allocate_locals(&mut self, num_locals: u16) {
        for _ in 0..num_locals {
            let slot = self.builder.create_sized_stack_slot(StackSlotData {
                kind: StackSlotKind::ExplicitSlot,
                size: Self::VALUE_SIZE as u32,
                align_shift: Self::ALIGNMENT,
            });
            self.locals.push(slot);
        }
    }

    fn process_mutable_variable(&mut self) {
        let (_symbol, locator) = self.pop_reference();
        let (operand, _) = self.dereference();

        let slot = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.emit_store_operand_to_slot(&operand, slot, 0);
    }

    fn process_immutable_variable(&mut self) {
        let (_symbol, locator) = self.pop_reference();
        let (operand, _) = self.dereference();

        let slot = match locator {
            Locator::Local(index) => self.locals[index as usize],
            _ => unreachable!(),
        };

        self.emit_store_operand_to_slot(&operand, slot, 0);
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

    // 13.5.1.2 Runtime Semantics: Evaluation
    fn process_delete(&mut self) {
        unimplemented!("delete operator");
    }

    // 13.5.2.1 Runtime Semantics: Evaluation
    fn process_void(&mut self) {
        self.operand_stack.pop();
        self.operand_stack.push(Operand::Undefined);
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

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let boolean = self.emit_less_than(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let boolean = self.emit_greater_than(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_less_than_or_equal(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let boolean = self.emit_less_than_or_equal(lhs, rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.10.1 Runtime Semantics: Evaluation
    fn process_greater_than_or_equal(&mut self) {
        let (lhs, _) = self.dereference();
        let lhs = self.apply_to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.apply_to_numeric(rhs);

        let boolean = self.emit_greater_than_or_equal(lhs, rhs);
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
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let boolean = self.perform_is_loosely_equal(&lhs, &rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_inequality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let eq = self.perform_is_loosely_equal(&lhs, &rhs);
        let boolean = self.emit_logical_not(eq);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_equality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let boolean = self.perform_is_strictly_equal(&lhs, &rhs);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.11.1 Runtime Semantics: Evaluation
    fn process_strict_inequality(&mut self) {
        // TODO: comparing references improves the performance.
        let (lhs, _) = self.dereference();
        let (rhs, _) = self.dereference();

        let eq = self.perform_is_strictly_equal(&lhs, &rhs);
        let boolean = self.emit_logical_not(eq);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_and(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.apply_to_numeric(lval);
        let rnum = self.apply_to_numeric(rval);
        // TODO: BigInt

        let number = self.emit_bitwise_and(lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_xor(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.apply_to_numeric(lval);
        let rnum = self.apply_to_numeric(rval);
        // TODO: BigInt

        let number = self.emit_bitwise_xor(lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    // 13.12.1 Runtime Semantics: Evaluation
    fn process_bitwise_or(&mut self) {
        // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
        let (lval, _) = self.dereference();
        let (rval, _) = self.dereference();

        // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
        let lnum = self.apply_to_numeric(lval);
        let rnum = self.apply_to_numeric(rval);
        // TODO: BigInt

        let number = self.emit_bitwise_or(lnum, rnum);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Number(number, None));
    }

    fn process_ternary(&mut self) {
        let flow = self.control_flow_stack.pop_if_then_else_flow();

        let (else_operand, _) = self.dereference();
        let any_ir = self.peek_any();
        self.emit_store_operand_to_any(&else_operand, any_ir);
        self.emit_jump(flow.merge_block);

        self.switch_to_block(flow.merge_block);
    }

    // 13.15.2 Runtime Semantics: Evaluation
    fn process_assignment(&mut self) {
        let (rhs, _) = self.dereference();

        match self.operand_stack.pop().unwrap() {
            // Operand::VariableReference(symbol, Locator::Global) => {
            //     let object = self.emit_nullptr();
            //     let value = self.create_to_any(&rhs);
            //     // TODO(feat): ReferenceError, TypeError
            //     self.bridge
            //         .create_set_value_by_symbol(object, symbol, value);
            // }
            Operand::VariableReference(symbol, locator) => {
                let var = self.emit_get_variable(symbol, locator);
                // TODO: throw a TypeError in the strict mode.
                // auto* flags_ptr = CreateGetFlagsPtr(value_ptr);
                self.emit_store_operand_to_any(&rhs, var);
            }
            // Operand::PropertyReference(key) => {
            //     // TODO(refactor): reduce code clone
            //     self.perform_to_object();
            //     let object = self.pop_object();
            //     let value = self.create_to_any(&rhs);
            //     match key {
            //         PropertyKey::Symbol(key) => {
            //             self.bridge.create_set_value_by_symbol(object, key, value);
            //         }
            //         PropertyKey::Number(key) => {
            //             self.bridge.create_set_value_by_number(object, key, value);
            //         }
            //         PropertyKey::Value(key) => {
            //             self.bridge.create_set_value_by_value(object, key, value);
            //         }
            //     }
            // }
            operand => unreachable!("{operand:?}"),
        }

        self.operand_stack.push(rhs);
    }

    fn process_truthy(&mut self) {
        let (operand, _) = self.dereference();
        let boolean = self.perform_to_boolean(operand);
        // TODO(perf): compile-time evaluation
        self.operand_stack.push(Operand::Boolean(boolean, None));
    }

    fn process_if_then(&mut self, expr: bool) {
        let cond_value = self.pop_boolean();
        let then_block = self.create_block();
        let else_block = self.create_block();
        let merge_block = self.create_block();
        if expr {
            let any_ir = self.emit_create_any();
            self.operand_stack.push(Operand::Any(any_ir, None));
        }
        self.builder
            .ins()
            .brif(cond_value.0, then_block, &[], else_block, &[]);
        self.switch_to_block(then_block);
        self.control_flow_stack
            .push_if_then_else_flow(then_block, else_block, merge_block);
    }

    fn process_else(&mut self, expr: bool) {
        if expr {
            let (operand, _) = self.dereference();
            let any_ir = self.peek_any();
            self.emit_store_operand_to_any(&operand, any_ir);
        } else {
            self.operand_stack.pop();
        }
        let merge_block = self.control_flow_stack.merge_block();
        self.emit_jump(merge_block);
        let then_block = self.builder.current_block().unwrap();
        let else_block = self.control_flow_stack.update_then_block(then_block);
        //self.bridge.move_basic_block_after(else_block);
        self.switch_to_block(else_block);
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

    fn process_dereference(&mut self) {
        let (operand, _) = self.dereference();
        self.operand_stack.push(operand);
    }

    // commonly used functions

    fn pop_boolean(&mut self) -> BooleanIr {
        match self.operand_stack.pop().unwrap() {
            Operand::Boolean(value, ..) => value,
            _ => unreachable!(),
        }
    }

    fn perform_to_boolean(&mut self, operand: Operand) -> BooleanIr {
        match operand {
            Operand::Undefined | Operand::Null => self.emit_boolean(false),
            Operand::Boolean(value, ..) => value,
            Operand::Number(value, ..) => self.emit_number_to_boolean(value),
            Operand::String(..) => todo!(),
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
            Operand::String(..) => unimplemented!("string.to_numeric"),
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

    fn peek_any(&mut self) -> AnyIr {
        match self.operand_stack.last().unwrap() {
            Operand::Any(value, _) => *value,
            _ => unreachable!(),
        }
    }

    fn perform_to_any(&mut self, operand: &Operand) -> AnyIr {
        let slot = self.builder.create_sized_stack_slot(StackSlotData {
            kind: StackSlotKind::ExplicitSlot,
            size: Self::VALUE_SIZE as u32,
            align_shift: Self::ALIGNMENT,
        });
        self.emit_store_operand_to_slot(operand, slot, 0);
        let addr = self.builder.ins().stack_addr(self.ptr_type, slot, 0);
        AnyIr(addr)
    }

    // 7.2.13 IsLooselyEqual ( x, y )
    fn perform_is_loosely_equal(&mut self, lhs: &Operand, rhs: &Operand) -> BooleanIr {
        logger::debug!(event = "perform_is_loosely_equal", ?lhs, ?rhs);
        if let Operand::Any(lhs, ..) = lhs {
            // TODO: compile-time evaluation
            let rhs = self.perform_to_any(rhs);
            return self.emit_call_is_loosely_equal(*lhs, rhs);
        }
        if let Operand::Any(rhs, ..) = rhs {
            // TODO: compile-time evaluation
            let lhs = self.perform_to_any(lhs);
            return self.emit_call_is_loosely_equal(lhs, *rhs);
        }

        // 1. If Type(x) is Type(y), then Return IsStrictlyEqual(x, y).
        if std::mem::discriminant(lhs) == std::mem::discriminant(rhs) {
            return self.perform_is_strictly_equal(lhs, rhs);
        }

        // 2. If x is null and y is undefined, return true.
        if matches!(lhs, Operand::Null) && matches!(rhs, Operand::Undefined) {
            return self.emit_boolean(true);
        }

        // 3. If x is undefined and y is null, return true.
        if matches!(lhs, Operand::Undefined) && matches!(rhs, Operand::Null) {
            return self.emit_boolean(true);
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
        self.emit_call_is_loosely_equal(lhs, rhs)
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
            return self.emit_boolean(false);
        }
        // TODO: BigInt
        match (lhs, rhs) {
            (Operand::Undefined, Operand::Undefined) => self.emit_boolean(true),
            (Operand::Null, Operand::Null) => self.emit_boolean(true),
            (Operand::Boolean(lhs, ..), Operand::Boolean(rhs, ..)) => {
                self.emit_is_same_boolean(*lhs, *rhs)
            }
            (Operand::Number(lhs, ..), Operand::Number(rhs, ..)) => {
                self.emit_is_same_number(*lhs, *rhs)
            }
            (Operand::String(_lhs, ..), Operand::String(_rhs, ..)) => {
                todo!();
            }
            // (Operand::Closure(lhs), Operand::Closure(rhs)) => {
            //     self.perform_is_same_closure(lhs, rhs)
            // }
            // (Operand::Promise(lhs), Operand::Promise(rhs)) => {
            //     self.perform_is_same_promise(lhs, rhs)
            // }
            // (Operand::Object(lhs), Operand::Object(rhs)) => {
            //     self.perform_is_same_object(lhs, rhs)
            // }
            (lhs, rhs) => unreachable!("({lhs:?}, {rhs:?})"),
        }
    }

    fn perform_any_is_strictly_equal(&mut self, lhs: AnyIr, rhs: &Operand) -> BooleanIr {
        logger::debug!(event = "create_any_is_strictly_equal", ?lhs, ?rhs);
        match rhs {
            Operand::Undefined => self.emit_is_undefined(lhs),
            Operand::Null => self.emit_is_null(lhs),
            Operand::Boolean(rhs, ..) => self.perform_is_same_boolean(lhs, *rhs),
            Operand::Number(rhs, ..) => self.perform_is_same_number(lhs, *rhs),
            Operand::String(_rhs, ..) => todo!(),
            // Operand::Closure(rhs) => self.emit_is_same_closure(lhs, rhs),
            // Operand::Object(rhs) => self.emit_is_same_object(lhs, rhs),
            // Operand::Promise(rhs) => self.emit_is_same_promise(lhs, rhs),
            Operand::Any(rhs, ..) => self.emit_call_is_strictly_equal(lhs, *rhs),
            Operand::VariableReference(..) => unreachable!("{rhs:?}"),
            // Operand::Lambda(_)
            // | Operand::Coroutine(_)
            // | Operand::VariableReference(..)
            // | Operand::PropertyReference(_) => unreachable!("{rhs:?}"),
        }
    }

    fn perform_is_same_boolean(&mut self, value: AnyIr, boolean: BooleanIr) -> BooleanIr {
        let then_block = self.create_block();
        let else_block = self.create_block();
        let merge_block = self.create_block();
        self.builder.append_block_param(merge_block, types::I8);

        // if value.kind == ValueKind::Boolean
        let cond = self.emit_is_boolean(value);
        self.builder
            .ins()
            .brif(cond.0, then_block, &[], else_block, &[]);
        // {
        self.switch_to_block(then_block);
        let b = self.emit_load_boolean(value);
        let then_value = self.emit_is_same_boolean(b, boolean);
        self.builder.ins().jump(merge_block, &[then_value.0]);
        // } else {
        self.switch_to_block(else_block);
        let else_value = self.emit_boolean(false);
        self.builder.ins().jump(merge_block, &[else_value.0]);
        // }

        self.switch_to_block(merge_block);
        BooleanIr(self.builder.block_params(merge_block)[0])
    }

    fn perform_is_same_number(&mut self, value: AnyIr, number: NumberIr) -> BooleanIr {
        let then_block = self.create_block();
        let else_block = self.create_block();
        let merge_block = self.create_block();
        self.builder.append_block_param(merge_block, types::I8);

        // if value.kind == ValueKind::Number
        let cond = self.emit_is_number(value);
        self.builder
            .ins()
            .brif(cond.0, then_block, &[], else_block, &[]);
        // {
        self.switch_to_block(then_block);
        let n = self.emit_load_number(value);
        let then_value = self.emit_is_same_number(n, number);
        self.builder.ins().jump(merge_block, &[then_value.0]);
        // } else {
        self.switch_to_block(else_block);
        let else_value = self.emit_boolean(false);
        self.builder.ins().jump(merge_block, &[else_value.0]);
        // }

        self.switch_to_block(merge_block);
        BooleanIr(self.builder.block_params(merge_block)[0])
    }

    fn pop_reference(&mut self) -> (Symbol, Locator) {
        match self.operand_stack.pop().unwrap() {
            Operand::VariableReference(symbol, locator) => (symbol, locator),
            operand => unreachable!("{operand:?}"),
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

    fn emit_create_string(&mut self, value: &[u16]) -> StringIr {
        const SIZE: usize = size_of::<crate::types::Char16Seq>();
        const ALIGNMENT: usize = align_of::<crate::types::Char16Seq>();

        let slot = self.builder.create_sized_stack_slot(StackSlotData {
            kind: StackSlotKind::ExplicitSlot,
            size: SIZE as u32,
            align_shift: ALIGNMENT as u8,
        });

        macro_rules! offset_of {
            ($field:ident) => {
                std::mem::offset_of!(crate::types::Char16Seq, $field) as i32
            };
        }

        let next = self.builder.ins().iconst(self.ptr_type, 0);
        self.builder.ins().stack_store(next, slot, offset_of!(next));

        let ptr = self
            .builder
            .ins()
            .iconst(self.ptr_type, value.as_ptr() as i64);
        self.builder.ins().stack_store(ptr, slot, offset_of!(ptr));

        let len = self.builder.ins().iconst(types::I32, value.len() as i64);
        self.builder.ins().stack_store(len, slot, offset_of!(len));

        // TODO: Char16SeqKind::Stack
        let kind = self.builder.ins().iconst(types::I8, 1);
        self.builder.ins().stack_store(kind, slot, offset_of!(kind));

        let addr = self.builder.ins().stack_addr(self.ptr_type, slot, 0);
        StringIr(addr)
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
            Operand::String(value, _) => {
                // TODO: Value::KIND_STRING
                let kind = self.builder.ins().iconst(types::I8, 5);
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

    fn emit_store_operand_to_any(&mut self, operand: &Operand, any_ir: AnyIr) {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        match operand {
            Operand::Undefined => {
                // TODO: Value::KIND_UNDEFINED
                let kind = self.builder.ins().iconst(types::I8, 1);
                self.builder.ins().store(FLAGS, kind, any_ir.0, 0);
            }
            Operand::Null => {
                // TODO: Value::KIND_NULL
                let kind = self.builder.ins().iconst(types::I8, 2);
                self.builder.ins().store(FLAGS, kind, any_ir.0, 0);
            }
            Operand::Boolean(value, _) => {
                // TODO: Value::KIND_BOOLEAN
                let kind = self.builder.ins().iconst(types::I8, 3);
                self.builder.ins().store(FLAGS, kind, any_ir.0, 0);
                self.builder.ins().store(FLAGS, value.0, any_ir.0, 8);
            }
            Operand::Number(value, _) => {
                // TODO: Value::KIND_NUMBER
                let kind = self.builder.ins().iconst(types::I8, 4);
                self.builder.ins().store(FLAGS, kind, any_ir.0, 0);
                self.builder.ins().store(FLAGS, value.0, any_ir.0, 8);
            }
            Operand::String(value, _) => {
                // TODO: Value::KIND_STRING
                let kind = self.builder.ins().iconst(types::I8, 5);
                self.builder.ins().store(FLAGS, kind, any_ir.0, 0);
                self.builder.ins().store(FLAGS, value.0, any_ir.0, 8);
            }
            Operand::Any(value, _) => {
                // TODO(perf): should use memcpy?
                static_assert_eq!(size_of::<crate::types::Value>() * 8, 128);
                let opaque = self.builder.ins().load(types::I128, FLAGS, value.0, 0);
                self.builder.ins().store(FLAGS, opaque, any_ir.0, 0);
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

    // 6.1.6.1.17 Number::bitwiseAND ( x, y )
    fn emit_bitwise_and(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_bitwise_and", ?x, ?y);
        let lnum = self.emit_to_int32(x);
        let rnum = self.emit_to_int32(y);
        let result = self.builder.ins().band(lnum, rnum);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, result))
    }

    // 6.1.6.1.18 Number::bitwiseXOR ( x, y )
    fn emit_bitwise_xor(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_bitwise_xor", ?x, ?y);
        let lnum = self.emit_to_int32(x);
        let rnum = self.emit_to_int32(y);
        let result = self.builder.ins().bxor(lnum, rnum);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, result))
    }

    // 6.1.6.1.19 Number::bitwiseOR ( x, y )
    fn emit_bitwise_or(&mut self, x: NumberIr, y: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_bitwise_or", ?x, ?y);
        let lnum = self.emit_to_int32(x);
        let rnum = self.emit_to_int32(y);
        let result = self.builder.ins().bor(lnum, rnum);
        NumberIr(self.builder.ins().fcvt_from_sint(types::F64, result))
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

    fn emit_less_than(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "emit_less_than", ?lhs, ?rhs);
        BooleanIr(self.builder.ins().fcmp(FloatCC::LessThan, lhs.0, rhs.0))
    }

    fn emit_greater_than(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "emit_greater_than", ?lhs, ?rhs);
        BooleanIr(self.builder.ins().fcmp(FloatCC::GreaterThan, lhs.0, rhs.0))
    }

    fn emit_less_than_or_equal(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "emit_less_than_or_equal", ?lhs, ?rhs);
        BooleanIr(
            self.builder
                .ins()
                .fcmp(FloatCC::LessThanOrEqual, lhs.0, rhs.0),
        )
    }

    fn emit_greater_than_or_equal(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        logger::debug!(event = "emit_greater_than_or_equal", ?lhs, ?rhs);
        BooleanIr(
            self.builder
                .ins()
                .fcmp(FloatCC::GreaterThanOrEqual, lhs.0, rhs.0),
        )
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
            Locator::Local(index) => {
                let slot = self.locals[index as usize];
                AnyIr(self.builder.ins().stack_addr(self.ptr_type, slot, 0))
            }
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

    fn emit_load_kind(&mut self, any: AnyIr) -> Value {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = 0;
        self.builder.ins().load(types::I8, FLAGS, any.0, OFFSET)
    }

    fn emit_load_boolean(&mut self, any: AnyIr) -> BooleanIr {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = 8;
        BooleanIr(self.builder.ins().load(types::I8, FLAGS, any.0, OFFSET))
    }

    fn emit_load_number(&mut self, any: AnyIr) -> NumberIr {
        const FLAGS: MemFlags = MemFlags::new().with_aligned().with_notrap();
        const OFFSET: i32 = 8;
        NumberIr(self.builder.ins().load(types::F64, FLAGS, any.0, OFFSET))
    }

    fn emit_is_undefined(&mut self, any: AnyIr) -> BooleanIr {
        let kind = self.emit_load_kind(any);
        // TODO(refactor): Value::KIND_UNDEFINED
        BooleanIr(self.builder.ins().icmp_imm(IntCC::Equal, kind, 1))
    }

    fn emit_is_null(&mut self, any: AnyIr) -> BooleanIr {
        let kind = self.emit_load_kind(any);
        // TODO(refactor): Value::KIND_NULL
        BooleanIr(self.builder.ins().icmp_imm(IntCC::Equal, kind, 2))
    }

    fn emit_is_boolean(&mut self, any: AnyIr) -> BooleanIr {
        let kind = self.emit_load_kind(any);
        // TODO(refactor): Value::KIND_BOOLEAN
        BooleanIr(self.builder.ins().icmp_imm(IntCC::Equal, kind, 3))
    }

    fn emit_is_same_boolean(&mut self, lhs: BooleanIr, rhs: BooleanIr) -> BooleanIr {
        BooleanIr(self.builder.ins().icmp(IntCC::Equal, lhs.0, rhs.0))
    }

    fn emit_is_number(&mut self, any: AnyIr) -> BooleanIr {
        let kind = self.emit_load_kind(any);
        // TODO(refactor): Value::KIND_NUMBER
        BooleanIr(self.builder.ins().icmp_imm(IntCC::Equal, kind, 4))
    }

    fn emit_is_same_number(&mut self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        BooleanIr(self.builder.ins().fcmp(FloatCC::Equal, lhs.0, rhs.0))
    }

    fn emit_call_is_loosely_equal(&mut self, lhs: AnyIr, rhs: AnyIr) -> BooleanIr {
        let func = self
            .runtime_func_cache
            .get_is_loosely_equal(self.module, self.builder.func);
        let args = [self.get_runtime_ptr(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
    }

    fn emit_call_is_strictly_equal(&mut self, lhs: AnyIr, rhs: AnyIr) -> BooleanIr {
        let func = self
            .runtime_func_cache
            .get_is_strictly_equal(self.module, self.builder.func);
        let args = [self.get_runtime_ptr(), lhs.0, rhs.0];
        let call = self.builder.ins().call(func, &args);
        BooleanIr(self.builder.inst_results(call)[0])
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

    /// Runtime value and optional compile-time constant value of number type.
    // TODO(perf): compile-time evaluation
    String(StringIr, #[allow(unused)] Option<crate::types::Char16Seq>),

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
struct StringIr(Value);

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
