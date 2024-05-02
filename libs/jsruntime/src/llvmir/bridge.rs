#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::Symbol;

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

impl Default for Runtime {
    fn default() -> Self {
        Self {
            declare_const: Some(runtime_declare_const),
            declare_variable: Some(runtime_declare_variable),
            declare_function: Some(runtime_declare_function),
            get_argument: Some(runtime_get_argument),
            get_local: Some(runtime_get_local),
            put_argument: Some(runtime_put_argument),
            put_local: Some(runtime_put_local),
            push_arg: Some(runtime_push_arg),
            call: Some(runtime_call),
            ret: Some(runtime_ret),
            allocate_bindings: Some(runtime_allocate_bindings),
            release_bindings: Some(runtime_release_bindings),
            inspect_number: Some(runtime_inspect_number),
            inspect_any: Some(runtime_inspect_any),
        }
    }
}

macro_rules! into_runtime {
    ($context:expr) => {
        &mut *($context as *mut super::Runtime)
    };
}

unsafe extern "C" fn runtime_declare_const(context: usize, symbol_id: u32, index: u16, value: f64) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_const(symbol, index, value);
}

unsafe extern "C" fn runtime_declare_variable(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: f64,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_variable(symbol, index, value);
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

unsafe extern "C" fn runtime_get_argument(
    context: usize,
    symbol_id: u32,
    index: u16,
    value: *mut Value,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.get_argument(symbol, index).store(value);
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
    runtime.get_local(symbol, stack, index).store(value);
}

unsafe extern "C" fn runtime_put_argument(context: usize, symbol_id: u32, index: u16, value: f64) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_argument(symbol, index, value);
}

unsafe extern "C" fn runtime_put_local(
    context: usize,
    symbol_id: u32,
    stack: u16,
    index: u16,
    value: f64,
) {
    let runtime = into_runtime!(context);
    let symbol = Symbol::from(symbol_id);
    runtime.put_local(symbol, stack, index, value);
}

unsafe extern "C" fn runtime_push_arg(context: usize, arg: f64) {
    let runtime = into_runtime!(context);
    runtime.push_arg(arg);
}

unsafe extern "C" fn runtime_call(context: usize, value: *const Value) -> f64 {
    let runtime = into_runtime!(context);
    let value = crate::Value::load(value);
    runtime.call(value)
}

unsafe extern "C" fn runtime_ret(context: usize, value: f64) {
    let runtime = into_runtime!(context);
    runtime.ret(value);
}

unsafe extern "C" fn runtime_allocate_bindings(context: usize, n: u16) {
    let runtime = into_runtime!(context);
    runtime.allocate_bindings(n);
}

unsafe extern "C" fn runtime_release_bindings(context: usize, n: u16) {
    let runtime = into_runtime!(context);
    runtime.release_bindings(n);
}

unsafe extern "C" fn runtime_inspect_number(context: usize, value: f64) {
    let runtime = into_runtime!(context);
    runtime.inspect_number(value);
}

unsafe extern "C" fn runtime_inspect_any(context: usize, value: *const Value) {
    let runtime = into_runtime!(context);
    runtime.inspect_any(crate::Value::load(value));
}

impl crate::Value {
    #[inline(always)]
    unsafe fn load(value: *const Value) -> Self {
        let value = &*value;
        match value.kind {
            ValueKind_Undefined => Self::Undefined,
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
