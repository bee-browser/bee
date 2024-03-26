mod bridge;
mod llvmir;
mod logger;

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::rc::Rc;

use hashbrown::HashMap;

use base::delegate_all;
use jsparser::Symbol;
use jsparser::SymbolTable;

pub use llvmir::Module;

macro_rules! return_if_abrupt {
    ($completion:expr) => {
        match $completion {
            Completion::Normal(value) => value,
            Completion::Throw => return Completion::Throw,
        }
    };
}

macro_rules! normal {
    ($value:expr) => {
        Completion::Normal($value)
    };
}

macro_rules! throw {
    () => {
        Completion::Throw
    };
}

pub struct Runtime {
    symbol_table: SymbolTable,
    world: World,
    fiber: Fiber,
    functions: Vec<Function>,
    executor: llvmir::Executor,
}

impl Runtime {
    pub fn initialize() {
        llvmir::initialize();
    }

    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::with_builtin_symbols(),
            world: World::new(),
            fiber: Fiber::new(),
            functions: vec![Function::Native(NativeFunction {
                formal_parameters: vec![],
                name: CString::new("main".to_string()).unwrap(),
                func: None,
            })],
            executor: Default::default(),
        }
    }

    pub fn with_host_function(mut self, name: &str, func: fn(&[f64])) -> Self {
        let symbol = self.symbol_table.intern(name.encode_utf16().collect());
        let func_id = self.register_host_function(func);
        {
            let mut scope = self.world.global_scope_ref.borrow_mut();
            scope.create_immutable_binding(symbol, false).unwrap();
            scope
                .initialize_binding(symbol, Value::Function(func_id))
                .unwrap();
        }
        self
    }

    pub fn eval(&mut self, module: Module) {
        self.executor.register_module(module);
        let args = vec![];
        let scope = self.world.global_scope_ref.clone();
        self.fiber
            .call_stack
            .push(Call::new(FunctionId(0), args, scope));
        match self.executor.get_main() {
            Some(main) => unsafe {
                main(self as *mut Self as *mut std::ffi::c_void);
            },
            None => unreachable!(),
        }
        self.fiber.call_stack.pop();
    }

    fn create_native_function(&mut self, formal_parameters: Vec<Symbol>) -> (FunctionId, &CStr) {
        let id = self.functions.len();
        debug_assert!(id <= u32::MAX as usize);
        let name = CString::new(format!("fn{id}")).unwrap();
        self.functions.push(Function::Native(NativeFunction {
            formal_parameters,
            name,
            func: None,
        }));
        let name = match self.functions.last().unwrap() {
            Function::Native(func) => func.name.as_c_str(),
            _ => unreachable!(),
        };
        (FunctionId(id as u32), name)
    }

    pub fn register_host_function(&mut self, func: fn(&[f64])) -> FunctionId {
        let id = self.functions.len();
        debug_assert!(id <= u32::MAX as usize);
        self.functions.push(Function::Host(HostFunction { func }));
        FunctionId(id as u32)
    }

    // ((OrdinaryCallEvaluateBody))
    // ((EvaludateBody)) of Function.[[ECMAScriptCode]]
    // ((EvaludateFunctionBody))
    // ((FunctionDeclarationInstantiation))
    fn ordinary_call_evaludate_body(&mut self) -> FunctionId {
        let call = self.fiber.call_stack.last().unwrap();
        // TODO: impl other parts
        // Populate formal parameters into the function scope.
        if let Function::Native(func) = &self.functions[call.func_id.0 as usize] {
            for (i, symbol) in func.formal_parameters.iter().cloned().enumerate() {
                match call.args.get(i) {
                    Some(value) => {
                        let mut scope = call.lexical_scope.borrow_mut();
                        scope.create_mutable_binding(symbol, false);
                        scope.initialize_binding(symbol, Value::Number(*value));
                    }
                    None => {
                        // TODO: undefined or initial value
                    }
                }
            }
        }
        call.func_id
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime::new()
    }
}

