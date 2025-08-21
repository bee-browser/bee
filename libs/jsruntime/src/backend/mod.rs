mod bridge;
mod clir;

pub use clir::CodeRegistry;
pub use clir::compile;
pub use clir::compile_function;
pub use clir::initialize;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    // TODO: define errors
}
