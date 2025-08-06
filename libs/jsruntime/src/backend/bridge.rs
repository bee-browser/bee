use std::ffi::c_char;
use std::ffi::c_void;

use base::utf16;
use base::static_assert_size_eq;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaKind;
use crate::logger;
use crate::objects::Object;
use crate::objects::PropertyKey;
use crate::types::Capture;
use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::Lambda;
use crate::types::Status;
use crate::types::U16Chunk;
use crate::types::U16String;
use crate::types::Value;

#[derive(Clone)]
#[repr(C)]
pub struct RuntimeFunctions {
    pub lazy_compile_normal: unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *mut Value,
        u16,
        *mut Value,
        *mut Value,
    ) -> Status,
    pub lazy_compile_ramp: unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *mut Value,
        u16,
        *mut Value,
        *mut Value,
    ) -> Status,
    pub lazy_compile_coroutine: unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *mut Value,
        u16,
        *mut Value,
        *mut Value,
    ) -> Status,
    pub to_boolean: unsafe extern "C" fn(*mut c_void, *const Value) -> bool,
    pub to_numeric: unsafe extern "C" fn(*mut c_void, *const Value) -> f64,
    pub to_string: unsafe extern "C" fn(*mut c_void, *const Value) -> *const U16Chunk,
    pub number_to_string: unsafe extern "C" fn(*mut c_void, f64) -> *const U16Chunk,
    pub to_object: unsafe extern "C" fn(*mut c_void, *const Value) -> *mut c_void,
    pub to_int32: unsafe extern "C" fn(*mut c_void, f64) -> i32,
    pub to_uint32: unsafe extern "C" fn(*mut c_void, f64) -> u32,
    pub is_same_string: unsafe extern "C" fn(*mut c_void, *const U16Chunk, *const U16Chunk) -> bool,
    pub is_loosely_equal: unsafe extern "C" fn(*mut c_void, *const Value, *const Value) -> bool,
    pub is_strictly_equal: unsafe extern "C" fn(*mut c_void, *const Value, *const Value) -> bool,
    pub get_typeof: unsafe extern "C" fn(*mut c_void, *const Value) -> *const U16Chunk,
    pub migrate_string_to_heap:
        unsafe extern "C" fn(*mut c_void, *const U16Chunk) -> *const U16Chunk,
    pub create_capture: unsafe extern "C" fn(*mut c_void, *mut Value) -> *mut Capture,
    pub create_closure: unsafe extern "C" fn(*mut c_void, Lambda, u32, u16) -> *mut Closure,
    pub create_coroutine:
        unsafe extern "C" fn(*mut c_void, *mut Closure, u16, u16) -> *mut Coroutine,
    pub register_promise: unsafe extern "C" fn(*mut c_void, *mut Coroutine) -> u32,
    pub await_promise: unsafe extern "C" fn(*mut c_void, u32, u32),
    pub resume: unsafe extern "C" fn(*mut c_void, u32),
    pub emit_promise_resolved: unsafe extern "C" fn(*mut c_void, u32, *const Value),
    pub create_object: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void,
    // TODO(perf): `get_value()` and `set_value()` are slow... Compute the address of the value by
    // using a base address and the offset for each property instead of calling these functions.
    pub get_value_by_symbol:
        unsafe extern "C" fn(*mut c_void, *mut c_void, u32, bool) -> *const Value,
    pub get_value_by_number:
        unsafe extern "C" fn(*mut c_void, *mut c_void, f64, bool) -> *const Value,
    pub get_value_by_value:
        unsafe extern "C" fn(*mut c_void, *mut c_void, *const Value, bool) -> *const Value,
    pub set_value_by_symbol: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *const Value),
    pub set_value_by_number: unsafe extern "C" fn(*mut c_void, *mut c_void, f64, *const Value),
    pub set_value_by_value:
        unsafe extern "C" fn(*mut c_void, *mut c_void, *const Value, *const Value),
    pub concat_strings:
        unsafe extern "C" fn(*mut c_void, *const U16Chunk, *const U16Chunk) -> *const U16Chunk,
    pub create_data_property_by_symbol:
        unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *const Value, *mut Value) -> Status,
    pub create_data_property_by_number:
        unsafe extern "C" fn(*mut c_void, *mut c_void, f64, *const Value, *mut Value) -> Status,
    pub create_data_property_by_value: unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *const Value,
        *const Value,
        *mut Value,
    ) -> Status,
    pub copy_data_properties:
        unsafe extern "C" fn(*mut c_void, *mut c_void, *const Value, *mut Value) -> Status,
    pub push_value:
        unsafe extern "C" fn(*mut c_void, *mut c_void, *const Value, *mut Value) -> Status,
    pub assert: unsafe extern "C" fn(*mut c_void, bool, *const c_char),
    pub print_bool: unsafe extern "C" fn(*mut c_void, bool, *const c_char),
    pub print_u32: unsafe extern "C" fn(*mut c_void, u32, *const c_char),
    pub print_f64: unsafe extern "C" fn(*mut c_void, f64, *const c_char),
    pub print_string: unsafe extern "C" fn(*mut c_void, *const U16Chunk, *const c_char),
    pub print_value: unsafe extern "C" fn(*mut c_void, *const Value, *const c_char),
    pub print_capture: unsafe extern "C" fn(*mut c_void, *const Capture, *const c_char),
    pub print_message: unsafe extern "C" fn(*mut c_void, *const c_char),
    pub launch_debugger: unsafe extern "C" fn(*mut c_void),
}

