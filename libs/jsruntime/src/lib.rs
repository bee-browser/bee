mod bridge;
mod fiber;
mod function;
mod llvmir;
mod logger;
mod semantics;

#[cfg(test)]
mod tests;

use jsparser::Symbol;
use jsparser::SymbolRegistry;

use fiber::Fiber;
use function::FunctionId;
use function::FunctionRegistry;
pub use llvmir::Module;

pub struct Runtime {
    symbol_registry: SymbolRegistry,
    function_registry: FunctionRegistry,
    fiber: Fiber,
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
            fiber: Fiber::new(),
            executor: Default::default(),
        }
    }

    pub fn with_host_function(mut self, name: &str, func: fn(&[Value])) -> Self {
        let symbol = self.symbol_registry.intern(name.encode_utf16().collect());
        let func_id = self.function_registry.register_host_function(name, func);
        logger::debug!(event = "with_host_function", name, ?symbol, ?func_id);
        self
    }

    pub fn eval(&mut self, module: Module) {
        self.executor.register_module(module);
        let func = Function {
            id: FunctionId::native(0),
            lexical_scope_index: 0,
        };
        self.fiber.start_call(func);
        let func = self.function_registry.get_native_mut(func.id.value());
        match self.executor.get_func(&func.name) {
            Some(main) => unsafe {
                main(self as *mut Self as *mut std::ffi::c_void);
            },
            None => unreachable!(),
        }
        self.fiber.end_call();
    }

    // ((OrdinaryCallEvaluateBody))
    // ((EvaludateBody)) of Function.[[ECMAScriptCode]]
    // ((EvaludateFunctionBody))
    // ((FunctionDeclarationInstantiation))
    fn ordinary_call_evaludate_body(&mut self, func: Function) {
        if func.id.is_native() {
            let func = self.function_registry.get_native_mut(func.id.value());
            // ((Evaluation)) of FunctionStatementList
            let callable = match func.func {
                Some(callable) => callable,
                None => {
                    let callable = self.executor.get_func(&func.name).unwrap();
                    func.func = Some(callable);
                    callable
                }
            };
            unsafe {
                callable(self as *mut Self as *mut std::ffi::c_void);
            }
        } else {
            let func = self.function_registry.get_host(func.id.value());
            let callable = func.func;
            // TODO
            let args = &[self.fiber.get_argument(Symbol::NONE, 0)];
            callable(args);
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
    Function(Function),
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
pub struct Function {
    pub id: FunctionId,

    // The index of `Call` in `Fiber::call_stack`, in which the function was created.
    pub lexical_scope_index: u32,
}
