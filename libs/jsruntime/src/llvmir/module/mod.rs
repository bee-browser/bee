mod bridge;

use base::macros::delegate_all;

use bridge::ModuleBridge;

// `ModulePeer` is used in other bridge modules.
pub use bridge::ModulePeer;

pub struct Module(ModuleBridge);

delegate_all! {Module => ModuleBridge}

impl Module {
    pub fn new(peer: ModulePeer) -> Self {
        Self(ModuleBridge::new(peer))
    }
}
