use std::ffi::c_void;
use std::mem::offset_of;
use std::ptr::NonNull;
use std::ptr::addr_eq;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::logger;
use crate::objects::Object;
use crate::objects::ObjectHandle;

// CAUTION: This module contains types used in JIT-generated code.  Please carefully check the
// memory layout of a type you want to change.  It's recommended to use compile-time assertions
// that ensure the memory layout of the type.

/// A data type to hold a JavaScript value.
//
// DO NOT CHANGE THE ORDER OF THE VARIANTS.
// Some operations heavily rely on the order.
#[repr(C, u8)]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None = Self::KIND_NONE,
    Undefined = Self::KIND_UNDEFINED,
    Null = Self::KIND_NULL,
    Boolean(bool) = Self::KIND_BOOLEAN,
    Number(f64) = Self::KIND_NUMBER,
    String(StringHandle) = Self::KIND_STRING,
    Promise(Promise) = Self::KIND_PROMISE,
    Object(ObjectHandle) = Self::KIND_OBJECT,
}

static_assertions::const_assert_eq!(size_of::<Value>(), 16);
static_assertions::const_assert_eq!(align_of::<Value>(), 8);

impl Value {
    // There is no way to define const function to extract the discriminant of each variant.
    pub(crate) const KIND_NONE: u8 = 0;
    pub(crate) const KIND_UNDEFINED: u8 = 1;
    pub(crate) const KIND_NULL: u8 = 2;
    pub(crate) const KIND_BOOLEAN: u8 = 3;
    pub(crate) const KIND_NUMBER: u8 = 4;
    pub(crate) const KIND_STRING: u8 = 5;
    pub(crate) const KIND_PROMISE: u8 = 6;
    pub(crate) const KIND_OBJECT: u8 = 7;

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const HOLDER_SIZE: usize = size_of::<u64>();
    pub(crate) const KIND_OFFSET: usize = 0;
    pub(crate) const HOLDER_OFFSET: usize = size_of::<u64>();

    pub fn is_valid(&self) -> bool {
        !matches!(self, Value::None)
    }

    // 7.1.18 ToObject ( argument )
    pub fn to_object(&self) -> Result<&Object, Value> {
        match self {
            Self::Undefined | Self::Null => Err(1001.into()), // TODO: TypeError
            Self::Boolean(_value) => unimplemented!("new Boolean(value)"),
            Self::Number(_value) => unimplemented!("new Number(value)"),
            Self::String(_value) => unimplemented!("new String(value)"),
            Self::Promise(_value) => unimplemented!("new Promise()"),
            Self::Object(value) => Ok(value.as_object()),
            Self::None => unreachable!(),
        }
    }

    pub fn into_result(self, status: Status) -> Result<Value, Value> {
        logger::debug!(event = "into_result", ?status);
        match status {
            Status::Normal => Ok(self),
            Status::Exception => Err(self),
            _ => unreachable!("{status:?}"),
        }
    }

    // 13.5.3.1 Runtime Semantics: Evaluation
    pub fn get_typeof(&self) -> StringHandle {
        const UNDEFINED: StringHandle = const_string!("undefined");
        const BOOLEAN: StringHandle = const_string!("boolean");
        const NUMBER: StringHandle = const_string!("number");
        const STRING: StringHandle = const_string!("string");
        const OBJECT: StringHandle = const_string!("object");
        const FUNCTION: StringHandle = const_string!("function");

        match self {
            Self::None => unreachable!(),
            Self::Undefined => UNDEFINED,
            Self::Null => OBJECT,
            Self::Boolean(_) => BOOLEAN,
            Self::Number(_) => NUMBER,
            Self::String(_) => STRING,
            Self::Promise(_) => OBJECT,
            Self::Object(object) => {
                if object.is_callable() {
                    FUNCTION
                } else {
                    OBJECT
                }
            }
        }
    }

