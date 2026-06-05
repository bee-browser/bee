use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::hash::Hasher;

use rustc_hash::FxHashMap;

use jsgc::Handle;
use jsgc::HandleMut;
use jsgc::Trace;
use jsgc::VisitList;
use jsparser::Symbol;

use crate::Error;
use crate::types::Closure;
use crate::types::Promise;
use crate::types::String;
use crate::types::Value;

#[derive(Clone)]
pub enum PropertyKey {
    Symbol(Symbol),
    Number(f64),
}

impl Eq for PropertyKey {}

impl From<u32> for PropertyKey {
    fn from(value: u32) -> Self {
        Symbol::from(value).into()
    }
}

impl From<Symbol> for PropertyKey {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

impl From<f64> for PropertyKey {
    fn from(value: f64) -> Self {
        if value.is_nan() {
            Symbol::NAN.into()
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                Symbol::INFINITY.into()
            } else {
                Symbol::NEG_INFINITY.into()
            }
        } else if value == 0. {
            Self::Number(0.) // convert `-0.` to `0.`
        } else {
            Self::Number(value)
        }
    }
}

impl Hash for PropertyKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Symbol(v) => {
                state.write_u8(1);
                state.write_u64(v.id() as u64);
            }
            Self::Number(v) => {
                state.write_u8(2);
                state.write_u64(v.to_bits());
            }
        }
    }
}

impl PartialEq for PropertyKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Debug for PropertyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Symbol(symbol) => write!(f, "Symbol({symbol})"),
            Self::Number(number) => write!(f, "Number({number})"),
        }
    }
}

// 6.1.7.1 Property Attributes

// TODO(feat): accessor property
// The accessor property can be represented as a tuple of two pointers.  Its size is within
// `size_of::<Value>()` (16 bytes) in any architectures.  Replace `value: Value` with
// `value: [u8; 16]` and define access methods on `Property`.
//
// TODO(refactor): memory layout
// The type of the discriminant value of `Value` is `u8`.  So, there is enough space for storing
// `flags` in `Value`.  We can use the same memory layout in `Value` and `Property`.  When we
// represents the [[Get]] and [[Set]] by using a pair of offsets or indexes shorter than 6 bytes,
// we can also place it in `Value`.
pub struct Property {
    /// The `[[Value]]` attribute.
    value: Value,

    /// Flags for boolean attributes.
    flags: PropertyFlags,
}

// NOTE: Current we use `data_*()` factory methods in order hide internal details of this type.
// Because we'll change its memory layout in the future.
impl Property {
    /// Creates a data property with `[[Writable]]=false`, `[[Enumerable]]=false` and
    /// `[[Configurable]]=false`.
    pub const fn data_xxx(value: Value) -> Self {
        Self::data(value, PropertyFlags::XXX)
    }

    /// Creates a data property with `[[Writable]]=true`, `[[Enumerable]]=false` and
    /// `[[Configurable]]=false`.
    pub const fn data_wxx(value: Value) -> Self {
        Self::data(value, PropertyFlags::WXX)
    }

    /// Creates a data property with `[[Writable]]=false`, `[[Enumerable]]=false` and
    /// `[[Configurable]]=true`.
    pub const fn data_xxc(value: Value) -> Self {
        Self::data(value, PropertyFlags::XXC)
    }

    /// Creates a data property with `[[Writable]]=true`, `[[Enumerable]]=true` and
    /// `[[Configurable]]=true`.
    pub const fn data_wec(value: Value) -> Self {
        Self::data(value, PropertyFlags::WEC)
    }

    /// Creates a data property with `[[Writable]]=true`, `[[Enumerable]]=false` and
    /// `[[Configurable]]=true`.
    pub const fn data_wxc(value: Value) -> Self {
        Self::data(value, PropertyFlags::WXC)
    }

    /// Creates a data property.
    pub const fn data(value: Value, flags: PropertyFlags) -> Self {
        Self {
            value,
            flags: PropertyFlags::DATA.union(flags),
        }
    }

    pub const fn is_writable(&self) -> bool {
        self.flags.is_writable()
    }

    pub const fn is_enumerable(&self) -> bool {
        self.flags.is_enumerable()
    }

    pub const fn is_configurable(&self) -> bool {
        self.flags.is_configurable()
    }

    pub fn value(&self) -> &Value {
        debug_assert!(self.flags.is_data_property());
        &self.value
    }
}

base::auto_bitflags! {
    #[derive(Clone, Copy)]
    pub struct PropertyFlags: u8 {
        /// The data property (true) or the accessor property (false).
        DATA,

        /// The `[[Writable]]` attribute.
        ///
        /// Available only for the data property.
        WRITABLE,

        /// The `[[Enumerable]]` attribute.
        ENUMERABLE,

        /// The `[[Configurable]]` attribute.
        CONFIGURABLE,
    }
}

