logging::define_logger! {"bee::jsruntime"}

mod backend;
mod jobs;
mod lambda;
mod objects;
mod semantics;
mod types;

use std::ffi::c_void;
use std::pin::Pin;

use jsparser::Symbol;
use jsparser::SymbolRegistry;

use backend::CodeRegistry;
use jobs::JobRunner;
use lambda::LambdaKind;
use lambda::LambdaRegistry;
use objects::Object;
use objects::Property;
use objects::PropertyKey;
use semantics::Program;
use types::Lambda;
use types::ReturnValue;

pub use backend::CompileError;
pub use lambda::LambdaId; // TODO: private
pub use types::U16Chunk; // TODO: remove
pub use types::U16String;
pub use types::Value;

pub type ParseError = jsparser::Error;

pub fn initialize() {
    backend::initialize();
}

/// Runtime preferences.
#[derive(Default)]
struct RuntimePref {
    /// Enables the scope cleanup checker.
    ///
    /// Insert IR instructions to check if the cleanup for each scope is performed properly.
    /// Immediately panic the current thread evaluating a JavaScript program if the check fails.
    enable_scope_cleanup_checker: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgramId(u32);

impl ProgramId {
    const INVALID: Self = Self(u32::MAX);

    fn new(index: usize) -> Self {
        debug_assert!(index < u32::MAX as usize);
        Self(index as u32)
    }

    fn index(&self) -> usize {
        debug_assert!(self.0 < u32::MAX);
        self.0 as usize
    }
}

pub type BasicRuntime = Runtime<()>;

impl BasicRuntime {
    pub fn new() -> Self {
        Runtime::with_extension(())
    }
}

pub struct Runtime<X> {
    pref: RuntimePref,
    symbol_registry: SymbolRegistry,
    lambda_registry: LambdaRegistry,
    code_registry: CodeRegistry<X>,
    programs: Vec<Program>,
    // TODO: GcArena
    allocator: bumpalo::Bump,
    job_runner: JobRunner,
    global_object: Pin<Box<Object>>,
    monitor: Option<Box<dyn Monitor>>,
    extension: X,

    // %Object.prototype%
    object_prototype: *mut c_void,
    // %String.prototype%
    string_prototype: *mut c_void,
    // %Function.prototype%
    function_prototype: *mut c_void,
}

impl<X> Runtime<X> {
    pub fn with_extension(extension: X) -> Self {
        let global_object = Box::pin(Object::new(std::ptr::null_mut())); // TODO: [[Prototype]]

        let mut runtime = Self {
            pref: Default::default(),
            symbol_registry: Default::default(),
            lambda_registry: LambdaRegistry::new(),
            code_registry: CodeRegistry::new(),
            programs: vec![],
            allocator: bumpalo::Bump::new(),
            job_runner: JobRunner::new(),
            global_object,
            monitor: None,
            extension,
            object_prototype: std::ptr::null_mut(),
            string_prototype: std::ptr::null_mut(),
            function_prototype: std::ptr::null_mut(),
        };

        runtime.define_builtin_global_properties();

        runtime
    }

    fn is_scope_cleanup_checker_enabled(&self) -> bool {
        self.pref.enable_scope_cleanup_checker
    }

    pub fn extension(&self) -> &X {
        &self.extension
    }

    pub fn extension_mut(&mut self) -> &mut X {
        &mut self.extension
    }

    pub fn enable_scope_cleanup_checker(&mut self) {
        self.pref.enable_scope_cleanup_checker = true;
    }

    pub fn set_monitor(&mut self, monitor: Box<dyn Monitor>) {
        self.monitor = Some(monitor);
    }

    pub fn register_host_function<F, R>(&mut self, name: &str, host_fn: F)
    where
        F: Fn(&mut Self, &[Value]) -> R + Send + Sync + 'static,
        R: Clone + ReturnValue,
    {
        let symbol = self.symbol_registry.intern_str(name);
        logger::debug!(event = "register_host_function", name, ?symbol);
        let lambda = types::into_lambda(host_fn);
        let closure = self.create_closure(lambda, LambdaId::HOST, 0);
        let object = self.create_object(std::ptr::null_mut()); // TODO
        object.set_closure(closure);
        let value = Value::Function(object.as_ptr());
        // TODO: add `flags` to the arguments.
        let prop = Property::data_xxx(value);
        let result = self.global_object.define_own_property(symbol.into(), prop);
        debug_assert!(matches!(result, Ok(true)));
    }

