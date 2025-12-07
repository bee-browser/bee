use jsparser::Symbol;

use crate::Runtime;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::objects::Property;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::StringHandle;
use crate::types::Value;

use super::BuiltinFunctionParams;

impl<X> Runtime<X> {
    pub(super) fn create_error_constructor(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_error_constructor");
        debug_assert!(self.error_prototype.is_some());
        let mut constructor = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: constructor::<X>,
            name: const_string!(jsparser::symbol::builtin::names::ERROR),
            length: 1,
            slots: &[],
            prototype: self.error_prototype,
        });
        let func = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: error_is_error,
            name: const_string!(jsparser::symbol::builtin::names::IS_ERROR),
            length: 1,
            slots: &[],
            prototype: None,
        });
        let _ = constructor.define_own_property(
            Symbol::IS_ERROR.into(),
            Property::data_xxx(Value::Object(func)),
        );
        if let Some(mut prototype) = self.error_prototype {
            let _ = prototype.define_own_property(
                Symbol::CONSTRUCTOR.into(),
                Property::data_xxx(Value::Object(constructor)),
            );
        }
        constructor
    }

    pub(super) fn create_error_prototype(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_error_prototype");
        debug_assert!(self.object_prototype.is_some());

        let mut prototype = self.create_object(self.object_prototype);

        let _ = prototype
            .define_own_property(Symbol::NAME.into(), Property::data_xxx(Value::String(NAME)));

        let _ = prototype.define_own_property(
            Symbol::MESSAGE.into(),
            Property::data_xxx(Value::String(StringHandle::EMPTY)),
        );

        let to_string = self.create_builtin_function(&BuiltinFunctionParams {
            lambda: error_prototype_to_string,
            name: const_string!(jsparser::symbol::builtin::names::TO_STRING),
            length: 0,
            slots: &[],
            prototype: None,
        });
        let _ = prototype.define_own_property(
            Symbol::TO_STRING.into(),
            Property::data_xxx(Value::Object(to_string)),
        );

        prototype
    }

    pub(crate) fn create_error(
        &mut self,
        new: bool,
        message: &Value,
        options: &Value,
    ) -> Result<ObjectHandle, Value> {
        logger::debug!(event = "create_error", new, ?message, ?options);
        // TODO(feat): NewTarget
        let mut object = self.create_object(self.error_prototype);

        object.set_error();

        match message {
            Value::Undefined => (),
            _ => {
                let msg = match self.value_to_string(message) {
                    Ok(string) => string,
                    Err(err) => return Err(self.create_exception(err)),
                };
                // TODO: error handling
                let _ = object.define_own_property(
                    Symbol::MESSAGE.into(),
                    Property::data_wxc(Value::String(msg)),
                );
            }
        }

        if let Value::Object(value) = options {
            let key = Symbol::CAUSE.into();
            if let Some(value) = value.get_value(&key) {
                // TODO: error handling
                let _ = object.define_own_property(key, Property::data_wxc(value.clone()));
            }
        }

        Ok(object)
    }
}

// lambda functions

extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let new = context.is_new();
    let args = context.args();
    let message = args.first().unwrap_or(&Value::Undefined);
    let options = args.get(1).unwrap_or(&Value::Undefined);
    match runtime.create_error(new, message, options) {
        Ok(value) => {
            *retv = Value::Object(value);
            Status::Normal
        }
        Err(value) => {
            *retv = value;
            Status::Exception
        }
    }
}

const NAME: StringHandle = const_string!(jsparser::symbol::builtin::names::ERROR);

// 20.5.2.1 Error.isError ( arg )
extern "C" fn error_is_error<X>(
    _runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    match context.args().first() {
        Some(Value::Object(value)) => {
            *retv = Value::Boolean(value.is_error());
        }
        _ => *retv = Value::Boolean(false),
    }
    Status::Normal
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
        None | Some(Value::Undefined) => NAME,
        Some(value) => match runtime.value_to_string(value) {
            Ok(string) => string,
            Err(err) => {
                *retv = runtime.create_exception(err);
                return Status::Exception;
            }
        },
    };

    let message = match object.get_value(&Symbol::MESSAGE.into()) {
        None | Some(Value::Undefined) => StringHandle::EMPTY,
        Some(value) => match runtime.value_to_string(value) {
            Ok(string) => string,
            Err(err) => {
                *retv = runtime.create_exception(err);
                return Status::Exception;
            }
        },
    };

    let result = if name.is_empty() {
        message
    } else if message.is_empty() {
        name
    } else {
        let result = runtime.concat_strings(const_string!(&[0x003A, 0x0020]), message);
        runtime.concat_strings(name, result)
    };

    *retv = Value::String(result);
    Status::Normal
}
