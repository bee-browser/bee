mod object;
mod string;

use jsparser::Symbol;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::logger;
use crate::objects::Property;
use crate::types::Lambda;
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

        self.object_prototype = self.create_object(std::ptr::null_mut()).as_ptr();
        self.function_prototype = self.create_object(self.object_prototype).as_ptr();
        self.string_prototype = self.create_string_prototype();

        let this = self.global_object.as_ptr();

        define! {
            // TODO: 19.1.1 globalThis
            Symbol::GLOBAL_THIS => Value::Object(this),
            // 19.1.2 Infinity
            Symbol::INFINITY => Value::Number(f64::INFINITY),
            // 19.1.3 NaN
            Symbol::NAN => Value::Number(f64::NAN),
            // 19.1.4 undefined
            Symbol::UNDEFINED => Value::Undefined,

            // 19.3.23 Object()
            Symbol::INTRINSIC_OBJECT => self.create_builtin_function(object::constructor::<X>, Value::Object(self.object_prototype)),

            // 19.3.31 String()
            Symbol::INTRINSIC_STRING => self.create_builtin_function(string::constructor::<X>, Value::Object(self.string_prototype)),
        }
    }

    fn create_builtin_function(&mut self, lambda: Lambda<X>, prototype: Value) -> Value {
        logger::debug!(event = "creater_builtin_function");
        debug_assert!(!self.function_prototype.is_null());
        let closure = self.create_closure(lambda, LambdaId::HOST, 0);
        let object = self.create_object(self.function_prototype);
        object.set_closure(closure);
        if prototype.is_valid() {
            let _ =
                object.define_own_property(Symbol::PROTOTYPE.into(), Property::data_xxx(prototype));
        }
        Value::Function(object.as_ptr())
    }

    // 7.1.4 ToNumber ( argument )
    // TODO: code clone, see backend::bridge::runtime_to_numeric
    fn value_to_number(&mut self, value: &Value) -> Result<f64, Error> {
        logger::debug!(event = "Runtime::to_numeric", ?value);
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
            Value::Object(_) | Value::Function(_) => Ok(f64::NAN),
        }
    }

    // 7.1.5 ToIntegerOrInfinity ( argument )
    fn value_to_integer_or_infinity(&mut self, value: &Value) -> Result<f64, Error> {
        let number = self.value_to_number(value)?;
        if number.is_nan() || number == 0.0 || number == -0.0 {
            Ok(0.0)
        } else if number.is_infinite() {
            Ok(number)
        } else {
            Ok(number.trunc())
        }
    }
}
