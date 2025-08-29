use std::ffi::c_void;

use base::utf16;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::lambda::LambdaKind;
use crate::logger;
use crate::objects::PropertyKey;
use crate::types::CallContext;
use crate::types::Capture;
use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::Lambda;
use crate::types::Status;
use crate::types::U16Chunk;
use crate::types::U16String;
use crate::types::Value;

macro_rules! into_string {
    ($value:expr) => {
        // SAFETY: `value` is always a non-null pointer to a `U16String`.
        unsafe {
            debug_assert!(!$value.is_null());
            debug_assert!($value.is_aligned());
            &*($value)
        }
    };
}

macro_rules! into_object {
    ($value:expr) => {
        // SAFETY: `value` is always a non-null pointer to an `Object`.
        unsafe {
            debug_assert!(!$value.is_null());
            &mut *($value as *mut crate::objects::Object)
        }
    };
}

macro_rules! into_capture {
    ($capture:expr) => {
        // SAFETY: `capture` is always a non-null pointer to a `Capture`.
        unsafe {
            debug_assert!(!$capture.is_null());
            debug_assert!($capture.is_aligned());
            &*($capture)
        }
    };
}

// lazy compilation

pub(crate) extern "C" fn runtime_lazy_compile_normal<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_normal");

    let lambda_id = context.closure().lambda_id;
    let lambda = if let Some(lambda) = runtime.code_registry.get_lambda(lambda_id) {
        lambda
    } else {
        let lambda_info = runtime.lambda_registry.get(lambda_id);
        debug_assert!(matches!(lambda_info.kind, LambdaKind::Normal));
        let program_id = lambda_info.program_id;
        let function_index = lambda_info.function_index as usize;
        super::compile_function(runtime, program_id, function_index, true).unwrap();
        runtime.code_registry.get_lambda(lambda_id).unwrap()
    };

    debug_assert_eq!(
        context.closure().lambda,
        (runtime_lazy_compile_normal::<X> as usize).into()
    );
    context.closure_mut().lambda = lambda.into();

    lambda(runtime, context, retv)
}

pub(crate) extern "C" fn runtime_lazy_compile_ramp<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_ramp");

    let lambda_id = context.closure().lambda_id;
    let lambda = if let Some(lambda) = runtime.code_registry.get_lambda(lambda_id) {
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
        runtime.code_registry.get_lambda(lambda_id).unwrap()
    };

    debug_assert_eq!(
        context.closure().lambda,
        (runtime_lazy_compile_ramp::<X> as usize).into()
    );
    context.closure_mut().lambda = lambda.into();

    lambda(runtime, context, retv)
}

pub(crate) extern "C" fn runtime_lazy_compile_coroutine<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_coroutine");

    let coroutine = context.coroutine();

    // SAFETY: `coroutine.closure` is a non-null pointer to a `Closure`.
    let closure = unsafe {
        debug_assert!(!coroutine.closure.is_null());
        debug_assert!(coroutine.closure.is_aligned());
        &mut *coroutine.closure
    };

    let lambda_id = closure.lambda_id;
    // The coroutine lambda has already been compiled in `runtime_lazy_compile_ramp()`.
    let lambda = runtime.code_registry.get_lambda(lambda_id).unwrap();

    debug_assert_eq!(
        closure.lambda,
        (runtime_lazy_compile_coroutine::<X> as usize).into()
    );
    closure.lambda = lambda.into();

    lambda(runtime, context, retv)
}

// 7.1.2 ToBoolean ( argument )
pub(crate) extern "C" fn runtime_to_boolean<X>(_runtime: &mut Runtime<X>, value: &Value) -> bool {
    logger::debug!(event = "runtime_to_boolean", ?value);
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
pub(crate) extern "C" fn runtime_to_numeric<X>(_runtime: &mut Runtime<X>, value: &Value) -> f64 {
    logger::debug!(event = "runtime_to_numeric", ?value);
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
pub(crate) extern "C" fn runtime_to_string<X>(
    runtime: &mut Runtime<X>,
    value: &Value,
) -> *const U16Chunk {
    logger::debug!(event = "runtime_to_string", ?value);
    let result = runtime.perform_to_string(value);
    result.first_chunk() as *const U16Chunk
}

impl<X> Runtime<X> {
    pub(crate) fn perform_to_string(&mut self, value: &Value) -> U16String {
        logger::debug!(event = "perform_to_string", ?value);

        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"undefined"));
                U16String::new(&CHUNK)
            }
            Value::Null => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"null"));
                U16String::new(&CHUNK)
            }
            Value::Boolean(true) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"true"));
                U16String::new(&CHUNK)
            }
            Value::Boolean(false) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"false"));
                U16String::new(&CHUNK)
            }
            Value::Number(value) => {
                self.number_to_string(*value) // TODO
            }
            Value::String(value) => *value,
            Value::Promise(_) => todo!(),
            Value::Object(_) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"[object Object]"));
                U16String::new(&CHUNK)
            }
            Value::Function(_) => todo!(),
        }
    }
}

