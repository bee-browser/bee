use std::ffi::CStr;
use std::ffi::CString;

use super::bridge;
use super::Module;

use crate::Runtime;
use crate::Value;

pub struct Executor {
    peer: *mut bridge::Executor,
}

type HostFunc = unsafe extern "C" fn(
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
    u32,
    *mut std::ffi::c_void,
) -> bridge::Value;

// Took from https://stackoverflow.com/questions/32270030.
fn wrap<F>(_func: F) -> HostFunc
where
    F: Fn(&mut Runtime, &[Value]) -> Value,
{
    assert_eq!(std::mem::size_of::<F>(), 0, "Function must have zero size");
    wrapper::<F>
}

unsafe extern "C" fn wrapper<F>(
    exec_context: *mut std::ffi::c_void,
    outer_scope: *mut std::ffi::c_void,
    argc: u32,
    argv: *mut std::ffi::c_void,
) -> bridge::Value
where
    F: Fn(&mut Runtime, &[Value]) -> Value,
{
    // TODO: re-implement without MaybeUninit.
    #[allow(clippy::uninit_assumed_init)]
    let host_fn = std::mem::MaybeUninit::<F>::uninit().assume_init();
    let runtime = &mut *(exec_context as *mut Runtime);
    let _ = outer_scope;
    let args = std::slice::from_raw_parts(argv as *const bridge::Value, argc as usize);
    // TODO: use c-type
    let args: Vec<crate::Value> = args.iter().map(|value| crate::Value::load(value)).collect();
    host_fn(runtime, &args).save()
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

    pub fn register_host_function<F>(&self, name: &str, func: F)
    where
        F: Fn(&mut Runtime, &[Value]) -> Value,
    {
        let name = CString::new(name).unwrap();
        unsafe {
            bridge::executor_peer_register_host_function(
                self.peer,
                name.as_ptr(),
                Some(wrap(func)),
            );
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

    pub fn get_native_func(&self, name: &CStr) -> bridge::NativeFuncPtr {
        unsafe { bridge::executor_peer_get_native_func(self.peer, name.as_ptr()) }
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
