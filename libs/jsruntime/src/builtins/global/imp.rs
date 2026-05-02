//$id global

use jsparser::Symbol;

use crate::Error;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Value;
use crate::types::object::Property;

//#sec-globalthis global.property
pub fn global_global_this<X>(runtime: &mut Runtime<X>) {
    let prop = Property::data_wxc(Value::Object(runtime.builtins.global_object));
    runtime.define_global_property(Symbol::GLOBAL_THIS, prop);
}

//#sec-value-properties-of-the-global-object-infinity global.property
pub fn global_infinity<X>(runtime: &mut Runtime<X>) {
    let prop = Property::data_xxx(Value::Number(f64::INFINITY));
    runtime.define_global_property(Symbol::INFINITY, prop);
}

//#sec-value-properties-of-the-global-object-nan global.property
pub fn global_nan<X>(runtime: &mut Runtime<X>) {
    let prop = Property::data_xxx(Value::Number(f64::NAN));
    runtime.define_global_property(Symbol::NAN, prop);
}

//#sec-undefined global.property
pub fn global_undefined<X>(runtime: &mut Runtime<X>) {
    let prop = Property::data_xxx(Value::Undefined);
    runtime.define_global_property(Symbol::KEYWORD_UNDEFINED, prop);
}

//#sec-eval-x global.function
pub fn eval<X>(_runtime: &mut Runtime<X>, _context: &mut CallContext) -> Result<Value, Error> {
    runtime_todo!("eval: not implemented")
}

//#sec-isfinite-number global.function
pub fn is_finite<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    let number = context.args().first().unwrap_or(&Value::Undefined);
    let num = runtime.value_to_number(number)?;
    Ok(Value::Boolean(num.is_finite()))
}

//#sec-isnan-number global.function
pub fn is_nan<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    let number = context.args().first().unwrap_or(&Value::Undefined);
    let num = runtime.value_to_number(number)?;
    Ok(Value::Boolean(num.is_nan()))
}

//#sec-parsefloat-string global.function
pub fn parse_float<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    let string = context.args().first().unwrap_or(&Value::Undefined);
    let input_string = runtime.value_to_string(string)?;
    let trimmed_string = runtime.trim_string(input_string, true, false)?;
    // TODO: 11.1.6 Static Semantics: ParseText ( sourceText, goalSymbol )
    let utf8 = char::decode_utf16(trimmed_string.code_units())
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect::<std::string::String>();
    match utf8.parse::<f64>() {
        Ok(n) => Ok(Value::Number(n)),
        Err(_) => syntax_error!(),
    }
}

//#sec-parseint-string-radix global.function
pub fn parse_int<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    // TODO: impl
    let string = context.args().first().unwrap_or(&Value::Undefined);
    let input_string = runtime.value_to_string(string)?;
    let s = runtime.trim_string(input_string, true, false)?;
    let radix = context.args().get(1).unwrap_or(&Value::Undefined);
    let radix = runtime.value_to_number(radix)?;
    let radix = if radix.is_finite() { radix as i32 } else { 10 };
    if !(2..=36).contains(&radix) {
        return Ok(Value::Number(f64::NAN));
    }
    let utf8 = char::decode_utf16(s.code_units())
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect::<std::string::String>();
    match i64::from_str_radix(&utf8, radix as u32) {
        Ok(n) => Ok(Value::Number(n as f64)),
        Err(_) => Ok(Value::Number(f64::NAN)),
    }
}

//#sec-constructor-properties-of-the-global-object-aggregate-error global.constructor
pub fn define_aggregate_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.aggregate_error_constructor;
    runtime.define_constructor(Symbol::AGGREGATE_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-error global.constructor
pub fn define_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.error_constructor;
    runtime.define_constructor(Symbol::ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-evalerror global.constructor
pub fn define_eval_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.eval_error_constructor;
    runtime.define_constructor(Symbol::EVAL_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-function global.constructor
pub fn define_function_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.function_constructor;
    runtime.define_constructor(Symbol::FUNCTION, constructor);
}

//#_internalerror global.constructor { "name": "InternalError" }
pub fn define_internal_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.internal_error_constructor;
    runtime.define_constructor(Symbol::INTERNAL_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-object global.constructor
pub fn define_object_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.object_constructor;
    runtime.define_constructor(Symbol::OBJECT, constructor);
}

//#sec-constructor-properties-of-the-global-object-promise global.constructor
pub fn define_promise_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.promise_constructor;
    runtime.define_constructor(Symbol::PROMISE, constructor);
}

//#sec-constructor-properties-of-the-global-object-rangeerror global.constructor
pub fn define_range_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.range_error_constructor;
    runtime.define_constructor(Symbol::RANGE_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-referenceerror global.constructor
pub fn define_reference_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.reference_error_constructor;
    runtime.define_constructor(Symbol::REFERENCE_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-string global.constructor
pub fn define_string_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.string_constructor;
    runtime.define_constructor(Symbol::STRING, constructor);
}

//#sec-constructor-properties-of-the-global-object-syntaxerror global.constructor
pub fn define_syntax_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.syntax_error_constructor;
    runtime.define_constructor(Symbol::SYNTAX_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-typeerror global.constructor
pub fn define_type_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.type_error_constructor;
    runtime.define_constructor(Symbol::TYPE_ERROR, constructor);
}

//#sec-constructor-properties-of-the-global-object-urierror global.constructor
pub fn define_uri_error_constructor<X>(runtime: &mut Runtime<X>) {
    let constructor = runtime.builtins.uri_error_constructor;
    runtime.define_constructor(Symbol::URI_ERROR, constructor);
}