// 6.1.6.1.20 Number::toString ( x, radix )
pub(crate) extern "C" fn runtime_number_to_string<X>(
    runtime: &mut Runtime<X>,
    value: f64,
) -> *const U16Chunk {
    logger::debug!(event = "runtime_number_to_string", ?value);
    runtime.number_to_string(value).as_ptr()
}

impl<X> Runtime<X> {
    fn number_to_string(&mut self, value: f64) -> U16String {
        // TODO(feat): implment Number::toString()
        let utf16 = self.alloc_utf16(&format!("{value}"));
        let chunk = U16Chunk::new_stack(utf16);
        let string = U16String::new(&chunk);
        self.migrate_string_to_heap(string)
    }
}

// 7.1.18 ToObject ( argument )
pub(crate) extern "C" fn runtime_to_object<X>(
    runtime: &mut Runtime<X>,
    value: &Value,
) -> *mut c_void {
    logger::debug!(event = "runtime_to_object", ?value);
    runtime.value_to_object(value)
}

impl<X> Runtime<X> {
    fn value_to_object(&mut self, value: &Value) -> *mut c_void {
        logger::debug!(event = "to_object", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined | Value::Null => todo!(),
            Value::Boolean(_value) => todo!(),
            Value::Number(_value) => todo!(),
            Value::String(value) => match self.string_constructor(&[Value::String(*value)], true) {
                Ok(Value::Object(object)) => object,
                Ok(_) => unreachable!(),
                Err(_error) => todo!(),
            },
            Value::Object(value) | Value::Function(value) => *value,
            Value::Promise(_value) => todo!(),
        }
    }
}

