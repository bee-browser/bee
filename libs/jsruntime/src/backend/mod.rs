// NOTE: `cbindgen` does NOT work property together with cfg_if and match_cfg macros.  As a
// temporal workaround, we use `cfg!` instead until we remove the `llvm` module completely.
// At that time, we don't need bridge code between Rust and C++ and we can remove `cbindgen`.
//
// We use a custom key called `backend` that takes `"llvm"` or `"cranelift"` as value.  See
// `build.rs` to check which value is currently specified.

mod bridge;
mod cranelift;
mod llvm;

use std::ffi::CStr;

use jsparser::Symbol;

use crate::Program;
use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaInfo;

pub use bridge::RuntimeFunctions;
pub use llvm::Executor;
pub use llvm::Module;

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    // TODO: define errors
}

pub fn initialize() {
    if cfg!(backend = "llvm") {
        llvm::initialize();
    } else if cfg!(backend = "cranelift") {
        cranelift::initialize();
    } else {
        unreachable!();
    }
}

pub fn compile<X>(
    runtime: &mut Runtime<X>,
    program: &Program,
    optimize: bool,
) -> Result<Module, CompileError> {
    if cfg!(backend = "llvm") {
        llvm::compiler::compile(runtime, program, optimize)
    } else if cfg!(backend = "cranelift") {
        todo!();
    } else {
        unreachable!();
    }
}

trait CompilerSupport {
    // RuntimePref
    fn is_scope_cleanup_checker_enabled(&self) -> bool;
    fn is_llvmir_labels_enabled(&self) -> bool;

    // SymbolRegistry
    fn make_symbol_from_name(&mut self, name: Vec<u16>) -> Symbol;

    // LambdaRegistry
    fn get_lambda_info(&self, lambda_id: LambdaId) -> &LambdaInfo;
    fn get_lambda_info_mut(&mut self, lambda_id: LambdaId) -> &mut LambdaInfo;

    // Executor
    fn get_data_layout(&self) -> &CStr;
    fn get_target_triple(&self) -> &CStr;
}

impl<X> CompilerSupport for Runtime<X> {
    fn is_scope_cleanup_checker_enabled(&self) -> bool {
        self.pref.enable_scope_cleanup_checker
    }

    fn is_llvmir_labels_enabled(&self) -> bool {
        self.pref.enable_llvmir_labels
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
}
