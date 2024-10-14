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

impl Into<u32> for PromiseId {
    fn into(self) -> u32 {
        self.0.get()
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
            Message::PromiseResolved { promise_id, result } => {
                self.process_promise(runtime, promise_id, result, Value::NONE);
            }
            Message::PromiseRejected { promise_id, error } => {
                self.process_promise(runtime, promise_id, Value::NONE, error);
            }
        }
    }

    // promises

    pub fn register_promise(&mut self, coroutine: *mut Coroutine) -> PromiseId {
        let promise_id = PromiseId(NonZeroU32::new(self.next_promise_id).unwrap());
        self.promises.insert(promise_id, Promise::new(coroutine));
        self.next_promise_id += 1;
        promise_id
    }

    pub fn emit_promise_resolved(&mut self, promise_id: PromiseId, result: Value) {
        crate::logger::debug!(event = "emit_promise_resolved", ?promise_id, ?result);
        self.messages.push_back(Message::PromiseResolved { promise_id, result });
    }

    pub fn emit_promise_rejected(&mut self, promise_id: PromiseId, error: Value) {
        self.messages.push_back(Message::PromiseRejected { promise_id, error });
    }

    pub fn process_promise(&mut self, runtime: *mut std::ffi::c_void, promise_id: PromiseId, result: Value, error: Value) {
        crate::logger::debug!(event = "process_promise", ?promise_id, ?result, ?error);
        if let Some(promise) = self.promises.get(&promise_id) {
            match Coroutine::resume(runtime, promise.coroutine, result, error) {
                CoroutineStatus::Done(result) => {
                    if let Some(promise_id) = promise.awaiting {
                        self.emit_promise_resolved(promise_id, result);
                    }
                    self.promises.remove(&promise_id);
                }
                CoroutineStatus::Error(error) => {
                    if let Some(promise_id) = promise.awaiting {
                        self.emit_promise_rejected(promise_id, error);
                    } else {
                        crate::logger::warn!(?promise_id, "unhandled promise");
                    }
                    self.promises.remove(&promise_id);
                }
                CoroutineStatus::Suspend => (),
            }
        } else {
            crate::logger::warn!(?promise_id, "promise not found");
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
    }
}

// promise

// TODO: should the coroutine be separated from the promise?
struct Promise {
    // TODO(issue#237): GcCellRef
    coroutine: *mut Coroutine,
    awaiting: Option<PromiseId>,
}

impl Promise {
    fn new(coroutine: *mut Coroutine) -> Self {
        Self {
            coroutine,
            awaiting: None,
        }
    }
}
