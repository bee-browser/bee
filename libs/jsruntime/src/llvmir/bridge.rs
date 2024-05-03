#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::Symbol;

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
            get_local: Some(runtime_get_local),
            get_local_boolean: Some(runtime_get_local_boolean),
            get_local_number: Some(runtime_get_local_number),
            put_local: Some(runtime_put_local),
            put_local_undefined: Some(runtime_put_local_undefined),
            put_local_boolean: Some(runtime_put_local_boolean),
            put_local_number: Some(runtime_put_local_number),
            push_argument: Some(runtime_push_argument),
            push_argument_undefined: Some(runtime_push_argument_undefined),
            push_argument_boolean: Some(runtime_push_argument_boolean),
            push_argument_number: Some(runtime_push_argument_number),
            get_argument: Some(runtime_get_argument),
            get_argument_boolean: Some(runtime_get_argument_boolean),
            get_argument_number: Some(runtime_get_argument_number),
            put_argument: Some(runtime_put_argument),
            put_argument_undefined: Some(runtime_put_argument_undefined),
            put_argument_boolean: Some(runtime_put_argument_boolean),
            put_argument_number: Some(runtime_put_argument_number),
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
    symbol_id: u32,
    index: u16,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    // TODO: transmute the value
    runtime.declare_immutable(symbol, index, crate::Value::load(value));
}

unsafe extern "C" fn runtime_declare_immutable_undefined(
    context: usize,
    symbol_id: u32,
    index: u16,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_immutable(symbol, index, crate::Value::Undefined);
}

unsafe extern "C" fn runtime_declare_immutable_boolean(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: bool,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_immutable(symbol, index, crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_declare_immutable_number(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: f64,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_immutable(symbol, index, crate::Value::Number(value));
}

unsafe extern "C" fn runtime_declare_mutable(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    // TODO: transmute
    runtime.declare_mutable(symbol, index, crate::Value::load(value));
}

unsafe extern "C" fn runtime_declare_mutable_undefined(context: usize, symbol_id: u32, index: u16) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_mutable(symbol, index, crate::Value::Undefined);
}

unsafe extern "C" fn runtime_declare_mutable_boolean(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: bool,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_mutable(symbol, index, crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_declare_mutable_number(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: f64,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_mutable(symbol, index, crate::Value::Number(value));
}

unsafe extern "C" fn runtime_declare_function(
    context: usize,
    symbol_id: u32,
    index: u16,
    func_id: u32,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    let func_id = crate::FunctionId::from(func_id);
    runtime.declare_function(symbol, index, func_id);
}

unsafe extern "C" fn runtime_get_local(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
    value: *mut Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    // TODO: transmute
    runtime.get_local(symbol, stack, index).store(value);
}

unsafe extern "C" fn runtime_get_local_boolean(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
) -> bool {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    match runtime.get_local(symbol, stack, index) {
        crate::Value::Boolean(value) => value,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_get_local_number(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
) -> f64 {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    match runtime.get_local(symbol, stack, index) {
        crate::Value::Number(value) => value,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_put_local(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    // TODO: transmute
    runtime.put_local(symbol, stack, index, crate::Value::load(value));
}

unsafe extern "C" fn runtime_put_local_undefined(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_local(symbol, stack, index, crate::Value::Undefined);
}

unsafe extern "C" fn runtime_put_local_boolean(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
    value: bool,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_local(symbol, stack, index, crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_put_local_number(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
    value: f64,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_local(symbol, stack, index, crate::Value::Number(value));
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

unsafe extern "C" fn runtime_get_argument(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: *mut Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    // TODO: transmute
    runtime.get_argument(symbol, index).store(value);
}

unsafe extern "C" fn runtime_get_argument_boolean(
    context: usize,
    symbol_id: u32,
    index: u16,
) -> bool {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    match runtime.get_argument(symbol, index) {
        crate::Value::Boolean(value) => value,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_get_argument_number(
    context: usize,
    symbol_id: u32,
    index: u16,
) -> f64 {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    match runtime.get_argument(symbol, index) {
        crate::Value::Number(value) => value,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_put_argument(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: *const Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    // TODO: transmute
    runtime.put_argument(symbol, index, crate::Value::load(value));
}

unsafe extern "C" fn runtime_put_argument_undefined(context: usize, symbol_id: u32, index: u16) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_argument(symbol, index, crate::Value::Undefined);
}

unsafe extern "C" fn runtime_put_argument_boolean(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: bool,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_argument(symbol, index, crate::Value::Boolean(value));
}

unsafe extern "C" fn runtime_put_argument_number(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: f64,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_argument(symbol, index, crate::Value::Number(value));
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
            ValueKind_Function => Self::Function(crate::Function {
                id: value.holder.function.id.into(),
                lexical_scope_index: value.holder.function.lexical_call_index,
            }),
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
            crate::Value::Function(func) => Value {
                kind: ValueKind_Function,
                holder: ValueHolder {
                    function: Function {
                        id: func.id.into(),
                        lexical_call_index: func.lexical_scope_index,
                    },
                },
            },
        })
    }
}
