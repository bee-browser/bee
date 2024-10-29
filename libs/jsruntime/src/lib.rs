mod function;
mod llvmir;
mod logger;
mod semantics;
mod tasklet;

use jsparser::SymbolRegistry;

use function::FunctionId;
use function::FunctionRegistry;
use llvmir::Executor;
use llvmir::ReturnValue;
use llvmir::Status;

pub use llvmir::CompileError;
pub use llvmir::Module;
pub use llvmir::Value;
pub use semantics::Program;

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
        let runtime_bridge = llvmir::runtime_bridge::<X>();
        Self {
            pref: Default::default(),
            symbol_registry: Default::default(),
            function_registry: FunctionRegistry::new(),
            executor: Executor::with_runtime_bridge(&runtime_bridge),
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
            .register_host_function(func_id, into_host_lambda(host_fn));
        logger::debug!(event = "register_host_function", name, ?symbol, ?func_id);
    }

    pub fn evaluate(&mut self, module: Module) -> Result<Value, Value> {
        logger::debug!(event = "evaluate");
        self.executor.register_module(module);
        let mut ret = Value::UNDEFINED;
        let status = match self.executor.get_native_function(FunctionId::MAIN) {
            Some(main) => unsafe {
                main(
                    // gctx
                    self as *mut Self as *mut std::ffi::c_void,
                    // lctx
                    std::ptr::null_mut(),
                    // argc
                    0,
                    // argv
                    std::ptr::null_mut(),
                    // ret
                    &mut ret as *mut Value,
                )
            },
            None => unreachable!(),
        };
        ret.into_result(status)
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

// See https://www.reddit.com/r/rust/comments/ksfk4j/comment/gifzlhg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

type HostLambda = unsafe extern "C" fn(
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
    usize,
    *mut Value,
    *mut Value,
) -> Status;

// This function generates a wrapper function for each `host_func` at compile time.
#[inline(always)]
fn into_host_lambda<F, R, X>(host_fn: F) -> HostLambda
where
    F: Fn(&mut Runtime<X>, &[Value]) -> R + Send + Sync + 'static,
    R: Clone + ReturnValue,
{
    debug_assert_eq!(std::mem::size_of::<F>(), 0, "Function must have zero size");
    std::mem::forget(host_fn);
    host_fn_wrapper::<F, R, X>
}

unsafe extern "C" fn host_fn_wrapper<F, R, X>(
    ctx: *mut std::ffi::c_void,
    _caps: *mut std::ffi::c_void,
    argc: usize,
    argv: *mut Value,
    ret: *mut Value,
) -> Status
where
    F: Fn(&mut Runtime<X>, &[Value]) -> R + Send + Sync + 'static,
    R: Clone + ReturnValue,
{
    #[allow(clippy::uninit_assumed_init)]
    let host_fn = std::mem::MaybeUninit::<F>::uninit().assume_init();
    let runtime = &mut *(ctx as *mut Runtime<X>);
    let args = std::slice::from_raw_parts(argv as *const Value, argc);
    // TODO: the return value is copied twice.  that's inefficient.
    let retval = host_fn(runtime, args);
    *ret = retval.value();
    retval.status()
}
