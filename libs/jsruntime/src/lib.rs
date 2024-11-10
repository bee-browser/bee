mod function;
mod llvmir;
mod logger;
mod semantics;
mod tasklet;
mod types;

use std::ffi::c_void;

use jsparser::SymbolRegistry;

use function::FunctionId;
use function::FunctionRegistry;
use llvmir::Executor;
use types::ReturnValue;

pub use llvmir::CompileError;
pub use llvmir::Module;
pub use semantics::Program;
pub use types::Value;

pub fn initialize() {
    llvmir::initialize();
}

/// Runtime preferences.
#[derive(Default)]
struct RuntimePref {
    /// Enables the scope cleanup checker.
    ///
    /// Insert LLVM IR instructions to check if the cleanup for each scope is performed properly.
    /// Immediately panic the current thread evaluating a JavaScript program if the check fails.
    enable_scope_cleanup_checker: bool,

    /// Enables contextual labels for registers and basic blocks in LLVM IR.
    ///
    /// The labels are disabled by default in a performance point of view.  In this case, the
    /// compiler assigns a sequential number to each register and basic block in the generated LLVM
    /// IR as its name.  When this option is enabled, the compiler generates a contextual label and
    /// assigns it to each one.  This option is useful when you need to read the generated LLVM IR.
    enable_llvmir_labels: bool,
}

pub type BasicRuntime = Runtime<()>;

impl BasicRuntime {
    pub fn new() -> Self {
        Runtime::with_extension(())
    }
}

pub struct Runtime<X> {
    pref: RuntimePref,
    symbol_registry: SymbolRegistry,
    function_registry: FunctionRegistry,
    executor: Executor,
    // TODO: GcArena
    allocator: bumpalo::Bump,
    tasklet_system: tasklet::System,
    extension: X,
}

impl<X> Runtime<X> {
    pub fn with_extension(extension: X) -> Self {
        let functions = llvmir::RuntimeFunctions::new::<X>();
        Self {
            pref: Default::default(),
            symbol_registry: Default::default(),
            function_registry: FunctionRegistry::new(),
            executor: Executor::new(&functions),
            allocator: bumpalo::Bump::new(),
            tasklet_system: tasklet::System::new(),
            extension,
        }
    }

    pub fn extension(&self) -> &X {
        &self.extension
    }

    pub fn extension_mut(&mut self) -> &mut X {
        &mut self.extension
    }

    pub fn enable_scope_cleanup_checker(&mut self) {
        self.pref.enable_scope_cleanup_checker = true;
    }

    pub fn enable_llvmir_labels(&mut self) {
        self.pref.enable_llvmir_labels = true;
    }

    pub fn register_host_function<F, R>(&mut self, name: &str, host_fn: F)
    where
        F: Fn(&mut Self, &[Value]) -> R + Send + Sync + 'static,
        R: Clone + ReturnValue,
    {
        let symbol = self.symbol_registry.intern_str(name);
        let func_id = self.function_registry.register_host_function(symbol);
        self.executor
            .register_host_function(func_id, types::into_lambda(host_fn));
        logger::debug!(event = "register_host_function", name, ?symbol, ?func_id);
    }

    pub fn evaluate(&mut self, module: Module) -> Result<Value, Value> {
        logger::debug!(event = "evaluate");
        self.executor.register_module(module);
        let mut retv = Value::Undefined;
        let status = match self.executor.get_native_function(FunctionId::MAIN) {
            Some(main) => unsafe {
                main(
                    // runtime
                    self.as_void_ptr(),
                    // context
                    std::ptr::null_mut(),
                    // argc
                    0,
                    // argv
                    std::ptr::null_mut(),
                    // retv
                    &mut retv,
                )
            },
            None => unreachable!(),
        };
        retv.into_result(status)
    }

    fn as_void_ptr(&mut self) -> *mut c_void {
        self as *mut Self as *mut c_void
    }

    fn allocator(&self) -> &bumpalo::Bump {
        &self.allocator
    }
}

impl<X> Default for Runtime<X>
where
    X: Default,
{
    fn default() -> Self {
        Runtime::with_extension(Default::default())
    }
}
