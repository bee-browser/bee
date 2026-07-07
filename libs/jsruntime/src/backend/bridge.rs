use jsgc::Handle;
use jsgc::HandleMut;
use jsparser::Symbol;

use crate::Error;
use crate::Runtime;
use crate::lambda::LambdaKind;
use crate::semantics::Function;
use crate::semantics::ThisBinding;
use crate::types::CallContext;
use crate::types::Capture;
use crate::types::Closure;
use crate::types::Coroutine;
use crate::types::Lambda;
use crate::types::Object;
use crate::types::Promise;
use crate::types::PropertyKey;
use crate::types::Status;
use crate::types::String;
use crate::types::Value;

logging::define_logger! {}

macro_rules! into_object {
    ($value:expr) => {
        // SAFETY: `value` is always a non-null pointer to an `Object`.
        unsafe {
            debug_assert!(!$value.is_null());
            &mut *($value as *mut crate::types::Object)
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
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_normal");

    let lambda_id = cc.closure().lambda_id;
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

    let func = runtime.get_function_by_lambda_id(lambda_id);
    let call_stub = get_call_stub(func).unwrap_or(lambda);
    let construct_stub = get_construct_stub(func).unwrap_or(lambda);

    debug_assert_eq!(
        cc.closure().lambda,
        (runtime_lazy_compile_normal::<X> as *const () as usize).into()
    );
    cc.closure().lambda = lambda.into();
    cc.closure().call_stub = call_stub.into();
    cc.closure().construct_stub = construct_stub.into();

    if cc.new_target().is_none() {
        call_stub(runtime, cc, retv)
    } else {
        construct_stub(runtime, cc, retv)
    }
}

pub(crate) extern "C" fn runtime_lazy_compile_ramp<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_ramp");

    let lambda_id = cc.closure().lambda_id;
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

    let func = runtime.get_function_by_lambda_id(lambda_id);
    let call_stub = get_call_stub(func).unwrap_or(lambda);
    let construct_stub = get_construct_stub(func).unwrap_or(lambda);

    debug_assert_eq!(
        cc.closure().lambda,
        (runtime_lazy_compile_ramp::<X> as *const () as usize).into()
    );
    cc.closure().lambda = lambda.into();
    cc.closure().call_stub = call_stub.into();
    cc.closure().construct_stub = construct_stub.into();

    if cc.new_target().is_none() {
        call_stub(runtime, cc, retv)
    } else {
        construct_stub(runtime, cc, retv)
    }
}

pub(crate) extern "C" fn runtime_lazy_compile_coroutine<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_lazy_compile_coroutine");

    let coroutine = cc.coroutine_mut();

    let lambda_id = coroutine.closure.lambda_id;
    // The coroutine lambda has already been compiled in `runtime_lazy_compile_ramp()`.
    let lambda = runtime.code_registry.get_lambda(lambda_id).unwrap();

    debug_assert_eq!(
        coroutine.closure.lambda,
        (runtime_lazy_compile_coroutine::<X> as *const () as usize).into()
    );
    coroutine.closure.lambda = lambda.into();
    coroutine.closure.call_stub = lambda.into();
    coroutine.closure.construct_stub = lambda.into();

    lambda(runtime, cc, retv)
}

// Stub functions for [[Call]]

fn get_call_stub<X>(func: &Function) -> Option<Lambda<X>> {
    if func.is_class_constructor() {
        Some(call_stub_for_class_constructor)
    } else {
        match func.this_binding {
            ThisBinding::Capture => Some(call_stub_this_binding_capture),
            ThisBinding::GlobalObject => Some(call_stub_this_binding_global_object),
            ThisBinding::Quirk => Some(call_stub_this_binding_quirk),
            _ => None,
        }
    }
}

// For [[IsClassConstructor]]
extern "C" fn call_stub_for_class_constructor<X>(
    runtime: &mut Runtime<X>,
    _cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "call_stub_for_class_constructor");

    // Always throw a TypeError.
    type_error!(runtime, retv)
}

