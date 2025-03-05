mod bridge;

use base::macros::delegate_all;

use bridge::ExecutorBridge;

use crate::backend::bridge::RuntimeFunctions;

pub struct Executor(ExecutorBridge);

delegate_all! { Executor => ExecutorBridge }

impl Executor {
    pub fn new(functions: &RuntimeFunctions) -> Self {
        Self(ExecutorBridge::new(functions))
    }
}
