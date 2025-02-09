mod lambda;
mod llvmir;
mod logger;
mod objects;
mod semantics;
mod tasklet;
mod types;

use std::ffi::c_void;

use jsparser::SymbolRegistry;

use lambda::LambdaId;
use lambda::LambdaRegistry;
use llvmir::Executor;
use objects::Object;
use objects::Property;
use objects::PropertyFlags;
use objects::PropertyKey;
use types::ReturnValue;

pub use llvmir::CompileError;
pub use llvmir::Module;
pub use semantics::Program;
pub use types::Char16Seq;
pub use types::U16String;
pub use types::Value;

pub fn initialize() {
    llvmir::initialize();
}

/// Runtime preferences.
#[derive(Default)]
struct RuntimePref {
    /// Enables the scope cleanup checker.
    ///
    /// Insert LLVM IR instructions to check if the cleanup for each scope is performed properly.
    /// Immediately panic the current thread evaluating a JavaScript program if the check fails.
    enable_scope_cleanup_checker: bool,

    /// Enables contextual labels for registers and basic blocks in LLVM IR.
    ///
    /// The labels are disabled by default in a performance point of view.  In this case, the
    /// compiler assigns a sequential number to each register and basic block in the generated LLVM
    /// IR as its name.  When this option is enabled, the compiler generates a contextual label and
    /// assigns it to each one.  This option is useful when you need to read the generated LLVM IR.
    enable_llvmir_labels: bool,
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
    executor: Executor,
    // TODO: GcArena
    allocator: bumpalo::Bump,
    tasklet_system: tasklet::System,
    global_object: Object,
    extension: X,
}

impl<X> Runtime<X> {
    pub fn with_extension(extension: X) -> Self {
        let functions = llvmir::RuntimeFunctions::new::<X>();

        let mut global_object = Object::default();
        global_object.define_builtin_global_properties();

        Self {
            pref: Default::default(),
            symbol_registry: Default::default(),
            lambda_registry: LambdaRegistry::new(),
            executor: Executor::new(&functions),
            allocator: bumpalo::Bump::new(),
            tasklet_system: tasklet::System::new(),
            global_object,
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

    pub fn enable_llvmir_labels(&mut self) {
        self.pref.enable_llvmir_labels = true;
    }

    pub fn register_host_function<F, R>(&mut self, name: &str, host_fn: F)
    where
        F: Fn(&mut Self, &[Value]) -> R + Send + Sync + 'static,
        R: Clone + ReturnValue,
    {
        let symbol = self.symbol_registry.intern_str(name);
        logger::debug!(event = "register_host_function", name, ?symbol);
        let lambda = types::into_lambda(host_fn);
        let closure = self.create_closure(lambda, 0);
        let value = Value::Closure(closure);
        // TODO: add `flags` to the arguments.
        let flags = PropertyFlags::empty();
        let prop = Property::Data { value, flags };
        let result = self.global_object.define_own_property(symbol.into(), prop);
        debug_assert!(matches!(result, Ok(true)));
    }

    pub fn link(&mut self, module: Module) {
        logger::debug!(event = "link");
        self.executor.register_module(&module);
    }

    pub fn evaluate(&mut self, program: &Program) -> Result<Value, Value> {
        logger::debug!(event = "evaluate");
        let mut retv = Value::Undefined;
        let status = match self.executor.get_lambda(program.entry_lambda_id()) {
            Some(entry_lambda) => unsafe {
                entry_lambda(
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
            },
            None => unreachable!(),
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

    pub fn clone_value(&mut self, value: &Value) -> Value {
        match value {
            Value::String(string) if string.first_seq().on_stack() => Value::String(
                U16String::new(self.migrate_string_to_heap(string.first_seq())),
            ),
            _ => value.clone(),
        }
    }

    // Migrate a UTF-16 string from the stack to the heap.
    pub fn migrate_string_to_heap(&mut self, seq: &Char16Seq) -> &Char16Seq {
        logger::debug!(event = "migrate_string_to_heap", ?seq);
        debug_assert!(seq.on_stack());

        if seq.is_empty() {
            return &Char16Seq::EMPTY;
        }

        // TODO(issue#237): GcCell
        // TODO: seq.next
        self.allocator
            .alloc(Char16Seq::new_heap_from_raw_parts(seq.ptr, seq.len))
    }

    fn create_object(&mut self) -> &mut Object {
        // TODO: GC
        self.allocator.alloc(Default::default())
    }

    // 7.3.5 CreateDataProperty ( O, P, V )
    fn create_data_property(
        &mut self,
        object: &mut Object,
        key: &PropertyKey,
        value: &Value,
    ) -> Result<bool, Value> {
        object.define_own_property(
            key.clone(),
            Property::Data {
                value: value.clone(),
                flags: PropertyFlags::WRITABLE
                    | PropertyFlags::ENUMERABLE
                    | PropertyFlags::CONFIGURABLE,
            },
        )
    }

    // 7.3.25 CopyDataProperties ( target, source, excludedItems )
    fn copy_data_properties(&mut self, target: &mut Object, source: &Value) -> Result<(), Value> {
        let from = source.to_object()?;
        for (key, desc) in from.iter_own_properties() {
            // TODO: excludedItems
            if desc.is_enumerable() {
                // TODO: 7.3.2 Get ( O, P )
                let value = match desc {
                    Property::Data { value, .. } => value,
                    Property::Accessor { .. } => todo!(),
                };
                self.create_data_property(target, key, value)?;
            }
        }
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
