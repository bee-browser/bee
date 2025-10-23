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

//#sec-string.prototype.endswith prototype.function
pub fn string_prototype_ends_with<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_ends_with");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let search_str = args.first().unwrap_or(&Value::Undefined);
    // TODO(feat): RegExp
    let search_str = runtime.value_to_string(search_str)?;

    let len = s.len() as f64;

    let pos = if let Some(end_position) = args.get(1) {
        runtime.value_to_integer_or_infinity(end_position)?
    } else {
        len
    };

    let end = pos.clamp(0.0, len) as i64;

    let search_len = search_str.len() as i64;
    if search_len == 0 {
        return Ok(Value::Boolean(true));
    }

    let start = end - search_len;
    if start < 0 {
        return Ok(Value::Boolean(false));
    }

    // TODO(perf): inefficient
    let substring = s
        .code_units()
        .skip(start as usize)
        .take(search_len as usize);
    Ok(Value::Boolean(substring.eq(search_str.code_units())))
}

//#sec-string.prototype.includes prototype.function
pub fn string_prototype_includes<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_includes");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let search_str = args.first().unwrap_or(&Value::Undefined);
    // TODO(feat): RegExp
    let search_str = runtime.value_to_string(search_str)?;

    let position = args.get(1).unwrap_or(&Value::Undefined);
    let pos = runtime.value_to_integer_or_infinity(position)?;

    let len = s.len() as f64;

    let start = pos.clamp(0.0, len) as u32;
    let result = s.index_of(search_str, start).is_some();
    Ok(Value::Boolean(result))
}

//#sec-string.prototype.indexof prototype.function
pub fn string_prototype_index_of<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_index_of");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let search_str = args.first().unwrap_or(&Value::Undefined);
    let search_str = runtime.value_to_string(search_str)?;

    let position = args.get(1).unwrap_or(&Value::Undefined);
    let pos = runtime.value_to_integer_or_infinity(position)?;

    let len = s.len() as f64;

    let start = pos.clamp(0.0, len) as u32;
    let index = s.index_of(search_str, start).map_or(-1.0, |i| i as f64);
    Ok(Value::Number(index))
}

//#sec-string.prototype.iswellformed prototype.function
pub fn string_prototype_is_well_formed<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_is_well_formed");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    Ok(Value::Boolean(s.is_well_formed()))
}

//#sec-string.prototype.lastindexof prototype.function
pub fn string_prototype_last_index_of<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_last_index_of");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let search_str = args.first().unwrap_or(&Value::Undefined);
    let search_str = runtime.value_to_string(search_str)?;

    let position = args.get(1).unwrap_or(&Value::Undefined);
    let num_pos = runtime.value_to_number(position)?;
    let pos = if num_pos.is_nan() {
        f64::INFINITY
    } else {
        runtime.value_to_integer_or_infinity(&Value::Number(num_pos))?
    };

    let len = s.len() as f64;
    let search_len = search_str.len() as f64;

    if len < search_len {
        return Ok(Value::Number(-1.0));
    }

    let start = pos.clamp(0.0, len - search_len) as u32;
    let index = s
        .last_index_of(search_str, start)
        .map_or(-1.0, |i| i as f64);
    Ok(Value::Number(index))
}

//#sec-string.prototype.padend prototype.function
pub fn string_prototype_pad_end<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_pad_end");
    string_padding_builtins_impl(runtime, context, PaddingPlacement::End)
}

//#sec-string.prototype.padstart prototype.function
pub fn string_prototype_pad_start<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_pad_start");
    string_padding_builtins_impl(runtime, context, PaddingPlacement::Start)
}

// 22.1.3.17.1 StringPaddingBuiltinsImpl ( O, maxLength, fillString, placement )
fn string_padding_builtins_impl<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    placement: PaddingPlacement,
) -> Result<Value, Error> {
    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let max_length = args.first().unwrap_or(&Value::Undefined);
    let int_max_length = runtime.value_to_length(max_length)?;

    let string_length = s.len() as u64;
    if int_max_length <= string_length {
        return Ok(Value::String(s));
    }

    if int_max_length > u32::MAX as u64 {
        return Err(Error::InternalError);
    }

    let fill_string = args.get(1).unwrap_or(&Value::Undefined);
    let fill_string = match fill_string {
        Value::Undefined => StringHandle::SPACE,
        _ => runtime.value_to_string(fill_string)?,
    };

    string_pad(runtime, s, int_max_length as u32, fill_string, placement)
}