/// Implements the `Realm Record` specification type.
pub struct World {
    // TODO: gc-related members
    // TODO: global_object: Object,

    // [[GlobalEnv]]
    global_scope_ref: ScopeRef,
}

impl World {
    // ((CreateRealm)) and ((SetRealmGlobalObject))
    fn new() -> Self {
        Self {
            global_scope_ref: Scope::new_global_scope(),
        }
    }
}

pub struct Fiber {
    args_stack: Vec<Vec<f64>>,
    call_stack: Vec<Call>,
}

impl Fiber {
    pub(crate) fn new() -> Self {
        Self {
            args_stack: vec![],
            call_stack: vec![],
        }
    }

    pub(crate) fn declare_const(&self, symbol: Symbol, value: f64) {
        let call = self.call_stack.last().unwrap();
        call.lexical_scope
            .borrow_mut()
            .create_immutable_binding(symbol, false);
        let binding = call.resolve_binding(symbol, None).unwrap();
        binding.initialize_binding(Value::Number(value)).unwrap();
    }

    pub(crate) fn declare_variable(&self, symbol: Symbol, value: f64) {
        let call = self.call_stack.last().unwrap();
        call.lexical_scope
            .borrow_mut()
            .create_mutable_binding(symbol, true);
        let binding = call.resolve_binding(symbol, None).unwrap();
        binding.initialize_binding(Value::Number(value)).unwrap();
    }

    pub(crate) fn declare_undefined(&self, symbol: Symbol) {
        let call = self.call_stack.last().unwrap();
        call.lexical_scope
            .borrow_mut()
            .create_mutable_binding(symbol, true);
        let binding = call.resolve_binding(symbol, None).unwrap();
        binding.initialize_binding(Value::Undefined).unwrap();
    }

    pub(crate) fn declare_function(&self, symbol: Symbol, func_id: FunctionId) {
        let call = self.call_stack.last().unwrap();
        call.lexical_scope
            .borrow_mut()
            .create_mutable_binding(symbol, true);
        let binding = call.resolve_binding(symbol, None).unwrap();
        binding
            .initialize_binding(Value::Function(func_id))
            .unwrap();
    }

    pub(crate) fn get_value(&self, symbol: Symbol) -> f64 {
        let call = self.call_stack.last().unwrap();
        let binding = call.resolve_binding(symbol, None).unwrap();
        match binding.get_value().unwrap() {
            Value::Number(value) => value,
            _ => panic!(),
        }
    }

    pub(crate) fn put_value(&self, symbol: Symbol, value: f64) {
        let call = self.call_stack.last().unwrap();
        let binding = call.resolve_binding(symbol, None).unwrap();
        binding.put_value(Value::Number(value)).unwrap(); // TODO: throw
                                                          // TODO: return rval
    }

    #[inline]
    pub(crate) fn push_args(&mut self) {
        self.args_stack.push(vec![]);
    }

    #[inline]
    pub(crate) fn push_arg(&mut self, arg: f64) {
        self.args_stack.last_mut().unwrap().push(arg);
    }

    // The top-half of Function.[[Call]]
    pub(crate) fn start_call(&mut self, symbol: Symbol) {
        self.prepare_for_ordinary_call(symbol);
        // TODO: constructor
        // TODO: ((OrdinaryCallBindThis))
    }

    // ((PrepareForOrdinaryCall))
    fn prepare_for_ordinary_call(&mut self, symbol: Symbol) {
        let call = self.call_stack.last().unwrap();
        let outer_scope = call.lexical_scope.clone();
        let binding = call.resolve_binding(symbol, None).unwrap();
        match binding.get_value().unwrap() {
            Value::Function(func_id) => {
                let args = self.args_stack.pop().unwrap();
                let scope = Scope::new_lexical_scope(outer_scope);
                // TODO: [[VariableEnvironment]]
                // TODO: [[PrivateEnvironment]]
                self.call_stack.push(Call::new(func_id, args, scope));
            }
            _ => unreachable!(),
        }
    }

