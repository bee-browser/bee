use std::ffi::c_char;
use std::ffi::c_void;

use jsparser::Symbol;

use crate::logger;
use crate::types::Capture;
use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::Lambda;
use crate::types::Value;
use crate::Runtime;

pub fn initialize() {
    unsafe {
        llvmir_initialize();
    }
}

#[repr(C)]
pub struct RuntimeFunctions {
    to_boolean: unsafe extern "C" fn(*mut c_void, *const Value) -> bool,
    to_numeric: unsafe extern "C" fn(*mut c_void, *const Value) -> f64,
    to_int32: unsafe extern "C" fn(*mut c_void, f64) -> i32,
    to_uint32: unsafe extern "C" fn(*mut c_void, f64) -> u32,
    is_loosely_equal: unsafe extern "C" fn(*mut c_void, *const Value, *const Value) -> bool,
    is_strictly_equal: unsafe extern "C" fn(*mut c_void, *const Value, *const Value) -> bool,
    create_capture: unsafe extern "C" fn(*mut c_void, *mut Value) -> *mut Capture,
    create_closure: unsafe extern "C" fn(*mut c_void, Lambda, u16) -> *mut Closure,
    create_coroutine: unsafe extern "C" fn(*mut c_void, *mut Closure, u16, u16) -> *mut Coroutine,
    register_promise: unsafe extern "C" fn(*mut c_void, *mut Coroutine) -> u32,
    await_promise: unsafe extern "C" fn(*mut c_void, u32, u32),
    resume: unsafe extern "C" fn(*mut c_void, u32),
    emit_promise_resolved: unsafe extern "C" fn(*mut c_void, u32, *const Value),
    // TODO(perf): `get()` and `set()` are slow... Compute the address of the value by using a base
    // address and the offset for each property instead of calling these functions.
    get: unsafe extern "C" fn(*mut c_void, u32) -> *const Value,
    set: unsafe extern "C" fn(*mut c_void, u32, *const Value),
    assert: unsafe extern "C" fn(*mut c_void, bool, *const c_char),
    print_u32: unsafe extern "C" fn(*mut c_void, u32, *const c_char),
    print_f64: unsafe extern "C" fn(*mut c_void, f64, *const c_char),
    print_value: unsafe extern "C" fn(*mut c_void, *const Value, *const c_char),
    print_message: unsafe extern "C" fn(*mut c_void, *const c_char),
    launch_debugger: unsafe extern "C" fn(*mut c_void),
}

impl RuntimeFunctions {
    pub fn new<X>() -> Self {
        Self {
            to_boolean: runtime_to_boolean,
            to_numeric: runtime_to_numeric,
            to_int32: runtime_to_int32,
            to_uint32: runtime_to_uint32,
            is_loosely_equal: runtime_is_loosely_equal,
            is_strictly_equal: runtime_is_strictly_equal,
            create_capture: runtime_create_capture::<X>,
            create_closure: runtime_create_closure::<X>,
            create_coroutine: runtime_create_coroutine::<X>,
            register_promise: runtime_register_promise::<X>,
            await_promise: runtime_await_promise::<X>,
            resume: runtime_resume::<X>,
            emit_promise_resolved: runtime_emit_promise_resolved::<X>,
            get: runtime_get::<X>,
            set: runtime_set::<X>,
            assert: runtime_assert,
            print_u32: runtime_print_u32,
            print_f64: runtime_print_f64,
            print_value: runtime_print_value,
            print_message: runtime_print_message,
            launch_debugger: runtime_launch_debugger,
        }
    }
}

macro_rules! into_runtime {
    ($runtime:expr, $extension:ident) => {
        &mut *($runtime as *mut crate::Runtime<$extension>)
    };
}

macro_rules! into_value {
    ($value:expr) => {
        &*($value)
    };
}

// 7.1.2 ToBoolean ( argument )
unsafe extern "C" fn runtime_to_boolean(_runtime: *mut c_void, value: *const Value) -> bool {
    let value = into_value!(value);
    match value {
        Value::None => unreachable!("Value::None"),
        Value::Undefined => false,
        Value::Null => false,
        Value::Boolean(value) => *value,
        Value::Number(value) if *value == 0.0 => false,
        Value::Number(value) if value.is_nan() => false,
        Value::Number(_) => true,
        Value::Closure(_) => true,
        Value::Promise(_) => true,
    }
}

