//$id number
//$class Number
//$inherits object

use jsgc::HandleMut;

use crate::Error;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Value;

use super::logger;

//#sec-number-constructor-number-value constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "number_constructor");

    let n = if cc.args().is_empty() {
        0.0
    } else {
        let primitive = runtime.value_to_numeric(cc.arg(0))?;
        // TODO(feat): BigInt
        primitive
    };
    match cc.new_target() {
        Some(new_target) => {
            let proto = runtime
                .get_prototype_from_constructor(new_target, runtime.builtins.number_prototype)?;
            let mut obj = runtime.create_object();
            obj.set_prototype(proto);
            obj.set_number(n);
            Ok(Value::Object(obj))
        }
        None => Ok(Value::Number(n)),
    }
}

//#sec-number.prototype.tostring prototype.function
pub fn number_prototype_to_string<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "number_prototype_to_string");
    let _x = this_number_value(cc.this())?;
    let radix = cc.arg(0);
    let radix_mv = if matches!(radix, Value::Undefined) {
        10
    } else {
        runtime.value_to_integer_or_infinity(radix)? as i64
    };
    if (2..=36).contains(&radix_mv) {
        // TODO
        Ok(Value::String(const_string_handle!("0")))
    } else {
        range_error!()
    }
}

// #sec-thisnumbervalue
fn this_number_value(value: &Value) -> Result<f64, Error> {
    match value {
        Value::Number(value) => Ok(*value),
        Value::Object(value) if value.is_number() => Ok(value.number()),
        _ => type_error!(),
    }
}

// helpers

impl<X> Runtime<X> {
    pub(crate) fn create_number_object(&mut self, value: f64) -> HandleMut<Object> {
        let mut obj = self.create_object();
        obj.set_prototype(self.builtins.number_prototype);
        obj.set_number(value);
        obj
    }
}
