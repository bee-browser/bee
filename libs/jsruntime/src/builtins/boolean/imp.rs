//$id boolean
//$class Boolean
//$inherits object

use jsgc::HandleMut;

use crate::Error;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Value;

use super::logger;

//#sec-boolean-constructor-boolean-value constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "boolean_constructor");

    let value = cc.arg(0).to_boolean();
    match cc.new_target() {
        Some(new_target) => {
            let proto = runtime
                .get_prototype_from_constructor(new_target, runtime.builtins.boolean_prototype)?;
            let mut obj = runtime.create_object();
            obj.set_prototype(proto);
            obj.set_boolean(value);
            Ok(Value::Object(obj))
        }
        None => Ok(Value::Boolean(value)),
    }
}

//#sec-boolean.prototype.tostring prototype.function
pub fn boolean_prototype_to_string<X>(
    _runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "boolean_prototype_to_string");
    let value = this_boolean_value(cc.this())?;
    if value {
        Ok(Value::String(const_string_handle!("true")))
    } else {
        Ok(Value::String(const_string_handle!("false")))
    }
}

fn this_boolean_value(value: &Value) -> Result<bool, Error> {
    match value {
        Value::Boolean(value) => Ok(*value),
        Value::Object(value) if value.is_boolean() => Ok(value.boolean()),
        _ => type_error!(),
    }
}

// helpers

impl<X> Runtime<X> {
    pub(crate) fn create_boolean_object(&mut self, value: bool) -> HandleMut<Object> {
        let mut obj = self.create_object();
        obj.set_prototype(self.builtins.boolean_prototype);
        obj.set_boolean(value);
        obj
    }
}