    pub fn dummy_object() -> Self {
        Self::Object(ObjectHandle::dummy_for_testing())
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Undefined
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::from(value as f64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::from(value as f64)
    }
}

impl From<Promise> for Value {
    fn from(value: Promise) -> Self {
        Self::Promise(value)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Undefined => write!(f, "undefined"),
            Self::Null => write!(f, "null"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Promise(value) => write!(f, "{value:?}"),
            Self::Object(value) => write!(f, "object({value:?})"),
        }
    }
}

/// A data type to hold an **immutable** UTF-16 string.
///
/// A UTF-16 string is represented as a *chain* of **immutable** fragments of UTF-16 code units.
///
/// This type is usually allocated on the stack and holds a pointer to a `StringFragment` that is
/// allocated in the heap or on the stack.
// TODO(issue#237): GcCell
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct StringHandle(NonNull<StringFragment>);

static_assertions::const_assert_eq!(align_of::<StringHandle>(), align_of::<usize>());

impl StringHandle {
    /// An empty string.
    pub const EMPTY: Self = Self::new(&StringFragment::EMPTY);

    /// Creates a new UTF-16 string.
    pub const fn new(frag: &StringFragment) -> Self {
        Self(NonNull::from_ref(frag))
    }

    /// Creates a new constant UTF-16 string.
    pub const fn new_const(frag: &'static StringFragment) -> Self {
        debug_assert!(frag.is_const());
        Self(NonNull::from_ref(frag))
    }

    /// Returns `true` if the string is empty.
    pub const fn is_empty(&self) -> bool {
        self.fragment().is_empty()
    }

    pub(crate) fn is_const(&self) -> bool {
        let frag = self.fragment();
        frag.is_const() && frag.next().is_none()
    }

    /// Returns `true` if the string is allocated on the stack.
    pub(crate) fn on_stack(&self) -> bool {
        self.fragment().on_stack()
    }

    /// Returns the number of UTF-16 code units in the string.
    pub fn len(&self) -> u32 {
        self.fragment().total_len()
    }

    /// Returns the first string fragment.
    pub(crate) const fn fragment(&self) -> &StringFragment {
        // SAFETY: `self.0` is always convertible to a reference.
        unsafe { self.0.as_ref() }
    }

    /// Creates a `Vec` containing UTF-16 code units of the string.
    pub(crate) fn make_utf16(&self) -> Vec<u16> {
        self.fragment().make_utf16()
    }

    pub(crate) unsafe fn from_addr(addr: usize) -> Self {
        debug_assert_ne!(addr, 0);
        Self::new(unsafe {
            let ptr = addr as *const StringFragment;
            debug_assert!(ptr.is_aligned());
            &*ptr
        })
    }

    pub(crate) fn as_addr(&self) -> usize {
        self.0.addr().get()
    }
}

impl PartialEq for StringHandle {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        self.fragment() == other.fragment()
    }
}

impl std::fmt::Debug for StringHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "StringHandle()")
        } else {
            write!(f, "StringHandle({:?})", self.fragment())
        }
    }
}

impl std::fmt::Display for StringHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            Ok(())
        } else {
            write!(f, "{}", self.fragment())
        }
    }
}

/// A data type representing an **immutable** fragment of UTF-16 code units.
///
/// This type may be allocated on the stack.
// TODO(issue#237): GcCell
#[derive(Clone, Debug)]
#[repr(C)]
pub struct StringFragment {
    /// A pointer to the next string fragment if it exists.
    next: *const StringFragment,

    /// A pointer to the array of UTF-16 code units if it exists.
    ptr: *const u16,

    /// The number of the UTF-16 code units in the string fragment.
    len: u32,

    kind: StringFragmentKind,
}

static_assertions::const_assert_eq!(align_of::<StringFragment>(), align_of::<usize>());

impl StringFragment {
    pub(crate) const EMPTY: Self = Self::new_const_from_raw_parts(std::ptr::null(), 0);

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const NEXT_OFFSET: usize = std::mem::offset_of!(Self, next);
    pub(crate) const PTR_OFFSET: usize = std::mem::offset_of!(Self, ptr);
    pub(crate) const LEN_OFFSET: usize = std::mem::offset_of!(Self, len);
    pub(crate) const KIND_OFFSET: usize = std::mem::offset_of!(Self, kind);

    // TODO(refactor): should be private
    pub const fn new_const(slice: &'static [u16]) -> Self {
        Self::new_const_from_raw_parts(slice.as_ptr(), slice.len() as u32)
    }

