mod compiler;

use cranelift::prelude::*;
use cranelift_jit::JITModule;

use super::CompileError;
use super::Program;
use super::RuntimeFunctions;

pub use compiler::compile;

pub fn initialize() {
    // TODO
}

pub struct Module {
    _inner: JITModule,
    context: codegen::Context,
}

impl Module {
    pub fn print(&self, stderr: bool) {
        if stderr {
            eprintln!("{}", self.context.func);
        } else {
            println!("{}", self.context.func);
        }
    }
}

pub struct Executor;

impl Executor {
    pub fn new(_functions: &RuntimeFunctions) -> Self {
        Self
    }
}
