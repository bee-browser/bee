use crate::Runtime;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

pub extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    match runtime.object_constructor(context) {
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
    fn object_constructor(&mut self, context: &mut CallContext) -> Result<Value, Value> {
        let o = match context.args().first() {
            None | Some(Value::Undefined) | Some(Value::Null) => {
                self.create_object(self.object_prototype)
            }
            Some(_v) => todo!(), // TODO(feat): ToObject()
        };
        // TODO(feat): NewTarget
        Ok(Value::Object(o))
    }
}
