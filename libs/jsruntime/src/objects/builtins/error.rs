use jsparser::Symbol;

use crate::Runtime;
use crate::U16Chunk;
use crate::U16String;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::objects::Property;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

const NAME: U16Chunk = U16Chunk::new_const(jsparser::symbol::builtin::names::ERROR);

pub extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let args = context.args();
    let new = context.is_new();
    match runtime.error_constructor(args, new) {
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

// 20.5.3.4 Error.prototype.toString ( )
extern "C" fn error_prototype_to_string<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let object = match context.this() {
        Value::Object(object) => object,
        _ => {
            let type_error = runtime.create_object(runtime.type_error_prototype);
            *retv = Value::Object(type_error);
            return Status::Exception;
        }
    };

    let name = match object.get_value(&Symbol::NAME.into()) {
        None | Some(Value::Undefined) => U16String::new(&NAME),
        Some(value) => runtime.perform_to_string(value),
    };

    let message = match object.get_value(&Symbol::MESSAGE.into()) {
        None | Some(Value::Undefined) => U16String::EMPTY,
        Some(value) => runtime.perform_to_string(value),
    };

    let result = if name.is_empty() {
        message
    } else if message.is_empty() {
        name
    } else {
        const SEP: U16Chunk = U16Chunk::new_const(&[0x003A, 0x0020]);
        let result = runtime.concat_strings(U16String::new(&SEP), message);
        runtime.concat_strings(name, result)
    };

    *retv = Value::String(result);
    Status::Normal
}

impl<X> Runtime<X> {
    pub(super) fn create_error_prototype(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_error_prototype");
        debug_assert!(self.object_prototype.is_some());

        let mut prototype = self.create_object(self.object_prototype);

        let _ = prototype.define_own_property(
            Symbol::NAME.into(),
            Property::data_xxx(Value::String(U16String::new(&NAME))),
        );

        let _ = prototype.define_own_property(
            Symbol::MESSAGE.into(),
            Property::data_xxx(Value::String(U16String::EMPTY)),
        );

        let to_string =
            self.create_builtin_function(error_prototype_to_string, self.function_prototype);
        let _ =
            prototype.define_own_property(Symbol::TO_STRING.into(), Property::data_xxx(to_string));

        prototype
    }

    fn error_constructor(&mut self, args: &[Value], new: bool) -> Result<Value, Value> {
        logger::debug!(event = "error_constructor", ?args, new);
        // TODO(feat): NewTarget
        let object = self.create_object(self.error_prototype);
        // TODO(feat): message
        Ok(Value::Object(object))
    }
}