// 7.1.6 ToInt32 ( argument )
pub(crate) extern "C" fn runtime_to_int32<X>(_runtime: &mut Runtime<X>, value: f64) -> i32 {
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
pub(crate) extern "C" fn runtime_to_uint32<X>(_runtime: &mut Runtime<X>, value: f64) -> u32 {
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

pub(crate) extern "C" fn runtime_is_same_string<X>(
    _runtime: &mut Runtime<X>,
    a: *const U16Chunk,
    b: *const U16Chunk,
) -> bool {
    let a = into_string!(a);
    let b = into_string!(b);

    // TODO(perf): slow...
    let a = a.make_utf16();
    let b = b.make_utf16();
    a == b
}

// 7.2.13 IsLooselyEqual ( x, y )
pub(crate) extern "C" fn runtime_is_loosely_equal<X>(
    runtime: &mut Runtime<X>,
    x: &Value,
    y: &Value,
) -> bool {
    debug_assert!(!matches!(x, Value::None));
    debug_assert!(!matches!(y, Value::None));

    let x_kind = std::mem::discriminant(x);
    let y_kind = std::mem::discriminant(y);

    // 1. If Type(x) is Type(y)
    if x_kind == y_kind {
        // a. Return IsStrictlyEqual(x, y).
        return runtime_is_strictly_equal(runtime, x, y);
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
            let xnum = runtime_to_numeric(runtime, x);
            let ynum = runtime_to_numeric(runtime, y);
            if xnum.is_nan() || ynum.is_nan() {
                return false;
            }
            xnum == ynum
        }
    }
}

// 7.2.14 IsStrictlyEqual ( x, y )
pub(crate) extern "C" fn runtime_is_strictly_equal<X>(
    _runtime: &mut Runtime<X>,
    x: &Value,
    y: &Value,
) -> bool {
    debug_assert!(!matches!(x, Value::None));
    debug_assert!(!matches!(y, Value::None));
    x == y
}

pub(crate) extern "C" fn runtime_get_typeof<X>(
    _runtime: &mut Runtime<X>,
    value: &Value,
) -> *const U16Chunk {
    debug_assert!(!matches!(value, Value::None));
    value.get_typeof() as *const U16Chunk
}

pub(crate) extern "C" fn runtime_migrate_string_to_heap<X>(
    runtime: &mut Runtime<X>,
    string: *const U16Chunk,
) -> *const U16Chunk {
    let chunk = into_string!(string);
    runtime
        .migrate_string_to_heap(U16String::new(chunk))
        .as_ptr()
}

pub(crate) extern "C" fn runtime_create_capture<X>(
    runtime: &mut Runtime<X>,
    target: *mut Value,
) -> *mut Capture {
    logger::debug!(event = "runtime_create_capture", ?target);

    debug_assert!(
        std::alloc::Layout::from_size_align(
            std::mem::size_of::<Capture>(),
            std::mem::align_of::<Capture>()
        )
        .is_ok()
    );
    // SAFETY: `from_size_align()` always succeeds.
    const LAYOUT: std::alloc::Layout = unsafe {
        std::alloc::Layout::from_size_align_unchecked(
            std::mem::size_of::<Capture>(),
            std::mem::align_of::<Capture>(),
        )
    };

    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(LAYOUT);

    // SAFETY: `ptr` is a non-null pointer to a `Capture`.
    let capture = unsafe { ptr.cast::<Capture>().as_mut() };
    capture.target = target;
    // `capture.escaped` will be filled with an actual value.

    capture as *mut Capture
}

impl<X> Runtime<X> {
    pub(crate) fn create_closure(
        &mut self,
        lambda: Lambda<X>,
        lambda_id: LambdaId,
        num_captures: u16,
    ) -> *mut Closure {
        debug_assert!(
            std::alloc::Layout::from_size_align(
                std::mem::offset_of!(Closure, captures),
                std::mem::align_of::<Closure>()
            )
            .is_ok(),
        );
        // SAFETY: `from_size_align()` always succeeds.
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

        // SAFETY: `ptr` is a non-null pointer to a `Closure`.
        let closure = unsafe { ptr.cast::<Closure>().as_mut() };
        closure.lambda = lambda.into();
        closure.lambda_id = lambda_id;
        closure.num_captures = num_captures;
        // `closure.captures[]` will be filled with actual pointers to `Captures`.

        closure as *mut Closure
    }
}

pub(crate) extern "C" fn runtime_create_closure<X>(
    runtime: &mut Runtime<X>,
    lambda: Lambda<X>,
    lambda_id: u32,
    num_captures: u16,
) -> *mut Closure {
    logger::debug!(
        event = "runtime_create_closure",
        ?lambda,
        lambda_id,
        num_captures
    );
    runtime.create_closure(lambda, lambda_id.into(), num_captures)
}

pub(crate) extern "C" fn runtime_create_coroutine<X>(
    runtime: &mut Runtime<X>,
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

    debug_assert!(
        std::alloc::Layout::from_size_align(
            std::mem::offset_of!(Coroutine, locals),
            std::mem::align_of::<Coroutine>()
        )
        .is_ok()
    );
    // SAFETY: `from_size_align()` always succeeds.
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

    let allocator = runtime.allocator();

    // TODO: GC
    let ptr = allocator.alloc_layout(layout);

    // SAFETY: `ptr` is a non-null pointer to a `Coroutine`.
    let coroutine = unsafe { ptr.cast::<Coroutine>().as_mut() };
    coroutine.closure = closure;
    coroutine.state = 0;
    coroutine.num_locals = num_locals;
    coroutine.scope_id = 0;
    coroutine.scratch_buffer_len = scratch_buffer_len;
    // `coroutine.locals[]` will be initialized in the coroutine.

    coroutine as *mut Coroutine
}

pub(crate) extern "C" fn runtime_register_promise<X>(
    runtime: &mut Runtime<X>,
    coroutine: *mut Coroutine,
) -> u32 {
    runtime.register_promise(coroutine).into()
}

pub(crate) extern "C" fn runtime_resume<X>(runtime: &mut Runtime<X>, promise: u32) {
    runtime.process_promise(promise.into(), &Value::None, &Value::None);
}

pub(crate) extern "C" fn runtime_await_promise<X>(
    runtime: &mut Runtime<X>,
    promise: u32,
    awaiting: u32,
) {
    runtime.await_promise(promise.into(), awaiting.into());
}

pub(crate) extern "C" fn runtime_emit_promise_resolved<X>(
    runtime: &mut Runtime<X>,
    promise: u32,
    result: &Value,
) {
    runtime.emit_promise_resolved(promise.into(), result.clone());
}

pub(crate) extern "C" fn runtime_create_object<X>(
    runtime: &mut Runtime<X>,
    prototype: *mut c_void,
) -> *mut c_void {
    runtime.create_object(prototype).as_ptr()
}

pub(crate) extern "C" fn runtime_get_value_by_symbol<X>(
    _runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: u32,
    strict: bool,
) -> *const Value {
    const UNDEFINED: Value = Value::Undefined;

    let object = into_object!(object);

    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);

    match object.get_value(&key) {
        Some(v) => v as *const Value,
        None if strict => std::ptr::null(),
        None => &UNDEFINED as *const Value,
    }
}

pub(crate) extern "C" fn runtime_get_value_by_number<X>(
    _runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: f64,
    strict: bool,
) -> *const Value {
    const UNDEFINED: Value = Value::Undefined;

    let object = into_object!(object);

    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);

    match object.get_value(&key) {
        Some(v) => v as *const Value,
        None if strict => std::ptr::null(),
        None => &UNDEFINED as *const Value,
    }
}

