pub(crate) mod builtins;

use std::ffi::c_void;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::ops::DerefMut;

use bitflags::bitflags;
use rustc_hash::FxHashMap;

use jsparser::Symbol;

use crate::types::Closure;
use crate::types::StringHandle;
use crate::types::Value;

#[derive(Clone, Debug)]
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
            Self::Symbol(v) => state.write_u32(v.id()),
            Self::Number(v) => state.write_u64(v.to_bits()),
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
    const fn data(value: Value, flags: PropertyFlags) -> Self {
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

bitflags! {
    #[derive(Clone, Copy)]
    struct PropertyFlags: u8 {
        /// The data property (true) or the accessor property (false).
        const DATA         = 1 << 0;

        /// The `[[Writable]]` attribute.
        ///
        /// Available only for the data property.
        const WRITABLE     = 1 << 1;

        /// The `[[Enumerable]]` attribute.
        const ENUMERABLE   = 1 << 2;

        /// The `[[Configurable]]` attribute.
        const CONFIGURABLE = 1 << 3;
    }
}

impl PropertyFlags {
    /// `[[Writable]]: false`, `[[Enumerable]]: false`, `[[Configurable]]: false`
    const XXX: Self = Self::empty();

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
    userdata: usize,

    flags: ObjectFlags,

    // [[Prototype]]
    prototype: Option<ObjectHandle>,
    properties: FxHashMap<PropertyKey, Property>,
}

impl Object {
    pub(crate) const NUCLEUS_OFFSET: usize = std::mem::offset_of!(Self, userdata);
    pub(crate) const FLAGS_OFFSET: usize = std::mem::offset_of!(Self, flags);

    pub fn new(prototype: Option<ObjectHandle>) -> Self {
        Self {
            userdata: 0,
            flags: ObjectFlags::empty(),
            prototype,
            properties: Default::default(),
        }
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
                    .map(ObjectHandle::as_object)
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
    pub fn define_own_property(&mut self, key: PropertyKey, prop: Property) -> Result<bool, Value> {
        self.properties.insert(key, prop);
        Ok(true)
    }

    pub fn iter_own_properties(&self) -> impl Iterator<Item = (&PropertyKey, &Property)> {
        self.properties.iter()
    }

    pub(crate) fn userdata(&self) -> usize {
        self.userdata
    }

    pub(crate) fn set_closure(&mut self, closure: *mut Closure) {
        self.userdata = closure.addr();
        self.set_callable();
    }

    pub(crate) fn string(&self) -> StringHandle {
        // SAFETY: `self.userdata` is non-null and convertible to a reference.
        unsafe { StringHandle::from_addr(self.userdata) }
    }

    pub(crate) fn set_string(&mut self, string: StringHandle) {
        self.userdata = string.as_addr();
    }

    pub fn as_handle(&mut self) -> ObjectHandle {
        // SAFETY: `self` is a non-null pointer to an `Object`.
        ObjectHandle(unsafe { NonZeroUsize::new_unchecked(self as *mut Self as usize) })
    }

    fn is_instance_of(&self, prototype: Option<ObjectHandle>) -> bool {
        debug_assert!(prototype.is_some());
        // TODO: prototype chain
        self.prototype == prototype
    }

    pub fn set_constructor(&mut self) {
        self.flags.insert(ObjectFlags::CONSTRUCTOR)
    }

    pub fn is_callable(&self) -> bool {
        self.flags.contains(ObjectFlags::CALLABLE)
    }

    fn set_callable(&mut self) {
        self.flags.insert(ObjectFlags::CALLABLE);
    }

    fn is_error(&self) -> bool {
        self.flags.contains(ObjectFlags::ERROR)
    }

    fn set_error(&mut self) {
        self.flags.insert(ObjectFlags::ERROR);
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub(crate) struct ObjectFlags: u8 {
        const CONSTRUCTOR = 1 << 0;
        const CALLABLE    = 1 << 1;
        const ERROR       = 1 << 2;
    }
}

// TODO(issue#237): GcCellRef
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ObjectHandle(NonZeroUsize);

static_assertions::const_assert_eq!(size_of::<ObjectHandle>(), size_of::<*const Object>());

impl ObjectHandle {
    pub fn is_valid(self) -> bool {
        self.0.get() != 0
    }

    pub fn from_ptr(ptr: *mut c_void) -> Option<ObjectHandle> {
        NonZeroUsize::new(ptr as usize).map(ObjectHandle)
    }

    pub fn as_ptr(self) -> *mut c_void {
        self.0.get() as *mut c_void
    }

    pub fn as_addr(self) -> usize {
        self.0.get()
    }

    pub fn as_object<'a>(self) -> &'a Object {
        let ptr = self.0.get() as *const Object;
        debug_assert!(ptr.is_null() || ptr.is_aligned());
        // SAFETY: `ptr` is a non-null pointer to an `Object`.
        unsafe { &*ptr }
    }

    pub fn as_object_mut<'a>(self) -> &'a mut Object {
        let ptr = self.0.get() as *mut Object;
        debug_assert!(ptr.is_null() || ptr.is_aligned());
        // SAFETY: `ptr` is a non-null pointer to an `Object`.
        unsafe { &mut *ptr }
    }

    pub fn dummy_for_testing() -> Self {
        // SAFETY: it's just a dummy data for testing.
        Self(unsafe { NonZeroUsize::new_unchecked(16) })
    }
}

impl Deref for ObjectHandle {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        self.as_object()
    }
}

impl DerefMut for ObjectHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_object_mut()
    }
}

impl Debug for ObjectHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.0.get() as *const Object)
    }
}