    pub(crate) const fn new_stack(slice: &[u16]) -> Self {
        Self::new_stack_from_raw_parts(slice.as_ptr(), slice.len() as u32)
    }

    pub(crate) const fn new_const_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: StringFragmentKind::Const,
        }
    }

    pub(crate) const fn new_stack_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: StringFragmentKind::Stack,
        }
    }

    pub(crate) const fn new_heap_from_raw_parts(
        next: *const Self,
        ptr: *const u16,
        len: u32,
    ) -> Self {
        Self {
            next,
            ptr,
            len,
            kind: StringFragmentKind::Heap,
        }
    }

    pub(crate) const fn is_empty(&self) -> bool {
        debug_assert!(self.len > 0 || self.next.is_null());
        self.len == 0
    }

    pub(crate) const fn is_const(&self) -> bool {
        matches!(self.kind, StringFragmentKind::Const)
    }

    pub(crate) const fn on_stack(&self) -> bool {
        matches!(self.kind, StringFragmentKind::Stack)
    }

    pub(crate) fn total_len(&self) -> u32 {
        // SAFETY: `self.next` is null or a valid pointer to a `StringFragment`.
        if let Some(next) = unsafe { self.next.as_ref() } {
            debug_assert!(self.len > 0);
            self.len + next.total_len()
        } else {
            self.len
        }
    }

    pub(crate) fn raw_ptr(&self) -> *const u16 {
        self.ptr
    }

    pub(crate) fn len(&self) -> u32 {
        self.len
    }

    pub(crate) fn as_slice(&self) -> &[u16] {
        debug_assert_ne!(self.len, 0);
        debug_assert!(!self.ptr.is_null());
        debug_assert!(self.ptr.is_aligned());
        // SAFETY: `self.ptr` is always pointer to an array of `u16`.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) }
    }

    pub(crate) fn next(&self) -> Option<&Self> {
        // SAFETY: `self.next` is null or convertible to a reference.
        debug_assert!(self.next.is_null() || self.next.is_aligned());
        unsafe { self.next.as_ref() }
    }

    pub(crate) fn make_utf16(&self) -> Vec<u16> {
        if self.is_empty() {
            return vec![];
        }

        let mut result = vec![];
        let mut chunk = self;
        loop {
            result.extend_from_slice(chunk.as_slice());
            // SAFETY: `chunk.next` is null or a valid pointer to a `StringFragment`.
            chunk = if let Some(next) = unsafe { chunk.next.as_ref() } {
                next
            } else {
                break;
            };
        }
        result
    }

    pub(crate) fn as_ptr(&self) -> *const Self {
        self as *const Self
    }
}

// The UTF-16 code units never change.
unsafe impl Send for StringFragment {}
unsafe impl Sync for StringFragment {}

impl PartialEq for StringFragment {
    fn eq(&self, other: &Self) -> bool {
        // TODO(perf): slow...
        let lhs = self.make_utf16();
        let rhs = other.make_utf16();
        lhs == rhs
    }
}