    /// Performs AOT-compilations of all functions in a program.
    ///
    /// Unused functions are always compiled.
    pub fn compile(&mut self, program_id: ProgramId, optimize: bool) -> Result<(), CompileError> {
        logger::debug!(event = "compile", ?program_id, optimize);
        backend::compile(self, program_id, optimize)
    }

    /// Evaluates statements in a program.
    ///
    /// Functions in a program must be compiled by [`Runtime::compile()`] before the evaluation.
    pub fn evaluate(&mut self, program_id: ProgramId) -> Result<Value, Value> {
        logger::debug!(event = "evaluate", ?program_id);
        let lambda_id = self.programs[program_id.index()].entry_lambda_id();
        let lambda = self.code_registry.get_lambda(lambda_id).unwrap();
        let module = self.programs[program_id.index()].module;
        self.call_entry_lambda(lambda, module)
    }

    /// Runs a program.
    ///
    /// Functions will be compiled just before being called for the first time.
    pub fn run(&mut self, program_id: ProgramId, optimize: bool) -> Result<Value, Value> {
        logger::debug!(event = "run", ?program_id);
        let lambda_id = self.programs[program_id.index()].entry_lambda_id();
        let lambda = if let Some(lambda) = self.code_registry.get_lambda(lambda_id) {
            lambda
        } else {
            // TODO: compile only top-level statements in the program.
            let function_index = self.programs[program_id.index()].functions.len() - 1;
            let lambda_kind = self.lambda_registry.get(lambda_id).kind;
            if matches!(lambda_kind, LambdaKind::Ramp) {
                debug_assert!(function_index > 0);
                let coroutine_index =
                    self.get_index_of_coroutine_function(program_id, function_index);
                // TODO(fix): handle compilation errors
                backend::compile_function(self, program_id, coroutine_index, optimize).unwrap();
            }
            // TODO(fix): handle compilation errors
            backend::compile_function(self, program_id, function_index, optimize).unwrap();
            self.code_registry.get_lambda(lambda_id).unwrap()
        };
        let module = self.programs[program_id.index()].module;
        let value = self.call_entry_lambda(lambda, module)?;
        // TODO(perf): Memory related to `lambda` can be removed safely after the call.
        // Because the top-level statements are performed only once.
        Ok(value)
    }

    fn get_index_of_coroutine_function(
        &self,
        program_id: ProgramId,
        function_index: usize,
    ) -> usize {
        debug_assert!(function_index > 0);
        // It's assumed that a ramp function contains only a single inner (coroutine) function
        // which has been appended to `Program::functions` just before the ramp function.
        let coroutine_index = function_index - 1;
        debug_assert!(coroutine_index < self.programs[program_id.index()].functions.len());
        debug_assert!(matches!(
            self.lambda_registry
                .get(self.programs[program_id.index()].functions[coroutine_index].id)
                .kind,
            LambdaKind::Coroutine
        ));
        coroutine_index
    }

    /// Calls an entry lambda function.
    fn call_entry_lambda(&mut self, lambda: Lambda, module: bool) -> Result<Value, Value> {
        logger::debug!(event = "call_entry_lambda", ?lambda, module);
        // Specify the global object in the `this` parameter.
        // See also `semantics::Analyzer::start()`.
        //
        // TODO: immutable
        let mut this = Value::Undefined;
        let mut retv = Value::Undefined;
        let status = unsafe {
            lambda(
                // runtime
                self.as_void_ptr(),
                // context
                std::ptr::null_mut(),
                // this
                &mut this,
                // argc
                0,
                // argv
                std::ptr::null_mut(),
                // retv
                &mut retv,
            )
        };
        retv.into_result(status)
    }

    fn as_void_ptr(&mut self) -> *mut c_void {
        self as *mut Self as *mut c_void
    }

    fn allocator(&self) -> &bumpalo::Bump {
        &self.allocator
    }

    fn global_object(&self) -> &Object {
        &self.global_object
    }

