use super::logger;
use super::FunctionId;
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
    pub(crate) fn declare_immutable(&mut self, symbol: Symbol, index: u16, value: Value) {
        debug!(event = "declare_immutable", ?symbol, index, ?value);
        self.fiber.declare_const(symbol, index, value);
    }

    #[inline(always)]
    pub(crate) fn declare_mutable(&mut self, symbol: Symbol, index: u16, value: Value) {
        debug!(event = "declare_mutable", ?symbol, index, ?value);
        self.fiber.declare_variable(symbol, index, value);
    }

    #[inline(always)]
    pub(crate) fn declare_function(&mut self, symbol: Symbol, index: u16, func_id: FunctionId) {
        debug!(event = "declare_function", ?symbol, index, ?func_id);
        self.fiber.declare_function(symbol, index, func_id);
    }

    #[inline(always)]
    pub(crate) fn get_local(&self, symbol: Symbol, stack: u16, index: u16) -> Value {
        debug!(event = "get_local", ?symbol, stack, index);
        self.fiber.get_local(symbol, stack, index)
    }

    #[inline(always)]
    pub(crate) fn put_local(&mut self, symbol: Symbol, stack: u16, index: u16, value: Value) {
        debug!(event = "set_local", ?symbol, stack, index, ?value);
        self.fiber.put_local(symbol, stack, index, value);
    }

    #[inline(always)]
    pub(crate) fn push_argument(&mut self, value: Value) {
        debug!(event = "push_argument", ?value);
        self.fiber.push_arg(value);
    }

    #[inline(always)]
    pub(crate) fn get_argument(&self, symbol: Symbol, index: u16) -> Value {
        debug!(event = "get_argument", ?symbol, index);
        self.fiber.get_argument(symbol, index)
    }

    #[inline(always)]
    pub(crate) fn put_argument(&mut self, symbol: Symbol, index: u16, value: Value) {
        debug!(event = "put_argument", ?symbol, index, ?value);
        self.fiber.put_argument(symbol, index, value);
    }

    #[inline(always)]
    pub(crate) fn call(&mut self, value: Value) -> Value {
        debug!(event = "call", ?value);
        let func = match value {
            Value::Function(func) => func,
            _ => panic!(), // TODO: throw!()
        };
        // TODO: refactoring
        self.fiber.start_call(func);
        self.ordinary_call_evaludate_body(func);
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
