mod bridge;

use base::macros::delegate_all;

use bridge::ModuleBridge;

use crate::LambdaId;

// `ModulePeer` is used in other bridge modules.
pub use bridge::ModulePeer;

pub struct Module {
    bridge: ModuleBridge,
    entry_lambda_id: LambdaId,
}

delegate_all! {Module => bridge: ModuleBridge}

impl Module {
    pub fn new(peer: ModulePeer, entry_lambda_id: LambdaId) -> Self {
        Self {
            bridge: ModuleBridge::new(peer),
            entry_lambda_id,
        }
    }

    pub fn entry_lambda_id(&self) -> LambdaId {
        self.entry_lambda_id
    }
}