impl RuntimeFunctions {
    pub fn new<X>() -> Self {
        Self {
            lazy_compile_normal: runtime_lazy_compile_normal::<X>,
            lazy_compile_ramp: runtime_lazy_compile_ramp::<X>,
            lazy_compile_coroutine: runtime_lazy_compile_coroutine::<X>,
            to_boolean: runtime_to_boolean,
            to_numeric: runtime_to_numeric,
            to_string: runtime_to_string::<X>,
            number_to_string: runtime_number_to_string::<X>,
            to_object: runtime_to_object::<X>,
            to_int32: runtime_to_int32,
            to_uint32: runtime_to_uint32,
            is_same_string: runtime_is_same_string,
            is_loosely_equal: runtime_is_loosely_equal,
            is_strictly_equal: runtime_is_strictly_equal,
            get_typeof: runtime_get_typeof,
            migrate_string_to_heap: runtime_migrate_string_to_heap::<X>,
            create_capture: runtime_create_capture::<X>,
            create_closure: runtime_create_closure::<X>,
            create_coroutine: runtime_create_coroutine::<X>,
            register_promise: runtime_register_promise::<X>,
            await_promise: runtime_await_promise::<X>,
            resume: runtime_resume::<X>,
            emit_promise_resolved: runtime_emit_promise_resolved::<X>,
            create_object: runtime_create_object::<X>,
            get_value_by_symbol: runtime_get_value_by_symbol::<X>,
            get_value_by_number: runtime_get_value_by_number::<X>,
            get_value_by_value: runtime_get_value_by_value::<X>,
            set_value_by_symbol: runtime_set_value_by_symbol::<X>,
            set_value_by_number: runtime_set_value_by_number::<X>,
            set_value_by_value: runtime_set_value_by_value::<X>,
            concat_strings: runtime_concat_strings::<X>,
            create_data_property_by_symbol: runtime_create_data_property_by_symbol::<X>,
            create_data_property_by_number: runtime_create_data_property_by_number::<X>,
            create_data_property_by_value: runtime_create_data_property_by_value::<X>,
            copy_data_properties: runtime_copy_data_properties::<X>,
            push_value: runtime_push_value::<X>,
            assert: runtime_assert,
            print_bool: runtime_print_bool,
            print_u32: runtime_print_u32,
            print_f64: runtime_print_f64,
            print_string: runtime_print_string,
            print_value: runtime_print_value,
            print_capture: runtime_print_capture,
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

macro_rules! into_closure_mut {
    ($context:expr) => {
        &mut *($context as *mut crate::types::Closure)
    };
}

macro_rules! into_coroutine_mut {
    ($context:expr) => {
        &mut *($context as *mut crate::types::Coroutine)
    };
}

macro_rules! into_string {
    ($value:expr) => {
        &*($value)
    };
}

macro_rules! into_value {
    ($value:expr) => {
        &*($value)
    };
}

macro_rules! into_value_mut {
    ($value:expr) => {
        &mut *($value)
    };
}

macro_rules! into_capture {
    ($capture:expr) => {
        &*($capture)
    };
}

// lazy compilation

unsafe extern "C" fn runtime_lazy_compile_normal<X>(
    runtime: *mut c_void,
    context: *mut c_void,
    this: *mut Value,
    argc: u16,
    argv: *mut Value,
    retv: *mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_normal", ?context);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(context, std::ptr::null_mut());
    let closure = unsafe { into_closure_mut!(context) };

    let lambda_id = closure.lambda_id;
    let lambda = if let Some(lambda) = runtime.executor.get_lambda(lambda_id) {
        lambda
    } else {
        let lambda_info = runtime.lambda_registry.get(lambda_id);
        debug_assert!(matches!(lambda_info.kind, LambdaKind::Normal));
        let program_id = lambda_info.program_id;
        let function_index = lambda_info.function_index as usize;
        super::compile_function(runtime, program_id, function_index, true).unwrap();
        runtime.executor.get_lambda(lambda_id).unwrap()
    };

    debug_assert_eq!(
        closure.lambda as usize,
        runtime_lazy_compile_normal::<X> as usize
    );
    closure.lambda = lambda;

    unsafe { lambda(runtime.as_void_ptr(), context, this, argc, argv, retv) }
}

unsafe extern "C" fn runtime_lazy_compile_ramp<X>(
    runtime: *mut c_void,
    context: *mut c_void,
    this: *mut Value,
    argc: u16,
    argv: *mut Value,
    retv: *mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_ramp", ?context);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(context, std::ptr::null_mut());
    let closure = unsafe { into_closure_mut!(context) };

    let lambda_id = closure.lambda_id;
    let lambda = if let Some(lambda) = runtime.executor.get_lambda(lambda_id) {
        lambda
    } else {
        let lambda_info = runtime.lambda_registry.get(lambda_id);
        debug_assert!(matches!(lambda_info.kind, LambdaKind::Ramp));

        let program_id = lambda_info.program_id;
        let function_index = lambda_info.function_index as usize;

        // Compile the coroutine function before the ramp function in order to compute the scratch
        // buffer size.
        let coroutine_index = runtime.get_index_of_coroutine_function(program_id, function_index);
        super::compile_function(runtime, program_id, coroutine_index, true).unwrap();

        // Then compile the ramp function.
        super::compile_function(runtime, program_id, function_index, true).unwrap();

        // Get the lambda function compiled from the ramp function.
        runtime.executor.get_lambda(lambda_id).unwrap()
    };

    debug_assert_eq!(
        closure.lambda as usize,
        runtime_lazy_compile_ramp::<X> as usize
    );
    closure.lambda = lambda;

    unsafe { lambda(runtime.as_void_ptr(), context, this, argc, argv, retv) }
}

unsafe extern "C" fn runtime_lazy_compile_coroutine<X>(
    runtime: *mut c_void,
    context: *mut c_void,
    this: *mut Value,
    argc: u16,
    argv: *mut Value,
    retv: *mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_coroutine", ?context);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(context, std::ptr::null_mut());
    let coroutine = unsafe { into_coroutine_mut!(context) };

    debug_assert_ne!(coroutine.closure, std::ptr::null_mut());
    let closure = unsafe { &mut *coroutine.closure };

    let lambda_id = closure.lambda_id;
    // The coroutine lambda has already been compiled in `runtime_lazy_compile_ramp()`.
    let lambda = runtime.executor.get_lambda(lambda_id).unwrap();

    debug_assert_eq!(
        closure.lambda as usize,
        runtime_lazy_compile_coroutine::<X> as usize
    );
    closure.lambda = lambda;

    unsafe { lambda(runtime.as_void_ptr(), context, this, argc, argv, retv) }
}

// 7.1.2 ToBoolean ( argument )
unsafe extern "C" fn runtime_to_boolean(_runtime: *mut c_void, value: *const Value) -> bool {
    logger::debug!(event = "runtime_to_boolean", ?value);
    let value = unsafe { into_value!(value) };
    match value {
        Value::None => unreachable!("Value::None"),
        Value::Undefined => false,
        Value::Null => false,
        Value::Boolean(value) => *value,
        Value::Number(0.0) => false,
        Value::Number(value) if value.is_nan() => false,
        Value::Number(_) => true,
        Value::String(value) if value.is_empty() => false,
        Value::String(_) => true,
        Value::Promise(_) => true,
        Value::Object(_) => true,
        Value::Function(_) => true,
    }
}

// 7.1.3 ToNumeric ( value )
// 7.1.4 ToNumber ( argument )
unsafe extern "C" fn runtime_to_numeric(_runtime: *mut c_void, value: *const Value) -> f64 {
    logger::debug!(event = "runtime_to_numeric", ?value);
    let value = unsafe { into_value!(value) };
    match value {
        Value::None => unreachable!("Value::None"),
        Value::Undefined => f64::NAN,
        Value::Null => 0.0,
        Value::Boolean(true) => 1.0,
        Value::Boolean(false) => 0.0,
        Value::Number(value) => *value,
        Value::String(_value) => todo!(),
        Value::Promise(_) => f64::NAN,
        Value::Object(_) | Value::Function(_) => f64::NAN, // TODO(feat): 7.1.1 ToPrimitive()
    }
}

// 7.1.17 ToString ( argument )
unsafe extern "C" fn runtime_to_string<X>(
    runtime: *mut c_void,
    value: *const Value,
) -> *const U16Chunk {
    logger::debug!(event = "runtime_to_string", ?value);
    let runtime = unsafe { into_runtime!(runtime, X) };
    let value = unsafe { into_value!(value) };
    let result = runtime.perform_to_string(value);
    result.first_chunk() as *const U16Chunk
}

impl<X> Runtime<X> {
    pub(crate) fn perform_to_string(&mut self, value: &Value) -> U16String {
        logger::debug!(event = "perform_to_string", ?value);

        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(& "undefined"));
                U16String::new(&CHUNK)
            }
            Value::Null => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(& "null"));
                U16String::new(&CHUNK)
            }
            Value::Boolean(true) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(& "true"));
                U16String::new(&CHUNK)
            }
            Value::Boolean(false) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(& "false"));
                U16String::new(&CHUNK)
            }
            Value::Number(value) => {
                unsafe { self.number_to_string(*value) } // TODO
            }
            Value::String(value) => *value,
            Value::Promise(_) => todo!(),
            Value::Object(_) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(& "[object Object]"));
                U16String::new(&CHUNK)
            }
            Value::Function(_) => todo!(),
        }
    }
}

