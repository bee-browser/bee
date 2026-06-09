//$id error
//$class Error
//$inherits object

use jsgc::Handle;
use jsgc::HandleMut;
use jsparser::Symbol;

use crate::Error;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Property;
use crate::types::String;
use crate::types::Value;

use super::logger;

//#sec-error-message constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "error_constructor");

    let new_target = match cc.new_target() {
        Some(new_target) => new_target,
        None => cc.func().unwrap(),
    };

    let mut object =
        runtime.ordinary_create_from_constructor(new_target, runtime.builtins.error_constructor)?;

    object.set_error();

    match cc.args().first().unwrap_or(&Value::Undefined) {
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

    if let Value::Object(value) = cc.args().get(1).unwrap_or(&Value::Undefined) {
        let key = Symbol::CAUSE.into();
        if let Some(value) = value.get_value(&key) {
            // TODO: error handling
            let _ = object.define_own_property(key, Property::data_wxc(value.clone()));
        }
    }

    Ok(Value::Object(object))
}

//#sec-error.iserror constructor.function {"signature": "Error.isError ( arg )"}
pub fn error_is_error<X>(_runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "error_is_error");
    match cc.args().first() {
        Some(Value::Object(value)) => Ok(Value::Boolean(value.is_error())),
        _ => Ok(Value::Boolean(false)),
    }
}

//#sec-error.prototype.message prototype.property
pub fn error_prototype_message<X>(_runtime: &mut Runtime<X>, mut prototype: HandleMut<Object>) {
    let _ = prototype.define_own_property(
        Symbol::MESSAGE.into(),
        Property::data_xxx(Value::String(crate::types::string::EMPTY)),
    );
}

//#sec-error.prototype.name prototype.property
pub fn error_prototype_name<X>(_runtime: &mut Runtime<X>, mut prototype: HandleMut<Object>) {
    let _ = prototype.define_own_property(
        Symbol::NAME.into(),
        Property::data_xxx(Value::String(crate::types::string::EMPTY)),
    );
}

//#sec-error.prototype.tostring prototype.function
pub fn error_prototype_to_string<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "error_prototype_to_string");
    let object = match cc.this() {
        Value::Object(object) => object,
        _ => return type_error!(),
    };

    let name = match object.get_value(&Symbol::NAME.into()) {
        None | Some(Value::Undefined) => NAME,
        Some(value) => runtime.value_to_string(value)?,
    };

    let message = match object.get_value(&Symbol::MESSAGE.into()) {
        None | Some(Value::Undefined) => crate::types::string::EMPTY,
        Some(value) => runtime.value_to_string(value)?,
    };

    let result = if name.is_empty() {
        message
    } else if message.is_empty() {
        name
    } else {
        let result = runtime.concat_strings(const_string_handle!(&[0x003A, 0x0020]), message);
        runtime.concat_strings(name, result)
    };

    Ok(Value::String(result))
}

const NAME: Handle<String> = const_string_handle!(jsparser::symbol::builtin::names::ERROR);
