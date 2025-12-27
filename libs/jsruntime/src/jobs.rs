use std::collections::VecDeque;

use rustc_hash::FxHashMap;

use crate::Runtime;
use crate::Value;
use crate::gc::Handle;
use crate::logger;
use crate::types::CallContext;
use crate::types::Coroutine;
use crate::types::Lambda;
use crate::types::Object;
use crate::types::Promise;
use crate::types::Status;

impl<X> Runtime<X> {
    /// Perform all jobs.
    pub fn process_jobs(&mut self) {
        while let Some(msg) = self.job_runner.next_msg() {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: Message) {
        logger::debug!(event = "handle_message", ?msg);
        match msg {
            Message::PromiseResolved {
                promise,
                ref result,
            } => self.process_promise(promise, result, &Value::None),
            Message::PromiseRejected { promise, ref error } => {
                self.process_promise(promise, &Value::None, error)
            }
        }
    }

    // promise

    pub fn register_promise(&mut self, coroutine: *mut Coroutine) -> Promise {
        logger::debug!(event = "register_promise", ?coroutine);
        self.job_runner.register_promise(coroutine)
    }

    pub fn process_promise(&mut self, promise: Handle<Object>, result: &Value, error: &Value) {
        // TODO(feat): `result` may hold a Promise object
        logger::debug!(event = "process_promise", ?promise, ?result, ?error);
        debug_assert!(self.is_promise_object(promise));
        let promise_id = promise.get_promise();
        debug_assert!(promise_id.is_valid());
        let coroutine = self.job_runner.get_coroutine(promise_id);
        match self.resume(coroutine, promise, result, error) {
            (Status::Normal, retv) => self.job_runner.resolve_promise(promise_id, retv),
            (Status::Exception, retv) => self.job_runner.reject_promise(promise_id, retv),
            (Status::Suspend, _) => (),
        }
    }

    fn resume(
        &mut self,
        coroutine: *mut Coroutine,
        promise: Handle<Object>,
        result: &Value,
        error: &Value,
    ) -> (Status, Value) {
        logger::debug!(event = "resume", ?coroutine, ?promise, ?result, ?error);
        let mut args = [promise.into(), result.clone(), error.clone()];
        let mut context = CallContext::new_for_promise(coroutine, &mut args);
        let mut retv = Value::None;
        // SAFETY: `coroutine` is always a non-null pointer to a `Coroutine`.
        let lambda = unsafe {
            debug_assert!(!coroutine.is_null());
            debug_assert!(!(*coroutine).closure.is_null());
            Lambda::from((*(*coroutine).closure).lambda)
        };
        let status = lambda(self, &mut context, &mut retv);
        (status, retv)
    }

    pub fn emit_promise_resolved(&mut self, promise: Handle<Object>, result: Value) {
        debug_assert!(self.is_promise_object(promise));
        match result {
            Value::Object(object) if self.is_promise_object(object) => {
                self.job_runner.await_promise(object, promise)
            }
            _ => self.job_runner.emit_promise_resolved(promise, result),
        }
    }

    pub fn emit_promise_rejected(&mut self, promise: Handle<Object>, error: Value) {
        debug_assert!(self.is_promise_object(promise));
        self.job_runner.emit_promise_rejected(promise, error);
    }
}

pub struct JobRunner {
    messages: VecDeque<Message>,
    promises: FxHashMap<Promise, PromiseDriver>,
    next_promise: u32,
}

impl JobRunner {
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
            promises: Default::default(),
            next_promise: 1, // 0 is invalid.
        }
    }

    // promises

    fn register_promise(&mut self, coroutine: *mut Coroutine) -> Promise {
        let promise = self.new_promise();
        self.promises.insert(promise, PromiseDriver::new(coroutine));
        promise
    }

    fn new_promise(&mut self) -> Promise {
        assert!(self.promises.len() < u32::MAX as usize);
        loop {
            let promise = self.next_promise.into();
            if self.next_promise == u32::MAX {
                self.next_promise = 1;
            } else {
                self.next_promise += 1;
            }
            if !self.promises.contains_key(&promise) {
                return promise;
            }
        }
        // never reach here
    }

    fn await_promise(&mut self, promise: Handle<Object>, awaiting: Handle<Object>) {
        logger::debug!(event = "await_promise", ?promise, ?awaiting);
        let promise_id = promise.get_promise();
        debug_assert!(promise_id.is_valid());
        let awaiting_id = awaiting.get_promise();
        debug_assert!(awaiting_id.is_valid());
        debug_assert!(self.promises.contains_key(&promise_id));
        debug_assert!(self.promises.contains_key(&awaiting_id));
        let driver = self.promises.get_mut(&promise_id).unwrap();
        debug_assert!(driver.awaiting.is_none());
        match driver.state {
            PromiseState::Pending => driver.awaiting = Some(awaiting),
            PromiseState::Resolved(ref result) => {
                let result = result.clone();
                self.emit_promise_resolved(awaiting, result);
                self.promises.remove(&promise_id);
            }
            PromiseState::Rejected(ref error) => {
                let error = error.clone();
                self.emit_promise_rejected(awaiting, error);
                self.promises.remove(&promise_id);
            }
        }
    }

    fn get_coroutine(&self, promise: Promise) -> *mut Coroutine {
        self.promises.get(&promise).unwrap().coroutine
    }

    fn emit_promise_resolved(&mut self, promise: Handle<Object>, result: Value) {
        logger::debug!(event = "emit_promise_resolved", ?promise, ?result);
        self.messages
            .push_back(Message::PromiseResolved { promise, result });
    }

    fn emit_promise_rejected(&mut self, promise: Handle<Object>, error: Value) {
        logger::debug!(event = "emit_promise_rejected", ?promise, ?error);
        self.messages
            .push_back(Message::PromiseRejected { promise, error });
    }

    fn next_msg(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    fn resolve_promise(&mut self, promise: Promise, result: Value) {
        logger::debug!(event = "resolve_promise", ?promise, ?result);
        let driver = self.promises.get_mut(&promise).unwrap();
        debug_assert!(matches!(driver.state, PromiseState::Pending));
        if let Some(awaiting) = driver.awaiting {
            self.promises.remove(&promise);
            self.emit_promise_resolved(awaiting, result);
        } else {
            driver.state = PromiseState::Resolved(result);
        }
    }

    fn reject_promise(&mut self, promise: Promise, error: Value) {
        logger::debug!(event = "reject_promise", ?promise, ?error);
        let driver = self.promises.get_mut(&promise).unwrap();
        debug_assert!(matches!(driver.state, PromiseState::Pending));
        if let Some(awaiting) = driver.awaiting {
            self.promises.remove(&promise);
            self.emit_promise_rejected(awaiting, error);
        } else {
            driver.state = PromiseState::Rejected(error);
        }
    }
}

// messages

#[derive(Debug)]
enum Message {
    PromiseResolved {
        promise: Handle<Object>,
        result: Value,
    },
    PromiseRejected {
        promise: Handle<Object>,
        error: Value,
    },
}

// promise

// TODO: should the coroutine be separated from the promise?
struct PromiseDriver {
    // TODO(issue#237): GcCellRef
    coroutine: *mut Coroutine,
    awaiting: Option<Handle<Object>>,
    state: PromiseState,
}

impl PromiseDriver {
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
