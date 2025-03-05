mod bridge;
pub mod compiler;
mod executor;
mod module;

pub use executor::Executor;
pub use module::Module;

pub fn initialize() {
    bridge::initialize();
}
