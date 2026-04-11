use std::collections::VecDeque;

use jsgc::HandleMut;

use crate::Runtime;
use crate::Value;
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
            Message::PromiseResolved { object, ref result } => {
                self.process_promise(object, result, &Value::None)
            }
            Message::PromiseRejected { object, ref error } => {
                self.process_promise(object, &Value::None, error)
            }
        }
    }

    // promise

    pub fn process_promise(&mut self, object: HandleMut<Object>, result: &Value, error: &Value) {
        logger::debug!(event = "process_promise", ?object, ?result, ?error);
        debug_assert!(self.is_promise_object(object));
        let promise = object.promise();
        let coroutine = promise.coroutine();
        match self.resume(coroutine, object, result, error) {
            (Status::Normal, result) => self.job_runner.resolve_promise(promise, result),
            (Status::Exception, error) => self.job_runner.reject_promise(promise, error),
            (Status::Suspend, _) => (),
        }
    }

    fn resume(
        &mut self,
        coroutine: HandleMut<Coroutine>,
        object: HandleMut<Object>,
        result: &Value,
        error: &Value,
    ) -> (Status, Value) {
        logger::debug!(event = "resume", ?coroutine, ?object, ?result, ?error);
        let mut args = [object.into(), result.clone(), error.clone()];
        let mut context = CallContext::new_for_promise(coroutine, &mut args);
        let mut retv = Value::None;
        let lambda = Lambda::from(coroutine.closure.lambda);
        let status = lambda(self, &mut context, &mut retv);
        (status, retv)
    }

    pub fn emit_promise_resolved(&mut self, object: HandleMut<Object>, result: Value) {
        debug_assert!(self.is_promise_object(object));
        match result {
            Value::Object(object_) if self.is_promise_object(object_) => {
                self.job_runner.await_promise(object_, object)
            }
            _ => self.job_runner.emit_promise_resolved(object, result),
        }
    }

    pub fn emit_promise_rejected(&mut self, promise: HandleMut<Object>, error: Value) {
        debug_assert!(self.is_promise_object(promise));
        self.job_runner.emit_promise_rejected(promise, error);
    }
}

pub struct JobRunner {
    messages: VecDeque<Message>,
}

impl JobRunner {
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
        }
    }

    // promises

    fn await_promise(&mut self, object: HandleMut<Object>, awaiting: HandleMut<Object>) {
        logger::debug!(event = "await_promise", ?object, ?awaiting);
        match object.promise().do_await(awaiting) {
            Some(Ok(result)) => self.emit_promise_resolved(awaiting, result),
            Some(Err(error)) => self.emit_promise_rejected(awaiting, error),
            None => (),
        }
    }

    fn emit_promise_resolved(&mut self, object: HandleMut<Object>, result: Value) {
        logger::debug!(event = "emit_promise_resolved", ?object, ?result);
        self.messages
            .push_back(Message::PromiseResolved { object, result });
    }

    fn emit_promise_rejected(&mut self, object: HandleMut<Object>, error: Value) {
        logger::debug!(event = "emit_promise_rejected", ?object, ?error);
        self.messages
            .push_back(Message::PromiseRejected { object, error });
    }

    fn next_msg(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    fn resolve_promise(&mut self, mut promise: HandleMut<Promise>, result: Value) {
        logger::debug!(event = "resolve_promise", ?promise, ?result);
        if let Some(awaiting) = promise.resolve(&result) {
            self.emit_promise_resolved(awaiting, result);
        }
    }

    fn reject_promise(&mut self, mut promise: HandleMut<Promise>, error: Value) {
        logger::debug!(event = "reject_promise", ?promise, ?error);
        if let Some(awaiting) = promise.reject(&error) {
            self.emit_promise_rejected(awaiting, error);
        }
    }

    pub(crate) fn collect_gc_roots(&self, roots: &mut Vec<usize>) {
        dbg!(self.messages.len());
        for msg in self.messages.iter() {
            match msg {
                Message::PromiseResolved { object, result } => {
                    roots.push(object.as_addr());
                    match result {
                        Value::String(string) => roots.push(string.as_addr()),
                        Value::Object(object) => roots.push(object.as_addr()),
                        _ => (),
                    }
                }
                Message::PromiseRejected { object, error } => {
                    roots.push(object.as_addr());
                    match error {
                        Value::String(string) => roots.push(string.as_addr()),
                        Value::Object(object) => roots.push(object.as_addr()),
                        _ => (),
                    }
                }
            }
        }
    }
}

// messages

#[derive(Debug)]
enum Message {
    PromiseResolved {
        object: HandleMut<Object>,
        result: Value,
    },
    PromiseRejected {
        object: HandleMut<Object>,
        error: Value,
    },
}
