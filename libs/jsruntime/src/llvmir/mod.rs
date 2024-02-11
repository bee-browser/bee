mod compiler;

pub struct Runtime(*mut bridge::Runtime);

impl Runtime {
    pub fn initialize() {
        unsafe {
            bridge::runtime_initialize();
        }
    }

    pub fn compile_script(&self, source: &str) -> bool {
        let session = compiler::Session::new(self);
        jsparser::for_script(source, session.compiler())
            .parse()
            .is_ok()
    }

    pub fn dump_module(&self) {
        unsafe {
            bridge::runtime_dump_module(self.0);
        }
    }

    pub fn eval(&self) {
        unsafe {
            bridge::runtime_eval(self.0);
        }
    }

    fn new() -> Self {
        Self(unsafe { bridge::runtime_new() })
    }

    fn with_host(host: bridge::Host) -> Self {
        let runtime = Self::new();
        unsafe { bridge::runtime_register_host(runtime.0, &host) }
        runtime
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
            bridge::runtime_delete(self.0);
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
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A}"), host);
    }

    #[test]
    fn test_eval_addition_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A + B);
        }
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A} + {B}"), host);
    }

    #[test]
    fn test_eval_subtraction_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A - B);
        }
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A} - {B}"), host);
    }

    #[test]
    fn test_eval_multiplication_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A * B);
        }
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A} * {B}"), host);
    }

    #[test]
    fn test_eval_division_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A / B);
        }
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A} / {B}"), host);
    }

    #[test]
    fn test_eval_remainder_expression() {
        const A: f64 = 1.0;
        const B: f64 = 3.0;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A % B);
        }
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A} % {B}"), host);
    }

    #[test]
    fn test_eval_group_expression() {
        const A: f64 = 1.2;
        const B: f64 = 3.4;
        const C: f64 = 5.6;

        unsafe extern "C" fn validate(value: f64) {
            assert_eq!(value, A * (B + C));
        }
        let mut host = bridge::Host::default();
        host.print_f64 = Some(validate);

        eval(&format!("{A} * ({B} + {C})"), host);
    }

    fn eval(source: &str, host: bridge::Host) {
        Runtime::initialize();
        let runtime = Runtime::with_host(host);
        let _ = runtime.compile_script(source);
        runtime.eval();
    }
}
