use std::ffi::c_void;

pub struct ModuleBridge(ModulePeer);

impl ModuleBridge {
    pub fn print(&self, stderr: bool) {
        unsafe {
            module_peer_print(self.0, stderr);
        }
    }

    pub fn new(peer: ModulePeer) -> Self {
        Self(peer)
    }

    pub fn peer(&self) -> ModulePeer {
        self.0
    }
}

impl Drop for ModuleBridge {
    fn drop(&mut self) {
        unsafe {
            module_peer_delete(self.0);
        }
    }
}

// `ModulePeer` is used in other bridge modules.
pub type ModulePeer = *mut c_void;

#[link(name = "backend-llvm")]
unsafe extern "C" {
    fn module_peer_delete(peer: ModulePeer);
    fn module_peer_print(peer: ModulePeer, stderr: bool);
}