// 6.1.6.1.20 Number::toString ( x, radix )
unsafe extern "C" fn runtime_number_to_string<X>(
    runtime: *mut c_void,
    value: f64,
) -> *const U16Chunk {
    logger::debug!(event = "runtime_number_to_string", ?value);
    let runtime = unsafe { into_runtime!(runtime, X) };
    unsafe { runtime.number_to_string(value).as_ptr() }
}

impl<X> Runtime<X> {
    unsafe fn number_to_string(&mut self, value: f64) -> U16String {
        // TODO(feat): implment Number::toString()
        let utf16 = self.alloc_utf16(&format!("{value}"));
        let chunk = U16Chunk::new_stack(utf16);
        let string = U16String::new(&chunk);
        unsafe { self.migrate_string_to_heap(string) }
    }
}

// 7.1.18 ToObject ( argument )
unsafe extern "C" fn runtime_to_object<X>(
    runtime: *mut c_void,
    value: *const Value,
) -> *mut c_void {
    logger::debug!(event = "runtime_to_object", ?value);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    runtime.to_object(value)
}

impl<X> Runtime<X> {
    fn to_object(&self, value: &Value) -> *mut c_void {
        logger::debug!(event = "to_object", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined | Value::Null => todo!(),
            Value::Boolean(_value) => todo!(),
            Value::Number(_value) => todo!(),
            Value::String(_value) => todo!(),
            Value::Object(value) | Value::Function(value) => *value,
            Value::Promise(_value) => todo!(),
        }
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

unsafe extern "C" fn runtime_is_same_string(
    _runtime: *mut c_void,
    a: *const U16Chunk,
    b: *const U16Chunk,
) -> bool {
    debug_assert!(!a.is_null());
    debug_assert!(!b.is_null());

    let a = unsafe { into_string!(a) };
    let b = unsafe { into_string!(b) };

    // TODO(perf): slow...
    let a = a.make_utf16();
    let b = b.make_utf16();
    a == b
}

// 7.2.13 IsLooselyEqual ( x, y )
unsafe extern "C" fn runtime_is_loosely_equal(
    runtime: *mut c_void,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = unsafe { into_value!(a) };
    debug_assert!(!matches!(x, Value::None));

    let y = unsafe { into_value!(b) };
    debug_assert!(!matches!(y, Value::None));

    let x_kind = std::mem::discriminant(x);
    let y_kind = std::mem::discriminant(y);

    // 1. If Type(x) is Type(y)
    if x_kind == y_kind {
        // a. Return IsStrictlyEqual(x, y).
        return unsafe { runtime_is_strictly_equal(runtime, a, b) };
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
            let xnum = unsafe { runtime_to_numeric(runtime, a) };
            let ynum = unsafe { runtime_to_numeric(runtime, b) };
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
    let x = unsafe { into_value!(a) };
    debug_assert!(!matches!(x, Value::None));

    let y = unsafe { into_value!(b) };
    debug_assert!(!matches!(y, Value::None));

    x == y
}

unsafe extern "C" fn runtime_get_typeof(
    _runtime: *mut c_void,
    value: *const Value,
) -> *const U16Chunk {
    let value = unsafe { into_value!(value) };
    debug_assert!(!matches!(value, Value::None));

    value.get_typeof() as *const U16Chunk
}

unsafe extern "C" fn runtime_migrate_string_to_heap<X>(
    runtime: *mut c_void,
    string: *const U16Chunk,
) -> *const U16Chunk {
    let runtime = unsafe { into_runtime!(runtime, X) };
    let chunk = unsafe { into_string!(string) };
    unsafe {
        runtime
            .migrate_string_to_heap(U16String::new(chunk))
            .as_ptr()
    }
}

unsafe extern "C" fn runtime_create_capture<X>(
    runtime: *mut c_void,
    target: *mut Value,
) -> *mut Capture {
    logger::debug!(event = "runtime_create_capture", ?target);

    const LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<Capture>(),
            std::mem::align_of::<Capture>(),
        )
    };

    let runtime = unsafe { into_runtime!(runtime, X) };
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(LAYOUT);

    let capture = unsafe { ptr.cast::<Capture>().as_mut() };
    capture.target = target;
    // `capture.escaped` will be filled with an actual value.

    capture as *mut Capture
}

impl<X> Runtime<X> {
    pub(crate) fn create_closure(
        &mut self,
        lambda: Lambda,
        lambda_id: LambdaId,
        num_captures: u16,
    ) -> *mut Closure {
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
        closure.lambda_id = lambda_id;
        closure.num_captures = num_captures;
        // `closure.captures[]` will be filled with actual pointers to `Captures`.

        closure as *mut Closure
    }
}

unsafe extern "C" fn runtime_create_closure<X>(
    runtime: *mut c_void,
    lambda: Lambda,
    lambda_id: u32,
    num_captures: u16,
) -> *mut Closure {
    logger::debug!(
        event = "runtime_create_closure",
        ?lambda,
        lambda_id,
        num_captures
    );
    let runtime = unsafe { into_runtime!(runtime, X) };
    runtime.create_closure(lambda, lambda_id.into(), num_captures)
}

unsafe extern "C" fn runtime_create_coroutine<X>(
    runtime: *mut c_void,
    closure: *mut Closure,
    num_locals: u16,
    scratch_buffer_len: u16,
) -> *mut Coroutine {
    logger::debug!(
        event = "runtime_create_coroutine",
        ?closure,
        num_locals,
        scratch_buffer_len
    );

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

    let runtime = unsafe { into_runtime!(runtime, X) };
    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(layout);

    let coroutine = unsafe { ptr.cast::<Coroutine>().as_mut() };
    coroutine.closure = closure;
    coroutine.state = 0;
    coroutine.num_locals = num_locals;
    coroutine.scope_id = 0;
    coroutine.scratch_buffer_len = scratch_buffer_len;
    // `coroutine.locals[]` will be initialized in the coroutine.

    coroutine as *mut Coroutine
}

unsafe extern "C" fn runtime_register_promise<X>(
    runtime: *mut c_void,
    coroutine: *mut Coroutine,
) -> u32 {
    let runtime = unsafe { into_runtime!(runtime, X) };
    runtime.register_promise(coroutine).into()
}

unsafe extern "C" fn runtime_resume<X>(runtime: *mut c_void, promise: u32) {
    let runtime = unsafe { into_runtime!(runtime, X) };
    runtime.process_promise(promise.into(), &Value::None, &Value::None);
}

unsafe extern "C" fn runtime_await_promise<X>(runtime: *mut c_void, promise: u32, awaiting: u32) {
    let runtime = unsafe { into_runtime!(runtime, X) };
    runtime.await_promise(promise.into(), awaiting.into());
}

unsafe extern "C" fn runtime_emit_promise_resolved<X>(
    runtime: *mut c_void,
    promise: u32,
    result: *const Value,
) {
    let runtime = unsafe { into_runtime!(runtime, X) };
    let result = unsafe { into_value!(result) };
    runtime.emit_promise_resolved(promise.into(), result.clone());
}

unsafe extern "C" fn runtime_create_object<X>(
    runtime: *mut c_void,
    prototype: *mut c_void,
) -> *mut c_void {
    let runtime = unsafe { into_runtime!(runtime, X) };
    runtime.create_object(prototype).as_ptr()
}

unsafe extern "C" fn runtime_get_value_by_symbol<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: u32,
    strict: bool,
) -> *const Value {
    // FIXME: `Value` cannot be defined with `static` because it doesn't implement `Sync`.
    static UNDEFINED: (u8, u64) = (1, 0);
    static_assert_size_eq!((u8, u64), Value);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    // `object` may be null.
    let object = unsafe { object.cast::<Object>().as_ref() };

    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);

    let result = match object {
        Some(object) => object.get_value(&key),
        None => runtime.global_object().get_value(&key),
    };

    match result {
        Some(v) => v as *const Value,
        None if strict => std::ptr::null(),
        None => unsafe { std::mem::transmute::<&(u8, u64), &Value>(&UNDEFINED) as *const Value },
    }
}

