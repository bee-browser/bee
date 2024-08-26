mod bridge;
mod compiler;
mod control_flow;
mod executor;

pub use bridge::runtime_bridge;
pub use bridge::Locator;
pub use bridge::ReturnValue;
pub use bridge::Status;
pub use bridge::Value;
pub use compiler::CompileError;
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
    pub fn print(&self, stderr: bool) {
        unsafe {
            bridge::module_peer_print(self.peer, stderr);
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
