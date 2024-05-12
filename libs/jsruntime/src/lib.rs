mod function;
mod llvmir;
mod logger;
mod semantics;

#[cfg(test)]
mod tests;

use jsparser::SymbolRegistry;

use function::FunctionId;
use function::FunctionRegistry;

pub use function::HostFn;
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

    pub fn with_host_function<F>(mut self, name: &str, func: F) -> Self
    where
        F: Fn(&mut Runtime, &[Value]) -> Value,
    {
        let symbol = self.symbol_registry.intern_str(name);
        let func_id = self.function_registry.register_host_function(name);
        self.executor.register_host_function(name, func);
        logger::debug!(event = "with_host_function", name, ?symbol, ?func_id);
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
