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
use crate::types::Lambda;

pub use bridge::RuntimeFunctions;

macro_rules! use_cranelift_backend {
    () => {
        std::env::var_os("BEE_JSRUNTIME_BACKEND").is_none() ||
            matches!(std::env::var_os("BEE_JSRUNTIME_BACKEND"),
                     Some(backend) if backend == "cranelift")
    };
}

macro_rules! use_llvm_backend {
    () => {
        matches!(std::env::var_os("BEE_JSRUNTIME_BACKEND"),
                 Some(backend) if backend == "llvm")
    };
}

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    // TODO: define errors
}

pub fn initialize() {
    match std::env::var_os("BEE_JSRUNTIME_BACKEND") {
        Some(backend) if backend == "cranelift" => cranelift::initialize(),
        Some(backend) if backend == "llvm" => llvm::initialize(),
        None => cranelift::initialize(),
        _ => unreachable!(),
    }
}

pub fn compile<X>(
    runtime: &mut Runtime<X>,
    program: &Program,
    optimize: bool,
) -> Result<Module, CompileError> {
    match std::env::var_os("BEE_JSRUNTIME_BACKEND") {
        Some(backend) if backend == "cranelift" => {
            cranelift::compile(runtime, program, optimize).map(Module::Cranelift)
        }
        Some(backend) if backend == "llvm" => {
            llvm::compile(runtime, program, optimize).map(Module::Llvm)
        }
        None => cranelift::compile(runtime, program, optimize).map(Module::Cranelift),
        _ => unreachable!(),
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

    fn get_runtime_functions(&self) -> RuntimeFunctions;
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

    fn get_runtime_functions(&self) -> RuntimeFunctions {
        RuntimeFunctions::new::<X>()
    }
}

pub enum Module {
    Cranelift(cranelift::Module),
    Llvm(llvm::Module),
}

impl Module {
    pub fn entry_lambda_id(&self) -> LambdaId {
        match self {
            Self::Cranelift(_module) => {
                if use_cranelift_backend!() {
                    todo!();
                } else {
                    panic!();
                }
            }
            Self::Llvm(module) => {
                if use_llvm_backend!() {
                    module.entry_lambda_id()
                } else {
                    panic!();
                }
            }
        }
    }

    pub fn print(&self, stderr: bool) {
        match self {
            Self::Cranelift(module) => {
                if use_cranelift_backend!() {
                    module.print(stderr);
                } else {
                    panic!();
                }
            }
            Self::Llvm(module) => {
                if use_llvm_backend!() {
                    module.print(stderr);
                } else {
                    panic!();
                }
            }
        }
    }
}

pub enum Executor {
    Cranelift(cranelift::Executor),
    Llvm(llvm::Executor),
}

impl Executor {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        match std::env::var_os("BEE_JSRUNTIME_BACKEND") {
            Some(backend) if backend == "cranelift" => {
                Self::Cranelift(cranelift::Executor::new(functions))
            }
            Some(backend) if backend == "llvm" => Self::Llvm(llvm::Executor::new(functions)),
            None => Self::Cranelift(cranelift::Executor::new(functions)),
            _ => unreachable!(),
        }
    }

    pub fn register_module(&mut self, module: Module) {
        match (self, module) {
            (Self::Cranelift(executor), Module::Cranelift(module)) => {
                if use_cranelift_backend!() {
                    executor.register_module(module)
                } else {
                    panic!();
                }
            }
            (Self::Llvm(executor), Module::Llvm(module)) => {
                if use_llvm_backend!() {
                    executor.register_module(&module);
                } else {
                    panic!();
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn get_data_layout(&self) -> &CStr {
        match self {
            Self::Cranelift(_executor) => {
                if use_cranelift_backend!() {
                    todo!();
                } else {
                    panic!();
                }
            }
            Self::Llvm(executor) => {
                if use_llvm_backend!() {
                    executor.get_data_layout()
                } else {
                    panic!();
                }
            }
        }
    }

    pub fn get_target_triple(&self) -> &CStr {
        match self {
            Self::Cranelift(_executor) => {
                if use_cranelift_backend!() {
                    todo!();
                } else {
                    panic!();
                }
            }
            Self::Llvm(executor) => {
                if use_llvm_backend!() {
                    executor.get_target_triple()
                } else {
                    panic!();
                }
            }
        }
    }

    pub fn get_lambda(&self, lambda_id: LambdaId) -> Option<Lambda> {
        match self {
            Self::Cranelift(executor) => {
                if use_cranelift_backend!() {
                    executor.get_lambda(lambda_id)
                } else {
                    panic!();
                }
            }
            Self::Llvm(executor) => {
                if use_llvm_backend!() {
                    executor.get_lambda(lambda_id)
                } else {
                    panic!();
                }
            }
        }
    }
}
