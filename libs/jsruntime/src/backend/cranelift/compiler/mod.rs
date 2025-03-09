mod control_flow;

use core::f64;
use std::ops::Deref;
use std::ops::DerefMut;

use cranelift::prelude::*;
use cranelift_jit::JITBuilder;
use cranelift_jit::JITModule;
use cranelift_module::DataDescription;
use cranelift_module::Module as _;
use jsparser::Symbol;

use super::CompileError;
use super::Module;
use super::Program;
use crate::backend::CompilerSupport;
use crate::logger;
use crate::semantics::CompileCommand;
use crate::semantics::Function;
use crate::semantics::Locator;
use crate::semantics::ScopeRef;
use crate::semantics::ScopeTree;
use crate::semantics::VariableRef;

use control_flow::ControlFlowStack;

pub fn compile<R>(
    support: &mut R,
    program: &Program,
    optimize: bool,
) -> Result<Module, CompileError>
where
    R: CompilerSupport,
{
    // TODO: Deferring the compilation until it's actually called improves the performance.
    // Because the program may contain unused functions.
    let mut context = CraneliftContext::new();

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
        _inner: context.module,
        context: context.context,
    })
}

struct CraneliftContext {
    builder_context: FunctionBuilderContext,
    context: codegen::Context,
    _data_description: DataDescription,
    module: JITModule,
}

impl CraneliftContext {
    fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();

        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });

        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        // TODO: builder.symbol("runtime_func", runtime_func_addr);

        let module = JITModule::new(builder);

        // TODO:

        Self {
            builder_context: FunctionBuilderContext::new(),
            context: module.make_context(),
            _data_description: DataDescription::new(),
            module,
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
        let ptr_type = self.module.target_config().pointer_type();

        // formal parameters
        let params = &mut self.context.func.signature.params;
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
        self.context
            .func
            .signature
            .returns
            .push(AbiParam::new(types::I32));

        let builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);
        Compiler::new(runtime, scope_tree, builder, &mut self.module)
    }
}

struct Compiler<'r, 's, 'm, R> {
    support: &'r mut R,

    /// The scope tree of the JavaScript program to compile.
    scope_tree: &'s ScopeTree,

    /// A stack to hold sets of basic blocks which construct of a region in the control flow graph
    /// (CFG) finally built.
    control_flow_stack: ControlFlowStack,

    pending_labels: Vec<Symbol>,

    builder: FunctionBuilder<'m>,
    _module: &'m mut JITModule,

    /// A stack for operands.
    operand_stack: OperandStack,

    // The following values must be reset in the end of compilation for each function.
    locals: Vec<AnyIr>,
}

impl<'r, 's, 'm, R> Compiler<'r, 's, 'm, R>
where
    R: CompilerSupport,
{
    fn new(
        support: &'r mut R,
        scope_tree: &'s ScopeTree,
        builder: FunctionBuilder<'m>,
        _module: &'m mut JITModule,
    ) -> Self {
        Self {
            support,
            scope_tree,
            control_flow_stack: Default::default(),
            pending_labels: Default::default(),
            builder,
            _module,
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

        let entry_block = self.create_entry_block();

        // Unlike LLVM IR, we cannot specify a label for each basic block.  This is bad from a
        // debugging and readability perspective...
        let locals_block = self.create_block();
        let init_block = self.create_block();
        let args_block = self.create_block();
        let body_block = self.create_block();
        let return_block = self.create_block();

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
        self.emit_jump(locals_block);

        // Immediately call `seal_block()` with the `locals_block`.  This block is always inserted
        // just after the `entry_block`.  Blocks may be inserted between the `locals_block` and the
        // `init_block`.
        self.seal_block(locals_block);

        // The `entry_block` is already in the layout.  We can insert empty blocks after it.
        self.insert_block_after(locals_block, entry_block);
        self.insert_block_after(init_block, locals_block);
        self.insert_block_after(args_block, init_block);
        self.insert_block_after(body_block, args_block);
        self.insert_block_after(return_block, body_block);

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

        self.emit_jump(flow.return_block);

        // The `locals_block` has already been sealed in start_compile().
        self.switch_to_block(flow.locals_block);
        self.emit_jump(flow.init_block);

        self.seal_block(flow.init_block);
        self.switch_to_block(flow.init_block);
        self.emit_jump(flow.args_block);

        self.seal_block(flow.args_block);
        self.switch_to_block(flow.args_block);
        self.emit_jump(flow.body_block);

        self.seal_block(flow.return_block);
        self.switch_to_block(flow.return_block);
        if let Some(_block) = dormant_block {
            //self.move_block_after(block);
        }

        if self.support.is_scope_cleanup_checker_enabled() {
            // TODO: self.bridge.assert_scope_id(ScopeRef::NONE);
        }

        // TODO: self.bridge.end_function(optimize);
        self.builder.ins().return_(&[]);

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
            CompileCommand::Number(value) => self.process_number(*value),
            CompileCommand::VariableReference(symbol) => self.process_variable_reference(*symbol),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::DeclareVars(scope_ref) => self.process_declare_vars(*scope_ref),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(*scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
            CompileCommand::Addition => self.process_addition(),
            CompileCommand::Discard => self.process_discard(),
            CompileCommand::Swap => self.process_swap(),
            _ => todo!(),
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

    fn process_call(&mut self, _nargs: u16) {
        // TODO
    }

    fn process_push_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);
        // TODO
    }

    fn process_pop_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);
        // TODO
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

    fn process_discard(&mut self) {
        debug_assert!(!self.operand_stack.is_empty());
        self.operand_stack.pop();
    }

    fn process_swap(&mut self) {
        self.swap();
    }

    // commonly used functions

    // 7.1.4 ToNumber ( argument )
    fn apply_to_numeric(&mut self, operand: Operand) -> NumberIr {
        logger::debug!(event = "to_numeric", ?operand);
        match operand {
            Operand::Undefined => self.emit_number(f64::NAN),
            // Operand::Null => self.bridge.get_zero(),
            // Operand::Boolean(value, ..) => self.bridge.create_boolean_to_number(value),
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

    fn switch_to_block(&mut self, block: Block) {
        self.builder.switch_to_block(block);
    }

    fn seal_block(&mut self, block: Block) {
        self.builder.seal_block(block);
    }

    // instructions

    fn emit_number(&mut self, value: f64) -> NumberIr {
        logger::debug!(event = "emit_number", value);
        NumberIr(self.builder.ins().f64const(value))
    }

    fn emit_to_numeric(&mut self, _any: AnyIr) -> NumberIr {
        todo!();
    }

    fn emit_add(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_add", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fadd(lhs.0, rhs.0))
    }

    fn emit_jump(&mut self, block: Block) {
        self.builder.ins().jump(block, &[]);
    }

    fn emit_get_variable(&mut self, symbol: Symbol, locator: Locator) -> AnyIr {
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

    fn emit_get_global_variable(&mut self, _symbol: Symbol) -> AnyIr {
        todo!();
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

    /// Runtime value and optional compile-time constant value of number type.
    // TODO(perf): compile-time evaluation
    #[allow(unused)]
    Number(NumberIr, Option<f64>),

    /// Runtime value and optional compile-time constant value of any type.
    // TODO(perf): compile-time evaluation
    Any(AnyIr, #[allow(unused)] Option<Value>),

    // Compile-time constant value types.
    VariableReference(Symbol, Locator),
}

#[derive(Clone, Copy, Debug)]
struct NumberIr(Value);

#[derive(Clone, Copy, Debug)]
struct AnyIr(#[allow(unused)] Value);
