use super::logger;
use super::FunctionId;
use super::Locator;
use super::Runtime;
use super::Symbol;
use super::Value;

// Enable logging only in the debug build.
//
// Functions in this module will be called many times in a short period and logger macros affect
// the performance.
//
// TODO: trace level?
macro_rules! debug {
    ($($tokens:tt)*) => {
        #[cfg(debug_assertions)]
        logger::debug!($($tokens)*);
    };
}

impl Runtime {
    #[inline(always)]
    pub(crate) fn declare_immutable(&mut self, symbol: Symbol, locator: Locator, value: Value) {
        debug!(event = "declare_immutable", ?symbol, ?locator, ?value);
        self.fiber.declare_immutable(symbol, locator, value);
    }

    #[inline(always)]
    pub(crate) fn declare_mutable(&mut self, symbol: Symbol, locator: Locator, value: Value) {
        debug!(event = "declare_mutable", ?symbol, ?locator, ?value);
        self.fiber.declare_mutable(symbol, locator, value);
    }

    #[inline(always)]
    pub(crate) fn declare_function(
        &mut self,
        symbol: Symbol,
        locator: Locator,
        func_id: FunctionId,
    ) {
        debug!(event = "declare_function", ?symbol, ?locator, ?func_id);
        self.fiber.declare_function(symbol, locator, func_id);
    }

    #[inline(always)]
    pub(crate) fn get_binding(&self, symbol: Symbol, locator: Locator) -> Value {
        debug!(event = "get_binding", ?symbol, ?locator);
        self.fiber.get_binding(symbol, locator)
    }

    #[inline(always)]
    pub(crate) fn put_binding(&mut self, symbol: Symbol, locator: Locator, value: Value) {
        debug!(event = "put_binding", ?symbol, ?locator, ?value);
        self.fiber.put_binding(symbol, locator, value);
    }

    #[inline(always)]
    pub(crate) fn push_argument(&mut self, value: Value) {
        debug!(event = "push_argument", ?value);
        self.fiber.push_argument(value);
    }

    #[inline(always)]
    pub(crate) fn call(&mut self, value: Value) -> Value {
        debug!(event = "call", ?value);
        let closure = match value {
            Value::Closure(closure) => closure,
            _ => panic!(), // TODO: throw!()
        };
        // TODO: refactoring
        self.fiber.start_call(closure);
        self.ordinary_call_evaludate_body(closure);
        self.fiber.end_call()
    }

    #[inline(always)]
    pub(crate) fn return_value(&mut self, value: Value) {
        debug!(event = "return_value", ?value);
        self.fiber.return_value(value);
    }

    #[inline(always)]
    pub(crate) fn allocate_bindings(&mut self, n: u16) {
        debug!(event = "allocate_bingings", n);
        self.fiber.allocate_bindings(n);
    }

    #[inline(always)]
    pub(crate) fn release_bindings(&mut self, n: u16) {
        debug!(event = "release_bindings");
        self.fiber.release_bindings(n);
    }

    #[inline(always)]
    pub(crate) fn inspect(&self, value: Value) {
        logger::debug!(event = "inspect", ?value);
    }
}
