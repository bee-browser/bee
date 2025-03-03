mod bridge;
mod compiler;
mod executor;
mod module;

pub use compiler::CompileError;
pub use executor::Executor;
pub use module::Module;

pub use bridge::RuntimeFunctions;

pub fn initialize() {
    bridge::initialize();
}
