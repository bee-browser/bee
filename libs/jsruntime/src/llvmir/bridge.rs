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
            print_bool: Some(print_bool),
            print_f64: Some(print_f64),
            print_str: Some(print_str),
            runtime_declare_const: Some(runtime_declare_const),
            runtime_declare_variable: Some(runtime_declare_variable),
            runtime_declare_undefined: Some(runtime_declare_undefined),
            runtime_declare_function: Some(runtime_declare_function),
            runtime_get: Some(runtime_get),
            runtime_set: Some(runtime_set),
            runtime_set_undefined: Some(runtime_set_undefined),
            runtime_push_args: Some(runtime_push_args),
            runtime_push_arg: Some(runtime_push_arg),
            runtime_call: Some(runtime_call),
            runtime_ret: Some(runtime_ret),
            runtime_push_scope: Some(runtime_push_scope),
            runtime_pop_scope: Some(runtime_pop_scope),
        }
    }
}

unsafe extern "C" fn print_bool(value: bool) {
    println!("{value}");
}

unsafe extern "C" fn print_f64(value: f64) {
    println!("{value}");
}

unsafe extern "C" fn print_str(value: *const std::ffi::c_char) {
    // std::ffi::CStr::from_ptr(value).to_str() is safer but slower than the following code.
    let value = std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(value).to_bytes());
    println!("{value}");
}

unsafe extern "C" fn runtime_declare_const(context: usize, symbol_id: u32, value: f64) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.declare_const(symbol, value);
}

unsafe extern "C" fn runtime_declare_variable(context: usize, symbol_id: u32, value: f64) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.declare_variable(symbol, value);
}

unsafe extern "C" fn runtime_declare_undefined(context: usize, symbol_id: u32) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.declare_undefined(symbol);
}

unsafe extern "C" fn runtime_declare_function(context: usize, symbol_id: u32, func_id: u32) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
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
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.set(symbol, value);
}

unsafe extern "C" fn runtime_set_undefined(context: usize, symbol_id: u32) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.set_undefined(symbol);
}

unsafe extern "C" fn runtime_push_args(context: usize) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    runtime.push_args();
}

unsafe extern "C" fn runtime_push_arg(context: usize, arg: f64) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    runtime.push_arg(arg);
}

unsafe extern "C" fn runtime_call(context: usize, symbol_id: u32) -> f64 {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    let symbol = Symbol::from(symbol_id);
    runtime.call(symbol)
}

unsafe extern "C" fn runtime_ret(context: usize, value: f64) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    runtime.ret(value);
}

unsafe extern "C" fn runtime_push_scope(context: usize) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    runtime.push_scope();
}

unsafe extern "C" fn runtime_pop_scope(context: usize) {
    let runtime = (context as *mut Runtime).as_mut().unwrap();
    runtime.pop_scope();
}
