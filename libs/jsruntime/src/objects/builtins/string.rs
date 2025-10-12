use jsparser::Symbol;

use super::into_lambda;

use crate::Error;
use crate::Runtime;
use crate::StringFragment;
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

    // 22.1.2 Properties of the String Constructor
    pub(super) fn create_string_constructor(&mut self) -> ObjectHandle {
        logger::debug!(event = "create_string_constructor");

        let mut constructor = self.create_builtin_function(constructor::<X>, self.string_prototype);

        let mut method = self.create_builtin_function(into_lambda(string_from_char_code), None);
        let _ = method.define_own_property(
            Symbol::LENGTH.into(),
            Property::data_xxx(Value::Number(1.0)),
        );
        let _ = constructor.define_own_property(
            Symbol::FROM_CHAR_CODE.into(),
            Property::data_xxx(Value::Object(method)),
        );

        let mut method = self.create_builtin_function(into_lambda(string_from_code_point), None);
        let _ = method.define_own_property(
            Symbol::LENGTH.into(),
            Property::data_xxx(Value::Number(1.0)),
        );
        let _ = constructor.define_own_property(
            Symbol::FROM_CODE_POINT.into(),
            Property::data_xxx(Value::Object(method)),
        );

        constructor
    }

    pub(crate) fn create_string_object(
        &mut self,
        this: Option<&Value>,
        args: &[Value],
        new: bool,
    ) -> Result<Value, Value> {
        logger::debug!(event = "create_string_object", ?this, ?args, new);
        let string = match args.first() {
            Some(v) => {
                // TODO: a. If NewTarget is undefined and value is a Symbol,
                // return SymbolDescriptiveString(value).
                match self.value_to_string(v) {
                    Ok(string) => string,
                    Err(err) => return Err(self.create_exception(err)),
                }
            }
            None => StringHandle::EMPTY,
        };
        // TODO(feat): NewTarget
        if new {
            let mut object = if let Some(&Value::Object(this)) = this {
                this
            } else {
                // 10.4.3.4 StringCreate ( value, prototype )
                self.create_object(self.string_prototype)
            };
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

        let method = self.create_builtin_function(into_lambda(string_prototype_at), None);
        let _ = prototype
            .define_own_property(Symbol::AT.into(), Property::data_xxx(Value::Object(method)));

        let method = self.create_builtin_function(into_lambda(string_prototype_char_at), None);
        let _ = prototype.define_own_property(
            Symbol::CHAR_AT.into(),
            Property::data_xxx(Value::Object(method)),
        );

        let method = self.create_builtin_function(into_lambda(string_prototype_index_of), None);
        let _ = prototype.define_own_property(
            Symbol::INDEX_OF.into(),
            Property::data_xxx(Value::Object(method)),
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
    let this = context.this();
    let args = context.args();
    let new = context.is_new();
    // TODO(feat): target
    match runtime.create_string_object(Some(this), args, new) {
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

// 22.1.2 Properties of the String Constructor
fn string_from_char_code<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_from_char_code");
    let mut utf16 = vec![];
    for arg in context.args().iter() {
        let code_unit = crate::types::number::to_uint16(arg)?;
        utf16.push(code_unit);
    }
    let slice = runtime.allocator.alloc_slice_copy(&utf16);
    let frag = StringFragment::new_stack(slice, true);
    let string = StringHandle::new(&frag);
    Ok(Value::String(runtime.migrate_string_to_heap(string)))
}

// 22.1.2.2 String.fromCodePoint ( ...codePoints )
fn string_from_code_point<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_from_code_point");
    let mut buf = [0; 2];
    let mut utf16 = vec![];
    for arg in context.args().iter() {
        let num = crate::types::number::to_number(arg)?;
        if num.is_infinite() || num.is_nan() || num.fract() != 0.0 {
            return Err(Error::RangeError);
        }
        let cp = num as i64;
        if !(0..0x10FFFF).contains(&cp) {
            return Err(Error::RangeError);
        }
        // TODO(perf): inefficient.  implement an iterator to encode a code point to UTF-16
        // code units.
        utf16.extend_from_slice(char::from_u32(cp as u32).unwrap().encode_utf16(&mut buf));
    }
    let slice = runtime.allocator.alloc_slice_copy(&utf16);
    let frag = StringFragment::new_stack(slice, true);
    let string = StringHandle::new(&frag);
    Ok(Value::String(runtime.migrate_string_to_heap(string)))
}

// 22.1.3.1 String.prototype.at ( index )
fn string_prototype_at<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_at");
    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;
    let len = s.len() as f64;
    let index = context.args().first().unwrap_or(&Value::Undefined);
    let relative_index = runtime.value_to_integer_or_infinity(index)?;
    let k = if relative_index >= 0.0 {
        relative_index
    } else {
        len + relative_index
    };
    if k < 0.0 || k >= len {
        return Ok(Value::Undefined);
    }
    // TODO(perf): memory inefficient
    let code_unit = s.at(k as u32);
    let slice = runtime.allocator.alloc_slice_copy(code_unit.as_slice());
    let frag = StringFragment::new_stack(slice, true);
    let string = StringHandle::new(&frag);
    Ok(Value::String(runtime.migrate_string_to_heap(string)))
}

// 22.1.3.2 String.prototype.charAt ( pos )
fn string_prototype_char_at<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_char_at");
    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;
    let pos = context.args().first().unwrap_or(&Value::Undefined);
    let position = runtime.value_to_integer_or_infinity(pos)?;
    let size = s.len() as f64;
    if position < 0.0 || position >= size {
        return Ok(Value::String(StringHandle::EMPTY));
    }
    // TODO(perf): memory inefficient
    let code_unit = s.at(position as u32);
    let slice = runtime.allocator.alloc_slice_copy(code_unit.as_slice());
    let frag = StringFragment::new_stack(slice, true);
    let string = StringHandle::new(&frag);
    Ok(Value::String(runtime.migrate_string_to_heap(string)))
}

// 7.2.1 RequireObjectCoercible ( argument )
fn require_object_coercible(value: &Value) -> Result<(), Error> {
    match value {
        Value::None => unreachable!(),
        Value::Undefined | Value::Null => Err(Error::TypeError),
        _ => Ok(()),
    }
}

// 22.1.3.9 String.prototype.indexOf ( searchString [ , position ] )
fn string_prototype_index_of<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_index_of");

    let string = runtime.value_to_string(context.this())?;

    let args = context.args();
    let search_str = args.first().unwrap_or(&Value::Undefined);
    let search_str = runtime.value_to_string(search_str)?;

    let position = args.get(1).unwrap_or(&Value::Undefined);
    let pos = runtime.value_to_integer_or_infinity(position)?;

    let len = string.len();
    let start = pos.clamp(0.0, len as f64) as u32;
    let index = string
        .index_of(search_str, start)
        .map_or(-1.0, |i| i as f64);

    Ok(Value::Number(index))
}
