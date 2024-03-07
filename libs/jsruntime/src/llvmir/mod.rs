mod compiler;
pub mod bridge;
mod executor;

use jsparser::Symbol;

use super::logger;
use super::Runtime;
pub use compiler::Compiler;
pub use executor::Executor;

pub fn initialize() {
    unsafe {
        bridge::llvmir_initialize();
    }
}

pub struct Module {
    peer: *mut bridge::Module,
}

impl Module {
    pub fn dump(&self) {
        unsafe {
            bridge::module_peer_dump(self.peer);
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            bridge::module_peer_delete(self.peer);
        }
    }
}