unsafe extern "C" fn runtime_get_value_by_number<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: f64,
    strict: bool,
) -> *const Value {
    // FIXME: `Value` cannot be defined with `static` because it doesn't implement `Sync`.
    static UNDEFINED: (u8, u64) = (1, 0);
    static_assert_size_eq!((u8, u64), Value);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    // `object` may be null.
    let object = unsafe { object.cast::<Object>().as_ref() };

    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);

    let result = match object {
        Some(object) => object.get_value(&key),
        None => runtime.global_object().get_value(&key),
    };

    match result {
        Some(v) => v as *const Value,
        None if strict => std::ptr::null(),
        None => unsafe { std::mem::transmute::<&(u8, u64), &Value>(&UNDEFINED) as *const Value },
    }
}

unsafe extern "C" fn runtime_get_value_by_value<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: *const Value,
    strict: bool,
) -> *const Value {
    // FIXME: `Value` cannot be defined with `static` because it doesn't implement `Sync`.
    static UNDEFINED: (u8, u64) = (1, 0);
    static_assert_size_eq!((u8, u64), Value);

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    // `object` may be null.
    let object = unsafe { object.cast::<Object>().as_ref() };

    debug_assert_ne!(key, std::ptr::null());
    let key = unsafe { into_value!(key) };
    let key = runtime.make_property_key(key);

    let result = match object {
        Some(object) => object.get_value(&key),
        None => runtime.global_object().get_value(&key),
    };

    match result {
        Some(v) => v as *const Value,
        None if strict => std::ptr::null(),
        None => unsafe { std::mem::transmute::<&(u8, u64), &Value>(&UNDEFINED) as *const Value },
    }
}