// 22.1.3.17.2 StringPad ( S, maxLength, fillString, placement )
fn string_pad<X>(
    runtime: &mut Runtime<X>,
    s: StringHandle,
    max_length: u32,
    fill_string: StringHandle,
    placement: PaddingPlacement,
) -> Result<Value, Error> {
    let string_length = s.len();
    if max_length <= string_length {
        return Ok(Value::String(s));
    }
    if fill_string.is_empty() {
        return Ok(Value::String(s));
    }
    let fill_len = max_length - string_length;
    let filler = runtime.make_string_filler(fill_string, fill_len);
    let result = match placement {
        PaddingPlacement::Start => runtime.concat_strings(filler, s),
        PaddingPlacement::End => runtime.concat_strings(s, filler),
    };
    Ok(Value::String(result))
}

enum PaddingPlacement {
    Start,
    End,
}

//#sec-string.prototype.repeat prototype.function
pub fn string_prototype_repeat<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_repeat");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let count = args.first().unwrap_or(&Value::Undefined);
    let n = runtime.value_to_integer_or_infinity(count)?;

    if n < 0.0 || n.is_infinite() {
        return Err(Error::RangeError);
    }

    if n == 0.0 {
        return Ok(Value::String(StringHandle::EMPTY));
    }

    if n > u8::MAX as f64 {
        return Err(Error::InternalError);
    }

    let result = runtime.repeat_string(s, n as u8);
    Ok(Value::String(result))
}

//#sec-string.prototype.startswith prototype.function
pub fn string_prototype_starts_with<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_starts_with");

    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let args = context.args();

    let search_str = args.first().unwrap_or(&Value::Undefined);
    // TODO(feat): RegExp
    let search_str = runtime.value_to_string(search_str)?;

    let len = s.len();

    let pos = if let Some(position) = args.get(1) {
        runtime.value_to_integer_or_infinity(position)?
    } else {
        0.0
    };

    let start = pos.clamp(0.0, len as f64) as u32;

    let search_len = search_str.len();
    if search_len == 0 {
        return Ok(Value::Boolean(true));
    }

    let end = start + search_len;
    if end > len {
        return Ok(Value::Boolean(false));
    }

    // TODO(perf): inefficient
    let substring = s
        .code_units()
        .skip(start as usize)
        .take(search_len as usize);
    Ok(Value::Boolean(substring.eq(search_str.code_units())))
}

//#sec-string.prototype.trim prototype.function
pub fn string_prototype_trim<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_trim");
    trim_string(runtime, context, true, true)
}

//#sec-string.prototype.trimend prototype.function
pub fn string_prototype_trim_end<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_trim_end");
    trim_string(runtime, context, false, true)
}

//#sec-string.prototype.trimstart prototype.function
pub fn string_prototype_trim_start<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "string_prototype_trim_start");
    trim_string(runtime, context, true, false)
}

// 22.1.3.32.1 TrimString ( string, where )
fn trim_string<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    start: bool,
    end: bool,
) -> Result<Value, Error> {
    let o = context.this();
    require_object_coercible(o)?;
    let s = runtime.value_to_string(o)?;

    let start_index = if start {
        match s.position(is_non_whitespace) {
            Some(index) => index,
            None => return Ok(Value::String(StringHandle::EMPTY)),
        }
    } else {
        0
    };

    let end_index = if end {
        match s.last_position(is_non_whitespace) {
            Some(index) => {
                debug_assert!(index < u32::MAX);
                index + 1
            }
            None => unreachable!(),
        }
    } else {
        s.len()
    };

    let result = runtime.create_substring(s, start_index, end_index);
    Ok(Value::String(result))
}

fn is_non_whitespace(cp: u32) -> bool {
    char::from_u32(cp)
        .map(|c| !char::is_whitespace(c))
        .unwrap_or(true)
}
