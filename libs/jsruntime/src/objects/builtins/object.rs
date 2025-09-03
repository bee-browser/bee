use crate::Runtime;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

impl<X> Runtime<X> {
    pub(super) fn create_object_constructor(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_object_constructor");
        self.create_builtin_function(constructor::<X>, self.object_prototype)
    }

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

// lambda functions

extern "C" fn constructor<X>(
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
