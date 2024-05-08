#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

impl Default for Runtime {
    fn default() -> Self {
        Self {
            declare_immutable: Some(runtime_declare_immutable),
            declare_immutable_undefined: Some(runtime_declare_immutable_undefined),
            declare_immutable_boolean: Some(runtime_declare_immutable_boolean),
            declare_immutable_number: Some(runtime_declare_immutable_number),
            declare_mutable: Some(runtime_declare_mutable),
            declare_mutable_undefined: Some(runtime_declare_mutable_undefined),
            declare_mutable_boolean: Some(runtime_declare_mutable_boolean),
            declare_mutable_number: Some(runtime_declare_mutable_number),
            declare_function: Some(runtime_declare_function),
            get_binding: Some(runtime_get_binding),
            get_binding_boolean: Some(runtime_get_binding_boolean),
            get_binding_number: Some(runtime_get_binding_number),
            put_binding: Some(runtime_put_binding),
            put_binding_undefined: Some(runtime_put_binding_undefined),
            put_binding_boolean: Some(runtime_put_binding_boolean),
            put_binding_number: Some(runtime_put_binding_number),
            push_argument: Some(runtime_push_argument),
            push_argument_undefined: Some(runtime_push_argument_undefined),
            push_argument_boolean: Some(runtime_push_argument_boolean),
            push_argument_number: Some(runtime_push_argument_number),
            call: Some(runtime_call),
            return_value: Some(runtime_return_value),
            return_boolean: Some(runtime_return_boolean),
            return_number: Some(runtime_return_number),
            allocate_bindings: Some(runtime_allocate_bindings),
            release_bindings: Some(runtime_release_bindings),
            inspect: Some(runtime_inspect),
            inspect_boolean: Some(runtime_inspect_boolean),
            inspect_number: Some(runtime_inspect_number),
        }
    }
}

macro_rules! into_runtime {
    ($context:expr) => {
        &mut *($context as *mut super::Runtime)
    };
}

unsafe extern "C" fn runtime_declare_immutable(
    context: usize,
    symbol: u32,
    locator: u32,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    // TODO: transmute the value
    runtime.declare_immutable(symbol.into(), locator.into(), crate::Value::load(value));
}

unsafe extern "C" fn runtime_declare_immutable_undefined(
    context: usize,
    symbol: u32,
    locator: u32,
) {
    let runtime = into_runtime!(context);
    runtime.declare_immutable(symbol.into(), locator.into(), crate::Value::Undefined);
}

