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
    imp: *mut bridge::Runtime,
    symbol_table: SymbolTable,
    global_scope: Scope,
    next_func_id: usize,
}

impl Runtime {
    pub fn initialize() {
        unsafe {
            bridge::runtime_initialize();
        }
    }

    pub fn compile_script(&mut self, source: &str) -> bool {
        jsparser::for_script(source, Compiler::new(self))
            .parse()
            .is_ok()
    }

    pub fn dump_module(&self) {
        unsafe {
            bridge::runtime_dump_module(self.imp);
        }
    }

    pub fn eval(&self) {
        unsafe {
            bridge::runtime_eval(self.imp);
        }
    }

    fn new() -> Self {
        Self {
            imp: unsafe { bridge::runtime_new() },
            symbol_table: SymbolTable::with_builtin_symbols(),
            global_scope: Scope::new(),
            next_func_id: 1,
        }
    }

    fn with_host(host: bridge::Host) -> Self {
        let runtime = Self::new();
        unsafe { bridge::runtime_register_host(runtime.imp, &host) }
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
            bridge::runtime_delete(self.imp);
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

    fn eval(source: String, host: bridge::Host) {
        Runtime::initialize();
        let mut runtime = Runtime::with_host(host);
        let _ = runtime.compile_script(&source);
        runtime.eval();
    }
}
