use super::Value;
use crate::Error;

// 7.1.4 ToNumber ( argument )
pub fn to_number(value: &Value) -> Result<f64, Error> {
    match value {
        Value::None => unreachable!(),
        Value::Undefined => Ok(f64::NAN),
        Value::Null => Ok(0.0),
        Value::Boolean(false) => Ok(0.0),
        Value::Boolean(true) => Ok(1.0),
        Value::Number(value) => Ok(*value),
        Value::String(_) => Ok(f64::NAN), // TODO(feat): 7.1.4.1.1 StringToNumber ( str )
        Value::Promise(_) => Ok(f64::NAN),
        Value::Object(_) => Ok(f64::NAN), // TODO(feat): 7.1.1 ToPrimitive()
    }
}

// 7.1.9 ToUint16 ( argument )
pub fn to_uint16(value: &Value) -> Result<u16, Error> {
    let num = to_number(value)?;
    if num.is_infinite() || num == 0.0 {
        return Ok(0);
    }
    Ok(num as u16)
}
