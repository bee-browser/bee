mod bridge;
mod clir;

use cranelift::codegen;
use cranelift::prelude::isa;
use cranelift_module::FuncId;

use jsparser::Symbol;
use rustc_hash::FxHashMap;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaInfo;
use crate::semantics::Function;
use crate::semantics::Program;
use crate::types::Lambda;

pub use bridge::RuntimeFunctions;
pub use clir::RuntimeFunctionIds;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    // TODO: define errors
}

pub fn initialize() {
    clir::initialize()
}

pub fn compile<X>(
    runtime: &mut Runtime<X>,
    program: &Program,
    optimize: bool,
) -> Result<(), CompileError> {
    clir::compile(runtime, program, optimize)
}

trait CompilerSupport {
    // RuntimePref
    fn is_scope_cleanup_checker_enabled(&self) -> bool;

    // SymbolRegistry
    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol;

    // LambdaRegistry
    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo;
    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo;

    fn target_config(&self) -> isa::TargetFrontendConfig;
    fn id_map(&self) -> &FxHashMap<LambdaId, FuncId>;
    fn runtime_func_ids(&self) -> &RuntimeFunctionIds;
    fn declare_functions(&mut self, program: &Program);
    fn define_function(&mut self, func: &Function, ctx: &mut codegen::Context);
}

impl<X> CompilerSupport for Runtime<X> {
    fn is_scope_cleanup_checker_enabled(&self) -> bool {
        self.pref.enable_scope_cleanup_checker
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

    fn target_config(&self) -> isa::TargetFrontendConfig {
        self.executor.target_config()
    }

    fn id_map(&self) -> &FxHashMap<LambdaId, FuncId> {
        self.executor.id_map()
    }

    fn runtime_func_ids(&self) -> &RuntimeFunctionIds {
        self.executor.runtime_func_ids()
    }

    fn declare_functions(&mut self, program: &Program) {
        self.executor.declare_functions(program);
    }

    fn define_function(&mut self, func: &Function, ctx: &mut codegen::Context) {
        self.executor.define_function(func, ctx);
    }
}

pub struct Executor(clir::Executor);

impl Executor {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        Self(clir::Executor::new(functions))
    }

    pub fn target_config(&self) -> isa::TargetFrontendConfig {
        self.0.target_config()
    }

    pub fn id_map(&self) -> &FxHashMap<LambdaId, FuncId> {
        self.0.id_map()
    }

    pub fn runtime_func_ids(&self) -> &RuntimeFunctionIds {
        self.0.runtime_func_ids()
    }

    pub fn link(&mut self) {
        self.0.link();
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda> {
        self.0.get_lambda(lambda_id)
    }

    pub fn declare_functions(&mut self, program: &Program) {
        self.0.declare_functions(program);
    }

    pub fn define_function(&mut self, func: &Function, ctx: &mut codegen::Context) {
        self.0.define_function(func, ctx);
    }
}
