use std::ffi::c_void;
use std::mem::offset_of;
use std::ptr::addr_eq;

use crate::Runtime;
use crate::logger;
use crate::objects::Object;

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
    String(U16String) = Self::KIND_STRING,
    // TODO(issue#237): GcCellRef
    Closure(*mut Closure) = Self::KIND_CLOSURE,
    Promise(Promise) = Self::KIND_PROMISE,
    // TODO(issue#237): GcCellRef
    // TODO: *mut Object
    Object(*mut c_void) = Self::KIND_OBJECT,
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
    pub(crate) const KIND_CLOSURE: u8 = 6;
    pub(crate) const KIND_PROMISE: u8 = 7;
    pub(crate) const KIND_OBJECT: u8 = 8;

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const HOLDER_SIZE: usize = size_of::<u64>();
    pub(crate) const KIND_OFFSET: usize = 0;
    pub(crate) const HOLDER_OFFSET: usize = size_of::<u64>();

    // 7.1.18 ToObject ( argument )
    pub fn to_object(&self) -> Result<&mut Object, Value> {
        match self {
            Self::Undefined | Self::Null => Err(1001.into()), // TODO: TypeError
            Self::Boolean(_value) => unimplemented!("new Boolean(value)"),
            Self::Number(_value) => unimplemented!("new Number(value)"),
            Self::String(_value) => unimplemented!("new String(value)"),
            Self::Closure(_value) => unimplemented!("new Function()"),
            Self::Promise(_value) => unimplemented!("new Promise()"),
            Self::Object(value) => unsafe { Ok(value.cast::<Object>().as_mut().unwrap()) },
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
    pub fn get_typeof(&self) -> &'static U16Chunk {
        use jsparser::symbol::builtin::names;

        const UNDEFINED: U16Chunk = U16Chunk::new_const(names::UNDEFINED);
        const BOOLEAN: U16Chunk = U16Chunk::new_const(names::BOOLEAN);
        const NUMBER: U16Chunk = U16Chunk::new_const(names::NUMBER);
        const STRING: U16Chunk = U16Chunk::new_const(names::STRING);
        const FUNCTION: U16Chunk = U16Chunk::new_const(names::FUNCTION);
        const OBJECT: U16Chunk = U16Chunk::new_const(names::OBJECT);

        match self {
            Self::None => unreachable!(),
            Self::Undefined => &UNDEFINED,
            Self::Null => &OBJECT,
            Self::Boolean(_) => &BOOLEAN,
            Self::Number(_) => &NUMBER,
            Self::String(_) => &STRING,
            Self::Closure(_) => &FUNCTION,
            Self::Object(_) => &OBJECT,
            Self::Promise(_) => &OBJECT,
        }
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
            Self::Closure(value) => write!(f, "{:?}", unsafe { value.as_ref().unwrap() }),
            Self::Promise(value) => write!(f, "{value:?}"),
            Self::Object(value) => write!(f, "object({value:?})"),
        }
    }
}

/// A data type to hold a UTF-16 string.
///
/// A UTF-16 string is represented as a *chain* of UTF-16 code sequences.
///
/// This type is usually allocated on the stack and holds a pointer to a `U16Chunk` that is
/// allocated on the heap or the stack.
// TODO(issue#237): GcCell
#[derive(Clone, Copy, PartialEq)]
pub struct U16String(*const U16Chunk); // Non-null

static_assertions::const_assert_eq!(align_of::<U16String>(), align_of::<usize>());

impl U16String {
    pub const EMPTY: Self = Self(std::ptr::from_ref(&U16Chunk::EMPTY));

    pub const fn new(chunk: &U16Chunk) -> Self {
        Self(std::ptr::from_ref(chunk))
    }

    pub const fn is_empty(&self) -> bool {
        debug_assert!(!self.0.is_null());
        unsafe { (*self.0).is_empty() }
    }

    pub fn len(&self) -> u32 {
        debug_assert!(!self.0.is_null());
        unsafe { (*self.0).total_len() }
    }

    pub(crate) fn first_chunk(&self) -> &U16Chunk {
        debug_assert!(!self.0.is_null());
        unsafe { &(*self.0) }
    }

    pub(crate) fn make_utf16(&self) -> Vec<u16> {
        debug_assert!(!self.0.is_null());
        unsafe { (*self.0).make_utf16() }
    }
}

impl std::fmt::Debug for U16String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "U16String()")
        } else {
            unsafe { write!(f, "U16String({:?})", self.0.as_ref().unwrap()) }
        }
    }
}

impl std::fmt::Display for U16String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            Ok(())
        } else {
            unsafe { write!(f, "{}", self.0.as_ref().unwrap()) }
        }
    }
}

/// A data type representing a sequence of UTF-16 code units.
///
/// This type may be allocated on the stack.
// TODO(issue#237): GcCell
#[derive(Clone, Debug)]
#[repr(C)]
pub struct U16Chunk {
    /// A pointer to the next sequence if it exists.
    pub(crate) next: *const U16Chunk,

    /// A pointer to the array of UTF-16 code units if it exists.
    pub(crate) ptr: *const u16,

    /// The number of the UTF-16 code units.
    pub(crate) len: u32,

    pub(crate) kind: U16ChunkKind,
}

static_assertions::const_assert_eq!(align_of::<U16Chunk>(), align_of::<usize>());

