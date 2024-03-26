pub mod bridge;
mod compiler;
mod executor;

use jsparser::Symbol;

use super::logger;
use super::FunctionId;
use super::Runtime;
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
