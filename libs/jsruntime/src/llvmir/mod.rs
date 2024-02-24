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
    Undefined,
    Number(f64),
    Function(String),
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
    fn next_func_name(&mut self) -> String {
        let id = self.next_func_id;
        self.next_func_id += 1;
        format!("fn{id}")
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
                runtime_get: Some(runtime_get),
                runtime_set: Some(runtime_set),
                runtime_set_undefined: Some(runtime_set_undefined),
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

    unsafe extern "C" fn runtime_get(context: usize, symbol_id: u32) -> f64 {
        use super::Symbol;
        use super::Value;

        let runtime = (context as *mut super::Runtime).as_mut().unwrap();
        let symbol = Symbol::from(symbol_id);

        let value = runtime.global_scope.bindings.get(&symbol);
        match value {
            Some(Value::Number(value)) => *value,
            _ => panic!(),
        }
    }

    unsafe extern "C" fn runtime_set(context: usize, symbol_id: u32, value: f64) {
        use super::Symbol;
        use super::Value;

        let runtime = (context as *mut super::Runtime).as_mut().unwrap();
        let symbol = Symbol::from(symbol_id);

        runtime
            .global_scope
            .bindings
            .insert(symbol, Value::Number(value));
    }

    unsafe extern "C" fn runtime_set_undefined(context: usize, symbol_id: u32) {
        use super::Symbol;
        use super::Value;

        let runtime = (context as *mut super::Runtime).as_mut().unwrap();
        let symbol = Symbol::from(symbol_id);

        runtime
            .global_scope
            .bindings
            .insert(symbol, Value::Undefined);
    }

    unsafe extern "C" fn runtime_call(userdata: usize, symbol_id: u32) -> f64 {
        use super::Symbol;
        use super::Value;

        let runtime = (userdata as *const super::Runtime).as_ref().unwrap();
        let symbol = Symbol::from(symbol_id);

        let value = runtime.global_scope.bindings.get(&symbol);
        let (name, len) = match value {
            Some(Value::Function(name)) => (name.as_ptr() as *const i8, name.len()),
            _ => panic!(),
        };

        let mut return_value = 0.0;
        runtime_peer_call(runtime.peer, name, len, &mut return_value);
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

    #[test]
    fn test_eval_const_declaration() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A);
        }

        eval(
            format!("const a={A},b={B}; a;"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_let_declaration() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A);
        }

        eval(
            format!("let a,b={B}; a={A}; a;"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_arithmetic_operations_with_variables() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;
        const C: f64 = 5.6;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A + B * C);
        }

        eval(
            format!("const a={A},b={B},c={C}; a+b*c;"),
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_conditional_expression() {
        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, 2.);
        }

        eval(
            "1 > 0 ? 2 : 3",
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_nested_conditional_expression() {
        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, 2.);
        }

        eval(
            "1 > 0 ? 1 > 0 ? 2 : 3 : 1 > 0 ? 4 : 5",
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_if_statement() {
        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, 2.);
        }

        eval(
            "let a = 1; if (1) { a = 2; } a;",
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_eval_if_else_statement() {
        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, 3.);
        }

        eval(
            "let a = 1; if (0) { a = 2; } else { a = 3; } a;",
            bridge::Host {
                print_f64: Some(validate),
                ..Default::default()
            },
        );
    }

    fn eval<T: AsRef<str>>(source: T, host: bridge::Host) {
        Runtime::initialize();
        let mut runtime = Runtime::with_host(host);
        let _ = runtime.compile_script(source.as_ref());
        runtime.eval();
    }
}
