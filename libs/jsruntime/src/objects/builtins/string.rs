use std::ffi::c_void;

use jsparser::Symbol;

use crate::Runtime;
use crate::U16Chunk;
use crate::U16String;
use crate::logger;
use crate::objects::Object;
use crate::objects::Property;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

pub extern "C" fn constructor<X>(
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
fn string_index_of(string: U16String, search_value: U16String, from_index: u32) -> Option<u32> {
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

impl<X> Runtime<X> {
    pub(crate) fn is_string_object(&self, object: &Object) -> bool {
        object.is_instance_of(self.string_prototype)
    }

    pub(crate) fn string_constructor(&mut self, args: &[Value], new: bool) -> Result<Value, Value> {
        logger::debug!(event = "string_constructor", ?args, new);
        let string = match args.first() {
            Some(v) => {
                // TODO: a. If NewTarget is undefined and value is a Symbol,
                // return SymbolDescriptiveString(value).
                self.perform_to_string(v)
            }
            None => U16String::EMPTY,
        };
        // TODO(feat): NewTarget
        if new {
            // 10.4.3.4 StringCreate ( value, prototype )
            let protptype = self.string_prototype;
            let object = self.create_object(protptype);
            let length = string.len();
            object.set_string(string);
            // TODO: check the result
            let _ = object.define_own_property(
                Symbol::LENGTH.into(),
                Property::data_xxx(Value::Number(length as f64)),
            );
            Ok(Value::Object(object.as_ptr()))
        } else {
            Ok(Value::String(string))
        }
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

impl Object {
    pub(crate) fn string(&self) -> U16String {
        // SAFETY: `self.nucleus` of a String object is a non-null point to a `U16Chunk`.
        let chunk = unsafe { &*(self.nucleus as *const U16Chunk) };
        U16String::new(chunk)
    }
}
