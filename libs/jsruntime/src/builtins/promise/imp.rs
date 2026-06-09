//$id promise
//$class Promise
//$inherits object

use jsgc::HandleMut;

use crate::Error;
use crate::Runtime;
use crate::lambda::LambdaId;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Status;
use crate::types::Value;

use super::BuiltinFunctionParams;
use super::logger;

//#sec-promise-executor constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "promise_constructor");

    let new_target = match cc.new_target() {
        Some(new_target) => new_target,
        None => return runtime_todo!(),
    };

    let executor = match cc.arg(0) {
        Value::Object(executor) if executor.is_callable() => *executor,
        _ => return type_error!(),
    };

    let mut object =
        runtime.ordinary_create_from_constructor(new_target, runtime.builtins.promise_prototype)?;

    let closure = runtime.create_closure(promise_coroutine, LambdaId::HOST, 0);
    let coroutine = runtime.create_coroutine(closure, 0, 0, 0);
    let promise = runtime.create_promise(coroutine);
    object.set_promise(promise);

    let (resolve, reject) = runtime.create_resolving_functions(object);
    let mut retv = Value::None;
    if let Status::Exception = runtime.call(
        cc,
        executor,
        &Value::Undefined,
        &[Value::Object(resolve), Value::Object(reject)],
        &mut retv,
    ) {
        runtime.emit_promise_rejected(object, retv);
    }

    Ok(Value::Object(object))
}

extern "C" fn promise_coroutine<X>(
    _runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let value = cc.args().get(1).unwrap();
    if value.is_valid() {
        *retv = value.clone();
        Status::Normal
    } else {
        let error = cc.args().get(2).unwrap();
        debug_assert!(error.is_valid());
        *retv = error.clone();
        Status::Exception
    }
}

// 27.2.1.3 CreateResolvingFunctions ( promise )
impl<X> Runtime<X> {
    fn create_resolving_functions(
        &mut self,
        promise: HandleMut<Object>,
    ) -> (HandleMut<Object>, HandleMut<Object>) {
        let resolve = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: promise_resolve,
            name: crate::types::string::EMPTY,
            length: 1,
            slots: &[Value::Object(promise)],
        });

        let reject = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: promise_reject,
            name: crate::types::string::EMPTY,
            length: 1,
            slots: &[Value::Object(promise)],
        });

        (resolve, reject)
    }
}

// 27.2.1.3.2 Promise Resolve Functions
extern "C" fn promise_resolve<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "promise_resolve");
    match promise_resolve_sync(runtime, cc) {
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

fn promise_resolve_sync<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    let func = match cc.func() {
        Some(func) => func,
        _ => return runtime_todo!(),
    };

    let promise = match func.slots().first() {
        Some(Value::Object(promise)) => *promise,
        _ => return runtime_todo!(),
    };
    debug_assert!(runtime.is_promise_object(promise));

    match cc.arg(0) {
        Value::Object(object) if *object == promise => type_error!(),
        // TODO: the 'then' property
        resolution => {
            // TODO: fullfill_promise
            runtime.emit_promise_resolved(promise, resolution.clone());
            Ok(Value::Undefined)
        }
    }
}

// 27.2.1.3.1 Promise Reject Functions
extern "C" fn promise_reject<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "promise_reject");

    let func = match cc.func() {
        Some(func) => func,
        _ => unreachable!(),
    };

    let promise = match func.slots().first() {
        Some(Value::Object(promise)) => *promise,
        _ => unreachable!(),
    };
    debug_assert!(runtime.is_promise_object(promise));

    let error = cc.arg(0);
    runtime.emit_promise_rejected(promise, error.clone());
    *retv = Value::Undefined;
    Status::Normal
}