impl PropertyFlags {
    /// `[[Writable]]: false`, `[[Enumerable]]: false`, `[[Configurable]]: false`
    const XXX: Self = Self::empty();

    /// `[[Writable]]: true`, `[[Enumerable]]: false`, `[[Configurable]]: true`
    const WXX: Self = Self::WRITABLE;

    /// `[[Writable]]: false`, `[[Enumerable]]: false`, `[[Configurable]]: true`
    const XXC: Self = Self::CONFIGURABLE;

    /// `[[Writable]]: true`, `[[Enumerable]]: true`, `[[Configurable]]: true`
    const WEC: Self = Self::WRITABLE
        .union(Self::ENUMERABLE)
        .union(Self::CONFIGURABLE);

    /// `[[Writable]]: true`, `[[Enumerable]]: false`, `[[Configurable]]: true`
    const WXC: Self = Self::WRITABLE.union(Self::CONFIGURABLE);

    const fn is_data_property(&self) -> bool {
        self.contains(Self::DATA)
    }

    const fn is_writable(&self) -> bool {
        self.contains(Self::WRITABLE)
    }

    const fn is_enumerable(&self) -> bool {
        self.contains(Self::ENUMERABLE)
    }

    const fn is_configurable(&self) -> bool {
        self.contains(Self::CONFIGURABLE)
    }
}

// 10 Ordinary and Exotic Objects Behaviours

// 10.1 Ordinary Object Internal Methods and Internal Slots

// TODO(refactor): memory layout
// Separate `properties` into the following two parts:
//
//   1. Memory layout information.
//      This type is used as a map from a property key to an index (or an offset from a base
//      address) of its property.
//      This type should be used as the *hidden class* of the object.
//
//   2. A list of properties (or chunks of properties).
//
// We use a simple hash map until we finishes implementing built-in objects.  After than, we'll
// start reconsidering about the memory layout.
pub struct Object {
    /// An opaque value of the object.
    ///
    /// A pointer to the `Closure` if this is a function object.
    /// A string handle if this is a string object.
    kernel: Kernel,

    flags: ObjectFlags,

    // [[Prototype]]
    prototype: Option<HandleMut<Self>>,
    properties: FxHashMap<PropertyKey, Property>,

    // TODO: rethink the memory layout.
    slots: Vec<Value>,
}

impl Object {
    pub(crate) const KERNEL_DATA_OFFSET: usize =
        std::mem::offset_of!(Self, kernel) + Kernel::DATA_OFFSET;
    pub(crate) const KERNEL_TRACING_OFFSET: usize =
        std::mem::offset_of!(Self, kernel) + Kernel::TRACING_OFFSET;
    pub(crate) const FLAGS_OFFSET: usize = std::mem::offset_of!(Self, flags);
    pub(crate) const PROTOTYPE_OFFSET: usize = std::mem::offset_of!(Self, prototype);

    pub fn new() -> Self {
        Self {
            kernel: Default::default(),
            flags: ObjectFlags::empty(),
            prototype: None,
            properties: Default::default(),
            slots: Default::default(),
        }
    }

    pub fn prototype(&self) -> Option<HandleMut<Object>> {
        self.prototype
    }

    pub fn set_prototype(&mut self, prototype: HandleMut<Object>) {
        self.prototype = Some(prototype);
    }

    pub fn set_no_prototype(&mut self) {
        self.prototype = None;
    }

    // TODO(perf): Which one is better?  `Option::None` or `&Value::None`.
    // In JIT-compiled code, we need a `nullptr` check if we choose `Option::None`.
    // If we choose `&Value::None`, we always need a memory access for the discriminant check of
    // the value but no `nullptr` access happens.
    //
    // TODO(perf): Returning a `Value` degrades the performance.
    // Returning the reference to the value improves the performance.  But it doesn't work in the
    // case of `Property::Accessor` if we don't use a *scratch* memory area in the object in order
    // to store the computation result temporarily and return it from this method as the return
    // value.  Returning the reference to the value works properly if and only if the value is used
    // before it's overwritten.  At this point, we are not sure whether or not it's always works in
    // any expression.
    pub fn get_value(&self, key: &PropertyKey) -> Option<&Value> {
        self.properties
            .get(key)
            .map(|prop| &prop.value)
            .or_else(|| {
                self.prototype
                    .as_ref()
                    .and_then(|prototype| prototype.get_value(key))
            })
    }

    // TODO(feat): strict, writable
    pub fn set_value(&mut self, key: &PropertyKey, value: &Value) {
        self.properties
            .entry(key.clone())
            .and_modify(|prop| {
                // TODO: debug_assert!(prop.is_writable());
                prop.value = value.clone();
            })
            .or_insert(Property::data_xxx(value.clone()));
    }

