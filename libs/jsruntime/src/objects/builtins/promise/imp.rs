use crate::Error;
use crate::Runtime;
use crate::StringHandle;
use crate::lambda::LambdaId;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::objects::builtins::BuiltinFunctionParams;
use crate::types::CallContext;
use crate::types::Lambda;
use crate::types::Promise;
use crate::types::Status;
use crate::types::Value;

//#sec-promise-executor constructor
pub fn promise<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "promise");

    // TODO(feat): NewTarget
    if !context.is_new() {
        return Err(Error::InternalError);
    }

    let args = context.args();

    let executor = match args.first() {
        Some(Value::Object(executor)) if executor.is_callable() => *executor,
        _ => return Err(Error::TypeError),
    };

    // TODO(feat): step#3..12
    let closure = runtime.create_closure(promise_coroutine, LambdaId::HOST, 0);
    let coroutine = runtime.create_coroutine(closure, 0, 0, 0);
    let promise = runtime.register_promise(coroutine);

    let mut object = if context.is_new() {
        if let Value::Object(this) = context.this() {
            *this
        } else {
            runtime.create_object(runtime.promise_prototype)
        }
    } else {
        unreachable!();
    };

    object.set_promise(promise);

    let (resolve, reject) = runtime.create_resolving_functions(object);
    let mut retv = Value::None;
    match runtime.call(
        context,
        executor,
        &mut [Value::Object(resolve), Value::Object(reject)],
        &mut retv,
    ) {
        Status::Exception => runtime.emit_promise_rejected(object, retv),
        _ => (),
    }

    Ok(Value::Object(object))
}

extern "C" fn promise_coroutine<X>(
    _runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let value = context.args().get(1).unwrap();
    if value.is_valid() {
        *retv = value.clone();
        Status::Normal
    } else {
        let error = context.args().get(2).unwrap();
        debug_assert!(error.is_valid());
        *retv = error.clone();
        Status::Exception
    }
}

impl<X> Runtime<X> {
    fn create_resolving_functions(
        &mut self,
        promise: ObjectHandle,
    ) -> (ObjectHandle, ObjectHandle) {
        let resolve = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: promise_resolve,
            name: StringHandle::EMPTY,
            length: 1,
            slots: &[Value::Object(promise)],
            prototype: None,
        });

        let reject = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: promise_reject,
            name: StringHandle::EMPTY,
            length: 1,
            slots: &[Value::Object(promise)],
            prototype: None,
        });

        (resolve, reject)
    }

    fn call(
        &mut self,
        caller: &CallContext,
        callable: ObjectHandle,
        args: &mut [Value],
        retv: &mut Value,
    ) -> Status {
        let closure = callable.closure();
        debug_assert!(!closure.is_null());
        let mut context = caller.new_child(callable, closure, args);
        // SAFETY:
        let lambda = unsafe { Lambda::from((*closure).lambda) };
        lambda(self, &mut context, retv)
    }
}

// 27.2.1.3.2 Promise Resolve Functions
extern "C" fn promise_resolve<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "promise_resolve");
    match promise_resolve_sync(runtime, context) {
        Ok(value) => {
            *retv = value;
            Status::Normal
        }
        Err(err) => {
            *retv = runtime.create_exception(err);
            Status::Exception
        }
    }
}

fn promise_resolve_sync<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    let func = context.func().ok_or(Error::InternalError)?;

    let promise = match func.slots.first() {
        Some(Value::Object(promise)) => *promise,
        _ => return Err(Error::InternalError),
    };
    debug_assert!(runtime.is_promise_object(promise));

    let resolution = context.args().first().unwrap_or(&Value::Undefined);
    match resolution {
        Value::Object(object) if *object == promise => Err(Error::TypeError),
        // TODO: the 'then' property
        _ => {
            // TODO: fullfill_promise
            runtime.emit_promise_resolved(promise, resolution.clone());
            Ok(Value::Undefined)
        }
    }
}

extern "C" fn promise_reject<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "promise_reject");

    let func = match context.func() {
        Some(func) => func,
        _ => unreachable!(),
    };

    let promise = match func.slots.first() {
        Some(Value::Object(promise)) => *promise,
        _ => unreachable!(),
    };
    debug_assert!(runtime.is_promise_object(promise));

    let error = context.args().first().unwrap_or(&Value::Undefined);
    runtime.emit_promise_rejected(promise, error.clone());
    *retv = Value::Undefined;
    Status::Normal
}

//#sec-promise.all constructor.function
pub fn promise_all<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "promise_all");
    Err(Error::InternalError)
}

//#sec-promise.prototype.catch prototype.function
pub fn promise_prototype_catch<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "promise_prototype_catch");
    Err(Error::InternalError)
}

// helpers

impl ObjectHandle {
    pub(crate) fn get_promise(&self) -> Promise {
        Promise::from(self.as_object().userdata() as u32)
    }
}
