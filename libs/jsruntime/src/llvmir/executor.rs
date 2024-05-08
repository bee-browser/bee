use std::ffi::CStr;

use super::bridge;
use super::Module;

pub struct Executor {
    peer: *mut bridge::Executor,
}

impl Executor {
    pub fn new() -> Self {
        let peer = unsafe {
            let peer = bridge::executor_peer_new();
            bridge::executor_peer_register_runtime(peer, &bridge::Runtime::default());
            peer
        };
        Self { peer }
    }

    pub fn register_module(&self, module: Module) {
        unsafe {
            bridge::executor_peer_register_module(self.peer, module.peer);
        }
    }

    pub fn get_data_layout(&self) -> &CStr {
        unsafe { CStr::from_ptr(bridge::executor_peer_get_data_layout(self.peer)) }
    }

    pub fn get_target_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(bridge::executor_peer_get_target_triple(self.peer)) }
    }

    pub fn get_func(&self, name: &CStr) -> bridge::FuncPtr {
        unsafe { bridge::executor_peer_get_func(self.peer, name.as_ptr()) }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        unsafe {
            bridge::executor_peer_delete(self.peer);
        }
    }
}
