mod compiler;

use jsparser::Parser;

pub struct Runtime(*mut bridge::Runtime);

impl Runtime {
    pub fn initialize() {
        unsafe {
            bridge::runtime_initialize();
        }
    }

    pub fn new() -> Self {
        Default::default()
    }

    pub fn compile_script(&self, source: &str) -> bool {
        let session = compiler::Session::new(self);
        Parser::for_script(source, session.compiler())
            .parse()
            .is_ok()
    }

    pub fn eval(&self) {
        unsafe {
            bridge::runtime_eval(self.0);
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self(unsafe { bridge::runtime_new() })
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
}
