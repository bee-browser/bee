mod function;
mod llvmir;
mod logger;
mod semantics;

#[cfg(test)]
mod tests;

use jsparser::SymbolRegistry;

use function::FunctionId;
use function::FunctionRegistry;

pub use llvmir::Module;

pub struct Runtime {
    symbol_registry: SymbolRegistry,
    function_registry: FunctionRegistry,
    executor: llvmir::Executor,
}

impl Runtime {
    pub fn initialize() {
        llvmir::initialize();
    }

    pub fn new() -> Self {
        Self {
            symbol_registry: Default::default(),
            function_registry: FunctionRegistry::new(),
            executor: Default::default(),
        }
    }

    pub fn with_host_function<F>(self, name: &str, func: F) -> Self
    where
        F: Fn(&mut Runtime, &[Value]) -> Value + Send + Sync + 'static,
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

    pub fn eval(&mut self, module: Module) {
        self.executor.register_module(module);
        let func = self.function_registry.get_native_mut(0);
        match self.executor.get_native_func(&func.name) {
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
                );
            },
            None => unreachable!(),
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime::new()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    #[default]
    Undefined,
    Boolean(bool),
    Number(f64),
    Closure(Closure),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Closure {
    func_id: FunctionId,

    // The index of a `Call` in `Fiber::call_stack`, where the function was defined.
    call_index: u32,
}

impl Closure {
    pub fn new(func_id: FunctionId, call_index: usize) -> Self {
        Self {
            func_id,
            call_index: call_index as u32,
        }
    }

    pub fn checked_new(func_id: FunctionId, call_index: usize) -> Option<Self> {
        if call_index > u32::MAX as usize {
            logger::error!(err = "too large", call_index);
            return None;
        }
        Some(Self::new(func_id, call_index))
    }

    #[inline(always)]
    pub fn func_id(&self) -> FunctionId {
        self.func_id
    }

    #[inline(always)]
    pub fn call_index(&self) -> usize {
        self.call_index as usize
    }
}

impl From<u64> for Closure {
    fn from(value: u64) -> Self {
        Self {
            func_id: (value as u32).into(),
            call_index: ((value >> 32) as u32),
        }
    }
}

impl From<Closure> for u64 {
    fn from(value: Closure) -> Self {
        u32::from(value.func_id()) as u64 | (value.call_index as u64) << 32
    }
}

// See https://www.reddit.com/r/rust/comments/ksfk4j/comment/gifzlhg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

type HostFn = unsafe extern "C" fn(
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
    u32,
    *mut std::ffi::c_void,
) -> llvmir::bridge::Value;

// This function generates a wrapper function for each `host_func` at compile time.
#[inline(always)]
fn wrap<F>(host_func: F) -> HostFn
where
    F: Fn(&mut Runtime, &[Value]) -> Value + Send + Sync + 'static,
{
    debug_assert_eq!(std::mem::size_of::<F>(), 0, "Function must have zero size");
    std::mem::forget(host_func);
    wrapper::<F>
}

unsafe extern "C" fn wrapper<F>(
    exec_context: *mut std::ffi::c_void,
    outer_scope: *mut std::ffi::c_void,
    argc: u32,
    argv: *mut std::ffi::c_void,
) -> llvmir::bridge::Value
where
    F: Fn(&mut Runtime, &[Value]) -> Value + Send + Sync + 'static,
{
    #[allow(clippy::uninit_assumed_init)]
    let host_fn = std::mem::MaybeUninit::<F>::uninit().assume_init();
    let runtime = &mut *(exec_context as *mut Runtime);
    let _ = outer_scope;
    let args = std::slice::from_raw_parts(argv as *const llvmir::bridge::Value, argc as usize);
    // TODO: use c-type
    let args: Vec<crate::Value> = args.iter().map(|value| crate::Value::load(value)).collect();
    host_fn(runtime, &args).save()
}