unsafe extern "C" fn runtime_set_value_by_symbol<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: u32,
    value: *const Value,
) {
    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    // `object` may be null.
    let object = unsafe { object.cast::<Object>().as_mut() };

    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    match object {
        Some(object) => object.set_value(&key, value),
        None => runtime.global_object_mut().set_value(&key, value),
    }
}

unsafe extern "C" fn runtime_set_value_by_number<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: f64,
    value: *const Value,
) {
    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    // `object` may be null.
    let object = unsafe { object.cast::<Object>().as_mut() };

    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    match object {
        Some(object) => object.set_value(&key, value),
        None => runtime.global_object_mut().set_value(&key, value),
    }
}

unsafe extern "C" fn runtime_set_value_by_value<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: *const Value,
    value: *const Value,
) {
    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    // `object` may be null.
    let object = unsafe { object.cast::<Object>().as_mut() };

    debug_assert_ne!(key, std::ptr::null());
    let key = unsafe { into_value!(key) };
    let key = runtime.make_property_key(key);

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    match object {
        Some(object) => object.set_value(&key, value),
        None => runtime.global_object_mut().set_value(&key, value),
    }
}

unsafe extern "C" fn runtime_concat_strings<X>(
    runtime: *mut c_void,
    head: *const U16Chunk,
    tail: *const U16Chunk,
) -> *const U16Chunk {
    debug_assert!(!runtime.is_null());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert!(!tail.is_null());
    debug_assert!(!head.is_null());

    let tail = unsafe { into_string!(tail) };
    if tail.is_empty() {
        return unsafe { runtime.alloc_string_rec(into_string!(head), std::ptr::null()) };
    }

    let tail = if tail.on_stack() {
        unsafe { runtime.alloc_string_rec(tail, std::ptr::null()) }
    } else {
        tail
    } as *const U16Chunk;

    let head = unsafe { into_string!(head) };
    if head.is_empty() {
        return tail;
    }

    unsafe { runtime.alloc_string_rec(head, tail) }
}

