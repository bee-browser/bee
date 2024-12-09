use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;

use crate::llvmir::module::Module;
use crate::llvmir::module::ModulePeer;
use crate::llvmir::RuntimeFunctions;
use crate::types::Lambda;
use crate::FunctionId;

pub struct ExecutorBridge(ExecutorPeer);

impl ExecutorBridge {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        Self(unsafe {
            let peer = executor_peer_new();
            executor_peer_register_runtime_functions(peer, functions);
            peer
        })
    }

    pub fn register_host_function(&self, func_id: FunctionId, lambda: Lambda) {
        unsafe {
            executor_peer_register_host_function(self.0, func_id.into(), lambda);
        }
    }

    pub fn register_module(&self, module: Module) {
        unsafe {
            executor_peer_register_module(self.0, module.peer());
        }
    }

    pub fn get_data_layout(&self) -> &CStr {
        unsafe { CStr::from_ptr(executor_peer_get_data_layout(self.0)) }
    }

    pub fn get_target_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(executor_peer_get_target_triple(self.0)) }
    }

    pub fn get_native_function(&self, func_id: FunctionId) -> Option<Lambda> {
        unsafe {
            std::mem::transmute::<Lambda, Option<Lambda>>(executor_peer_get_native_function(
                self.0,
                func_id.into(),
            ))
        }
    }
}

impl Drop for ExecutorBridge {
    fn drop(&mut self) {
        unsafe {
            executor_peer_delete(self.0);
        }
    }
}

type ExecutorPeer = *mut c_void;

#[link(name = "llvmir")]
extern "C" {
    fn executor_peer_new() -> ExecutorPeer;
    fn executor_peer_delete(peer: ExecutorPeer);
    fn executor_peer_register_runtime_functions(peer: ExecutorPeer, functions: &RuntimeFunctions);
    fn executor_peer_register_host_function(peer: ExecutorPeer, func_id: u32, func: Lambda);
    fn executor_peer_register_module(peer: ExecutorPeer, module: ModulePeer);
    fn executor_peer_get_data_layout(peer: ExecutorPeer) -> *const c_char;
    fn executor_peer_get_target_triple(peer: ExecutorPeer) -> *const c_char;
    fn executor_peer_get_native_function(peer: ExecutorPeer, func_id: u32) -> Lambda;
}