impl std::fmt::Display for StringFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let mut chunk = self;
        loop {
            let slice = chunk.as_slice();
            let chars: String = char::decode_utf16(slice.iter().cloned())
                .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
                .collect();
            write!(f, "{}", chars.escape_default())?;
            // SAFETY: `chunk.next` is null or a valid pointer to a `StringFragment`.
            chunk = if let Some(next) = unsafe { chunk.next.as_ref() } {
                next
            } else {
                break;
            };
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum StringFragmentKind {
    Const = 0,
    Stack,
    Heap,
}

/// A data type to represent a closure.
//
// TODO(issue#237): GcCell
#[repr(C)]
pub struct Closure {
    /// An address of a lambda function compiled from a JavaScript function definition.
    ///
    /// This filed is initially set to a runtime function that will perform the lazy compilation of
    /// the JavaScript function and set the actual lambda function to this field.
    //
    // NOTE: Using Lambda<X> instead of LambdaAddr causes some problems.  For example, functions
    // such as `std::mem::offset_of!()` and `std::mem::align_of()` does not work with generic
    // types such as Closure<X> even though the size of Lambda<X> is always equal to the size of
    // usize regardless of the actual type of X.
    pub lambda: LambdaAddr,

    /// The ID of `lambda`.
    pub lambda_id: LambdaId,

    /// The number of captures.
    ///
    /// Usually, this field does not used in the compiled function, but we add this field here for
    /// debugging purposes.  If we need to reduce the heap memory usage and `Closure`s dominant, we
    /// can remove this field.
    pub num_captures: u16,

    /// A variable-length list of captures used in the lambda function.
    //
    // TODO(issue#237): GcCellRef
    pub captures: [*mut Capture; 32],
}

static_assertions::const_assert_eq!(align_of::<Closure>(), 8);

impl Closure {
    pub(crate) const LAMBDA_OFFSET: usize = std::mem::offset_of!(Self, lambda);
    pub(crate) const CAPTURES_OFFSET: usize = std::mem::offset_of!(Self, captures);
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "closure({:?}, [", self.lambda_id)?;
        let len = self.num_captures as usize;
        let data = self.captures.as_ptr();
        // SAFETY: `data` is a non-null pointer to an array of pointers.
        let captures = unsafe {
            debug_assert!(!data.is_null());
            debug_assert!(data.is_aligned());
            std::slice::from_raw_parts(data, len)
        };
        let mut captures = captures.iter();
        if let Some(capture) = captures.next() {
            // SAFETY: `capture` is a non-null pointer to a `Capture`.
            write!(f, "{:?}", unsafe {
                debug_assert!(!(*capture).is_null());
                debug_assert!((*capture).is_aligned());
                &**capture
            })?;
            for capture in captures {
                // SAFETY: `capture` is a non-null pointer to a `Capture`.
                write!(f, ", {:?}", unsafe {
                    debug_assert!(!(*capture).is_null());
                    debug_assert!((*capture).is_aligned());
                    &**capture
                })?;
            }
        }
        write!(f, "])")
    }
}

/// A data type to track a captured value.
//
// NOTE: The `target` may point to the `escaped`.  In this case, the `target` must be updated if
// the capture is moved during GC, so that the `target` points to the `escaped` correctly.
//
// TODO(issue#237): GcCell
#[repr(C)]
pub struct Capture {
    /// A captured value.
    ///
    /// This may point to the `escaped`.
    pub target: *mut Value,

    /// Data storage for escaped value.
    pub escaped: Value,
}

static_assertions::const_assert_eq!(size_of::<Capture>(), 24);
static_assertions::const_assert_eq!(align_of::<Capture>(), 8);
static_assertions::const_assert_eq!(offset_of!(Capture, escaped), 8);

impl Capture {
    pub(crate) const TARGET_OFFSET: usize = std::mem::offset_of!(Self, target);
    pub(crate) const ESCAPED_OFFSET: usize = std::mem::offset_of!(Self, escaped);

    fn is_escaped(&self) -> bool {
        debug_assert!(!self.target.is_null());
        addr_eq(self.target, &self.escaped)
    }
}

impl std::fmt::Debug for Capture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_escaped() {
            write!(
                f,
                "capture(escaped: {:?}, value: {:?})",
                self.target, self.escaped
            )
        } else {
            write!(f, "capture(onstack: {:?})", self.target)
        }
    }
}

/// A data type to represent a coroutine.
///
/// The scratch_buffer starts from `&Coroutine::locals[Coroutine::num_locals]`.
//
// TODO(issue#237): GcCell
#[derive(Debug)]
#[repr(C)]
pub struct Coroutine {
    /// The closure of the coroutine.
    //
    // TODO(issue#237): GcCellRef
    pub closure: *mut Closure,

    /// The state of the coroutine.
    pub state: u32,

    /// The number of the local variables used in the coroutine.
    pub num_locals: u16,

    /// The current scope ID used by the scope cleanup checker.
    pub scope_id: u16,

    /// The size of the scratch buffer in bytes.
    pub scratch_buffer_len: u16,

    /// A variable-length list of local variables used in the coroutine.
    ///
    /// `Capture::target` may point to one of `locals[]`.
    pub locals: [Value; 32],
}

