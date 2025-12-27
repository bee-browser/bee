use jsparser::Symbol;

use crate::logger;

use crate::Runtime;
use crate::types::CallContext;
use crate::types::ObjectHandle;
use crate::types::Property;
use crate::types::Status;
use crate::types::StringHandle;
use crate::types::Value;

use super::BuiltinFunctionParams;

impl<X> Runtime<X> {
    pub(super) fn create_function_constructor(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_function_constructor");
        self.create_builtin_function(&BuiltinFunctionParams {
            lambda: constructor::<X>,
            name: const_string!(jsparser::symbol::builtin::names::FUNCTION),
            length: 1,
            slots: &[],
            prototype: self.function_prototype,
        })
    }

    pub(super) fn create_function_prototype(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_function_prototype");

        // TODO(fix): Function.prototype is a built-in function object.
        let mut prototype = self.create_object(self.object_prototype);
        let _ = prototype.define_own_property(
            Symbol::LENGTH.into(),
            Property::data_xxx(Value::Number(0.0)),
        );
        let _ = prototype.define_own_property(
            Symbol::NAME.into(),
            Property::data_xxx(Value::String(StringHandle::EMPTY)),
        );

        // TODO: Function.prototype.constructor

        prototype
    }
}

// lambda functions

extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    _context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    runtime_todo!(runtime, "TODO: Function constructor", retv)
}
