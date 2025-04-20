mod bridge;
mod clir;

use cranelift::codegen;
use cranelift::prelude::isa;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::semantics::Function;
use crate::semantics::Program;
use crate::types::Lambda;

pub use bridge::RuntimeFunctions;

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

pub struct Executor(clir::Executor);

impl Executor {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        Self(clir::Executor::new(functions))
    }

    pub fn target_config(&self) -> isa::TargetFrontendConfig {
        self.0.target_config()
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
