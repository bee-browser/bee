mod compiler;

use indexmap::IndexMap;

use jsparser::Symbol;
use jsparser::SymbolTable;

use compiler::Compiler;

struct Scope {
    bindings: IndexMap<Symbol, Value>,
}

impl Scope {
    const INITIAL_CAPACITY: usize = 32;

    fn new() -> Self {
        Self {
            bindings: IndexMap::with_capacity(Self::INITIAL_CAPACITY),
        }
    }
}

enum Value {
    Function(FuncId),
}

#[derive(Clone, Copy)]
struct FuncId(usize);

pub struct Runtime {
    peer: *mut bridge::Runtime,
    symbol_table: SymbolTable,
    global_scope: Scope,
    next_func_id: usize,
}

impl Runtime {
    pub fn initialize() {
        unsafe {
            bridge::runtime_peer_initialize();
        }
    }

    pub fn compile_script(&mut self, source: &str) -> bool {
        jsparser::for_script(source, Compiler::new(self))
            .parse()
            .is_ok()
    }

    pub fn dump_module(&self) {
        unsafe {
            bridge::runtime_peer_dump_module(self.peer);
        }
    }

    pub fn eval(&self) {
        unsafe {
            bridge::runtime_peer_eval(self.peer, self as *const Runtime as usize);
        }
    }

    fn new() -> Self {
        Self {
            peer: unsafe { bridge::runtime_peer_new() },
            symbol_table: SymbolTable::with_builtin_symbols(),
            global_scope: Scope::new(),
            next_func_id: 1,
        }
    }

    fn with_host(host: bridge::Host) -> Self {
        let runtime = Self::new();
        unsafe { bridge::runtime_peer_register_host(runtime.peer, &host) }
        runtime
    }
}

impl Runtime {
    fn next_func_id(&mut self) -> FuncId {
        let id = self.next_func_id;
        self.next_func_id += 1;
        FuncId(id)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::with_host(bridge::Host::default())
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            bridge::runtime_peer_delete(self.peer);
        }
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod bridge {
    include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

    impl Default for Host {
        fn default() -> Self {
            Self {
                print_bool: Some(print_bool),
                print_f64: Some(print_f64),
                print_str: Some(print_str),
                runtime_call: Some(runtime_call),
            }
        }
    }

    unsafe extern "C" fn print_bool(value: bool) {
        println!("{value}");
    }

    unsafe extern "C" fn print_f64(value: f64) {
        println!("{value}");
    }

    unsafe extern "C" fn print_str(value: *const std::os::raw::c_char) {
        // std::ffi::CStr::from_ptr(value).to_str() is safer but slower than the following code.
        let value = std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(value).to_bytes());
        println!("{value}");
    }

    unsafe extern "C" fn runtime_call(userdata: usize, symbol_id: u32) -> f64 {
        use super::Symbol;
        use super::Value;

        let runtime = (userdata as *const super::Runtime).as_ref().unwrap();
        let symbol = Symbol::from(symbol_id);

        let value = runtime.global_scope.bindings.get(&symbol).unwrap();
        let func_name = match value {
            Value::Function(func_id) => format!("fn{}", func_id.0),
        };

        let name = func_name.as_ptr() as *const i8;
        let mut return_value = 0.0;
        runtime_peer_call(runtime.peer, name, func_name.len(), &mut return_value);
        return_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        const A: f64 = 1.0;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A);
        }

        eval(
            format!("{A}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_addition_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A + B);
        }

        eval(
            format!("{A} + {B}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_subtraction_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A - B);
        }

        eval(
            format!("{A} - {B}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_multiplication_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A * B);
        }

        eval(
            format!("{A} * {B}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_division_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A / B);
        }

        eval(
            format!("{A} / {B}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_remainder_expression() {
        const A: f64 = 1.0;
        const B: f64 = 3.0;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A % B);
        }

        eval(
            format!("{A} % {B}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_group_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;
        const C: f64 = 5.6;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A * (B + C));
        }

        eval(
            format!("{A} * ({B} + {C})"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_call_with_no_argument() {
        const A: f64 = 1.2;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A);
        }

        eval(
            format!("function a() {{ return {A}; }} a();"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_call_with_no_argument_hoistable_declaration() {
        const A: f64 = 1.2;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A);
        }

        eval(
            format!("a(); function a() {{ return {A}; }}"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    fn eval(source: String, host: bridge::Host) {
        Runtime::initialize();
        let mut runtime = Runtime::with_host(host);
        let _ = runtime.compile_script(&source);
        runtime.eval();
    }
}