    fn global_object_mut(&mut self) -> &mut Object {
        &mut self.global_object
    }

    pub fn ensure_value_on_heap(&mut self, value: &Value) -> Value {
        match value {
            Value::String(string) if string.on_stack() => {
                Value::String(unsafe { self.migrate_string_to_heap(*string) })
            }
            _ => value.clone(),
        }
    }

    // Migrate a UTF-16 string from the stack to the heap.
    pub(crate) unsafe fn migrate_string_to_heap(&mut self, string: U16String) -> U16String {
        logger::debug!(event = "migrate_string_to_heap", ?string);
        debug_assert!(string.on_stack());

        if string.is_empty() {
            return U16String::EMPTY;
        }

        // TODO(issue#237): GcCell
        // TODO: chunk.next
        U16String::new(unsafe { self.alloc_string_rec(string.first_chunk(), std::ptr::null()) })
    }

    pub(crate) fn alloc_utf16(&mut self, utf8: &str) -> &mut [u16] {
        // TODO(perf): inefficient
        let utf16 = utf8.encode_utf16().collect::<Vec<u16>>();
        self.allocator.alloc_slice_copy(&utf16)
    }

    pub(crate) unsafe fn alloc_string_rec(
        &self,
        head: &U16Chunk,
        tail: *const U16Chunk,
    ) -> &U16Chunk {
        let result = self
            .allocator
            .alloc(U16Chunk::new_heap_from_raw_parts(head.ptr, head.len));
        if head.next.is_null() {
            result.next = tail;
        } else {
            let chunk = unsafe { head.next.as_ref().unwrap() };
            result.next = unsafe { self.alloc_string_rec(chunk, tail) };
        }
        result
    }

    fn create_object(&mut self, prototype: *mut c_void) -> &mut Object {
        // TODO: GC
        self.allocator.alloc(Object::new(prototype))
    }

    fn make_property_key(&mut self, value: &Value) -> PropertyKey {
        match value {
            Value::None => unreachable!(),
            Value::Undefined => Symbol::UNDEFINED.into(),
            Value::Null => Symbol::NULL.into(),
            Value::Boolean(false) => Symbol::FALSE.into(),
            Value::Boolean(true) => Symbol::TRUE.into(),
            Value::Number(value) => (*value).into(),
            Value::String(value) => self.symbol_registry.intern_utf16(value.make_utf16()).into(),
            Value::Object(_) | Value::Function(_) => todo!(),
            Value::Promise(_) => todo!(),
        }
    }

    // 7.3.5 CreateDataProperty ( O, P, V )
    fn create_data_property(
        &mut self,
        object: &mut Object,
        key: &PropertyKey,
        value: &Value,
    ) -> Result<bool, Value> {
        let value = self.ensure_value_on_heap(value);
        object.define_own_property(key.clone(), Property::data_wec(value))
    }

    // 7.3.25 CopyDataProperties ( target, source, excludedItems )
    fn copy_data_properties(&mut self, target: &mut Object, source: &Value) -> Result<(), Value> {
        let from = source.to_object()?;
        for (key, prop) in from.iter_own_properties() {
            // TODO: excludedItems
            if prop.is_enumerable() {
                // TODO: 7.3.2 Get ( O, P )
                self.create_data_property(target, key, prop.value())?;
            }
        }
        Ok(())
    }

    fn push_value(&mut self, target: &mut Object, value: &Value) -> Result<(), Value> {
        const LENGTH: PropertyKey = PropertyKey::Symbol(Symbol::LENGTH);

        let length = match target.get_value(&LENGTH) {
            Some(Value::Number(v)) => *v,
            _ => unreachable!(),
        };

        if length >= i32::MAX as f64 {
            // TODO(feat): throw a RangeError
        }

        // TODO: error handling
        let _ = self.create_data_property(target, &PropertyKey::from(length), value);

        target.set_value(&LENGTH, &Value::from(length + 1.0));
        Ok(())
    }
}

impl<X> Default for Runtime<X>
where
    X: Default,
{
    fn default() -> Self {
        Runtime::with_extension(Default::default())
    }
}

pub trait Monitor {
    fn print_function_ir(&mut self, id: LambdaId, ir: &dyn std::fmt::Display);
}