static_assertions::const_assert_eq!(align_of::<Coroutine>(), 8);

impl Coroutine {
    pub(crate) const CLOSURE_OFFSET: usize = std::mem::offset_of!(Self, closure);
    pub(crate) const STATE_OFFSET: usize = std::mem::offset_of!(Self, state);
    pub(crate) const NUM_LOCALS_OFFSET: usize = std::mem::offset_of!(Self, num_locals);
    pub(crate) const SCOPE_ID_OFFSET: usize = std::mem::offset_of!(Self, scope_id);
    pub(crate) const LOCALS_OFFSET: usize = std::mem::offset_of!(Self, locals);
}

pub trait ReturnValue {
    fn status(&self) -> Status;
    fn value(&self) -> Value;
}

impl<T> ReturnValue for T
where
    T: Clone + Into<Value>,
{
    fn status(&self) -> Status {
        Status::Normal
    }

    fn value(&self) -> Value {
        self.clone().into()
    }
}

impl<T, E> ReturnValue for Result<T, E>
where
    T: Clone + Into<Value>,
    E: Clone + Into<Value>,
{
    fn status(&self) -> Status {
        if self.is_ok() {
            Status::Normal
        } else {
            Status::Exception
        }
    }

    fn value(&self) -> Value {
        match self {
            Ok(v) => v.clone().into(),
            Err(err) => err.clone().into(),
        }
    }
}

/// Lambda function.
///
/// The actual type of `context` varies depending on usage of the lambda function:
///
/// * Entry function: 0 (null pointer)
/// * Regular functions: Closure*
/// * Coroutine functions: Coroutine*
///
pub type Lambda<X> =
    extern "C" fn(runtime: &mut Runtime<X>, context: &mut CallContext, retv: &mut Value) -> Status;

impl<X> From<LambdaAddr> for Lambda<X> {
    fn from(value: LambdaAddr) -> Self {
        debug_assert_ne!(value.0, 0);
        // SAFETY: `LambdaAddr` contains only an address of a lambda function and it is always
        // convertible to `Lambda`.
        unsafe { std::mem::transmute(value.0) }
    }
}

// See https://www.reddit.com/r/rust/comments/ksfk4j/comment/gifzlhg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

// This function generates a wrapper function for each `host_fn` at compile time.
pub fn into_lambda<F, R, X>(host_fn: F) -> Lambda<X>
where
    F: Fn(&mut Runtime<X>, &[Value]) -> R + 'static,
    R: Clone + ReturnValue,
{
    debug_assert_eq!(std::mem::size_of::<F>(), 0, "Function must have zero size");
    std::mem::forget(host_fn);
    host_fn_wrapper::<F, R, X>
}

extern "C" fn host_fn_wrapper<F, R, X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status
where
    F: Fn(&mut Runtime<X>, &[Value]) -> R + 'static,
    R: Clone + ReturnValue,
{
    // SAFETY: Parent ensured that F is zero sized and we use `ManuallyDrop` to ensure
    // it isn't dropped (even if the callback panics).
    #[allow(clippy::uninit_assumed_init)]
    let host_fn = unsafe { std::mem::MaybeUninit::<F>::uninit().assume_init() };
    let args = context.args();
    // TODO: The return value is copied twice.  That's inefficient.
    let result = host_fn(runtime, args);
    *retv = result.value();
    result.status()
}

#[derive(Debug)]
#[repr(C)]
pub struct CallContext {
    /// The `this` argument.
    this: Value,

    /// A pointer to the call environment.
    ///
    /// The actual type of the value varies depending on the type of the lambda function:
    ///
    /// * Entry functions: 0 (null pointer)
    /// * Regular functions: &mut Closure
    /// * Coroutine functions: &mut Coroutine
    ///
    envp: *mut c_void,

    /// A pointer to the call context of the caller.
    #[allow(unused)]
    caller: *const CallContext,

    /// Flags.
    flags: CallContextFlags,

    /// The depth of the call.
    depth: u16,

    /// The number of the arguments.
    argc: u16,

    /// The maximum number of the arguments.
    argc_max: u16,