    pub fn ret(&mut self, value: f64) {
        let call = self.call_stack.last_mut().unwrap();
        debug_assert!(call.return_value.is_none());
        call.return_value = Some(value);
    }

    // The bottom-half of Function.[[Call]]
    pub fn end_call(&mut self) -> f64 {
        let call = match self.call_stack.pop() {
            Some(call) => call,
            None => unreachable!(),
        };
        // TODO: exception, undefined
        call.return_value.unwrap_or(0.)
    }

    pub fn push_scope(&mut self) {
        let call = self.call_stack.last_mut().unwrap();
        call.lexical_scope = Scope::new_lexical_scope(call.lexical_scope.clone());
    }

    pub fn pop_scope(&mut self) {
        let call = self.call_stack.last_mut().unwrap();
        let scope = call.lexical_scope.borrow().outer().unwrap();
        call.lexical_scope = scope;
    }
}

// Represents the `Execution Context` specification type.
pub struct Call {
    // [[CodeEvaluationState]]
    args: Vec<f64>,
    return_value: Option<f64>,

    // [[Function]]
    func_id: FunctionId,

    // [[Realm]]

    // [[ScriptOrModule]]

    // [[LexicalEnvironment]]
    lexical_scope: ScopeRef,
    // [[VariableEnvironment]]

    // [[PrivateEnvironment]]
}

// Implementation of abstract operations for the `Execution Context` specification type.
impl Call {
    fn new(func_id: FunctionId, args: Vec<f64>, lexical_scope: ScopeRef) -> Self {
        Self {
            args,
            return_value: None,
            func_id,
            lexical_scope,
        }
    }

    // ((ResolveBinding))
    fn resolve_binding(&self, symbol: Symbol, scope_ref: Option<ScopeRef>) -> Completion<Binding> {
        let scope_ref = match scope_ref {
            Some(scope_ref) => scope_ref,
            None => self.lexical_scope.clone(),
        };
        // TODO: strict mode
        let strict = true;
        Scope::get_binding(scope_ref, symbol, strict)
    }
}

// TODO: Should re-implement using GcCellRef
#[derive(Clone)]
pub struct ScopeRef(Rc<RefCell<Scope>>);

delegate_all! { ScopeRef => Rc<RefCell<Scope>> }

/// Represents the `Environment Record` abstract specification type.
// TODO: Should re-implement using GcCell
pub enum Scope {
    Lexical(LexicalScope),
    Object(ObjectScope),
    Global(GlobalScope),
}

impl Scope {
    // [[OuterEnv]]
    fn outer(&self) -> Option<ScopeRef> {
        match self {
            Scope::Lexical(scope) => scope.outer.clone(),
            Scope::Object(scope) => scope.outer.clone(),
            Scope::Global(_) => None,
        }
    }

