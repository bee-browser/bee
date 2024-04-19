use super::logger;
use super::Function;
use super::FunctionId;
use super::Runtime;
use super::Symbol;

impl Runtime {
    #[inline(always)]
    pub(crate) fn declare_const(&mut self, symbol: Symbol, value: f64) {
        logger::debug!(event = "declare_const", ?symbol, value);
        self.fiber.declare_const(symbol, value);
    }

    #[inline(always)]
    pub(crate) fn declare_variable(&mut self, symbol: Symbol, value: f64) {
        logger::debug!(event = "declare_variable", ?symbol, value);
        self.fiber.declare_variable(symbol, value);
    }

    #[inline(always)]
    pub(crate) fn declare_function(&mut self, symbol: Symbol, func_id: FunctionId) {
        logger::debug!(event = "declare_function", ?symbol, ?func_id);
        self.fiber.declare_function(symbol, func_id);
    }

    #[inline(always)]
    pub(crate) fn get(&self, symbol: Symbol) -> f64 {
        logger::debug!(event = "get", ?symbol);
        self.fiber.get_value(symbol)
    }

    #[inline(always)]
    pub(crate) fn set(&mut self, symbol: Symbol, value: f64) {
        logger::debug!(event = "set", ?symbol, value);
        self.fiber.put_value(symbol, value);
    }

    #[inline(always)]
    pub(crate) fn push_args(&mut self) {
        logger::debug!(event = "push_args");
        self.fiber.push_args();
    }

    #[inline(always)]
    pub(crate) fn push_arg(&mut self, arg: f64) {
        logger::debug!(event = "push_arg", arg);
        self.fiber.push_arg(arg);
    }

    #[inline(always)]
    pub(crate) fn call(&mut self, symbol: Symbol) -> f64 {
        logger::debug!(event = "call", ?symbol);
        self.fiber.start_call(symbol);
        // TODO: refactoring
        let func_id = self.ordinary_call_evaludate_body();
        match &mut self.functions[func_id.0 as usize] {
            Function::Native(func) => {
                // ((Evaluation)) of FunctionStatementList
                let func = match func.func {
                    Some(func) => func,
                    None => {
                        let native_func = self.executor.get_func(&func.name).unwrap();
                        func.func = Some(native_func);
                        native_func
                    }
                };
                unsafe {
                    func(self as *mut Self as *mut std::ffi::c_void);
                }
            }
            // TODO
            Function::Host(func) => {
                let func = func.func;
                let args = self.fiber.call_stack.last().unwrap().args.as_slice();
                func(args);
            }
        }
        self.fiber.end_call()
    }

    #[inline(always)]
    pub(crate) fn ret(&mut self, value: f64) {
        logger::debug!(event = "ret", value);
        self.fiber.ret(value);
    }

    #[inline(always)]
    pub(crate) fn push_scope(&mut self) {
        logger::debug!(event = "push_scope");
        self.fiber.push_scope();
    }

    #[inline(always)]
    pub(crate) fn pop_scope(&mut self) {
        logger::debug!(event = "pop_scope");
        self.fiber.pop_scope();
    }
}
