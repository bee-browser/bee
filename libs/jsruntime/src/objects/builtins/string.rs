use crate::Runtime;
use crate::U16String;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

pub extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    match runtime.string_constructor(context) {
        Ok(value) => {
            *retv = value;
            Status::Normal
        }
        Err(value) => {
            *retv = value;
            Status::Exception
        }
    }
}

impl<X> Runtime<X> {
    fn string_constructor(&mut self, context: &mut CallContext) -> Result<Value, Value> {
        let s = match context.args().first() {
            Some(v) => self.perform_to_string(v),
            None => U16String::EMPTY,
        };
        // TODO(feat): NewTarget
        Ok(Value::String(s))
    }
}