// #sec-ordinarycallbindthis
extern "C" fn call_stub_this_binding_capture<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "call_stub_this_binding_capture");

    // SAFETY: The capture of the outer `this` binding is always the first element in the capture
    // list.
    cc.set_this(unsafe { cc.closure().captures().get_unchecked(0).value().clone() });

    let lambda = Lambda::from(cc.closure().lambda);
    // We expect TCO to optimize the following call into a jump instruction.
    // DO NOT USE local variables that implement `Drop`.
    lambda(runtime, cc, retv)
}

// #sec-ordinarycallbindthis
// const this = globalObject;
// DO NOT USE `globalThis` HERE.
extern "C" fn call_stub_this_binding_global_object<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "call_stub_this_binding_global_object");

    cc.set_this(Value::Object(runtime.builtins.global_object));

    let lambda = Lambda::from(cc.closure().lambda);
    // We expect TCO to optimize the following call into a jump instruction.
    // DO NOT USE local variables that implement `Drop`.
    lambda(runtime, cc, retv)
}

// #sec-ordinarycallbindthis
// const this =
//   thisArgument !== null && thisArgument !== undefined ?
//   ToObject(thisArgument) : globalObject;
extern "C" fn call_stub_this_binding_quirk<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "call_stub_this_binding_quirk");

    cc.set_this(Value::Object(match cc.this() {
        Value::Undefined | Value::Null => runtime.builtins.global_object,
        this => runtime.value_to_object(this).unwrap(),
    }));

    let lambda = Lambda::from(cc.closure().lambda);
    // We expect TCO to optimize the following call into a jump instruction.
    // DO NOT USE local variables that implement `Drop`.
    lambda(runtime, cc, retv)
}

// Stub functions for [[Construct]]

fn get_construct_stub<X>(func: &Function) -> Option<Lambda<X>> {
    if func.is_derived_constructor() {
        None
    } else {
        match func.this_binding {
            ThisBinding::Capture => Some(construct_stub_this_binding_capture),
            ThisBinding::GlobalObject => Some(construct_stub_this_binding_global_object),
            ThisBinding::Quirk => Some(construct_stub_this_binding_quirk),
            _ => None,
        }
    }
}

// #sec-ecmascript-function-objects-construct-argumentslist-newtarget
extern "C" fn construct_stub_this_binding_capture<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "construct_stub_this_binding_capture");

    // SAFETY: The capture of the outer `this` binding is always the first element in the capture
    // list.
    cc.set_this(unsafe { cc.closure().captures().get_unchecked(0).value().clone() });

    // TODO: initializers

    let lambda = Lambda::from(cc.closure().lambda);
    // We expect TCO to optimize the following call into a jump instruction.
    // DO NOT USE local variables that implement `Drop`.
    lambda(runtime, cc, retv)
}

// #sec-ecmascript-function-objects-construct-argumentslist-newtarget
// const this = globalObject;
// DO NOT USE `globalThis` HERE.
extern "C" fn construct_stub_this_binding_global_object<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "construct_stub_this_binding_global_object");

    cc.set_this(Value::Object(runtime.builtins.global_object));

    // TODO: initializers

    let lambda = Lambda::from(cc.closure().lambda);
    // We expect TCO to optimize the following call into a jump instruction.
    // DO NOT USE local variables that implement `Drop`.
    lambda(runtime, cc, retv)
}

// #sec-ecmascript-function-objects-construct-argumentslist-newtarget
// const this =
//   thisArgument !== null && thisArgument !== undefined ?
//   ToObject(thisArgument) : globalObject;
extern "C" fn construct_stub_this_binding_quirk<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "construct_stub_this_binding_quirk");

    cc.set_this(Value::Object(match cc.this() {
        Value::Undefined | Value::None => runtime.builtins.global_object,
        this => runtime.value_to_object(this).unwrap(),
    }));

    // TODO: initializers

    let lambda = Lambda::from(cc.closure().lambda);
    // We expect TCO to optimize the following call into a jump instruction.
    // DO NOT USE local variables that implement `Drop`.
    lambda(runtime, cc, retv)
}

