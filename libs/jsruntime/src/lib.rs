mod bridge;
mod compiler;
mod executor;
mod function;
mod logger;
mod semantics;

use jsparser::SymbolRegistry;

use bridge::ReturnValue;
use bridge::Status;
use executor::Executor;
use function::FunctionId;
use function::FunctionRegistry;

pub use bridge::Value;
pub use compiler::CompileError;
pub use semantics::Program;

pub struct Runtime {
    symbol_registry: SymbolRegistry,
    function_registry: FunctionRegistry,
    executor: Executor,
}

impl Runtime {
    pub fn initialize() {
        unsafe {
            bridge::llvmir_initialize();
        }
    }

    pub fn new() -> Self {
        Self {
            symbol_registry: Default::default(),
            function_registry: FunctionRegistry::new(),
            executor: Default::default(),
        }
    }

    pub fn with_host_function<F, R>(self, name: &str, func: F) -> Self
    where
        F: Fn(&mut Runtime, &[Value]) -> R + Send + Sync + 'static,
        R: Clone + ReturnValue,
    {
        self.with_host_function_internal(name, wrap(func))
    }

    fn with_host_function_internal(mut self, name: &str, func: HostFn) -> Self {
        let symbol = self.symbol_registry.intern_str(name);
        let func_id = self.function_registry.register_host_function(name);
        self.executor.register_host_function(name, func);
        logger::debug!(
            event = "with_host_function_internal",
            name,
            ?symbol,
            ?func_id
        );
        self
    }

    pub fn evaluate(&mut self, module: Module) -> Result<Value, Value> {
        logger::debug!(event = "evaluate");
        self.executor.register_module(module);
        let main = self.function_registry.get_native_mut(FunctionId::MAIN);
        let mut ret = Value::UNDEFINED;
        let status = match self.executor.get_native_func(&main.name) {
            Some(main) => unsafe {
                main(
                    // exec_context
                    self as *mut Self as *mut std::ffi::c_void,
                    // outer_scope
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
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime::new()
    }
}

pub struct Module {
    peer: *mut bridge::Module,
}

impl Module {
    pub fn print(&self, stderr: bool) {
        unsafe {
            bridge::module_peer_print(self.peer, stderr);
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            bridge::module_peer_delete(self.peer);
        }
    }
}

// See https://www.reddit.com/r/rust/comments/ksfk4j/comment/gifzlhg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

type HostFn = unsafe extern "C" fn(
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
    usize,
    *mut Value,
    *mut Value,
) -> Status;

// This function generates a wrapper function for each `host_func` at compile time.
#[inline(always)]
fn wrap<F, R>(host_func: F) -> HostFn
where
    F: Fn(&mut Runtime, &[Value]) -> R + Send + Sync + 'static,
    R: Clone + ReturnValue,
{
    debug_assert_eq!(std::mem::size_of::<F>(), 0, "Function must have zero size");
    std::mem::forget(host_func);
    wrapper::<F, R>
}

unsafe extern "C" fn wrapper<F, R>(
    exec_context: *mut std::ffi::c_void,
    outer_scope: *mut std::ffi::c_void,
    argc: usize,
    argv: *mut Value,
    ret: *mut Value,
) -> Status
where
    F: Fn(&mut Runtime, &[Value]) -> R + Send + Sync + 'static,
    R: Clone + ReturnValue,
{
    #[allow(clippy::uninit_assumed_init)]
    let host_fn = std::mem::MaybeUninit::<F>::uninit().assume_init();
    let runtime = &mut *(exec_context as *mut Runtime);
    let _ = outer_scope;
    let args = std::slice::from_raw_parts(argv as *const Value, argc);
    // TODO: the return value is copied twice.  that's inefficient.
    let retval = host_fn(runtime, args);
    *ret = retval.value();
    retval.status()
}