unsafe extern "C" fn runtime_declare_immutable_boolean(
    context: usize,
    symbol: u32,
    locator: u32,
    value: bool,
) {
    let runtime = into_runtime!(context);
    runtime.declare_immutable(symbol.into(), locator.into(), crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_declare_immutable_number(
    context: usize,
    symbol: u32,
    locator: u32,
    value: f64,
) {
    let runtime = into_runtime!(context);
    runtime.declare_immutable(symbol.into(), locator.into(), crate::Value::Number(value));
}

unsafe extern "C" fn runtime_declare_mutable(
    context: usize,
    symbol: u32,
    locator: u32,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    // TODO: transmute
    runtime.declare_mutable(symbol.into(), locator.into(), crate::Value::load(value));
}

unsafe extern "C" fn runtime_declare_mutable_undefined(context: usize, symbol: u32, locator: u32) {
    let runtime = into_runtime!(context);
    runtime.declare_mutable(symbol.into(), locator.into(), crate::Value::Undefined);
}

unsafe extern "C" fn runtime_declare_mutable_boolean(
    context: usize,
    symbol: u32,
    locator: u32,
    value: bool,
) {
    let runtime = into_runtime!(context);
    runtime.declare_mutable(symbol.into(), locator.into(), crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_declare_mutable_number(
    context: usize,
    symbol: u32,
    locator: u32,
    value: f64,
) {
    let runtime = into_runtime!(context);
    runtime.declare_mutable(symbol.into(), locator.into(), crate::Value::Number(value));
}

unsafe extern "C" fn runtime_declare_function(
    context: usize,
    symbol: u32,
    locator: u32,
    func_id: u32,
) {
    let runtime = into_runtime!(context);
    runtime.declare_function(symbol.into(), locator.into(), func_id.into());
}

unsafe extern "C" fn runtime_get_binding(
    context: usize,
    symbol: u32,
    locator: u32,
    value: *mut Value,
) {
    let runtime = into_runtime!(context);
    // TODO: transmute
    runtime
        .get_binding(symbol.into(), locator.into())
        .store(value);
}

unsafe extern "C" fn runtime_get_binding_boolean(
    context: usize,
    symbol: u32,
    locator: u32,
) -> bool {
    let runtime = into_runtime!(context);
    match runtime.get_binding(symbol.into(), locator.into()) {
        crate::Value::Boolean(value) => value,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_get_binding_number(context: usize, symbol: u32, locator: u32) -> f64 {
    let runtime = into_runtime!(context);
    match runtime.get_binding(symbol.into(), locator.into()) {
        crate::Value::Number(value) => value,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_put_binding(
    context: usize,
    symbol: u32,
    locator: u32,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    // TODO: transmute
    runtime.put_binding(symbol.into(), locator.into(), crate::Value::load(value));
}

unsafe extern "C" fn runtime_put_binding_undefined(context: usize, symbol: u32, locator: u32) {
    let runtime = into_runtime!(context);
    runtime.put_binding(symbol.into(), locator.into(), crate::Value::Undefined);
}

unsafe extern "C" fn runtime_put_binding_boolean(
    context: usize,
    symbol: u32,
    locator: u32,
    value: bool,
) {
    let runtime = into_runtime!(context);
    runtime.put_binding(symbol.into(), locator.into(), crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_put_binding_number(
    context: usize,
    symbol: u32,
    locator: u32,
    value: f64,
) {
    let runtime = into_runtime!(context);
    runtime.put_binding(symbol.into(), locator.into(), crate::Value::Number(value));
}

unsafe extern "C" fn runtime_push_argument(context: usize, value: *const Value) {
    let runtime = into_runtime!(context);
    // TODO: transmute
    runtime.push_argument(crate::Value::load(value));
}

unsafe extern "C" fn runtime_push_argument_undefined(context: usize) {
    let runtime = into_runtime!(context);
    runtime.push_argument(crate::Value::Undefined);
}

unsafe extern "C" fn runtime_push_argument_boolean(context: usize, value: bool) {
    let runtime = into_runtime!(context);
    runtime.push_argument(crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_push_argument_number(context: usize, value: f64) {
    let runtime = into_runtime!(context);
    runtime.push_argument(crate::Value::Number(value));
}

unsafe extern "C" fn runtime_call(context: usize, value: *const Value, result: *mut Value) {
    let runtime = into_runtime!(context);
    let value = crate::Value::load(value);
    runtime.call(value).store(result);
}

unsafe extern "C" fn runtime_return_value(context: usize, value: *const Value) {
    let runtime = into_runtime!(context);
    // TODO: transmute
    runtime.return_value(crate::Value::load(value));
}

unsafe extern "C" fn runtime_return_boolean(context: usize, value: bool) {
    let runtime = into_runtime!(context);
    runtime.return_value(crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_return_number(context: usize, value: f64) {
    let runtime = into_runtime!(context);
    runtime.return_value(crate::Value::Number(value));
}

unsafe extern "C" fn runtime_allocate_bindings(context: usize, n: u16) {
    let runtime = into_runtime!(context);
    runtime.allocate_bindings(n);
}

unsafe extern "C" fn runtime_release_bindings(context: usize, n: u16) {
    let runtime = into_runtime!(context);
    runtime.release_bindings(n);
}

unsafe extern "C" fn runtime_inspect(context: usize, value: *const Value) {
    let runtime = into_runtime!(context);
    // TODO: transmute
    runtime.inspect(crate::Value::load(value));
}

unsafe extern "C" fn runtime_inspect_boolean(context: usize, value: bool) {
    let runtime = into_runtime!(context);
    runtime.inspect(crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_inspect_number(context: usize, value: f64) {
    let runtime = into_runtime!(context);
    runtime.inspect(crate::Value::Number(value));
}

impl crate::Value {
    #[inline(always)]
    unsafe fn load(value: *const Value) -> Self {
        let value = &*value;
        match value.kind {
            ValueKind_Undefined => Self::Undefined,
            ValueKind_Boolean => Self::Boolean(value.holder.boolean),
            ValueKind_Number => Self::Number(value.holder.number),
            ValueKind_Closure => Self::Closure(value.holder.closure.into()),
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    unsafe fn store(self, value: *mut Value) {
        value.write(match self {
            crate::Value::Undefined => Value {
                kind: ValueKind_Undefined,
                holder: ValueHolder { opaque: 0 },
            },
            crate::Value::Boolean(boolean) => Value {
                kind: ValueKind_Boolean,
                holder: ValueHolder { boolean },
            },
            crate::Value::Number(number) => Value {
                kind: ValueKind_Number,
                holder: ValueHolder { number },
            },
            crate::Value::Closure(closure) => Value {
                kind: ValueKind_Closure,
                holder: ValueHolder {
                    closure: closure.into(),
                },
            },
        })
    }
}
