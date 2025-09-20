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

    fn create_object_object(&mut self, context: &mut CallContext) -> Result<Value, Value> {
        match context.args().first() {
            None | Some(Value::Undefined) | Some(Value::Null) => {
                let object = self.create_object(self.object_prototype);
                // TODO(feat): NewTarget
                Ok(Value::Object(object))
            }
            Some(value) => {
                let mut retv = Value::None;
                match self.value_to_object(value, &mut retv) {
                    Status::Normal => {
                        debug_assert!(matches!(retv, Value::Object(_)));
                        Ok(retv)
                    }
                    Status::Exception => Err(retv),
                    Status::Suspend => unreachable!(),
                }
            }
        }
    }
}

// lambda functions

extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    match runtime.create_object_object(context) {
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