// 7.3.5 CreateDataProperty ( O, P, V )
unsafe extern "C" fn runtime_create_data_property_by_symbol<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: u32,
    value: *const Value,
    retv: *mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(object, std::ptr::null_mut());
    let object = unsafe { object.cast::<Object>().as_mut().unwrap() };

    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    debug_assert_ne!(retv, std::ptr::null_mut());
    let retv = unsafe { into_value_mut!(retv) };

    match runtime.create_data_property(object, &key, value) {
        Ok(success) => {
            *retv = success.into();
            Status::Normal
        }
        Err(exception) => {
            *retv = exception;
            Status::Exception
        }
    }
}

// 7.3.5 CreateDataProperty ( O, P, V )
unsafe extern "C" fn runtime_create_data_property_by_number<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: f64,
    value: *const Value,
    retv: *mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(object, std::ptr::null_mut());
    let object = unsafe { object.cast::<Object>().as_mut().unwrap() };

    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    debug_assert_ne!(retv, std::ptr::null_mut());
    let retv = unsafe { into_value_mut!(retv) };

    match runtime.create_data_property(object, &key, value) {
        Ok(success) => {
            *retv = success.into();
            Status::Normal
        }
        Err(exception) => {
            *retv = exception;
            Status::Exception
        }
    }
}