impl U16Chunk {
    pub const EMPTY: Self = Self::new_const_from_raw_parts(std::ptr::null(), 0);

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const NEXT_OFFSET: usize = std::mem::offset_of!(Self, next);
    pub(crate) const PTR_OFFSET: usize = std::mem::offset_of!(Self, ptr);
    pub(crate) const LEN_OFFSET: usize = std::mem::offset_of!(Self, len);
    pub(crate) const KIND_OFFSET: usize = std::mem::offset_of!(Self, kind);

    pub const fn new_const(slice: &[u16]) -> Self {
        Self::new_const_from_raw_parts(slice.as_ptr(), slice.len() as u32)
    }

    pub const fn new_stack(slice: &[u16]) -> Self {
        Self::new_stack_from_raw_parts(slice.as_ptr(), slice.len() as u32)
    }

    pub const fn new_const_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: U16ChunkKind::Const,
        }
    }

    pub const fn new_stack_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: U16ChunkKind::Stack,
        }
    }

    pub const fn new_heap_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: U16ChunkKind::Heap,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.next.is_null() && self.len == 0
    }

    pub fn on_stack(&self) -> bool {
        matches!(self.kind, U16ChunkKind::Stack)
    }

    pub fn total_len(&self) -> u32 {
        // TODO: next
        self.len
    }

    pub(crate) fn make_utf16(&self) -> Vec<u16> {
        // TODO: next
        if self.is_empty() {
            return vec![];
        }

        debug_assert_ne!(self.len, 0);
        debug_assert_ne!(self.ptr, std::ptr::null());
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) };
        slice.to_vec()
    }
}

// The UTF-16 code units never change.
unsafe impl Send for U16Chunk {}
unsafe impl Sync for U16Chunk {}

impl std::fmt::Display for U16Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let units = unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) };
        let chars: String = char::decode_utf16(units.iter().cloned())
            .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect();
        write!(f, "{}", chars.escape_default())?;
        // TODO: next
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum U16ChunkKind {
    Const = 0,
    Stack,
    Heap,
}

/// A data type to represent a closure.
//
// TODO(issue#237): GcCell
#[repr(C)]
pub struct Closure {
    /// A pointer to a lambda function compiled from a JavaScript function definition.
    pub lambda: Lambda,

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
        let lambda = self.lambda;
        write!(f, "closure({lambda:?}, [")?;
        let len = self.num_captures as usize;
        let data = self.captures.as_ptr();
        let mut captures = unsafe { std::slice::from_raw_parts(data, len).iter() };
        if let Some(capture) = captures.next() {
            write!(f, "{:?}", unsafe { &**capture })?;
            for capture in captures {
                write!(f, ", {:?}", unsafe { &**capture })?;
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

    pub fn resume(
        runtime: *mut c_void,
        coroutine: *mut Coroutine,
        promise: Promise,
        result: &Value,
        error: &Value,
    ) -> CoroutineStatus {
        unsafe {
            let lambda = (*(*coroutine).closure).lambda;
            let mut args = [promise.into(), result.clone(), error.clone()];
            let mut retv = Value::None;
            let status = lambda(
                runtime,
                coroutine as *mut c_void,
                args.len() as u16,
                args.as_mut_ptr(),
                &mut retv as *mut Value,
            );
            match status {
                Status::Normal => CoroutineStatus::Done(retv),
                Status::Exception => CoroutineStatus::Error(retv),
                Status::Suspend => CoroutineStatus::Suspend,
            }
        }
    }
}

/// The return value type of `Coroutine::resume()`.
pub enum CoroutineStatus {
    Done(Value),
    Error(Value),
    Suspend,
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
/// * Regular functions: Capture**
/// * Coroutine functions: Coroutine*
///
pub type Lambda = unsafe extern "C" fn(
    runtime: *mut c_void,
    context: *mut c_void,
    argc: u16,
    argv: *mut Value,
    retv: *mut Value,
) -> Status;

// See https://www.reddit.com/r/rust/comments/ksfk4j/comment/gifzlhg/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

// This function generates a wrapper function for each `host_fn` at compile time.
pub fn into_lambda<F, R, X>(host_fn: F) -> Lambda
where
    F: Fn(&mut Runtime<X>, &[Value]) -> R + 'static,
    R: Clone + ReturnValue,
{
    debug_assert_eq!(std::mem::size_of::<F>(), 0, "Function must have zero size");
    std::mem::forget(host_fn);
    host_fn_wrapper::<F, R, X>
}

unsafe extern "C" fn host_fn_wrapper<F, R, X>(
    runtime: *mut c_void,
    _context: *mut c_void,
    argc: u16,
    argv: *mut Value,
    retv: *mut Value,
) -> Status
where
    F: Fn(&mut Runtime<X>, &[Value]) -> R + 'static,
    R: Clone + ReturnValue,
{
    #[allow(clippy::uninit_assumed_init)]
    let host_fn = unsafe { std::mem::MaybeUninit::<F>::uninit().assume_init() };
    let runtime = unsafe { &mut *(runtime as *mut Runtime<X>) };
    let args = unsafe { std::slice::from_raw_parts(argv as *const Value, argc as usize) };
    // TODO: The return value is copied twice.  That's inefficient.
    let result = host_fn(runtime, args);
    let retv = unsafe { &mut *retv };
    *retv = result.value();
    result.status()
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
