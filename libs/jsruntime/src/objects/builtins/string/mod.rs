use std::ffi::c_void;

use jsparser::Symbol;

use crate::Runtime;
use crate::U16String;
use crate::logger;
use crate::objects::Property;
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

// 22.1.3.9 String.prototype.indexOf ( searchString [ , position ] )
extern "C" fn string_prototype_index_of<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
    _retv: &mut Value,
) -> Status {
    logger::debug!(event = "string_prototype_index_of");
    // TODO
    Status::Normal
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

    pub(super) fn create_string_prototype(&mut self) -> *mut c_void {
        logger::debug!(event = "creater_string_prototype");
        debug_assert!(!self.object_prototype.is_null());

        let index_of = self.create_builtin_function(string_prototype_index_of, Value::None);

        let prototype = self.create_object(self.object_prototype);
        let _ =
            prototype.define_own_property(Symbol::INDEX_OF.into(), Property::data_xxx(index_of));
        prototype.as_ptr()
    }
}
