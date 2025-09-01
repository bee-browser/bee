use std::ffi::c_void;

use jsparser::Symbol;

use crate::logger;

use crate::Runtime;
use crate::objects::Object;
use crate::objects::Property;
use crate::types::CallContext;
use crate::types::Lambda;
use crate::types::Status;
use crate::types::U16String;
use crate::types::Value;

pub(super) extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let args = context.args();
    let new = context.is_new();
    match runtime.string_constructor(args, new) {
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
    pub(super) fn create_function_constructor(
        &mut self,
        lambda: Lambda<X>,
        prototype: *mut c_void,
    ) -> Value {
        logger::debug!(event = "creater_function_constructor");
        let constructor = self.create_builtin_function(lambda, prototype);
        match constructor {
            Value::Function(constructor) => {
                // SAFETY: `constructor` is a non-null pointer to an `Object`.
                let constructor = unsafe { &mut *(constructor as *mut Object) };
                let _ = constructor.define_own_property(
                    Symbol::LENGTH.into(),
                    Property::data_xxx(Value::Number(1.0)),
                );
            }
            _ => unreachable!(),
        }
        constructor
    }

    pub(super) fn create_function_prototype(&mut self) -> *mut c_void {
        logger::debug!(event = "creater_function_prototype");

        // TODO(fix): Function.prototype is a built-in function object.
        let prototype = self.create_object(self.object_prototype);
        let _ = prototype.define_own_property(
            Symbol::LENGTH.into(),
            Property::data_xxx(Value::Number(0.0)),
        );
        let _ = prototype.define_own_property(
            Symbol::NAME.into(),
            Property::data_xxx(Value::String(U16String::EMPTY)),
        );

        prototype.as_ptr()
    }
}
