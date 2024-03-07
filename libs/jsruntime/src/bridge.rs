use std::ffi::CStr;

use super::logger;
use super::Runtime;
use super::Symbol;

impl Runtime {
    pub(crate) fn declare_const(&mut self, symbol: Symbol, value: f64) {
        logger::debug!(event = "declare_const", ?symbol, value);
        self.fiber.declare_const(symbol, value);
    }

    pub(crate) fn declare_variable(&mut self, symbol: Symbol, value: f64) {
        logger::debug!(event = "declare_variable", ?symbol, value);
        self.fiber.declare_variable(symbol, value);
    }

    pub(crate) fn declare_undefined(&mut self, symbol: Symbol) {
        logger::debug!(event = "declare_undefined", ?symbol);
        self.fiber.declare_undefined(symbol);
    }

    pub(crate) fn declare_function(&mut self, symbol: Symbol, name: &'static CStr) {
        logger::debug!(event = "declare_function", ?symbol, ?name);
        self.fiber.declare_function(symbol, name);
    }

    pub(crate) fn get(&self, symbol: Symbol) -> f64 {
        logger::debug!(event = "get", ?symbol);
        self.fiber.get_value(symbol)
    }

    pub(crate) fn set(&mut self, symbol: Symbol, value: f64) {
        logger::debug!(event = "set", ?symbol, value);
        self.fiber.put_value(symbol, value);
    }

    pub(crate) fn set_undefined(&mut self, symbol: Symbol) {
        /*
        logger::debug!(
            event = "set_undefined",
            ?symbol,
            scope = self.scope_stack.len() - 1
        );
        self.scope_stack
            .last_mut()
            .unwrap()
            .bindings
            .insert(symbol, Value::Undefined);
        */
    }

    pub(crate) fn call(&mut self, symbol: Symbol) -> f64 {
        logger::debug!(event = "call", ?symbol);
        let name = self.fiber.call(symbol);
        match self.executor.get_func(name) {
            Some(func) => unsafe {
                func(self as *mut Self as *mut std::ffi::c_void);
            },
            None => panic!(),
        };
        // Remove scopes created in the function call from the stack
        // if those are remaining.
        //debug_assert!(self.scope_stack.len() >= scope_depth);
        //self.scope_stack.truncate(scope_depth);
        self.fiber.end_call()
    }

    pub(crate) fn ret(&mut self, value: f64) {
        logger::debug!(event = "ret", value);
        self.fiber.ret(value);
    }

    pub(crate) fn push_scope(&mut self) {
        logger::debug!(event = "push_scope");
        self.fiber.push_scope();
    }

    pub(crate) fn pop_scope(&mut self) {
        logger::debug!(event = "pop_scope");
        self.fiber.pop_scope();
    }
}