// 7.3.5 CreateDataProperty ( O, P, V )
unsafe extern "C" fn runtime_create_data_property_by_value<X>(
    runtime: *mut c_void,
    object: *mut c_void,
    key: *const Value,
    value: *const Value,
    retv: *mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(object, std::ptr::null_mut());
    let object = unsafe { object.cast::<Object>().as_mut().unwrap() };

    debug_assert_ne!(key, std::ptr::null());
    let key = unsafe { runtime.make_property_key(into_value!(value)) };

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    debug_assert_ne!(retv, std::ptr::null_mut());
    let retv = unsafe { into_value_mut!(retv) };

    match runtime.create_data_property(object, &key, value) {
        Ok(success) => {
            *retv = success.into();
            Status::Normal
        }
        Err(exception) => {
            *retv = exception;
            Status::Exception
        }
    }
}

// 7.3.25 CopyDataProperties ( target, source, excludedItems )
unsafe extern "C" fn runtime_copy_data_properties<X>(
    runtime: *mut c_void,
    target: *mut c_void,
    source: *const Value,
    retv: *mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script
    let runtime = unsafe { into_runtime!(runtime, X) };
    let target = unsafe { target.cast::<Object>().as_mut().unwrap() };
    let source = unsafe { into_value!(source) };
    let retv = unsafe { into_value_mut!(retv) };

    match runtime.copy_data_properties(target, source) {
        Ok(()) => {
            *retv = Value::None;
            Status::Normal
        }
        Err(exception) => {
            *retv = exception;
            Status::Exception
        }
    }
}

