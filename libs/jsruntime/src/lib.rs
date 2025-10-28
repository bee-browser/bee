logging::define_logger! {"bee::jsruntime"}

#[macro_use]
mod macros;

mod backend;
mod jobs;
mod lambda;
mod objects;
mod semantics;
mod types;

use std::pin::Pin;

use itertools::Itertools;

use jsparser::Symbol;
use jsparser::SymbolRegistry;

use backend::CodeRegistry;
use jobs::JobRunner;
use lambda::LambdaKind;
use lambda::LambdaRegistry;
use objects::Object;
use objects::ObjectHandle;
use objects::Property;
use objects::PropertyKey;
use semantics::Program;
use types::CallContext;
use types::Lambda;
use types::ReturnValue;
use types::Status;

pub use backend::CompileError;
pub use lambda::LambdaId; // TODO: private
pub use types::StringFragment; // TODO: private
pub use types::StringHandle;
pub use types::Value;

pub type ParseError = jsparser::Error;

pub fn initialize() {
    backend::initialize();
}

/// Runtime preferences.
struct RuntimePref {
    /// The maximum call stack depth.
    max_call_stack_depth: u16,

    /// Enables the scope cleanup checker.
    ///
    /// Insert IR instructions to check if the cleanup for each scope is performed properly.
    /// Immediately panic the current thread evaluating a JavaScript program if the check fails.
    enable_scope_cleanup_checker: bool,

    /// Enables the runtime assertions.
    enable_runtime_assert: bool,
}

impl Default for RuntimePref {
    fn default() -> Self {
        Self {
            max_call_stack_depth: 4096,
            enable_scope_cleanup_checker: false,
            enable_runtime_assert: false,
        }
    }
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

    // %Object.prototype%
    object_prototype: Option<ObjectHandle>,
    // %String.prototype%
    string_prototype: Option<ObjectHandle>,
    // %Function.prototype%
    function_prototype: Option<ObjectHandle>,
    // %Error.prototype%
    error_prototype: Option<ObjectHandle>,
    // %AggregateError.prototype%
    aggregate_error_prototype: Option<ObjectHandle>,
    // %EvalError.prototype%
    eval_error_prototype: Option<ObjectHandle>,
    // %InternalError.prototype%
    internal_error_prototype: Option<ObjectHandle>,
    // %RangeError.prototype%
    range_error_prototype: Option<ObjectHandle>,
    // %ReferenceError.prototype%
    reference_error_prototype: Option<ObjectHandle>,
    // %SyntaxError.prototype%
    syntax_error_prototype: Option<ObjectHandle>,
    // %TypeError.prototype%
    type_error_prototype: Option<ObjectHandle>,
    // URIError.prototype%
    uri_error_prototype: Option<ObjectHandle>,

    monitor: Option<Box<dyn Monitor>>,
    extension: X,
}

