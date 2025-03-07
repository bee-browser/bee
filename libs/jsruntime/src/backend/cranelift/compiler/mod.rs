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

pub fn compile<R>(
    support: &mut R,
    program: &Program,
    optimize: bool,
) -> Result<Module, CompileError>
where
    R: CompilerSupport,
{
    logger::debug!(event = "compile");
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

        let module = JITModule::new(builder);

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
    ) -> Compiler<'r, 's, '_, R> {
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

        let mut builder = FunctionBuilder::new(&mut self.context.func, &mut self.builder_context);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        Compiler::new(runtime, scope_tree, builder, &mut self.module)
    }
}

struct Compiler<'r, 's, 'm, R> {
    _support: &'r mut R,

    /// The scope tree of the JavaScript program to compile.
    scope_tree: &'s ScopeTree,

    builder: FunctionBuilder<'m>,
    _module: &'m mut JITModule,

    /// A stack for operands.
    operand_stack: OperandStack,
}

impl<'r, 's, 'm, R> Compiler<'r, 's, 'm, R> {
    fn new(
        _support: &'r mut R,
        scope_tree: &'s ScopeTree,
        builder: FunctionBuilder<'m>,
        _module: &'m mut JITModule,
    ) -> Self {
        Self {
            _support,
            scope_tree,
            builder,
            _module,
            operand_stack: Default::default(),
        }
    }

    fn compile(&mut self, func: &Function, optimize: bool) {
        logger::debug!(event = "start_compile", ?func.name, ?func.id, optimize);

        for command in func.commands.iter() {
            self.process_command(command);
        }

        logger::debug!(event = "end_compile", ?func.name, ?func.id, optimize);
        debug_assert!(self.operand_stack.is_empty());
    }

    fn process_command(&mut self, command: &CompileCommand) {
        logger::debug!(event = "process_command", ?command);
        match command {
            CompileCommand::Nop => (),
            CompileCommand::Number(value) => self.process_number(*value),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::DeclareVars(scope_ref) => self.process_declare_vars(*scope_ref),
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
        }
    }

    // commands

    fn process_number(&mut self, value: f64) {
        let value_ir = self.emit_number(value);
        self.operand_stack
            .push(Operand::Number(value_ir, Some(value)));
    }

    fn process_allocate_locals(&mut self, _num_locals: u16) {
        // TODO
    }

    fn process_declare_vars(&mut self, scope_ref: ScopeRef) {
        debug_assert!(self.scope_tree.scope(scope_ref).is_function());
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
        let lhs = self.to_numeric(lhs);

        let (rhs, _) = self.dereference();
        let rhs = self.to_numeric(rhs);

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
    fn to_numeric(&self, operand: Operand) -> NumberIr {
        logger::debug!(event = "to_numeric", ?operand);
        match operand {
            // Operand::Undefined => self.bridge.get_nan(),
            // Operand::Null => self.bridge.get_zero(),
            // Operand::Boolean(value, ..) => self.bridge.create_boolean_to_number(value),
            Operand::Number(value, ..) => value,
            // Operand::String(..) => unimplemented!("string.to_numeric"),
            // Operand::Closure(_) => self.bridge.get_nan(),
            // Operand::Object(_) => unimplemented!("object.to_numeric"),
            // Operand::Any(value, ..) => self.bridge.to_numeric(value),
            // Operand::Lambda(_)
            // | Operand::Coroutine(_)
            // | Operand::Promise(_)
            // | Operand::VariableReference(..)
            // | Operand::PropertyReference(_) => unreachable!(),
        }
    }

    fn dereference(&mut self) -> (Operand, Option<(Symbol, Locator)>) {
        logger::debug!(event = "dereference", operand_stack.top=?self.operand_stack.last());

        let operand = self.operand_stack.pop().unwrap();
        /*
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
        */
        (operand, None)
    }

    fn swap(&mut self) {
        logger::debug!(event = "swap");
        debug_assert!(self.operand_stack.len() > 1);
        let last_index = self.operand_stack.len() - 1;
        self.operand_stack.swap(last_index - 1, last_index);
    }

    // instructions

    fn emit_number(&mut self, value: f64) -> NumberIr {
        logger::debug!(event = "emit_number", value);
        NumberIr(self.builder.ins().f64const(value))
    }

    fn emit_add(&mut self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        logger::debug!(event = "emit_add", ?lhs, ?rhs);
        NumberIr(self.builder.ins().fadd(lhs.0, rhs.0))
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
    /// Runtime value and optional compile-time constant value of number type.
    // TODO(perf): compile-time evaluation
    #[allow(unused)]
    Number(NumberIr, Option<f64>),
}

#[derive(Clone, Debug)]
struct NumberIr(Value);
