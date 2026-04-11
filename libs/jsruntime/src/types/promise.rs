use jsgc::HandleMut;
use jsgc::Trace;

use crate::types::Coroutine;
use crate::types::Object;
use crate::types::Value;

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
        // TODO(fix): the following assertion fails in test262.
        debug_assert!(matches!(self.state, PromiseState::Pending));
        self.state = PromiseState::Resolved(result.clone());
        self.awaiting.take()
    }

    pub fn reject(&mut self, error: &Value) -> Option<HandleMut<Object>> {
        // TODO(fix): the following assertion fails in test262.
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
        write!(f, "Promise")?;
        // TODO
        Ok(())
    }
}

impl Trace for Promise {
    fn trace(&self, visits: &mut jsgc::VisitList) {
        visits.push(self.coroutine.as_addr());
        if let Some(awaiting) = self.awaiting {
            visits.push(awaiting.as_addr());
        }
        match self.state {
            PromiseState::Resolved(Value::String(string)) => visits.push(string.as_addr()),
            PromiseState::Resolved(Value::Object(object)) => visits.push(object.as_addr()),
            PromiseState::Rejected(Value::String(string)) => visits.push(string.as_addr()),
            PromiseState::Rejected(Value::Object(object)) => visits.push(object.as_addr()),
            _ => (),
        }
    }
}

enum PromiseState {
    Pending,
    Resolved(Value),
    Rejected(Value),
}
