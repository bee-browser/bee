use crate::Error;
use crate::Runtime;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::types::CallContext;
use crate::types::Promise;
use crate::types::Value;

//#sec-promise-executor constructor
pub fn promise<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "promise");

    // TODO(feat): NewTarget
    if !context.is_new() {
        return Err(Error::InternalError);
    }

    let args = context.args();

    let executor = args.first().unwrap_or(&Value::Undefined);
    if !executor.is_callable() {
        return Err(Error::InternalError);
    }

    // TODO(feat): OrdinaryCreateFromConstructor()
    let promise = if let &Value::Object(this) = context.this() {
        this
    } else {
        runtime.create_object(runtime.promise_prototype)
    };

    // TODO(feat): step#4..12

    Ok(Value::Object(promise))
}

//#sec-promise.all constructor.function
pub fn promise_all<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_from_char_code");
    Err(Error::InternalError)
}

// helpers

impl ObjectHandle {
    pub(crate) fn get_promise(&self) -> Promise {
        Promise::from(self.as_object().userdata() as u32)
    }
}