// 7.1.2 ToBoolean ( argument )
pub(crate) extern "C" fn runtime_to_boolean<X>(_runtime: &mut Runtime<X>, value: &Value) -> bool {
    logger::debug!(event = "runtime_to_boolean", ?value);
    value.to_boolean()
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
        Value::String(_value) => f64::NAN, // TODO(feat): 7.1.4.1.1 StringToNumber ( str )
        Value::Object(_) => f64::NAN,      // TODO(feat): 7.1.1 ToPrimitive()
    }
}

// 7.1.17 ToString ( argument )
pub(crate) extern "C" fn runtime_to_string<X>(
    runtime: &mut Runtime<X>,
    value: &Value,
) -> Handle<String> {
    logger::debug!(event = "runtime_to_string", ?value);
    runtime.value_to_string(value).unwrap()
}

// 6.1.6.1.20 Number::toString ( x, radix )
pub(crate) extern "C" fn runtime_number_to_string<X>(
    runtime: &mut Runtime<X>,
    value: f64,
) -> Handle<String> {
    logger::debug!(event = "runtime_number_to_string", ?value);
    runtime.number_to_string(value)
}

impl<X> Runtime<X> {
    pub(crate) fn number_to_string(&mut self, value: f64) -> Handle<String> {
        // TODO(feat): implment Number::toString()
        self.create_string_from_utf8(&format!("{value}"))
    }
}

