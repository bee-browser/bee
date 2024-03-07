use std::ffi::CStr;

use super::bridge;
use super::Module;

pub struct Executor {
    peer: *mut bridge::Executor,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            peer: unsafe { bridge::executor_peer_new() },
        }
    }

    pub fn with_host(host: bridge::Host) -> Self {
        let runtime = Self::new();
        unsafe { bridge::executor_peer_register_host(runtime.peer, &host) }
        runtime
    }

    pub fn register_module(&self, module: Module) {
        unsafe {
            bridge::executor_peer_register_module(self.peer, module.peer);
        }
    }

    pub fn get_main(&self) -> bridge::MainFn {
        unsafe { bridge::executor_peer_get_main(self.peer) }
    }

    pub fn get_func(&self, name: &CStr) -> bridge::FuncFn {
        unsafe { bridge::executor_peer_get_func(self.peer, name.as_ptr()) }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::with_host(bridge::Host::default())
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        unsafe {
            bridge::executor_peer_delete(self.peer);
        }
    }
}