    // ((HasBinding))
    fn has_binding(&self, symbol: Symbol) -> Completion<bool> {
        match self {
            Scope::Lexical(scope) => {
                normal!(scope.has_binding(symbol))
            }
            Scope::Object(_scope) => {
                // TODO
                normal!(false)
            }
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    return normal!(true);
                }
                normal!(scope.object_scope.has_binding(symbol))
            }
        }
    }

    // ((CreateMutableBinding))
    fn create_mutable_binding(&mut self, symbol: Symbol, deletable: bool) -> Completion<()> {
        match self {
            Scope::Lexical(scope) => {
                scope.create_mutable_binding(symbol, deletable);
                normal!(())
            }
            Scope::Object(_scope) => {
                // TODO
                normal!(())
            }
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    return throw!(); // TODO: TypeError
                }
                scope
                    .lexical_scope
                    .create_mutable_binding(symbol, deletable);
                normal!(())
            }
        }
    }

    // ((CreateImmutableBinding))
    fn create_immutable_binding(&mut self, symbol: Symbol, strict: bool) -> Completion<()> {
        match self {
            Scope::Lexical(scope) => {
                scope.create_immutable_binding(symbol, strict);
                normal!(())
            }
            Scope::Object(_scope) => {
                // TODO
                normal!(())
            }
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    throw!() // TODO: TypeError
                } else {
                    scope.lexical_scope.create_immutable_binding(symbol, strict);
                    normal!(())
                }
            }
        }
    }

    // ((InitializeBinding))
    fn initialize_binding(&mut self, symbol: Symbol, value: Value) -> Completion<()> {
        match self {
            Scope::Lexical(scope) => {
                scope.initialize_binding(symbol, value);
                normal!(())
            }
            Scope::Object(_scope) => {
                // TODO
                normal!(())
            }
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    scope.lexical_scope.initialize_binding(symbol, value);
                    normal!(())
                } else {
                    // TODO
                    normal!(())
                }
            }
        }
    }

    // ((SetMutableBinding))
    fn set_mutable_binding(
        &mut self,
        symbol: Symbol,
        value: Value,
        strict: bool,
    ) -> Completion<()> {
        match self {
            Scope::Lexical(scope) => scope.set_mutable_binding(symbol, value, strict),
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    scope
                        .lexical_scope
                        .set_mutable_binding(symbol, value, strict)
                } else {
                    // TODO
                    normal!(())
                }
            }
            _ => normal!(()),
        }
    }

    // ((GetBindingValue))
    fn get_binding_value(&self, symbol: Symbol, strict: bool) -> Completion<Value> {
        match self {
            Scope::Lexical(scope) => scope.get_binding_value(symbol, strict),
            Scope::Object(_scope) => {
                // TODO
                throw!()
            }
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    scope.lexical_scope.get_binding_value(symbol, strict)
                } else {
                    // TODO
                    throw!()
                }
            }
        }
    }

    // ((DeleteBinding))
    fn delete_binding(&mut self, symbol: Symbol) -> Completion<bool> {
        match self {
            Scope::Lexical(scope) => scope.delete_binding(symbol),
            Scope::Global(scope) => {
                if scope.lexical_scope.has_binding(symbol) {
                    scope.lexical_scope.delete_binding(symbol)
                } else {
                    // TODO
                    normal!(true)
                }
            }
            _ => unimplemented!(),
        }
    }

    // ((HasThisBinding))
    fn has_this_binding(&self) -> Completion<bool> {
        match self {
            Scope::Lexical(_) => normal!(false),
            Scope::Global(_) => normal!(true),
            _ => unimplemented!(),
        }
    }

    // ((HasSuperBinding))
    fn has_super_binding(&self) -> Completion<bool> {
        match self {
            Scope::Lexical(_) => normal!(false),
            Scope::Global(_) => normal!(false),
            _ => unimplemented!(),
        }
    }

    // ((WithBaseObject))
    fn with_base_object(&self) -> Completion<()> {
        match self {
            Scope::Lexical(_) => normal!(()),
            Scope::Global(_) => normal!(()),
            _ => unimplemented!(),
        }
    }

    // ((GetIdentifierReference))
    fn get_binding(scope_ref: ScopeRef, symbol: Symbol, strict: bool) -> Completion<Binding> {
        let mut scope_ref = scope_ref;
        loop {
            if return_if_abrupt!(scope_ref.borrow().has_binding(symbol)) {
                return normal!(Binding {
                    target: BindTarget::Scope(scope_ref.clone()),
                    symbol,
                    strict,
                });
            }
            let outer = match scope_ref.borrow().outer() {
                Some(scope_ref) => scope_ref,
                None => {
                    return normal!(Binding {
                        target: BindTarget::Unbound,
                        symbol,
                        strict,
                    })
                }
            };
            scope_ref = outer;
        }
    }

    // ((NewGlobalEnvironment))
    fn new_global_scope() -> ScopeRef {
        ScopeRef(Rc::new(RefCell::new(Scope::Global(GlobalScope::new()))))
    }

    // ((NewDeclarativeEnvironment))
    fn new_lexical_scope(outer: ScopeRef) -> ScopeRef {
        ScopeRef(Rc::new(RefCell::new(Scope::Lexical(
            LexicalScope::with_outer(outer),
        ))))
    }
}