// 7.1.18 ToObject ( argument )
pub(crate) extern "C" fn runtime_to_object<X>(
    runtime: &mut Runtime<X>,
    value: &Value,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "runtime_to_object", ?value);
    match runtime.value_to_object(value) {
        Ok(object) => {
            *retv = Value::Object(object);
            Status::Normal
        }
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

impl<X> Runtime<X> {
    // 7.1.18 ToObject ( argument )
    pub(crate) fn value_to_object(&mut self, value: &Value) -> Result<HandleMut<Object>, Error> {
        logger::debug!(event = "value_to_object", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined | Value::Null => type_error!(),
            Value::Boolean(value) => Ok(self.create_boolean_object(*value)),
            Value::Number(_value) => {
                runtime_todo!("ToObject: not yet implemented for Number values")
            }
            Value::String(value) => {
                // TODO(refactor): rewrite using `new String(value)`
                match self.create_string_object(None, &[Value::String(*value)], true)? {
                    Value::Object(object) => Ok(object),
                    _ => unreachable!(),
                }
            }
            Value::Object(object) => Ok(*object),
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
    a: Handle<String>,
    b: Handle<String>,
) -> bool {
    *a == *b
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
) -> Handle<String> {
    debug_assert!(!matches!(value, Value::None));
    value.get_typeof()
}

pub(crate) extern "C" fn runtime_instanceof<X>(
    runtime: &mut Runtime<X>,
    value: &Value,
    target: &Value,
    retv: &mut Value,
) -> Status {
    debug_assert!(!matches!(value, Value::None));
    debug_assert!(!matches!(target, Value::None));
    match runtime.instanceof(value, target) {
        Ok(result) => {
            *retv = Value::Boolean(result);
            Status::Normal
        }
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

pub(crate) extern "C" fn runtime_create_string<X>(
    runtime: &mut Runtime<X>,
    ptr: *const u16,
    len: usize,
) -> Handle<String> {
    // SAFETY: `from_raw_parts()` always succeeds.
    let utf16 = unsafe { std::slice::from_raw_parts(ptr, len) };
    runtime.create_string(utf16)
}

pub(crate) extern "C" fn runtime_create_capture<X>(
    runtime: &mut Runtime<X>,
    target: *mut Value,
) -> HandleMut<Capture> {
    logger::debug!(event = "runtime_create_capture", ?target);
    runtime.create_capture(target)
}

pub(crate) extern "C" fn runtime_create_closure<X>(
    runtime: &mut Runtime<X>,
    lambda: Lambda<X>,
    lambda_id: u32,
    num_captures: u16,
) -> HandleMut<Closure> {
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
    capture_buffer_len: u16,
) -> HandleMut<Coroutine> {
    logger::debug!(
        event = "runtime_create_coroutine",
        ?closure,
        num_locals,
        scratch_buffer_len,
        capture_buffer_len,
    );
    let closure = HandleMut::from_ptr(closure).expect("closure must be a non-null pointer");
    runtime.create_coroutine(closure, num_locals, scratch_buffer_len, capture_buffer_len)
}

pub(crate) extern "C" fn runtime_create_promise<X>(
    runtime: &mut Runtime<X>,
    coroutine: *mut Coroutine,
) -> HandleMut<Promise> {
    let coroutine = HandleMut::from_ptr(coroutine).expect("coroutine must be a non-null pointer");
    runtime.create_promise(coroutine)
}

pub(crate) extern "C" fn runtime_resume<X>(runtime: &mut Runtime<X>, promise: *mut Object) {
    let promise = HandleMut::from_ptr(promise).unwrap();
    debug_assert!(runtime.is_promise_object(promise));
    runtime.process_promise(promise, &Value::None, &Value::None);
}

pub(crate) extern "C" fn runtime_emit_promise_resolved<X>(
    runtime: &mut Runtime<X>,
    promise: *mut Object,
    result: &Value,
) {
    let promise = HandleMut::from_ptr(promise).unwrap();
    debug_assert!(runtime.is_promise_object(promise));
    runtime.emit_promise_resolved(promise, result.clone());
}

pub(crate) extern "C" fn runtime_create_object<X>(
    runtime: &mut Runtime<X>,
    prototype: *mut Object,
) -> HandleMut<Object> {
    let prototype = HandleMut::from_ptr(prototype).unwrap();
    let mut object = runtime.create_object();
    object.set_prototype(prototype);
    object
}

pub(crate) extern "C" fn runtime_create_reference_error<X>(
    runtime: &mut Runtime<X>,
) -> HandleMut<Object> {
    runtime.create_reference_error(None)
}

pub(crate) extern "C" fn runtime_create_type_error<X>(
    runtime: &mut Runtime<X>,
) -> HandleMut<Object> {
    runtime.create_type_error(None)
}

pub(crate) extern "C" fn runtime_create_internal_error<X>(
    runtime: &mut Runtime<X>,
    message: Handle<String>,
) -> HandleMut<Object> {
    runtime.create_internal_error(Some(message))
}

pub(crate) extern "C" fn runtime_get_value_by_symbol<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
    key: u32,
    strict: bool,
    retv: &mut Value,
) -> Status {
    let object = into_object!(object);

    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);

    match object.get_value(&key) {
        Some(v) => {
            *retv = v.clone();
            Status::Normal
        }
        None if strict => {
            *retv = Value::Object(runtime.create_reference_error(None));
            Status::Exception
        }
        None => {
            *retv = Value::Undefined;
            Status::Normal
        }
    }
}

pub(crate) extern "C" fn runtime_get_value_by_number<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
    key: f64,
    strict: bool,
    retv: &mut Value,
) -> Status {
    let object = into_object!(object);

    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);

    match object.get_value(&key) {
        Some(v) => {
            *retv = v.clone();
            Status::Normal
        }
        None if strict => {
            *retv = Value::Object(runtime.create_reference_error(None));
            Status::Exception
        }
        None => {
            *retv = Value::Undefined;
            Status::Normal
        }
    }
}

pub(crate) extern "C" fn runtime_get_value_by_value<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
    key: &Value,
    strict: bool,
    retv: &mut Value,
) -> Status {
    let object = into_object!(object);
    let key = match runtime.make_property_key(key) {
        Ok(key) => key,
        Err(err) => {
            *retv = runtime.create_exception(err);
            return Status::Exception;
        }
    };

    match object.get_value(&key) {
        Some(v) => {
            *retv = v.clone();
            Status::Normal
        }
        None if strict => {
            *retv = Value::Object(runtime.create_reference_error(None));
            Status::Exception
        }
        None => {
            *retv = Value::Undefined;
            Status::Normal
        }
    }
}

pub(crate) extern "C" fn runtime_set_value_by_symbol<X>(
    _runtime: &mut Runtime<X>,
    object: *mut Object,
    key: u32,
    value: &Value,
    _retv: &mut Value,
) -> Status {
    let object = into_object!(object);
    debug_assert_ne!(key, 0);
    let key = PropertyKey::from(key);
    object.set_value(&key, value);
    Status::Normal
}

