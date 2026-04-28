//$id object
//$class Object

use crate::Error;
use crate::Runtime;
use crate::logger;
use crate::types::CallContext;
use crate::types::Value;

//#sec-object-value constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "object");
    let this = context.this();
    let args = context.args();
    let new = context.is_new();
    runtime.create_object_object(Some(this), args, new)
}

// helpers

impl<X> Runtime<X> {
    pub(crate) fn create_object_object(
        &mut self,
        this: Option<&Value>,
        args: &[Value],
        new: bool,
    ) -> Result<Value, Error> {
        // TODO(feat): NewTarget
        if new {
            let object = if let Some(&Value::Object(this)) = this {
                this
            } else {
                // 10.4.3.4 StringCreate ( value, prototype )
                let mut object = self.create_object();
                object.set_prototype(self.builtins.string_prototype);
                object
            };
            Ok(Value::Object(object))
        } else {
            match args.first() {
                None | Some(Value::Undefined) | Some(Value::Null) => {
                    let mut object = self.create_object();
                    object.set_prototype(self.builtins.object_prototype);
                    // TODO(feat): NewTarget
                    Ok(Value::Object(object))
                }
                Some(value) => self.value_to_object(value),
            }
        }
    }
}