// 7.1.3 ToNumeric ( value )
// 7.1.4 ToNumber ( argument )
unsafe extern "C" fn runtime_to_numeric(_runtime: *mut c_void, value: *const Value) -> f64 {
    let value = into_value!(value);
    match value {
        Value::None => unreachable!("Value::None"),
        Value::Undefined => f64::NAN,
        Value::Null => 0.0,
        Value::Boolean(value) if *value => 1.0,
        Value::Boolean(_) => 0.0,
        Value::Number(value) => *value,
        Value::Closure(_) => f64::NAN,
        Value::Promise(_) => f64::NAN,
    }
}

// 7.1.6 ToInt32 ( argument )
unsafe extern "C" fn runtime_to_int32(_runtime: *mut c_void, value: f64) -> i32 {
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
unsafe extern "C" fn runtime_to_uint32(_runtime: *mut c_void, value: f64) -> u32 {
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

    // 5. Return 𝔽(int32bit).
    if int32bit < 0.0 {
        (int32bit + EXP2_31) as u32
    } else {
        int32bit as u32
    }
}

// 7.2.13 IsLooselyEqual ( x, y )
unsafe extern "C" fn runtime_is_loosely_equal(
    runtime: *mut c_void,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = into_value!(a);
    debug_assert!(!matches!(x, Value::None));

    let y = into_value!(b);
    debug_assert!(!matches!(y, Value::None));

    let x_kind = std::mem::discriminant(x);
    let y_kind = std::mem::discriminant(y);

    // 1. If Type(x) is Type(y)
    if x_kind == y_kind {
        // a. Return IsStrictlyEqual(x, y).
        return runtime_is_strictly_equal(runtime, a, b);
    }

    match (x, y) {
        // 2. If x is null and y is undefined, return true.
        (Value::Null, Value::Undefined) => true,
        // 3. If x is undefined and y is null, return true.
        (Value::Undefined, Value::Null) => true,
        // TODO: 4. NOTE: This step is replaced in section B.3.6.2.
        // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 7. If x is a BigInt and y is a String, then
        // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
        // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
        // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
        // ...
        _ => {
            let xnum = runtime_to_numeric(runtime, a);
            let ynum = runtime_to_numeric(runtime, b);
            if xnum.is_nan() || ynum.is_nan() {
                return false;
            }
            xnum == ynum
        }
    }
}

// 7.2.14 IsStrictlyEqual ( x, y )
unsafe extern "C" fn runtime_is_strictly_equal(
    _runtime: *mut c_void,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = into_value!(a);
    debug_assert!(!matches!(x, Value::None));

    let y = into_value!(b);
    debug_assert!(!matches!(y, Value::None));

    x == y
}

unsafe extern "C" fn runtime_create_capture<X>(
    runtime: *mut c_void,
    target: *mut Value,
) -> *mut Capture {
    const LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<Capture>(),
            std::mem::align_of::<Capture>(),
        )
    };

    let runtime = into_runtime!(runtime, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(LAYOUT);

    let capture = ptr.cast::<Capture>().as_ptr();
    (*capture).target = target;

    // `capture.escaped` will be filled with an actual value.

    capture
}

impl<X> Runtime<X> {
    pub(crate) fn create_closure(&mut self, lambda: Lambda, num_captures: u16) -> *mut Closure {
        const BASE_LAYOUT: std::alloc::Layout = unsafe {
            std::alloc::Layout::from_size_align_unchecked(
                std::mem::offset_of!(Closure, captures),
                std::mem::align_of::<Closure>(),
            )
        };

        let storage_layout =
            std::alloc::Layout::array::<*mut Capture>(num_captures as usize).unwrap();
        let (layout, _) = BASE_LAYOUT.extend(storage_layout).unwrap();

        let allocator = self.allocator();

        // TODO: GC
        let ptr = allocator.alloc_layout(layout);

        let closure = unsafe { ptr.cast::<Closure>().as_mut() };
        closure.lambda = lambda;
        closure.num_captures = num_captures;
        // `closure.captures[]` will be filled with actual pointers to `Captures`.

        closure as *mut Closure
    }
}

unsafe extern "C" fn runtime_create_closure<X>(
    runtime: *mut c_void,
    lambda: Lambda,
    num_captures: u16,
) -> *mut Closure {
    into_runtime!(runtime, X).create_closure(lambda, num_captures)
}