impl<X> Runtime<X> {
    pub fn with_extension(extension: X) -> Self {
        // TODO: pass [[Prototype]] of the global object.
        let global_object = Box::pin(Object::new(Default::default()));

        let mut runtime = Self {
            pref: Default::default(),
            symbol_registry: Default::default(),
            lambda_registry: LambdaRegistry::new(),
            code_registry: CodeRegistry::new(),
            programs: vec![],
            allocator: bumpalo::Bump::new(),
            job_runner: JobRunner::new(),
            global_object,
            object_prototype: None,
            string_prototype: None,
            function_prototype: None,
            error_prototype: None,
            aggregate_error_prototype: None,
            eval_error_prototype: None,
            internal_error_prototype: None,
            reference_error_prototype: None,
            range_error_prototype: None,
            syntax_error_prototype: None,
            type_error_prototype: None,
            uri_error_prototype: None,
            monitor: None,
            extension,
        };

        runtime.define_builtin_global_properties();

        runtime
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

    pub fn enable_runtime_assert(&mut self) {
        self.pref.enable_runtime_assert = true;
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
        let mut object = self.create_object(self.function_prototype);
        object.set_closure(closure);
        let value = Value::Object(object);
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
        self.call_entry_lambda(lambda_id, lambda, module)
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
        let value = self.call_entry_lambda(lambda_id, lambda, module)?;
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
    fn call_entry_lambda(
        &mut self,
        lambda_id: LambdaId,
        lambda: Lambda<X>,
        module: bool,
    ) -> Result<Value, Value> {
        logger::debug!(event = "call_entry_lambda", ?lambda_id, ?lambda, module);
        let mut args: [_; 0] = [];
        let mut context = CallContext::new_for_entry(&mut args);
        let mut retv = Value::Undefined;
        let status = lambda(self, &mut context, &mut retv);
        retv.into_result(status)
    }

    fn allocator(&self) -> &bumpalo::Bump {
        &self.allocator
    }

    pub fn ensure_value_on_heap(&mut self, value: &Value) -> Value {
        match value {
            Value::String(string) if string.on_stack() => {
                Value::String(self.migrate_string_to_heap(*string))
            }
            _ => value.clone(),
        }
    }

    // Migrate a UTF-16 string from the stack to the heap.
    pub(crate) fn migrate_string_to_heap(&mut self, string: StringHandle) -> StringHandle {
        logger::debug!(event = "migrate_string_to_heap", ?string);
        debug_assert!(string.on_stack());

        if string.is_empty() {
            return StringHandle::EMPTY;
        }

        // TODO(issue#237): GcCell
        StringHandle::new(self.alloc_string_fragment_recursively(string.fragment(), None))
    }

    pub(crate) fn alloc_utf16(&mut self, utf8: &str) -> &mut [u16] {
        // TODO(perf): inefficient
        let utf16 = utf8.encode_utf16().collect::<Vec<u16>>();
        self.allocator.alloc_slice_copy(&utf16)
    }

    pub(crate) fn alloc_string_fragment_recursively(
        &self,
        frag: &StringFragment,
        last: Option<&StringFragment>,
    ) -> &StringFragment {
        let next = if let Some(next) = frag.next() {
            self.alloc_string_fragment_recursively(next, last).as_ptr()
        } else {
            last.map_or(std::ptr::null(), StringFragment::as_ptr)
        };
        self.allocator.alloc(StringFragment::new_heap(next, frag))
    }

    fn create_substring(&mut self, string: StringHandle, start: u32, end: u32) -> StringHandle {
        debug_assert!(start < end);
        // TODO(perf): inefficient
        let utf16 = string
            .code_units()
            .skip(start as usize)
            .take((end - start) as usize)
            .collect_vec();
        let utf16 = self.allocator().alloc_slice_copy(&utf16);
        let frag = StringFragment::new_stack(utf16, true);
        let frag = self
            .allocator()
            .alloc(StringFragment::new_heap(std::ptr::null_mut(), &frag));
        StringHandle::new(frag)
    }

    fn create_object(&mut self, prototype: Option<ObjectHandle>) -> ObjectHandle {
        // TODO: GC
        self.allocator.alloc(Object::new(prototype)).as_handle()
    }

    fn make_property_key(&mut self, value: &Value) -> Result<PropertyKey, Value> {
        match value {
            Value::None => unreachable!(),
            Value::Undefined => Ok(Symbol::KEYWORD_UNDEFINED.into()),
            Value::Null => Ok(Symbol::KEYWORD_NULL.into()),
            Value::Boolean(false) => Ok(Symbol::KEYWORD_FALSE.into()),
            Value::Boolean(true) => Ok(Symbol::KEYWORD_TRUE.into()),
            Value::Number(value) => Ok((*value).into()),
            Value::String(value) => {
                Ok(self.symbol_registry.intern_utf16(value.make_utf16()).into())
            }
            Value::Promise(_) | Value::Object(_) => {
                const MESSAGE: StringHandle = const_string!("TODO: make_property_key");
                match self.create_internal_error(true, &Value::String(MESSAGE), &Value::Undefined) {
                    Ok(err) => Err(Value::Object(err)),
                    Err(err) => Err(err),
                }
            }
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

    fn throw_internal_error(&mut self, message: StringHandle, retv: &mut Value) -> Status {
        match self.create_internal_error(true, &Value::String(message), &Value::Undefined) {
            Ok(value) => *retv = Value::Object(value),
            Err(err) => *retv = err,
        }
        Status::Exception
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

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub(crate) enum Error {
    TypeError,
    RangeError,
    InternalError,
}
