use crate::Error;
use crate::Runtime;
use crate::StringFragment;
use crate::logger;
use crate::objects::builtins::require_object_coercible;
use crate::types::CallContext;
use crate::types::StringHandle;
use crate::types::Value;

//#sec-string.fromcharcode constructor.function
pub fn string_from_char_code<X>(
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

//#sec-string.fromcodepoint constructor.function
pub fn string_from_code_point<X>(
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

//#sec-string.prototype.at prototype.function
pub fn string_prototype_at<X>(
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

//#sec-string.prototype.charat prototype.function
pub fn string_prototype_char_at<X>(
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

//#sec-string.prototype.charcodeat prototype.function
pub fn string_prototype_char_code_at<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_char_code_at");
    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;
    let pos = context.args().first().unwrap_or(&Value::Undefined);
    let position = runtime.value_to_integer_or_infinity(pos)?;
    let size = s.len() as f64;
    if position < 0.0 || position >= size {
        return Ok(Value::Number(f64::NAN));
    }
    let code_unit = s.at(position as u32).unwrap();
    Ok(Value::Number(code_unit as f64))
}

//#sec-string.prototype.codepointat prototype.function
pub fn string_prototype_code_point_at<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_code_point_at");
    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;
    let pos = context.args().first().unwrap_or(&Value::Undefined);
    let position = runtime.value_to_integer_or_infinity(pos)?;
    let size = s.len() as f64;
    if position < 0.0 || position >= size {
        return Ok(Value::Undefined);
    }
    let result = s.code_point_at(position as u32);
    Ok(Value::Number(result.code_point as f64))
}

//#sec-string.prototype.concat prototype.function
pub fn string_prototype_concat<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_concat");
    let mut s = None;
    // TODO(refactor): process in the reverse order
    for arg in context.args().iter().rev() {
        let r = runtime.value_to_string(arg)?;
        s = if let Some(s) = s {
            Some(runtime.concat_strings(r, s))
        } else {
            Some(r)
        };
    }
    let o = context.this();
    require_object_coercible(o)?;
    let r = runtime.value_to_string(o)?;
    let s = if let Some(s) = s {
        runtime.concat_strings(r, s)
    } else {
        r
    };
    Ok(Value::String(s))
}

//#sec-string.prototype.indexof prototype.function
pub fn string_prototype_index_of<X>(
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
