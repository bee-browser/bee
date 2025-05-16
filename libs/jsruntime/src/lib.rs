logging::define_logger! {"bee::jsruntime"}

mod backend;
mod lambda;
mod objects;
mod semantics;
mod tasklet;
mod types;

use std::ffi::c_void;

use jsparser::Symbol;
use jsparser::SymbolRegistry;

use backend::Executor;
use lambda::LambdaRegistry;
use objects::Object;
use objects::Property;
use objects::PropertyKey;
use semantics::Program;
use types::ReturnValue;

pub use backend::CompileError;
pub use lambda::LambdaId;
pub use types::U16Chunk; // TODO: remove
pub use types::U16String;
pub use types::Value;

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
    programs: Vec<Program>,
    executor: Executor,
    // TODO: GcArena
    allocator: bumpalo::Bump,
    tasklet_system: tasklet::System,
    global_object: Object,
    monitor: Option<Box<dyn Monitor>>,
    extension: X,
}

impl<X> Runtime<X> {
    pub fn with_extension(extension: X) -> Self {
        let functions = backend::RuntimeFunctions::new::<X>();

        let mut global_object = Object::default();
        global_object.define_builtin_global_properties();

        Self {
            pref: Default::default(),
            symbol_registry: Default::default(),
            lambda_registry: LambdaRegistry::new(),
            programs: vec![],
            executor: Executor::new(&functions),
            allocator: bumpalo::Bump::new(),
            tasklet_system: tasklet::System::new(),
            global_object,
            monitor: None,
            extension,
        }
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
        let value = Value::Closure(closure);
        // TODO: add `flags` to the arguments.
        let prop = Property::data_xxx(value);
        let result = self.global_object.define_own_property(symbol.into(), prop);
        debug_assert!(matches!(result, Ok(true)));
    }

    pub fn compile(&mut self, program_id: ProgramId, optimize: bool) -> Result<(), CompileError> {
        logger::debug!(event = "compile", ?program_id, optimize);
        backend::compile(self, program_id, optimize)
    }

    pub fn evaluate(&mut self, program_id: ProgramId) -> Result<Value, Value> {
        logger::debug!(event = "evaluate", ?program_id);
        let lambda_id = self.programs[program_id.index()].entry_lambda_id();
        let mut retv = Value::Undefined;
        let lambda = self.executor.get_lambda(lambda_id).unwrap();
        let status = unsafe {
            lambda(
                // runtime
                self.as_void_ptr(),
                // context
                std::ptr::null_mut(),
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

    pub fn run(&mut self, program_id: ProgramId, optimize: bool) -> Result<Value, Value> {
        self.compile(program_id, optimize).unwrap(); // TODO(fix): handle compilation errors
        let value = self.evaluate(program_id)?;
        self.process_tasks();
        Ok(value)
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

    pub fn clone_value(&mut self, value: &Value) -> Value {
        match value {
            Value::String(string) if string.first_chunk().on_stack() => {
                let chunk = self.migrate_string_to_heap(string.first_chunk());
                Value::String(U16String::new(chunk))
            }
            _ => value.clone(),
        }
    }

    // Migrate a UTF-16 string from the stack to the heap.
    pub fn migrate_string_to_heap(&mut self, chunk: &U16Chunk) -> &U16Chunk {
        logger::debug!(event = "migrate_string_to_heap", ?chunk);
        debug_assert!(chunk.on_stack());

        if chunk.is_empty() {
            return &U16Chunk::EMPTY;
        }

        // TODO(issue#237): GcCell
        // TODO: chunk.next
        self.allocator
            .alloc(U16Chunk::new_heap_from_raw_parts(chunk.ptr, chunk.len))
    }

    fn create_object(&mut self) -> &mut Object {
        // TODO: GC
        self.allocator.alloc(Default::default())
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
            Value::Closure(_) => todo!(),
            Value::Object(_) => todo!(),
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
        object.define_own_property(key.clone(), Property::data_wec(value.clone()))
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
