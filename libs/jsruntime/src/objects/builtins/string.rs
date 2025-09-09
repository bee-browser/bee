use jsparser::Symbol;

use crate::Runtime;
use crate::logger;
use crate::objects::Object;
use crate::objects::ObjectHandle;
use crate::objects::Property;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::StringHandle;
use crate::types::Value;

impl<X> Runtime<X> {
    pub(crate) fn is_string_object(&self, object: ObjectHandle) -> bool {
        object.is_instance_of(self.string_prototype)
    }

    pub(super) fn create_string_constructor(&mut self) -> ObjectHandle {
        logger::debug!(event = "create_string_constructor");
        self.create_builtin_function(constructor::<X>, self.string_prototype)
    }

    pub(crate) fn create_string_object(
        &mut self,
        args: &[Value],
        new: bool,
    ) -> Result<Value, Value> {
        logger::debug!(event = "create_string_object", ?args, new);
        let string = match args.first() {
            Some(v) => {
                // TODO: a. If NewTarget is undefined and value is a Symbol,
                // return SymbolDescriptiveString(value).
                self.perform_to_string(v)
            }
            None => StringHandle::EMPTY,
        };
        // TODO(feat): NewTarget
        if new {
            // 10.4.3.4 StringCreate ( value, prototype )
            let mut object = self.create_object(self.string_prototype);
            let length = string.len();
            object.set_string(string);
            // TODO: check the result
            let _ = object.define_own_property(
                Symbol::LENGTH.into(),
                Property::data_xxx(Value::Number(length as f64)),
            );
            Ok(Value::Object(object))
        } else {
            Ok(Value::String(string))
        }
    }

    pub(super) fn create_string_prototype(&mut self) -> ObjectHandle {
        logger::debug!(event = "creater_string_prototype");
        debug_assert!(self.object_prototype.is_some());
        debug_assert!(self.function_prototype.is_some());

        let mut prototype = self.create_object(self.object_prototype);

        let index_of = self.create_builtin_function(string_prototype_index_of, None);
        let _ = prototype.define_own_property(
            Symbol::INDEX_OF.into(),
            Property::data_xxx(Value::Object(index_of)),
        );

        prototype
    }
}

impl Object {
    pub(crate) fn string(&self) -> StringHandle {
        // SAFETY: `self.nucleus` is non-null and convertible to a reference.
        unsafe { StringHandle::from_addr(self.nucleus) }
    }
}

// lambda functions

extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    let args = context.args();
    let new = context.is_new();
    match runtime.create_string_object(args, new) {
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
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "string_prototype_index_of");

    let string = match runtime.value_to_string(context.this()) {
        Ok(string) => string,
        Err(_err) => {
            // TODO: convert Error to JS error object.
            return Status::Exception;
        }
    };

    let args = context.args();
    let search_str = runtime.perform_to_string(args.first().unwrap_or(&Value::Undefined));
    let position = args.get(1).unwrap_or(&Value::Undefined);
    let pos = match runtime.value_to_integer_or_infinity(position) {
        Ok(pos) => pos,
        Err(_err) => {
            // TODO: convert Error to JS error object.
            return Status::Exception;
        }
    };

    let len = string.len();
    let start = pos.clamp(0.0, len as f64) as u32;
    let index = string_index_of(string, search_str, start).map_or(-1.0, |i| i as f64);

    *retv = Value::Number(index);
    Status::Normal
}

// 6.1.4.1 StringIndexOf ( string, searchValue, fromIndex )
fn string_index_of(
    string: StringHandle,
    search_value: StringHandle,
    from_index: u32,
) -> Option<u32> {
    // TODO(perf): slow and inefficient
    let len = string.len();
    if search_value.is_empty() && from_index <= len {
        return Some(from_index);
    }
    let search_len = search_value.len();
    let string = string.make_utf16();
    let search = search_value.make_utf16();
    for i in from_index..(len - search_len + 1) {
        let canditate = &string[(i as usize)..((i + search_len) as usize)];
        if canditate == search {
            return Some(i);
        }
    }
    None
}
