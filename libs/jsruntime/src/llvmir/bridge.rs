#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::FunctionId;
use super::Runtime;
use super::Symbol;

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

impl Default for Host {
    fn default() -> Self {
        Self {
            runtime_declare_const: Some(runtime_declare_const),
            runtime_declare_variable: Some(runtime_declare_variable),
            runtime_declare_function: Some(runtime_declare_function),
            runtime_get: Some(runtime_get),
            runtime_set: Some(runtime_set),
            runtime_push_args: Some(runtime_push_args),
            runtime_push_arg: Some(runtime_push_arg),
            runtime_call: Some(runtime_call),
            runtime_ret: Some(runtime_ret),
            runtime_push_scope: Some(runtime_push_scope),
            runtime_pop_scope: Some(runtime_pop_scope),
        }
    }
}

unsafe extern "C" fn runtime_declare_const(context: usize, symbol_id: u32, value: f64) {
    let runtime = &mut *(context as *mut Runtime);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_const(symbol, value);
}

unsafe extern "C" fn runtime_declare_variable(context: usize, symbol_id: u32, value: f64) {
    let runtime = &mut *(context as *mut Runtime);
    let symbol = Symbol::from(symbol_id);
    runtime.declare_variable(symbol, value);
}

unsafe extern "C" fn runtime_declare_function(context: usize, symbol_id: u32, func_id: u32) {
    let runtime = &mut *(context as *mut Runtime);
    let symbol = Symbol::from(symbol_id);
    let func_id = FunctionId::from(func_id);
    runtime.declare_function(symbol, func_id);
}

unsafe extern "C" fn runtime_get(context: usize, symbol_id: u32) -> f64 {
    let runtime = (context as *const Runtime).as_ref().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.get(symbol)
}

unsafe extern "C" fn runtime_set(context: usize, symbol_id: u32, value: f64) {
    let runtime = &mut *(context as *mut Runtime);
    let symbol = Symbol::from(symbol_id);
    runtime.set(symbol, value);
}

unsafe extern "C" fn runtime_push_args(context: usize) {
    let runtime = &mut *(context as *mut Runtime);
    runtime.push_args();
}

unsafe extern "C" fn runtime_push_arg(context: usize, arg: f64) {
    let runtime = &mut *(context as *mut Runtime);
    runtime.push_arg(arg);
}

unsafe extern "C" fn runtime_call(context: usize, symbol_id: u32) -> f64 {
    let runtime = &mut *(context as *mut Runtime);
    let symbol = Symbol::from(symbol_id);
    runtime.call(symbol)
}

unsafe extern "C" fn runtime_ret(context: usize, value: f64) {
    let runtime = &mut *(context as *mut Runtime);
    runtime.ret(value);
}

unsafe extern "C" fn runtime_push_scope(context: usize) {
    let runtime = &mut *(context as *mut Runtime);
    runtime.push_scope();
}

unsafe extern "C" fn runtime_pop_scope(context: usize) {
    let runtime = &mut *(context as *mut Runtime);
    runtime.pop_scope();
}
