use bitflags::bitflags;

use jsparser::Symbol;

use crate::function::FunctionId;
use crate::Closure;
use crate::Locator;
use crate::Value;

pub struct Fiber {
    binding_stack: Vec<Binding>,
    call_stack: Vec<Call>,
}

#[derive(Clone, Default)]
struct Binding {
    flags: BindingFlags,
    symbol: Symbol,
    value: Value,
}

bitflags! {
    #[derive(Clone, Debug, Default)]
    struct BindingFlags: u32 {
        const INITIALIZED = 0b0001;
        const DELETABLE   = 0b0010;
        const MUTABLE     = 0b0100;
        const STRICT      = 0b1000;
    }
}

impl Fiber {
    const INITIAL_BINDING_STACK_CAPACITY: usize = 128;
    const INITIAL_CALL_STACK_CAPACITY: usize = 64;

    pub(crate) fn new() -> Self {
        Self {
            binding_stack: Vec::with_capacity(Self::INITIAL_BINDING_STACK_CAPACITY),
            call_stack: Vec::with_capacity(Self::INITIAL_CALL_STACK_CAPACITY),
        }
    }

    pub(crate) fn declare_immutable(&mut self, symbol: Symbol, locator: Locator, value: Value) {
        debug_assert!(locator.is_local());
        debug_assert_eq!(locator.offset(), 0);
        let call = self.call_stack.last().unwrap();
        let i = call.local_base + locator.index() as usize;
        let binding = &mut self.binding_stack[i];
        // ((CreateImmutableBinding))
        debug_assert!(!binding.flags.contains(BindingFlags::INITIALIZED));
        binding.flags = BindingFlags::INITIALIZED;
        binding.symbol = symbol;
        binding.value = value;
    }

    pub(crate) fn declare_mutable(&mut self, symbol: Symbol, locator: Locator, value: Value) {
        debug_assert!(locator.is_local());
        debug_assert_eq!(locator.offset(), 0);
        let call = self.call_stack.last().unwrap();
        let i = call.local_base + locator.index() as usize;
        let binding = &mut self.binding_stack[i];
        // ((CreateMutableBinding))
        debug_assert!(!binding.flags.contains(BindingFlags::INITIALIZED));
        binding.flags = BindingFlags::INITIALIZED | BindingFlags::MUTABLE | BindingFlags::DELETABLE;
        binding.symbol = symbol;
        binding.value = value;
    }

    pub(crate) fn declare_function(
        &mut self,
        symbol: Symbol,
        locator: Locator,
        func_id: FunctionId,
    ) {
        debug_assert!(locator.is_local());
        debug_assert_eq!(locator.offset(), 0);
        let call_index = self.call_stack.len() - 1;
        let call = &self.call_stack[call_index];
        let i = call.local_base + locator.index() as usize;
        let binding = &mut self.binding_stack[i];
        // ((CreateMutableBinding))
        debug_assert!(!binding.flags.contains(BindingFlags::INITIALIZED));
        binding.flags = BindingFlags::INITIALIZED | BindingFlags::MUTABLE | BindingFlags::DELETABLE;
        binding.symbol = symbol;
        // TODO: should throw a runtime error if the following check fails.
        binding.value = Value::Closure(Closure::checked_new(func_id, call_index).unwrap());
    }

    pub(crate) fn get_binding(&self, symbol: Symbol, locator: Locator) -> Value {
        let mut call = self.call_stack.last().unwrap();
        for _ in 0..locator.offset() {
            call = &self.call_stack[call.closure.call_index()];
        }
        let base = if locator.is_argument() {
            call.arguments_base
        } else {
            debug_assert!(locator.is_local());
            call.local_base
        };
        let binding = &self.binding_stack[base + locator.index() as usize];
        debug_assert!(binding.flags.contains(BindingFlags::INITIALIZED));
        if locator.is_local() {
            debug_assert_eq!(binding.symbol, symbol);
        }
        binding.value.clone()
    }

    pub(crate) fn put_binding(&mut self, symbol: Symbol, locator: Locator, value: Value) {
        let mut call = self.call_stack.last().unwrap();
        for _ in 0..locator.offset() {
            call = &self.call_stack[call.closure.call_index()];
        }
        let base = if locator.is_argument() {
            call.arguments_base
        } else {
            debug_assert!(locator.is_local());
            call.local_base
        };
        let binding = &mut self.binding_stack[base + locator.index() as usize];
        debug_assert!(binding.flags.contains(BindingFlags::INITIALIZED));
        if locator.is_local() {
            debug_assert_eq!(binding.symbol, symbol);
        }
        binding.value = value;
        // TODO: return rval
    }

    #[inline]
    pub(crate) fn push_argument(&mut self, value: Value) {
        self.binding_stack.push(Binding {
            flags: BindingFlags::INITIALIZED | BindingFlags::MUTABLE,
            symbol: Symbol::NONE, // TODO
            value,
        });
    }

    // The top-half of Function.[[Call]]
    pub(crate) fn start_call(&mut self, closure: Closure) {
        self.prepare_for_ordinary_call(closure);
        // TODO: constructor
        // TODO: ((OrdinaryCallBindThis))
    }

    // ((PrepareForOrdinaryCall))
    fn prepare_for_ordinary_call(&mut self, closure: Closure) {
        let local_end = self
            .call_stack
            .last()
            .map(|call| call.local_end)
            .unwrap_or(0);
        // TODO: [[VariableEnvironment]]
        // TODO: [[PrivateEnvironment]]
        self.call_stack
            .push(Call::new(closure, self.binding_stack.len(), local_end));
    }

    pub fn return_value(&mut self, value: Value) {
        let call = self.call_stack.last_mut().unwrap();
        debug_assert!(matches!(call.return_value, Value::Undefined));
        call.return_value = value;
    }

    // The bottom-half of Function.[[Call]]
    pub fn end_call(&mut self) -> Value {
        let call = self.call_stack.pop().unwrap();
        // Drop arguments and local bindings.
        self.binding_stack.truncate(call.arguments_base);
        // TODO: exception
        call.return_value
    }

    pub fn allocate_bindings(&mut self, n: u16) {
        debug_assert!(n > 0);
        let new_len = self.binding_stack.len() + n as usize;
        self.binding_stack.resize_with(new_len, Default::default);
        self.call_stack.last_mut().unwrap().local_end = new_len;
    }

    pub fn release_bindings(&mut self, n: u16) {
        debug_assert!(n > 0);
        let new_len = self.binding_stack.len() - n as usize;
        self.binding_stack.truncate(new_len);
        self.call_stack.last_mut().unwrap().local_end = new_len;
    }
}

// Represents the `Execution Context` specification type.
#[derive(Debug)]
pub struct Call {
    // [[CodeEvaluationState]]
    arguments_base: usize,
    return_value: Value,

    // [[Function]]
    closure: Closure,

    // [[Realm]]

    // [[ScriptOrModule]]

    // [[LexicalEnvironment]]
    // [[VariableEnvironment]]
    // [[PrivateEnvironment]]
    local_base: usize,
    local_end: usize,
}

// Implementation of abstract operations for the `Execution Context` specification type.
impl Call {
    fn new(closure: Closure, local_base: usize, arguments_base: usize) -> Self {
        Self {
            arguments_base,
            return_value: Value::Undefined,
            closure,
            local_base,
            local_end: local_base,
        }
    }
}
