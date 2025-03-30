use std::ffi::c_void;
use std::mem::offset_of;
use std::ptr::addr_eq;

use crate::Runtime;
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
    None = 0,
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(U16String),
    // TODO(issue#237): GcCellRef
    Closure(*mut Closure),
    Promise(Promise),
    // TODO(issue#237): GcCellRef
    // TODO: *mut Object
    Object(*mut c_void),
}

static_assertions::const_assert_eq!(size_of::<Value>(), 16);
static_assertions::const_assert_eq!(align_of::<Value>(), 8);

impl Value {
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
        match status {
            Status::Normal => Ok(self),
            Status::Exception => Err(self),
            _ => unreachable!(),
        }
    }

    // 13.5.3.1 Runtime Semantics: Evaluation
    pub fn get_typeof(&self) -> &'static Char16Seq {
        use jsparser::symbol::builtin::names;

        const UNDEFINED: Char16Seq = Char16Seq::new_const(names::UNDEFINED);
        const BOOLEAN: Char16Seq = Char16Seq::new_const(names::BOOLEAN);
        const NUMBER: Char16Seq = Char16Seq::new_const(names::NUMBER);
        const STRING: Char16Seq = Char16Seq::new_const(names::STRING);
        const FUNCTION: Char16Seq = Char16Seq::new_const(names::FUNCTION);
        const OBJECT: Char16Seq = Char16Seq::new_const(names::OBJECT);

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
/// This type is usually allocated on the stack and holds a pointer to a `Char16Seq` that is
/// allocated on the heap or the stack.
// TODO(issue#237): GcCell
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct U16String(*const Char16Seq); // Non-null

static_assertions::const_assert_eq!(align_of::<U16String>(), align_of::<usize>());

impl U16String {
    // TODO(refactor): cbindgen cannot parse the following line...
    // Currently, we don't use union types.  So, now may be the time to switch to cxx.
    //pub const EMPTY: Self = Self(std::ptr::from_ref(&Char16Seq::EMPTY));

    pub fn new(seq: &Char16Seq) -> Self {
        Self(std::ptr::from_ref(seq))
    }

    pub const fn is_empty(&self) -> bool {
        debug_assert!(!self.0.is_null());
        unsafe { (*self.0).is_empty() }
    }

    pub fn len(&self) -> u32 {
        debug_assert!(!self.0.is_null());
        unsafe { (*self.0).total_len() }
    }

    pub(crate) fn first_seq(&self) -> &Char16Seq {
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
pub struct Char16Seq {
    /// A pointer to the next sequence if it exists.
    pub(crate) next: *const Char16Seq,

    /// A pointer to the array of UTF-16 code units if it exists.
    pub(crate) ptr: *const u16,

    /// The number of the UTF-16 code units.
    pub(crate) len: u32,

    pub(crate) kind: Char16SeqKind,
}

static_assertions::const_assert_eq!(align_of::<Char16Seq>(), align_of::<usize>());

impl Char16Seq {
    pub const EMPTY: Self = Self::new_const_from_raw_parts(std::ptr::null(), 0);

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
            kind: Char16SeqKind::Const,
        }
    }

    pub const fn new_stack_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: Char16SeqKind::Stack,
        }
    }

    pub const fn new_heap_from_raw_parts(ptr: *const u16, len: u32) -> Self {
        Self {
            next: std::ptr::null(),
            ptr,
            len,
            kind: Char16SeqKind::Heap,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.next.is_null() && self.len == 0
    }

    pub fn on_stack(&self) -> bool {
        matches!(self.kind, Char16SeqKind::Stack)
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
unsafe impl Send for Char16Seq {}
unsafe impl Sync for Char16Seq {}

impl std::fmt::Display for Char16Seq {
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
pub enum Char16SeqKind {
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
    pub captures: [Capture; 32],
}

static_assertions::const_assert_eq!(align_of::<Closure>(), 8);

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lambda = self.lambda;
        write!(f, "closure({lambda:?}, [")?;
        let len = self.num_captures as usize;
        let data = self.captures.as_ptr();
        let mut captures = unsafe { std::slice::from_raw_parts(data, len).iter() };
        if let Some(capture) = captures.next() {
            write!(f, "{capture:?}")?;
            for capture in captures {
                write!(f, ", {capture:?}")?;
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
    fn is_escaped(&self) -> bool {
        debug_assert!(!self.target.is_null());
        addr_eq(self.target, &self.escaped)
    }
}

impl std::fmt::Debug for Capture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_escaped() {
            write!(f, "capture(escaped: {:?})", self.target)
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
