mod bridge;
mod cranelift;

use std::ffi::CStr;

use jsparser::Symbol;

use crate::Program;
use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaInfo;
use crate::types::Lambda;

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
) -> Result<Module, CompileError> {
    cranelift::compile(runtime, program, optimize).map(Module)
}

trait CompilerSupport {
    // RuntimePref
    fn is_scope_cleanup_checker_enabled(&self) -> bool;

    // SymbolRegistry
    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol;

    // LambdaRegistry
    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo;
    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo;

    // Executor
    fn get_data_layout(&self) -> &CStr;
    fn get_target_triple(&self) -> &CStr;

    fn get_runtime_functions(&self) -> RuntimeFunctions;
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

    fn get_data_layout(&self) -> &CStr {
        self.executor.get_data_layout()
    }

    fn get_target_triple(&self) -> &CStr {
        self.executor.get_target_triple()
    }

    fn get_runtime_functions(&self) -> RuntimeFunctions {
        RuntimeFunctions::new::<X>()
    }
}

pub struct Module(Box<cranelift::Module>);

impl Module {
    pub fn entry_lambda_id(&self) -> LambdaId {
        todo!();
    }

    pub fn print(&self, stderr: bool) {
        self.0.print(stderr);
    }
}

pub struct Executor(cranelift::Executor);

impl Executor {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        Self(cranelift::Executor::new(functions))
    }

    pub fn register_module(&mut self, module: Module) {
        self.0.register_module(module.0)
    }

    pub fn get_data_layout(&self) -> &CStr {
        todo!();
    }

    pub fn get_target_triple(&self) -> &CStr {
        todo!();
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda> {
        self.0.get_lambda(lambda_id)
    }
}