unsafe extern "C" fn runtime_create_coroutine<X>(
    runtime: *mut c_void,
    closure: *mut Closure,
    num_locals: u16,
    scratch_buffer_len: u16,
) -> *mut Coroutine {
    const BASE_LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::offset_of!(Coroutine, locals),
            std::mem::align_of::<Coroutine>(),
        )
    };

    // num_locals may be 0.
    let locals_layout = std::alloc::Layout::array::<Value>(num_locals as usize).unwrap();
    let (layout, _) = BASE_LAYOUT.extend(locals_layout).unwrap();

    // scratch_buffer_len may be 0.
    debug_assert_eq!(scratch_buffer_len as usize % size_of::<u64>(), 0);
    let n = scratch_buffer_len as usize / size_of::<u64>();
    let scratch_buffer_layout = std::alloc::Layout::array::<u64>(n).unwrap();
    let (layout, _) = layout.extend(scratch_buffer_layout).unwrap();

    let runtime = into_runtime!(runtime, X);
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(layout);

    let coroutine = ptr.cast::<Coroutine>().as_ptr();
    (*coroutine).closure = closure;
    (*coroutine).state = 0;
    (*coroutine).num_locals = num_locals;
    (*coroutine).scope_id = 0;
    (*coroutine).scratch_buffer_len = scratch_buffer_len;
    // `(*coroutine).locals[]` will be initialized in the coroutine.

    coroutine
}

unsafe extern "C" fn runtime_register_promise<X>(
    runtime: *mut c_void,
    coroutine: *mut Coroutine,
) -> u32 {
    let runtime = into_runtime!(runtime, X);
    runtime.register_promise(coroutine).into()
}

unsafe extern "C" fn runtime_resume<X>(runtime: *mut c_void, promise: u32) {
    let runtime = into_runtime!(runtime, X);
    runtime.process_promise(promise.into(), &Value::None, &Value::None);
}

unsafe extern "C" fn runtime_await_promise<X>(runtime: *mut c_void, promise: u32, awaiting: u32) {
    let runtime = into_runtime!(runtime, X);
    runtime.await_promise(promise.into(), awaiting.into());
}

unsafe extern "C" fn runtime_emit_promise_resolved<X>(
    runtime: *mut c_void,
    promise: u32,
    result: *const Value,
) {
    let runtime = into_runtime!(runtime, X);
    let cloned = into_value!(result).clone();
    runtime.emit_promise_resolved(promise.into(), cloned);
}

unsafe extern "C" fn runtime_get<X>(runtime: *mut c_void, symbol: u32) -> *const Value {
    debug_assert_ne!(symbol, 0);
    let runtime = into_runtime!(runtime, X);
    let symbol = Symbol::from(symbol);
    match runtime.global_object().get(symbol) {
        Some(value) => value,
        None => std::ptr::null(),
    }
}

unsafe extern "C" fn runtime_set<X>(runtime: *mut c_void, symbol: u32, value: *const Value) {
    debug_assert_ne!(symbol, 0);
    let runtime = into_runtime!(runtime, X);
    let symbol = Symbol::from(symbol);
    let value = value.as_ref().unwrap();
    runtime.global_object_mut().set(symbol, value);
}

unsafe extern "C" fn runtime_assert(
    _runtime: *mut c_void,
    assertion: bool,
    msg: *const std::os::raw::c_char,
) {
    if !assertion {
        let msg = std::ffi::CStr::from_ptr(msg);
        panic!("runtime_assert: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_u32(
    _runtime: *mut c_void,
    value: u32,
    msg: *const std::os::raw::c_char,
) {
    let msg = std::ffi::CStr::from_ptr(msg);
    if msg.is_empty() {
        logger::debug!("runtime_print_u32: {value:08X}");
    } else {
        logger::debug!("runtime_print_u32: {value:08X}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_f64(
    _runtime: *mut c_void,
    value: f64,
    msg: *const std::os::raw::c_char,
) {
    let msg = std::ffi::CStr::from_ptr(msg);
    if msg.is_empty() {
        logger::debug!("runtime_print_f64: {value}");
    } else {
        logger::debug!("runtime_print_f64: {value}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_value(
    _runtime: *mut c_void,
    value: *const Value,
    msg: *const std::os::raw::c_char,
) {
    let value = into_value!(value);
    let msg = std::ffi::CStr::from_ptr(msg);
    if msg.is_empty() {
        logger::debug!("runtime_print_value: {value:?}");
    } else {
        logger::debug!("runtime_print_value: {value:?}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_message(
    _runtime: *mut c_void,
    msg: *const std::os::raw::c_char,
) {
    let msg = std::ffi::CStr::from_ptr(msg);
    logger::debug!("runtime_print_value: {msg:?}");
}

unsafe extern "C" fn runtime_launch_debugger(_runtime: *mut c_void) {
    logger::debug!("runtime_launch_debugger");
    // TODO(feat): Support debuggers such as Chrome DevTools.
}

#[link(name = "llvmir")]
extern "C" {
    fn llvmir_initialize();
}
