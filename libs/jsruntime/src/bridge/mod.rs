#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

macro_rules! into_runtime {
    ($context:expr, $extension:ident) => {
        &mut *($context as *mut crate::Runtime<$extension>)
    };
}

impl Locator {
    pub(crate) const NONE: Self = Self::new(LocatorKind_None, 0);

    const MAX_INDEX: usize = u16::MAX as usize;

    pub(crate) fn checked_argument(index: usize) -> Option<Self> {
        Self::checked_new(LocatorKind_Argument, index)
    }

    pub(crate) fn checked_local(index: usize) -> Option<Self> {
        Self::checked_new(LocatorKind_Local, index)
    }

    pub(crate) fn checked_capture(index: usize) -> Option<Self> {
        Self::checked_new(LocatorKind_Capture, index)
    }

    pub(crate) const fn argument(index: u16) -> Self {
        Self::new(LocatorKind_Argument, index)
    }

    pub(crate) const fn local(index: u16) -> Self {
        Self::new(LocatorKind_Local, index)
    }

    const fn new(kind: LocatorKind, index: u16) -> Self {
        Self { kind, index }
    }

    fn checked_new(kind: LocatorKind, index: usize) -> Option<Self> {
        if index > Self::MAX_INDEX {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(kind, index as u16))
    }
}

impl std::fmt::Debug for Locator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = self.index;
        match self.kind {
            LocatorKind_None => write!(f, "Locator::None"),
            LocatorKind_Argument => write!(f, "Locator::Argument({index})"),
            LocatorKind_Local => write!(f, "Locator::Local({index})"),
            LocatorKind_Capture => write!(f, "Locator::Capture({index})"),
            _ => unreachable!(),
        }
    }
}

impl Value {
    pub const UNDEFINED: Self = Self {
        kind: ValueKind_Undefined,
        holder: ValueHolder { opaque: 0 },
    };

    pub const NULL: Self = Self {
        kind: ValueKind_Null,
        holder: ValueHolder { opaque: 0 },
    };

    pub const TRUE: Self = Self::boolean(true);
    pub const FALSE: Self = Self::boolean(false);

    pub const fn boolean(boolean: bool) -> Self {
        Self {
            kind: ValueKind_Boolean,
            holder: ValueHolder { boolean },
        }
    }

    pub const fn number(number: f64) -> Self {
        Self {
            kind: ValueKind_Number,
            holder: ValueHolder { number },
        }
    }

