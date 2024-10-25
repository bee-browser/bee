use std::ffi::CStr;

use crate::FunctionId;
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

    pub fn register_host_function(&self, func_id: FunctionId, func: HostLambda) {
        unsafe {
            bridge::executor_peer_register_host_function(self.peer, func_id.into(), Some(func));
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

    pub fn get_native_function(&self, func_id: FunctionId) -> bridge::Lambda {
        unsafe { bridge::executor_peer_get_native_function(self.peer, func_id.into()) }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        unsafe {
            bridge::executor_peer_delete(self.peer);
        }
    }
}
