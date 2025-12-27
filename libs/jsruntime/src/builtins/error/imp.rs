//$id error
//$class Error

use jsparser::Symbol;

use crate::Error;
use crate::Runtime;
use crate::gc::Handle;
use crate::logger;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Property;
use crate::types::StringHandle;
use crate::types::Value;

//#sec-error-message constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "error_constructor");
    let args = context.args();

    // TODO(feat): NewTarget
    if !context.is_new() {
        return Err(Error::InternalError);
    }

    let mut object = if let Value::Object(this) = context.this() {
        *this
    } else {
        runtime.create_object(runtime.error_prototype)
    };
    object.set_error();

    match args.first().unwrap_or(&Value::Undefined) {
        Value::Undefined => (),
        message => {
            let msg = runtime.value_to_string(message)?;
            // TODO: error handling
            let _ = object.define_own_property(
                Symbol::MESSAGE.into(),
                Property::data_wxc(Value::String(msg)),
            );
        }
    }

    if let Value::Object(value) = args.get(1).unwrap_or(&Value::Undefined) {
        let key = Symbol::CAUSE.into();
        if let Some(value) = value.get_value(&key) {
            // TODO: error handling
            let _ = object.define_own_property(key, Property::data_wxc(value.clone()));
        }
    }

    Ok(Value::Object(object))
}

//#sec-error.iserror constructor.function {"signature": "Error.isError ( arg )"}
pub fn error_is_error<X>(
    _runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "error_is_error");
    match context.args().first() {
        Some(Value::Object(value)) => Ok(Value::Boolean(value.is_error())),
        _ => Ok(Value::Boolean(false)),
    }
}

//#sec-error.prototype.message prototype.property
pub fn error_prototype_message<X>(_runtime: &mut Runtime<X>, mut prototype: Handle<Object>) {
    let _ = prototype.define_own_property(
        Symbol::MESSAGE.into(),
        Property::data_xxx(Value::String(StringHandle::EMPTY)),
    );
}

//#sec-error.prototype.name prototype.property
pub fn error_prototype_name<X>(_runtime: &mut Runtime<X>, mut prototype: Handle<Object>) {
    let _ = prototype.define_own_property(
        Symbol::NAME.into(),
        Property::data_xxx(Value::String(StringHandle::EMPTY)),
    );
}

//#sec-error.prototype.tostring prototype.function
pub fn error_prototype_to_string<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "error_prototype_to_string");
    let object = match context.this() {
        Value::Object(object) => object,
        _ => return Err(Error::TypeError),
    };

    let name = match object.get_value(&Symbol::NAME.into()) {
        None | Some(Value::Undefined) => NAME,
        Some(value) => runtime.value_to_string(value)?,
    };

    let message = match object.get_value(&Symbol::MESSAGE.into()) {
        None | Some(Value::Undefined) => StringHandle::EMPTY,
        Some(value) => runtime.value_to_string(value)?,
    };

    let result = if name.is_empty() {
        message
    } else if message.is_empty() {
        name
    } else {
        let result = runtime.concat_strings(const_string!(&[0x003A, 0x0020]), message);
        runtime.concat_strings(name, result)
    };

    Ok(Value::String(result))
}

const NAME: StringHandle = const_string!(jsparser::symbol::builtin::names::ERROR);
