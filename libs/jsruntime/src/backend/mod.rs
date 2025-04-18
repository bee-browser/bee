mod bridge;
mod cranelift;

use cranelift_module::FuncId;
use cranelift_module::Module;

use jsparser::Symbol;
use rustc_hash::FxHashMap;

use crate::Program;
use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaInfo;
use crate::types::Lambda;

pub use cranelift::RuntimeFunctionIds;
pub use bridge::RuntimeFunctions;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    // TODO: define errors
}

pub fn initialize() {
    cranelift::initialize()
}

pub fn compile<X>(
    runtime: &mut Runtime<X>,
    program: &Program,
    optimize: bool,
) -> Result<(), CompileError> {
    cranelift::compile(runtime, program, optimize)
}

trait CompilerSupport {
    // RuntimePref
    fn is_scope_cleanup_checker_enabled(&self) -> bool;

    // SymbolRegistry
    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol;

    // LambdaRegistry
    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo;
    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo;

    fn module(&self) -> &impl Module;
    fn module_mut(&mut self) -> &mut impl Module;
    fn id_map(&self) -> &FxHashMap<LambdaId, FuncId>;
    fn runtime_func_ids(&self) -> &RuntimeFunctionIds;
    fn declare_functions(&mut self, program: &Program);
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

    fn module(&self) -> &impl Module {
        self.executor.module()
    }

    fn module_mut(&mut self) -> &mut impl Module {
        self.executor.module_mut()
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
}

pub struct Executor(cranelift::Executor);

impl Executor {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        Self(cranelift::Executor::new(functions))
    }

    pub fn module(&self) -> &impl Module {
        self.0.module()
    }

    pub fn module_mut(&mut self) -> &mut impl Module {
        self.0.module_mut()
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
}