    /// A pointer to the arguments.
    argv: *mut Value,
}

impl CallContext {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const ALIGNMENT: usize = std::mem::align_of::<Self>();
    pub const THIS_OFFSET: usize = std::mem::offset_of!(Self, this);
    pub const ENVP_OFFSET: usize = std::mem::offset_of!(Self, envp);
    pub const CALLER_OFFSET: usize = std::mem::offset_of!(Self, caller);
    pub const DEPTH_OFFSET: usize = std::mem::offset_of!(Self, depth);
    pub const ARGC_OFFSET: usize = std::mem::offset_of!(Self, argc);
    pub const ARGC_MAX_OFFSET: usize = std::mem::offset_of!(Self, argc_max);
    pub const ARGV_OFFSET: usize = std::mem::offset_of!(Self, argv);

    pub(crate) fn new_for_entry(args: &mut [Value]) -> Self {
        Self {
            this: Value::Undefined,
            envp: std::ptr::null_mut(),
            caller: std::ptr::null(),
            flags: CallContextFlags::empty(),
            depth: 0,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_mut_ptr(),
        }
    }

    pub(crate) fn new_for_promise(coroutine: *mut Coroutine, args: &mut [Value]) -> Self {
        Self {
            this: Value::Undefined,
            envp: coroutine as *mut std::ffi::c_void,
            caller: std::ptr::null(),
            flags: CallContextFlags::empty(),
            depth: 0,
            argc: args.len() as u16,
            argc_max: args.len() as u16,
            argv: args.as_mut_ptr(),
        }
    }

    pub(crate) fn is_new(&self) -> bool {
        self.flags.contains(CallContextFlags::NEW)
    }

    pub(crate) fn this(&self) -> &Value {
        debug_assert!(self.this.is_valid());
        &self.this
    }

    pub(crate) fn closure(&self) -> &Closure {
        // SAFETY: `envp` is always a non-null pointer to a `Closure`.
        unsafe {
            debug_assert!(!self.envp.is_null());
            debug_assert!(self.envp.is_aligned());
            &*(self.envp as *const Closure)
        }
    }

    pub(crate) fn closure_mut(&mut self) -> &mut Closure {
        // SAFETY: `envp` is always a non-null pointer to a `Closure`.
        unsafe {
            debug_assert!(!self.envp.is_null());
            debug_assert!(self.envp.is_aligned());
            &mut *(self.envp as *mut Closure)
        }
    }

    pub(crate) fn coroutine(&self) -> &Coroutine {
        // SAFETY: `envp` is always a non-null pointer to a `Coroutine`.
        unsafe {
            debug_assert!(!self.envp.is_null());
            debug_assert!(self.envp.is_aligned());
            &*(self.envp as *const Coroutine)
        }
    }

    pub(crate) fn args(&self) -> &[Value] {
        // SAFETY: `argv` is always non-null and a valid pointer to an array of `Value`s.
        unsafe {
            debug_assert!(!self.argv.is_null());
            debug_assert!(self.argv.is_aligned());
            std::slice::from_raw_parts(self.argv as *const Value, self.argc as usize)
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    struct CallContextFlags: u16  {
        const NEW = 1 << 1;
    }
}

/// The return value type of `Lambda` function.
#[derive(Debug)]
#[repr(u32)]
pub enum Status {
    Normal,
    Exception,
    Suspend,
}

static_assertions::const_assert_eq!(size_of::<Status>(), 4);

/// Address of a lambda function.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct LambdaAddr(usize);

impl From<usize> for LambdaAddr {
    fn from(value: usize) -> Self {
        debug_assert_ne!(value, 0);
        Self(value)
    }
}

impl<X> From<Lambda<X>> for LambdaAddr {
    fn from(value: Lambda<X>) -> Self {
        Self(value as usize)
    }
}

impl std::fmt::Debug for LambdaAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.0 as *const ())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Promise(u32);

static_assertions::const_assert_eq!(size_of::<Promise>(), 4);
static_assertions::const_assert_eq!(align_of::<Promise>(), 4);

impl From<u32> for Promise {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Promise> for u32 {
    fn from(value: Promise) -> Self {
        value.0
    }
}
