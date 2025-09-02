mod error;
mod function;
mod object;
mod string;

use base::utf16;
use jsparser::Symbol;

use crate::Runtime;
use crate::lambda::LambdaId;
use crate::logger;
use crate::objects::ObjectHandle;
use crate::objects::Property;
use crate::types::Lambda;
use crate::types::U16Chunk;
use crate::types::U16String;
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
        self.eval_error_prototype = Some(self.create_eval_error_prototype());
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
            // 19.3.10 Error ( . . . )
            Symbol::ERROR => self.create_builtin_function(
                error::constructor::<X>, self.error_prototype),
            // 19.3.11 EvalError ( . . . )
            Symbol::EVAL_ERROR => self.create_builtin_function(
                eval_error::constructor::<X>, self.eval_error_prototype),
            // 19.3.16 Function ( . . . )
            Symbol::FUNCTION => self.create_function_constructor(
                function::constructor::<X>, self.function_prototype),
            // 19.3.23 Object()
            Symbol::OBJECT => self.create_builtin_function(
                object::constructor::<X>, self.object_prototype),
            // 19.3.26 RangeError ( . . . )
            Symbol::RANGE_ERROR => self.create_builtin_function(
                range_error::constructor::<X>, self.range_error_prototype),
            // 19.3.27 ReferenceError ( . . . )
            Symbol::REFERENCE_ERROR => self.create_builtin_function(
                reference_error::constructor::<X>, self.reference_error_prototype),
            // 19.3.31 String()
            Symbol::STRING => self.create_builtin_function(
                string::constructor::<X>, self.string_prototype),
            // 19.3.33 SyntaxError ( . . . )
            Symbol::SYNTAX_ERROR => self.create_builtin_function(
                syntax_error::constructor::<X>, self.syntax_error_prototype),
            // 19.3.34 TypeError ( . . . )
            Symbol::TYPE_ERROR => self.create_builtin_function(
                type_error::constructor::<X>, self.type_error_prototype),
            // 19.3.39 URIError ( . . . )
            Symbol::URI_ERROR => self.create_builtin_function(
                uri_error::constructor::<X>, self.uri_error_prototype),
        }
    }

    fn create_builtin_function(
        &mut self,
        lambda: Lambda<X>,
        prototype: Option<ObjectHandle>,
    ) -> Value {
        logger::debug!(event = "creater_builtin_function");
        debug_assert!(self.function_prototype.is_some());
        let closure = self.create_closure(lambda, LambdaId::HOST, 0);
        let mut object = self.create_object(self.function_prototype);
        object.set_closure(closure);
        if let Some(prototype) = prototype {
            let _ = object.define_own_property(
                Symbol::PROTOTYPE.into(),
                Property::data_xxx(Value::Object(prototype)),
            );
        }
        Value::Function(object)
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
            Value::Object(_) | Value::Function(_) => Ok(f64::NAN),
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
    fn concat_strings(&mut self, a: U16String, b: U16String) -> U16String {
        if b.is_empty() {
            return U16String::new(self.alloc_string_rec(a.first_chunk(), std::ptr::null()));
        }

        let b = if b.on_stack() {
            U16String::new(self.alloc_string_rec(b.first_chunk(), std::ptr::null()))
        } else {
            b
        };

        if a.is_empty() {
            return b;
        }

        U16String::new(self.alloc_string_rec(a.first_chunk(), b.first_chunk() as *const U16Chunk))
    }

    // 7.1.17 ToString ( argument )
    // TODO: code clone, see backend::bridge::runtime_to_string
    fn value_to_string(&mut self, value: &Value) -> Result<U16String, Error> {
        logger::debug!(event = "runtime.value_to_string", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"undefined"));
                Ok(U16String::new(&CHUNK))
            }
            Value::Null => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"null"));
                Ok(U16String::new(&CHUNK))
            }
            Value::Boolean(true) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"true"));
                Ok(U16String::new(&CHUNK))
            }
            Value::Boolean(false) => {
                const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"false"));
                Ok(U16String::new(&CHUNK))
            }
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
                    const CHUNK: U16Chunk = U16Chunk::new_const(utf16!(&"[object Object]"));
                    Ok(U16String::new(&CHUNK))
                }
            }
            Value::Function(_) => todo!(),
        }
    }
}

macro_rules! define_error {
    ($name:ident, $create_prototype:ident, $constructor:ident, $symbol:ident) => {
        mod $name {
            use jsparser::Symbol;

            use crate::Runtime;
            use crate::U16Chunk;
            use crate::U16String;
            use crate::logger;
            use crate::objects::ObjectHandle;
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
                match runtime.$constructor(args, new) {
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

            impl<X> Runtime<X> {
                pub(super) fn $create_prototype(&mut self) -> ObjectHandle {
                    logger::debug!(event = stringify!($creater_prototype));
                    debug_assert!(self.error_prototype.is_some());

                    use jsparser::symbol::builtin::names;
                    const NAME: U16Chunk = U16Chunk::new_const(names::$symbol);

                    let mut prototype = self.create_object(self.error_prototype);
                    let _ = prototype.define_own_property(
                        Symbol::MESSAGE.into(),
                        Property::data_xxx(Value::String(U16String::EMPTY)),
                    );
                    let _ = prototype.define_own_property(
                        Symbol::NAME.into(),
                        Property::data_xxx(Value::String(U16String::new(&NAME))),
                    );

                    prototype
                }

                fn $constructor(&mut self, args: &[Value], new: bool) -> Result<Value, Value> {
                    logger::debug!(event = stringify!($constructor), ?args, new);
                    // TODO(feat): NewTarget
                    let object = self.create_object(self.eval_error_prototype);
                    Ok(Value::Object(object))
                }
            }
        }
    };
}

define_error! {
    eval_error,
    create_eval_error_prototype,
    range_eval_constructor,
    EVAL_ERROR
}

define_error! {
    range_error,
    create_range_error_prototype,
    range_error_constructor,
    RANGE_ERROR
}

define_error! {
    reference_error,
    create_reference_error_prototype,
    reference_error_constructor,
    REFERENCE_ERROR
}

define_error! {
    syntax_error,
    create_syntax_error_prototype,
    syntax_error_constructor,
    SYNTAX_ERROR
}

define_error! {
    type_error,
    create_type_error_prototype,
    type_error_constructor,
    TYPE_ERROR
}

define_error! {
    uri_error,
    create_uri_error_prototype,
    uri_error_constructor,
    URI_ERROR
}