unsafe extern "C" fn runtime_push_value<X>(
    runtime: *mut c_void,
    target: *mut c_void,
    value: *const Value,
    retv: *mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    debug_assert_ne!(runtime, std::ptr::null_mut());
    let runtime = unsafe { into_runtime!(runtime, X) };

    debug_assert_ne!(target, std::ptr::null_mut());
    let target = unsafe { target.cast::<Object>().as_mut().unwrap() };

    debug_assert_ne!(value, std::ptr::null());
    let value = unsafe { into_value!(value) };

    debug_assert_ne!(retv, std::ptr::null_mut());
    let retv = unsafe { into_value_mut!(retv) };

    match runtime.push_value(target, value) {
        Ok(()) => {
            *retv = Value::None;
            Status::Normal
        }
        Err(exception) => {
            *retv = exception;
            Status::Exception
        }
    }
}

unsafe extern "C" fn runtime_assert(
    _runtime: *mut c_void,
    assertion: bool,
    msg: *const std::os::raw::c_char,
) {
    if !assertion {
        let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
        panic!("runtime_assert: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_bool(
    _runtime: *mut c_void,
    value: bool,
    msg: *const std::os::raw::c_char,
) {
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
    if msg.is_empty() {
        logger::debug!("runtime_print_bool: {value}");
    } else {
        logger::debug!("runtime_print_bool: {value}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_u32(
    _runtime: *mut c_void,
    value: u32,
    msg: *const std::os::raw::c_char,
) {
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
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
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
    if msg.is_empty() {
        logger::debug!("runtime_print_f64: {value}");
    } else {
        logger::debug!("runtime_print_f64: {value}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_string(
    _runtime: *mut c_void,
    value: *const U16Chunk,
    msg: *const std::os::raw::c_char,
) {
    let value = unsafe { value.as_ref().unwrap() };
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
    if msg.is_empty() {
        logger::debug!("runtime_print_f64: {value:?}");
    } else {
        logger::debug!("runtime_print_f64: {value:?}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_value(
    _runtime: *mut c_void,
    value: *const Value,
    msg: *const std::os::raw::c_char,
) {
    let value = unsafe { into_value!(value) };
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
    if msg.is_empty() {
        logger::debug!("runtime_print_value: {value:?}");
    } else {
        logger::debug!("runtime_print_value: {value:?}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_capture(
    _runtime: *mut c_void,
    capture: *const Capture,
    msg: *const std::os::raw::c_char,
) {
    let capture = unsafe { into_capture!(capture) };
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
    if msg.is_empty() {
        logger::debug!("runtime_print_capture: {capture:?}");
    } else {
        logger::debug!("runtime_print_capture: {capture:?}: {msg:?}");
    }
}

unsafe extern "C" fn runtime_print_message(
    _runtime: *mut c_void,
    msg: *const std::os::raw::c_char,
) {
    let msg = unsafe { std::ffi::CStr::from_ptr(msg) };
    logger::debug!("runtime_print_value: {msg:?}");
}

unsafe extern "C" fn runtime_launch_debugger(_runtime: *mut c_void) {
    logger::debug!("runtime_launch_debugger");
    // TODO(feat): Support debuggers such as Chrome DevTools.
}
