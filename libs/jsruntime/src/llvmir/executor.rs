use std::ffi::CStr;
use std::ffi::CString;

use crate::HostLambda;

use super::bridge;
use super::Module;

pub struct Executor {
    peer: *mut bridge::Executor,
}

impl Executor {
    pub fn with_runtime_bridge(runtime: &bridge::Runtime) -> Self {
        let peer = unsafe {
            let peer = bridge::executor_peer_new();
            bridge::executor_peer_register_runtime(peer, runtime);
            peer
        };
        Self { peer }
    }

    pub fn register_host_function(&self, name: &str, func: HostLambda) {
        let name = CString::new(name).unwrap();
        unsafe {
            bridge::executor_peer_register_host_function(self.peer, name.as_ptr(), Some(func));
        }
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

    pub fn get_native_function(&self, name: &CStr) -> bridge::Lambda {
        unsafe { bridge::executor_peer_get_native_function(self.peer, name.as_ptr()) }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        unsafe {
            bridge::executor_peer_delete(self.peer);
        }
    }
}
