use crate::Runtime;
use crate::types::CallContext;
use crate::types::Value;

/// Lambda function.
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

/// The return value type of `Lambda` function.
#[derive(Debug)]
#[repr(u32)]
pub enum Status {
    Normal,
    Exception,
    Suspend,
}

base::static_assert_eq!(size_of::<Status>(), 4);

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