    pub fn into_result(self, status: Status) -> Result<Value, Value> {
        match status {
            Status_Normal => Ok(self),
            Status_Exception => Err(self),
            _ => unreachable!(),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::UNDEFINED
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::number(value)
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

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `unsafe` is needed for accessing the `holder` field.
        unsafe {
            match self.kind {
                ValueKind_Undefined => write!(f, "undefined"),
                ValueKind_Null => write!(f, "null"),
                ValueKind_Boolean if self.holder.boolean => write!(f, "true"),
                ValueKind_Boolean => write!(f, "false"),
                ValueKind_Number => write!(f, "{}", self.holder.number),
                ValueKind_Closure => {
                    let lambda = (*self.holder.closure).lambda.unwrap();
                    write!(f, "closure({lambda:?}, [")?;
                    let len = (*self.holder.closure).num_captures as usize;
                    let data = (*self.holder.closure).captures;
                    let mut captures = std::slice::from_raw_parts_mut(data, len)
                        .iter()
                        .map(|capture| capture.as_ref().unwrap());
                    if let Some(capture) = captures.next() {
                        write!(f, "{capture:?}")?;
                        for capture in captures {
                            write!(f, ", {capture:?}")?;
                        }
                    }
                    write!(f, "])")
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Capture {
    fn is_escaped(&self) -> bool {
        self.target as *const Variable == &self.escaped
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

pub trait ReturnValue {
    fn status(&self) -> Status;
    fn value(&self) -> Value;
}

impl<T> ReturnValue for T
where
    T: Clone + Into<Value>,
{
    fn status(&self) -> Status {
        Status_Normal
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
            Status_Normal
        } else {
            Status_Exception
        }
    }

    fn value(&self) -> Value {
        match self {
            Ok(v) => v.clone().into(),
            Err(err) => err.clone().into(),
        }
    }
}

pub fn runtime_bridge<X>() -> Runtime {
    Runtime {
        to_boolean: Some(runtime_to_boolean),
        to_numeric: Some(runtime_to_numeric),
        to_int32: Some(runtime_to_int32),
        to_uint32: Some(runtime_to_uint32),
        is_loosely_equal: Some(runtime_is_loosely_equal),
        is_strictly_equal: Some(runtime_is_strictly_equal),
        create_capture: Some(runtime_create_capture::<X>),
        create_closure: Some(runtime_create_closure::<X>),
        assert: Some(runtime_assert),
    }
}

// 7.1.2 ToBoolean ( argument )
unsafe extern "C" fn runtime_to_boolean(_: usize, value: *const Value) -> bool {
    let value = &*value;
    match value.kind {
        ValueKind_Undefined => false,
        ValueKind_Null => false,
        ValueKind_Boolean => value.holder.boolean,
        ValueKind_Number if value.holder.number == 0.0 => false,
        ValueKind_Number if value.holder.number.is_nan() => false,
        ValueKind_Number => true,
        ValueKind_Closure => true,
        _ => unreachable!(),
    }
}

// 7.1.3 ToNumeric ( value )
// 7.1.4 ToNumber ( argument )
unsafe extern "C" fn runtime_to_numeric(_: usize, value: *const Value) -> f64 {
    let value = &*value;
    match value.kind {
        ValueKind_Undefined => f64::NAN,
        ValueKind_Null => 0.0,
        ValueKind_Boolean if value.holder.boolean => 1.0,
        ValueKind_Boolean => 0.0,
        ValueKind_Number => value.holder.number,
        ValueKind_Closure => f64::NAN,
        _ => unreachable!(),
    }
}

// 7.1.6 ToInt32 ( argument )
unsafe extern "C" fn runtime_to_int32(_: usize, value: f64) -> i32 {
    const EXP2_31: f64 = (2u64 << 31) as f64;
    const EXP2_32: f64 = (2u64 << 32) as f64;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !value.is_finite() || value == 0.0 {
        return 0;
    }

    // 3. Let int be truncate(ℝ(number)).
    let int_ = value.trunc();

    // 4. Let int32bit be int modulo 2**32.
    let int32bit = int_ % EXP2_32;
    // int32bit may be negative.

    // 5. If int32bit ≥ 2**31, return 𝔽(int32bit - 2**32); otherwise return 𝔽(int32bit).
    if int32bit >= EXP2_31 {
        (int32bit - EXP2_32) as i32
    } else {
        int32bit as i32
    }
}

// 7.1.7 ToUint32 ( argument )
unsafe extern "C" fn runtime_to_uint32(_: usize, value: f64) -> u32 {
    const EXP2_31: f64 = (2u64 << 31) as f64;
    const EXP2_32: f64 = (2u64 << 32) as f64;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !value.is_finite() || value == 0.0 {
        return 0;
    }

    // 3. Let int be truncate(ℝ(number)).
    let int_ = dbg!(value.trunc());

    // 4. Let int32bit be int modulo 2**32.
    let int32bit = dbg!(int_ % EXP2_32);
    // int32bit may be negative.

    // 5. Return 𝔽(int32bit).
    if int32bit < 0.0 {
        dbg!((int32bit + EXP2_31) as u32)
    } else {
        dbg!(int32bit as u32)
    }
}

// 7.2.13 IsLooselyEqual ( x, y )
unsafe extern "C" fn runtime_is_loosely_equal(
    runtime: usize,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = &*a;
    let y = &*b;
    // 1. If Type(x) is Type(y)
    if x.kind == y.kind {
        // a. Return IsStrictlyEqual(x, y).
        return runtime_is_strictly_equal(runtime, a, b);
    }
    // 2. If x is null and y is undefined, return true.
    if x.kind == ValueKind_Null && y.kind == ValueKind_Undefined {
        return true;
    }
    // 3. If x is undefined and y is null, return true.
    if x.kind == ValueKind_Undefined && y.kind == ValueKind_Null {
        return true;
    }
    // TODO: 4. NOTE: This step is replaced in section B.3.6.2.
    // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
    // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
    // TODO: 7. If x is a BigInt and y is a String, then
    // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
    // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
    // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
    // ...
    let xnum = runtime_to_numeric(runtime, x);
    let ynum = runtime_to_numeric(runtime, y);
    if xnum.is_nan() || ynum.is_nan() {
        return false;
    }
    xnum == ynum
}

// 7.2.14 IsStrictlyEqual ( x, y )
unsafe extern "C" fn runtime_is_strictly_equal(_: usize, a: *const Value, b: *const Value) -> bool {
    let x = &*a;
    let y = &*b;
    if x.kind != y.kind {
        return false;
    }
    match x.kind {
        ValueKind_Undefined => true,
        ValueKind_Null => true,
        ValueKind_Boolean => x.holder.boolean == y.holder.boolean,
        ValueKind_Number => x.holder.number == y.holder.number,
        ValueKind_Closure => x.holder.closure == y.holder.closure,
        _ => unreachable!(),
    }
}

unsafe extern "C" fn runtime_create_capture<X>(context: usize, target: *mut Variable) -> *mut Capture {
    const LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<Capture>(),
            std::mem::align_of::<Capture>(),
        )
    };

    let runtime = into_runtime!(context, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(LAYOUT);

    let capture = ptr.cast::<Capture>().as_ptr();
    (*capture).target = target;

    // `capture.escaped` will be filled with an actual value.

    capture
}

unsafe extern "C" fn runtime_create_closure<X>(
    context: usize,
    lambda: Lambda,
    num_captures: u16,
) -> *mut Closure {
    const BASE_LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<Closure>(),
            std::mem::align_of::<Closure>(),
        )
    };

    let storage_layout = std::alloc::Layout::array::<*mut Capture>(num_captures as usize).unwrap();
    let (layout, offset) = BASE_LAYOUT.extend(storage_layout).unwrap();

    let runtime = into_runtime!(context, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(layout);

    let closure = ptr.cast::<Closure>().as_ptr();
    (*closure).lambda = lambda;
    (*closure).num_captures = num_captures;
    if num_captures == 0 {
        (*closure).captures = std::ptr::null_mut();
    } else {
        (*closure).captures = ptr.as_ptr().wrapping_add(offset).cast::<*mut Capture>();
    }

    // `closure.storage[]` will be filled with actual pointers to `Captures`.

    closure
}

unsafe extern "C" fn runtime_assert(
    _context: usize,
    assertion: bool,
    msg: *const std::os::raw::c_char,
) {
    if !assertion {
        panic!("{:?}", std::ffi::CStr::from_ptr(msg));
    }
}