/// Represents the `Declarative Environment Record` specification type.
#[derive(Default)]
pub struct LexicalScope {
    outer: Option<ScopeRef>,
    bindings: HashMap<Symbol, ValueHolder>,
}

impl LexicalScope {
    const INITIAL_CAPACITY: usize = 32;

    fn with_outer(outer: ScopeRef) -> Self {
        Self {
            outer: Some(outer),
            bindings: HashMap::with_capacity(Self::INITIAL_CAPACITY),
        }
    }

    #[inline]
    fn has_binding(&self, symbol: Symbol) -> bool {
        self.bindings.contains_key(&symbol)
    }

    fn create_mutable_binding(&mut self, symbol: Symbol, deletable: bool) {
        debug_assert!(!self.has_binding(symbol));
        self.bindings.insert(
            symbol,
            ValueHolder {
                value: None,
                deletable,
                mutable: true,
                strict: false,
            },
        );
    }

    fn create_immutable_binding(&mut self, symbol: Symbol, strict: bool) {
        debug_assert!(!self.has_binding(symbol));
        self.bindings.insert(
            symbol,
            ValueHolder {
                value: None,
                strict,
                deletable: false,
                mutable: true,
            },
        );
    }

    fn initialize_binding(&mut self, symbol: Symbol, value: Value) {
        let holder = self.bindings.get_mut(&symbol).unwrap();
        debug_assert!(holder.value.is_none());
        holder.value = Some(value);
    }

    fn set_mutable_binding(
        &mut self,
        symbol: Symbol,
        value: Value,
        mut strict: bool,
    ) -> Completion<()> {
        match self.bindings.get_mut(&symbol) {
            Some(holder) => {
                if holder.strict() {
                    strict = true;
                }
                if !holder.initialized() {
                    return throw!(); // TODO: ReferenceError
                }
                if holder.mutable() {
                    holder.put_value(value);
                } else if strict {
                    return throw!(); // TODO: TypeError
                }
                normal!(())
            }
            None => {
                if strict {
                    return throw!(); // TODO: ReferenceError
                }
                self.create_mutable_binding(symbol, true);
                self.initialize_binding(symbol, value);
                normal!(())
            }
        }
    }

    fn get_binding_value(&self, symbol: Symbol, _strict: bool) -> Completion<Value> {
        let holder = self.bindings.get(&symbol).unwrap();
        match holder.value {
            Some(ref value) => normal!(value.clone()),
            None => throw!(), // TODO: ReferenceError
        }
    }

    fn delete_binding(&mut self, symbol: Symbol) -> Completion<bool> {
        debug_assert!(self.has_binding(symbol));
        let holder = self.bindings.get_mut(&symbol).unwrap();
        if !holder.deletable() {
            return normal!(false);
        }
        self.bindings.remove(&symbol);
        normal!(true)
    }
}

/// Represents the `Object Environment Record` specification type.
#[derive(Default)]
pub struct ObjectScope {
    outer: Option<ScopeRef>,
}

impl ObjectScope {
    // ((NewObjectEnvironment))
    // TODO: [[BindingObject]], [[IsWithEnvironment]]
    fn new() -> Self {
        Self { outer: None }
    }

    fn has_binding(&self, symbol: Symbol) -> bool {
        // TODO
        false
    }
}

