use std::ffi::c_void;
use std::marker::PhantomPinned;
use std::mem::offset_of;
use std::ptr::addr_eq;

use crate::llvmir::bridge::Lambda;
use crate::llvmir::bridge::Status;
use crate::llvmir::bridge::STATUS_NORMAL;
use crate::llvmir::bridge::STATUS_EXCEPTION;
use crate::llvmir::bridge::STATUS_SUSPEND;
use crate::tasklet::Promise;

// CAUTION: This module contains types used in JIT-generated code.  Please carefully check the
// memory layout of a type you want to change.  It's recommended to use compile-time assertions
// that ensure the memory layout of the type.

/// A data type to hold a JavaScript value.
//
// DO NOT CHANGE THE ORDER OF THE VARIANTS.
// Some operations heavily rely on the order.
#[repr(C, u8)]
#[derive(Clone, PartialEq)]
pub enum Value {
    None = 0,
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    // TODO(issue#237): GcCellRef
    Closure(*mut Closure),
    Promise(Promise),
}

static_assertions::const_assert_eq!(size_of::<Value>(), 16);
static_assertions::const_assert_eq!(align_of::<Value>(), 8);

impl Value {
    pub fn into_result(self, status: Status) -> Result<Value, Value> {
        match status {
            STATUS_NORMAL => Ok(self),
            STATUS_EXCEPTION => Err(self),
            _ => unreachable!(),
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
        Self::Promise(value.into())
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Undefined => write!(f, "undefined"),
            Self::Null => write!(f, "null"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::Closure(value) => write!(f, "{:?}", &*value),
            Self::Promise(value) => write!(f, "{value:?}"),
        }
    }
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
        let lambda = self.lambda.unwrap();
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
    pub target: *mut Value,
    pub escaped: Value,
    _pinned: PhantomPinned,
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
    pub locals: [Value; 32],

    // The scratch_buffer starts from &locals[num_locals].
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
            let lambda = (&*(*coroutine).closure).lambda.unwrap();
            let mut args = [promise.into(), result.clone(), error.clone()];
            let mut retv = Value::None;
            let status = lambda(
                runtime,
                coroutine as *mut c_void,
                args.len(),
                args.as_mut_ptr() as *mut Value as *mut crate::llvmir::bridge::Value,
                &mut retv as *mut Value as *mut crate::llvmir::bridge::Value,
            );
            match status {
                STATUS_NORMAL => CoroutineStatus::Done(retv),
                STATUS_EXCEPTION => CoroutineStatus::Error(retv),
                STATUS_SUSPEND => CoroutineStatus::Suspend,
                _ => unreachable!(),
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
        STATUS_NORMAL
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
            STATUS_NORMAL
        } else {
            STATUS_EXCEPTION
        }
    }

    fn value(&self) -> Value {
        match self {
            Ok(v) => v.clone().into(),
            Err(err) => err.clone().into(),
        }
    }
}