pub(crate) extern "C" fn runtime_get_value_by_value<X>(
    runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: &Value,
    strict: bool,
) -> *const Value {
    const UNDEFINED: Value = Value::Undefined;

    let object = into_object!(object);
    let key = runtime.make_property_key(key);

    match object.get_value(&key) {
        Some(v) => v as *const Value,
        None if strict => std::ptr::null(),
        None => &UNDEFINED as *const Value,
    }
}

pub(crate) extern "C" fn runtime_set_value_by_symbol<X>(
    _runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: u32,
    value: &Value,
) {
    let object = into_object!(object);
    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);
    object.set_value(&key, value)
}

pub(crate) extern "C" fn runtime_set_value_by_number<X>(
    _runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: f64,
    value: &Value,
) {
    let object = into_object!(object);
    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);
    object.set_value(&key, value)
}

pub(crate) extern "C" fn runtime_set_value_by_value<X>(
    runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: &Value,
    value: &Value,
) {
    let object = into_object!(object);
    let key = runtime.make_property_key(key);
    object.set_value(&key, value)
}

pub(crate) extern "C" fn runtime_concat_strings<X>(
    runtime: &mut Runtime<X>,
    head: *const U16Chunk,
    tail: *const U16Chunk,
) -> *const U16Chunk {
    debug_assert!(!tail.is_null());
    debug_assert!(!head.is_null());

    let tail = into_string!(tail);
    if tail.is_empty() {
        let head = into_string!(head);
        return runtime.alloc_string_rec(head, std::ptr::null());
    }

    let tail = if tail.on_stack() {
        runtime.alloc_string_rec(tail, std::ptr::null())
    } else {
        tail
    } as *const U16Chunk;

    let head = into_string!(head);
    if head.is_empty() {
        return tail;
    }

    runtime.alloc_string_rec(head, tail)
}

// 7.3.5 CreateDataProperty ( O, P, V )
pub(crate) extern "C" fn runtime_create_data_property_by_symbol<X>(
    runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: u32,
    value: &Value,
    retv: &mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script
    let object = into_object!(object);

    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);

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
pub(crate) extern "C" fn runtime_create_data_property_by_number<X>(
    runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: f64,
    value: &Value,
    retv: &mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    let object = into_object!(object);

    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);

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
pub(crate) extern "C" fn runtime_create_data_property_by_value<X>(
    runtime: &mut Runtime<X>,
    object: *mut c_void,
    key: &Value,
    value: &Value,
    retv: &mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    let object = into_object!(object);
    let key = runtime.make_property_key(key);

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
pub(crate) extern "C" fn runtime_copy_data_properties<X>(
    runtime: &mut Runtime<X>,
    target: *mut c_void,
    source: &Value,
    retv: &mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script
    let target = into_object!(target);

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

pub(crate) extern "C" fn runtime_push_value<X>(
    runtime: &mut Runtime<X>,
    target: *mut c_void,
    value: &Value,
    retv: &mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    let target = into_object!(target);

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

pub(crate) extern "C" fn runtime_panic<X>(
    _runtime: &mut Runtime<X>,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    panic!("runtime_panic: {msg:?}");
}

pub(crate) extern "C" fn runtime_print_bool<X>(
    _runtime: &mut Runtime<X>,
    value: bool,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_bool: {value}");
    } else {
        logger::debug!("runtime_print_bool: {value}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_u32<X>(
    _runtime: &mut Runtime<X>,
    value: u32,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_u32: {value:08X}");
    } else {
        logger::debug!("runtime_print_u32: {value:08X}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_f64<X>(
    _runtime: &mut Runtime<X>,
    value: f64,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_f64: {value}");
    } else {
        logger::debug!("runtime_print_f64: {value}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_string<X>(
    _runtime: &mut Runtime<X>,
    value: *const U16Chunk,
    msg: *const std::os::raw::c_char,
) {
    let value = into_string!(value);
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_f64: {value:?}");
    } else {
        logger::debug!("runtime_print_f64: {value:?}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_value<X>(
    _runtime: &mut Runtime<X>,
    value: &Value,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_value: {value:?}");
    } else {
        logger::debug!("runtime_print_value: {value:?}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_capture<X>(
    _runtime: &mut Runtime<X>,
    capture: *const Capture,
    msg: *const std::os::raw::c_char,
) {
    let capture = into_capture!(capture);
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_capture: {capture:?}");
    } else {
        logger::debug!("runtime_print_capture: {capture:?}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_message<X>(
    _runtime: &mut Runtime<X>,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    logger::debug!("runtime_print_value: {msg:?}");
}

pub(crate) extern "C" fn runtime_launch_debugger<X>(_runtime: &mut Runtime<X>) {
    logger::debug!("runtime_launch_debugger");
    // TODO(feat): Support debuggers such as Chrome DevTools.
}
