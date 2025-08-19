use std::collections::VecDeque;

use rustc_hash::FxHashMap;

use crate::Runtime;
use crate::Value;
use crate::logger;
use crate::types::Coroutine;
use crate::types::Lambda;
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

    pub fn await_promise(&mut self, promise: Promise, awaiting: Promise) {
        logger::debug!(event = "await_promise", ?promise, ?awaiting);
        self.job_runner.await_promise(promise, awaiting);
    }

    pub fn process_promise(&mut self, promise: Promise, result: &Value, error: &Value) {
        logger::debug!(event = "process_promise", ?promise, ?result, ?error);
        let coroutine = self.job_runner.get_coroutine(promise);
        match self.resume(coroutine, promise, result, error) {
            (Status::Normal, retv) => self.job_runner.resolve_promise(promise, retv),
            (Status::Exception, retv) => self.job_runner.reject_promise(promise, retv),
            (Status::Suspend, _) => (),
        }
    }

    fn resume(
        &mut self,
        coroutine: *mut Coroutine,
        promise: Promise,
        result: &Value,
        error: &Value,
    ) -> (Status, Value) {
        logger::debug!(event = "resume", ?coroutine, ?promise, ?result, ?error);
        let mut this = Value::Undefined;
        let mut args = [promise.into(), result.clone(), error.clone()];
        let mut retv = Value::None;
        let status = unsafe {
            let lambda = Lambda::from((*(*coroutine).closure).lambda);
            lambda(
                self,
                coroutine as *mut std::ffi::c_void,
                &mut this as *mut Value,
                args.len() as u16,
                args.as_mut_ptr(),
                &mut retv,
            )
        };
        (status, retv)
    }

    pub fn emit_promise_resolved(&mut self, promise: Promise, result: Value) {
        self.job_runner.emit_promise_resolved(promise, result);
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
            next_promise: 0,
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
            if !self.promises.contains_key(&promise) {
                return promise;
            }
            self.next_promise = self.next_promise.wrapping_add(1);
        }
        // never reach here
    }

    fn await_promise(&mut self, promise: Promise, awaiting: Promise) {
        debug_assert!(self.promises.contains_key(&promise));
        debug_assert!(self.promises.contains_key(&awaiting));
        let driver = self.promises.get_mut(&promise).unwrap();
        debug_assert!(driver.awaiting.is_none());
        match driver.state {
            PromiseState::Pending => driver.awaiting = Some(awaiting),
            PromiseState::Resolved(ref result) => {
                let result = result.clone();
                self.emit_promise_resolved(awaiting, result);
                self.promises.remove(&promise);
            }
            PromiseState::Rejected(ref error) => {
                let error = error.clone();
                self.emit_promise_rejected(awaiting, error);
                self.promises.remove(&promise);
            }
        }
    }

    fn get_coroutine(&self, promise: Promise) -> *mut Coroutine {
        self.promises.get(&promise).unwrap().coroutine
    }

    fn emit_promise_resolved(&mut self, promise: Promise, result: Value) {
        logger::debug!(event = "emit_promise_resolved", ?promise, ?result);
        self.messages
            .push_back(Message::PromiseResolved { promise, result });
    }

    fn emit_promise_rejected(&mut self, promise: Promise, error: Value) {
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
    PromiseResolved { promise: Promise, result: Value },
    PromiseRejected { promise: Promise, error: Value },
}

// promise

// TODO: should the coroutine be separated from the promise?
struct PromiseDriver {
    // TODO(issue#237): GcCellRef
    coroutine: *mut Coroutine,
    awaiting: Option<Promise>,
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