/// Represents the `Global Environment Record` specification type.
pub struct GlobalScope {
    // [[ObjectRecord]]
    object_scope: ObjectScope,

    // [[DeclarativeRecord]]
    lexical_scope: LexicalScope,
}

impl GlobalScope {
    // ((NewGlobalEnvironment))
    fn new() -> Self {
        // TODO
        Self {
            object_scope: Default::default(),
            lexical_scope: Default::default(),
        }
    }
}

// Implementaion of abstruct operations for the `Global Environment Record` specification type.
impl Runtime {
    // ((GetThisBinding))

    // ((HasVarDeclaration))

    // ((HasLexicalDeclaration))

    // ((HasRestrictedGlobalProperty))

    // ((CanDeclareGlobalVar))

    // ((CanDeclareGlobalFunction))

    // ((CreateGlobalVarBinding))

    // ((CreateGlobalFunctionBinding))
}

#[derive(Debug)]
struct ValueHolder {
    value: Option<Value>,
    deletable: bool,
    mutable: bool,
    strict: bool,
}

impl ValueHolder {
    #[inline]
    fn put_value(&mut self, value: Value) {
        self.value = Some(value);
    }

    #[inline]
    fn initialized(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    fn deletable(&self) -> bool {
        self.deletable
    }

    #[inline]
    fn mutable(&self) -> bool {
        self.mutable
    }

    #[inline]
    fn strict(&self) -> bool {
        self.strict
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Undefined,
    Number(f64),
    Function(FunctionId),
}

#[derive(Clone, Copy, Debug)]
pub struct FunctionId(u32);

impl From<u32> for FunctionId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

enum Function {
    Native(NativeFunction),
    Host(HostFunction),
}

struct NativeFunction {
    // [[FormalParameters]]
    // TODO: Vec<BindingElement>
    formal_parameters: Vec<Symbol>,

    // [[ECMAScriptCode]]
    name: CString,
    func: Option<unsafe extern "C" fn(runtime: *mut std::os::raw::c_void) -> f64>,
}

struct HostFunction {
    func: fn(args: &[f64]),
}

/// Represents the `Completion Record` specification type.
pub enum Completion<T> {
    Normal(T),
    //Break,
    //Continue,
    //Return(T),
    Throw, // TODO: GcCellRef
}

impl<T> Completion<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Normal(value) => value,
            _ => panic!(),
        }
    }
}

/// Represents the `Reference Record` specification type.
pub struct Binding {
    // [[Base]]
    target: BindTarget,
    // [[ReferenceName]]
    symbol: Symbol,
    // [[Strict]]
    strict: bool,
    // TODO: [[ThisValue]]
}

pub enum BindTarget {
    Unbound,
    // TODO: Object,
    Scope(ScopeRef),
}

// Implementation of abstract operations for the `Reference Record` specification type.
impl Binding {
    // ((GetValue))
    fn get_value(&self) -> Completion<Value> {
        match self.target {
            BindTarget::Unbound => throw!(), // TODO: ReferenceError
            // TODO: BindTarget::Object
            BindTarget::Scope(ref scope) => {
                scope.borrow().get_binding_value(self.symbol, self.strict)
            }
        }
    }

    // ((PutValue))
    fn put_value(&self, value: Value) -> Completion<()> {
        match self.target {
            BindTarget::Unbound if self.strict => throw!(), // TODO: ReferenceError
            BindTarget::Unbound => {
                // TODO
                throw!()
            }
            // TODO: BindTarget::Object
            BindTarget::Scope(ref scope) => {
                scope
                    .borrow_mut()
                    .set_mutable_binding(self.symbol, value, self.strict)
            }
        }
    }

    // ((InitializeReferencedBinding))
    fn initialize_binding(&self, value: Value) -> Completion<()> {
        match self.target {
            BindTarget::Scope(ref scope) => {
                scope.borrow_mut().initialize_binding(self.symbol, value)
            }
            _ => panic!(),
        }
    }
}
