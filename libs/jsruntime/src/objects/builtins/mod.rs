mod aggregate_error;
mod error;
mod eval_error;
mod function;
mod internal_error;
mod object;
mod range_error;
mod reference_error;
mod string;
mod syntax_error;
mod type_error;
mod uri_error;

use jsparser::Symbol;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::objects::Property;
use crate::types::Lambda;
use crate::types::StringHandle;
use crate::types::Value;

#[allow(unused)]
enum Error {
    TypeError,
}

impl<X> Runtime<X> {
    // 19 The Global Object
    pub(crate) fn define_builtin_global_properties(&mut self) {
        macro_rules! define {
            ($key:expr => $value:expr,) => {
                define!(kv: $key, $value);
            };
            ($key:expr => $value:expr, $($keys:expr => $values:expr,)+) => {
                define!(kv: $key, $value);
                define!($($keys => $values,)+);
            };
            (kv: $key:expr, $value:expr) => {
                let prop = Property::data_xxx($value);
                let result = self.global_object.define_own_property($key.into(), prop);
                debug_assert!(matches!(result, Ok(true)));
            };
        }

        self.object_prototype = Some(self.create_object(None));
        self.function_prototype = Some(self.create_function_prototype());
        self.string_prototype = Some(self.create_string_prototype());
        self.error_prototype = Some(self.create_error_prototype());
        self.aggregate_error_prototype = Some(self.create_aggregate_error_prototype());
        self.eval_error_prototype = Some(self.create_eval_error_prototype());
        self.internal_error_prototype = Some(self.create_internal_error_prototype());
        self.range_error_prototype = Some(self.create_range_error_prototype());
        self.reference_error_prototype = Some(self.create_reference_error_prototype());
        self.syntax_error_prototype = Some(self.create_syntax_error_prototype());
        self.type_error_prototype = Some(self.create_type_error_prototype());
        self.uri_error_prototype = Some(self.create_uri_error_prototype());

        let this = self.global_object.as_handle();

        define! {
            // TODO: 19.1.1 globalThis
            Symbol::GLOBAL_THIS => Value::Object(this),
            // 19.1.2 Infinity
            Symbol::INFINITY => Value::Number(f64::INFINITY),
            // 19.1.3 NaN
            Symbol::NAN => Value::Number(f64::NAN),
            // 19.1.4 undefined
            Symbol::KEYWORD_UNDEFINED => Value::Undefined,
            // 19.3.1 AggregateError ( . . . )
            Symbol::AGGREGATE_ERROR => Value::Object(self.create_aggregate_error_constructor()),
            // 19.3.10 Error ( . . . )
            Symbol::ERROR => Value::Object(self.create_error_constructor()),
            // 19.3.11 EvalError ( . . . )
            Symbol::EVAL_ERROR => Value::Object(self.create_eval_error_constructor()),
            // 19.3.16 Function ( . . . )
            Symbol::FUNCTION => Value::Object(self.create_function_constructor()),
            // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/InternalError
            Symbol::INTERNAL_ERROR => Value::Object(self.create_internal_error_constructor()),
            // 19.3.23 Object()
            Symbol::OBJECT => Value::Object(self.create_object_constructor()),
            // 19.3.26 RangeError ( . . . )
            Symbol::RANGE_ERROR => Value::Object(self.create_range_error_constructor()),
            // 19.3.27 ReferenceError ( . . . )
            Symbol::REFERENCE_ERROR => Value::Object(self.create_reference_error_constructor()),
            // 19.3.31 String()
            Symbol::STRING => Value::Object(self.create_string_constructor()),
            // 19.3.33 SyntaxError ( . . . )
            Symbol::SYNTAX_ERROR => Value::Object(self.create_syntax_error_constructor()),
            // 19.3.34 TypeError ( . . . )
            Symbol::TYPE_ERROR => Value::Object(self.create_type_error_constructor()),
            // 19.3.39 URIError ( . . . )
            Symbol::URI_ERROR => Value::Object(self.create_uri_error_constructor()),
        }
    }

    fn create_builtin_function(
        &mut self,
        lambda: Lambda<X>,
        prototype: Option<ObjectHandle>,
    ) -> ObjectHandle {
        logger::debug!(event = "create_builtin_function");
        debug_assert!(self.function_prototype.is_some());
        let closure = self.create_closure(lambda, LambdaId::HOST, 0);
        let mut func = self.create_object(self.function_prototype);
        func.set_closure(closure);
        if let Some(prototype) = prototype {
            func.set_constructor();
            let _ = func.define_own_property(
                Symbol::PROTOTYPE.into(),
                Property::data_xxx(Value::Object(prototype)),
            );
        }
        func
    }

    // 7.1.4 ToNumber ( argument )
    // TODO: code clone, see backend::bridge::runtime_to_numeric
    fn value_to_number(&mut self, value: &Value) -> Result<f64, Error> {
        logger::debug!(event = "runtime.value_to_numeric", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined => Ok(f64::NAN),
            Value::Null => Ok(0.0),
            Value::Boolean(true) => Ok(1.0),
            Value::Boolean(false) => Ok(0.0),
            Value::Number(value) => Ok(*value),
            Value::String(_value) => todo!(),
            Value::Promise(_) => Ok(f64::NAN),
            // TODO(feat): 7.1.1 ToPrimitive()
            Value::Object(_) => Ok(f64::NAN),
        }
    }

    // 7.1.5 ToIntegerOrInfinity ( argument )
    fn value_to_integer_or_infinity(&mut self, value: &Value) -> Result<f64, Error> {
        logger::debug!(event = "runtime.value_to_integer_or_infinity", ?value);
        let number = self.value_to_number(value)?;
        if number.is_nan() || number == 0.0 || number == -0.0 {
            Ok(0.0)
        } else if number.is_infinite() {
            Ok(number)
        } else {
            Ok(number.trunc())
        }
    }

    // TODO(refactor): code clone, see runtime_concat_strings.
    fn concat_strings(&mut self, a: StringHandle, b: StringHandle) -> StringHandle {
        if b.is_empty() {
            return StringHandle::new(self.alloc_string_fragment_recursively(a.fragment(), None));
        }

        let b = if b.on_stack() {
            StringHandle::new(self.alloc_string_fragment_recursively(b.fragment(), None))
        } else {
            b
        };

        if a.is_empty() {
            return b;
        }

        StringHandle::new(self.alloc_string_fragment_recursively(a.fragment(), Some(b.fragment())))
    }

    // 7.1.17 ToString ( argument )
    // TODO: code clone, see backend::bridge::runtime_to_string
    fn value_to_string(&mut self, value: &Value) -> Result<StringHandle, Error> {
        logger::debug!(event = "runtime.value_to_string", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined => Ok(const_string!("undefined")),
            Value::Null => Ok(const_string!("null")),
            Value::Boolean(true) => Ok(const_string!("true")),
            Value::Boolean(false) => Ok(const_string!("false")),
            Value::Number(value) => {
                Ok(self.number_to_string(*value)) // TODO
            }
            Value::String(value) => Ok(*value),
            Value::Promise(_) => todo!(),
            Value::Object(value) => {
                let value = *value;
                if self.is_string_object(value) {
                    Ok(value.string())
                } else {
                    Ok(const_string!("[object Object]"))
                }
            }
        }
    }
}
