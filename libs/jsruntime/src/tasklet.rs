use std::collections::VecDeque;
use std::num::NonZeroU32;

use rustc_hash::FxHashMap;

use crate::llvmir::Coroutine;
use crate::llvmir::CoroutineStatus;
use crate::Value;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PromiseId(NonZeroU32);

impl From<u32> for PromiseId {
    fn from(value: u32) -> Self {
        Self(NonZeroU32::new(value).unwrap())
    }
}

impl From<PromiseId> for u32 {
    fn from(value: PromiseId) -> Self {
        value.0.get()
    }
}

pub struct System {
    messages: VecDeque<Message>,
    promises: FxHashMap<PromiseId, Promise>,
    next_promise_id: u32,
}

impl System {
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
            promises: Default::default(),
            next_promise_id: 1,
        }
    }

    pub fn run(&mut self, runtime: *mut std::ffi::c_void) {
        while let Some(msg) = self.messages.pop_front() {
            self.handle_message(runtime, msg);
        }
    }

    fn handle_message(&mut self, runtime: *mut std::ffi::c_void, msg: Message) {
        crate::logger::debug!(event = "handle_message", ?msg);
        match msg {
            Message::PromiseResolved {
                promise_id,
                ref result,
            } => {
                self.process_promise(runtime, promise_id, result, &Value::NONE);
            }
            Message::PromiseRejected {
                promise_id,
                ref error,
            } => {
                self.process_promise(runtime, promise_id, &Value::NONE, error);
            }
        }
    }

    // promises

    pub fn register_promise(&mut self, coroutine: *mut Coroutine) -> PromiseId {
        crate::logger::debug!(event = "register_promise", ?coroutine);
        let promise_id = PromiseId(NonZeroU32::new(self.next_promise_id).unwrap());
        self.promises.insert(promise_id, Promise::new(coroutine));
        self.next_promise_id += 1;
        promise_id
    }

    pub fn await_promise(&mut self, promise_id: PromiseId, awaiting: PromiseId) {
        crate::logger::debug!(event = "await_promise", ?promise_id, ?awaiting);
        debug_assert!(self.promises.contains_key(&promise_id));
        debug_assert!(self.promises.contains_key(&awaiting));
        let promise = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(promise.awaiting.is_none());
        match promise.state {
            PromiseState::Pending => promise.awaiting = Some(awaiting),
            PromiseState::Resolved(result) => {
                self.emit_promise_resolved(awaiting, result);
                self.promises.remove(&promise_id);
            }
            PromiseState::Rejected(error) => {
                self.emit_promise_rejected(awaiting, error);
                self.promises.remove(&promise_id);
            }
        }
    }

    pub fn emit_promise_resolved(&mut self, promise_id: PromiseId, result: Value) {
        crate::logger::debug!(event = "emit_promise_resolved", ?promise_id, ?result);
        self.messages
            .push_back(Message::PromiseResolved { promise_id, result });
    }

    pub fn emit_promise_rejected(&mut self, promise_id: PromiseId, error: Value) {
        crate::logger::debug!(event = "emit_promise_rejected", ?promise_id, ?error);
        self.messages
            .push_back(Message::PromiseRejected { promise_id, error });
    }

    pub fn process_promise(
        &mut self,
        gctx: *mut std::ffi::c_void,
        promise_id: PromiseId,
        result: &Value,
        error: &Value,
    ) {
        crate::logger::debug!(event = "process_promise", ?promise_id, ?result, ?error);
        let coroutine = self.promises.get(&promise_id).unwrap().coroutine;
        match Coroutine::resume(gctx, coroutine, promise_id, result, error) {
            CoroutineStatus::Done(result) => self.resolve_promise(promise_id, result),
            CoroutineStatus::Error(error) => self.reject_promise(promise_id, error),
            CoroutineStatus::Suspend => (),
        }
    }

    fn resolve_promise(&mut self, promise_id: PromiseId, result: Value) {
        crate::logger::debug!(event = "resolve_promise", ?promise_id, ?result);
        let promise = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(matches!(promise.state, PromiseState::Pending));
        if let Some(awaiting) = promise.awaiting {
            self.promises.remove(&promise_id);
            self.emit_promise_resolved(awaiting, result);
        } else {
            promise.state = PromiseState::Resolved(result);
        }
    }

    fn reject_promise(&mut self, promise_id: PromiseId, error: Value) {
        crate::logger::debug!(event = "reject_promise", ?promise_id, ?error);
        let promise = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(matches!(promise.state, PromiseState::Pending));
        if let Some(awaiting) = promise.awaiting {
            self.promises.remove(&promise_id);
            self.emit_promise_rejected(awaiting, error);
        } else {
            promise.state = PromiseState::Rejected(error);
        }
    }
}

// messages

#[derive(Debug)]
enum Message {
    PromiseResolved {
        promise_id: PromiseId,
        result: Value,
    },
    PromiseRejected {
        promise_id: PromiseId,
        error: Value,
    },
}

// promise

// TODO: should the coroutine be separated from the promise?
struct Promise {
    // TODO(issue#237): GcCellRef
    coroutine: *mut Coroutine,
    awaiting: Option<PromiseId>,
    state: PromiseState,
}

impl Promise {
    fn new(coroutine: *mut Coroutine) -> Self {
        Self {
            coroutine,
            awaiting: None,
            state: PromiseState::Pending,
        }
    }
}

enum PromiseState {
    Pending,
    Resolved(Value),
    Rejected(Value),
}