pub(crate) extern "C" fn runtime_set_value_by_number<X>(
    _runtime: &mut Runtime<X>,
    object: *mut Object,
    key: f64,
    value: &Value,
    _retv: &mut Value,
) -> Status {
    let object = into_object!(object);
    debug_assert!(f64::is_finite(key));
    let key = PropertyKey::from(key);
    object.set_value(&key, value);
    Status::Normal
}

pub(crate) extern "C" fn runtime_set_value_by_value<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
    key: &Value,
    value: &Value,
    retv: &mut Value,
) -> Status {
    let object = into_object!(object);
    let key = match runtime.make_property_key(key) {
        Ok(key) => key,
        Err(err) => {
            *retv = runtime.create_exception(err);
            return Status::Exception;
        }
    };
    object.set_value(&key, value);
    Status::Normal
}

pub(crate) extern "C" fn runtime_concat_strings<X>(
    runtime: &mut Runtime<X>,
    head: Handle<String>,
    tail: Handle<String>,
) -> Handle<String> {
    head.concat(tail, &mut runtime.heap)
}

// 7.3.5 CreateDataProperty ( O, P, V )
pub(crate) extern "C" fn runtime_create_data_property_by_symbol<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
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
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

// 7.3.5 CreateDataProperty ( O, P, V )
pub(crate) extern "C" fn runtime_create_data_property_by_number<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
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
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

// 7.3.5 CreateDataProperty ( O, P, V )
pub(crate) extern "C" fn runtime_create_data_property_by_value<X>(
    runtime: &mut Runtime<X>,
    object: *mut Object,
    key: &Value,
    value: &Value,
    retv: &mut Value,
) -> Status {
    // TODO(refactor): generate ffi-conversion code by script

    let object = into_object!(object);
    let key = match runtime.make_property_key(key) {
        Ok(key) => key,
        Err(err) => {
            *retv = runtime.create_exception(err);
            return Status::Exception;
        }
    };

    match runtime.create_data_property(object, &key, value) {
        Ok(success) => {
            *retv = success.into();
            Status::Normal
        }
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

// 7.3.25 CopyDataProperties ( target, source, excludedItems )
pub(crate) extern "C" fn runtime_copy_data_properties<X>(
    runtime: &mut Runtime<X>,
    target: *mut Object,
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
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

pub(crate) extern "C" fn runtime_push_value<X>(
    runtime: &mut Runtime<X>,
    target: *mut Object,
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
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

pub(crate) extern "C" fn runtime_build_class<X>(
    runtime: &mut Runtime<X>,
    name: u32,
    constructor: HandleMut<Object>,
    prototype: HandleMut<Object>,
) -> HandleMut<Object> {
    let name = Symbol::from(name);
    runtime.build_class(name, constructor, prototype)
}

pub(crate) extern "C" fn runtime_build_prototype_chain<X>(
    runtime: &mut Runtime<X>,
    parent: &Value,
    constructor: HandleMut<Object>,
    prototype: HandleMut<Object>,
    retv: &mut Value,
) -> Status {
    match runtime.build_prototype_chain(parent, constructor, prototype) {
        Ok(()) => {
            *retv = Value::None;
            Status::Normal
        }
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

pub(crate) extern "C" fn runtime_construct<X>(
    runtime: &mut Runtime<X>,
    constructor: HandleMut<Object>,
    new_target: HandleMut<Object>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    runtime.construct(constructor, new_target, cc, retv)
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
    value: Handle<String>,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_string: {value:?}");
    } else {
        logger::debug!("runtime_print_string: {value:?}: {msg:?}");
    }
}

pub(crate) extern "C" fn runtime_print_object<X>(
    _runtime: &mut Runtime<X>,
    value: HandleMut<Object>,
    msg: *const std::os::raw::c_char,
) {
    // SAFETY: `msg` is always non-null.
    let msg = unsafe {
        debug_assert!(!msg.is_null());
        std::ffi::CStr::from_ptr(msg)
    };
    if msg.is_empty() {
        logger::debug!("runtime_print_object: {value:?}");
    } else {
        logger::debug!("runtime_print_object: {value:?}: {msg:?}");
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