    pub fn get_own_property(&self, key: &PropertyKey) -> Option<&Property> {
        self.properties.get(key)
    }

    // TODO(feat): 10.1.6.3 ValidateAndApplyPropertyDescriptor ( O, P, extensible, Desc, current )
    pub fn define_own_property(&mut self, key: PropertyKey, prop: Property) -> Result<bool, Error> {
        self.properties.insert(key, prop);
        Ok(true)
    }

    pub fn iter_own_properties(&self) -> impl Iterator<Item = (&PropertyKey, &Property)> {
        self.properties.iter()
    }

    pub(crate) fn set_closure(&mut self, closure: HandleMut<Closure>) {
        self.set_handle_mut(closure);
        self.set_callable();
    }

    pub(crate) fn closure(&self) -> HandleMut<Closure> {
        debug_assert!(self.is_callable());
        HandleMut::from_addr(self.kernel.data).expect("must be a non-null pointer to a Closure")
    }

    pub(crate) fn string(&self) -> Handle<String> {
        // SAFETY: `self.userdata` is non-null and convertible to a reference.
        Handle::from_addr(self.kernel.data).unwrap()
    }

    pub(crate) fn set_string(&mut self, string: Handle<String>) {
        self.set_handle(string);
    }

    pub(crate) fn promise(&self) -> HandleMut<Promise> {
        // TODO: check prototype
        HandleMut::from_addr(self.kernel.data).expect("must be a non-null pointer to a Promise")
    }

    pub(crate) fn set_promise(&mut self, promise: HandleMut<Promise>) {
        self.set_handle_mut(promise);
    }

    fn set_handle<T>(&mut self, handle: Handle<T>) {
        self.kernel.data = handle.as_addr();
        self.kernel.tracing = true;
    }

    fn set_handle_mut<T>(&mut self, handle: HandleMut<T>) {
        self.kernel.data = handle.as_addr();
        self.kernel.tracing = true;
    }

    pub fn as_handle(&mut self) -> HandleMut<Self> {
        // SAFETY: `self` is a non-null pointer to an `Object`.
        HandleMut::from_mut(self)
    }

    pub fn is_instance_of(&self, prototype: HandleMut<Self>) -> bool {
        // TODO: prototype chain
        matches!(self.prototype, Some(p) if p == prototype)
    }

    pub(crate) fn is_constructor(&self) -> bool {
        self.flags.contains(ObjectFlags::CONSTRUCTOR)
    }

    pub(crate) fn set_constructor(&mut self) {
        self.flags.insert(ObjectFlags::CONSTRUCTOR)
    }

    pub(crate) fn is_class_constructor(&self) -> bool {
        self.flags.contains(ObjectFlags::CLASS_CONSTRUCTOR)
    }

    pub(crate) fn set_class_constructor(&mut self) {
        self.flags.insert(ObjectFlags::CLASS_CONSTRUCTOR)
    }

    pub fn is_callable(&self) -> bool {
        self.flags.contains(ObjectFlags::CALLABLE)
    }

    pub(crate) fn set_callable(&mut self) {
        self.flags.insert(ObjectFlags::CALLABLE);
    }

    pub(crate) fn is_error(&self) -> bool {
        self.flags.contains(ObjectFlags::ERROR)
    }

    pub(crate) fn set_error(&mut self) {
        self.flags.insert(ObjectFlags::ERROR);
    }

    pub(crate) fn slots(&self) -> &[Value] {
        &self.slots
    }

    pub(crate) fn slots_mut(&mut self) -> &mut Vec<Value> {
        &mut self.slots
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:p}")
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:p}")
    }
}

impl Trace for Object {
    fn trace(&self, visits: &mut VisitList) {
        self.kernel.trace(visits);
        self.prototype.trace(visits);
        for prop in self.properties.values() {
            prop.value().trace(visits);
        }
        for slot in self.slots.iter() {
            slot.trace(visits);
        }
    }
}

#[derive(Default)]
struct Kernel {
    data: usize,
    tracing: bool,
}

impl Kernel {
    const DATA_OFFSET: usize = std::mem::offset_of!(Self, data);
    const TRACING_OFFSET: usize = std::mem::offset_of!(Self, tracing);
}

impl Trace for Kernel {
    #[inline]
    fn trace(&self, visits: &mut VisitList) {
        if self.tracing {
            visits.push(self.data);
        }
    }
}

base::auto_bitflags! {
    #[derive(Clone, Copy)]
    pub struct ObjectFlags: u8 {
        CONSTRUCTOR,
        CLASS_CONSTRUCTOR,
        CALLABLE,
        ERROR,
    }
}
