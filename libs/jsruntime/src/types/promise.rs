use jsgc::HandleMut;
use jsgc::Trace;

use crate::types::Coroutine;
use crate::types::Object;
use crate::types::Value;

#[derive(jsgc_derive::Trace)]
pub struct Promise {
    coroutine: HandleMut<Coroutine>,
    awaiting: Option<HandleMut<Object>>,
    state: PromiseState,
}

impl Promise {
    pub fn new(coroutine: HandleMut<Coroutine>) -> Self {
        Self {
            coroutine,
            awaiting: None,
            state: PromiseState::Pending,
        }
    }

    pub fn coroutine(&self) -> HandleMut<Coroutine> {
        self.coroutine
    }

    pub fn resolve(&mut self, result: &Value) -> Option<HandleMut<Object>> {
        debug_assert!(matches!(self.state, PromiseState::Pending));
        self.state = PromiseState::Resolved(result.clone());
        self.awaiting.take()
    }

    pub fn reject(&mut self, error: &Value) -> Option<HandleMut<Object>> {
        debug_assert!(matches!(self.state, PromiseState::Pending));
        self.state = PromiseState::Rejected(error.clone());
        self.awaiting.take()
    }

    pub fn do_await(&mut self, awaiting: HandleMut<Object>) -> Option<Result<Value, Value>> {
        match self.state {
            PromiseState::Pending => {
                self.awaiting = Some(awaiting);
                None
            }
            PromiseState::Resolved(ref result) => Some(Ok(result.clone())),
            PromiseState::Rejected(ref error) => Some(Err(error.clone())),
        }
    }
}

impl std::fmt::Debug for Promise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Promise({:?})", self.state)
    }
}

#[derive(Debug)]
enum PromiseState {
    Pending,
    Resolved(Value),
    Rejected(Value),
}

// TODO(jsgc-derive): derive(Trace)
impl Trace for PromiseState {
    #[inline]
    fn trace(&self, visits: &mut jsgc::VisitList) {
        match self {
            Self::Resolved(v) | Self::Rejected(v) => v.trace(visits),
            _ => (),
        }
    }
}
