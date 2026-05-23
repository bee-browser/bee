mod aggregate_error;
mod error;
mod eval_error;
mod function;
mod global;
mod internal_error;
mod object;
mod promise;
mod range_error;
mod reference_error;
mod string;
mod syntax_error;
mod type_error;
mod uri_error;

use jsgc::Handle;
use jsgc::HandleMut;
use jsgc::Heap;
use jsparser::Symbol;

use crate::Error;
use crate::ErrorKind;
use crate::Runtime;
use crate::lambda::LambdaId;
use crate::types::Lambda;
use crate::types::Object;
use crate::types::Property;
use crate::types::String;
use crate::types::Value;

logging::define_logger! {}

// The built-in objects are created in two-phase construction in order to avoid circular
// references.
//
// 1. Create an empty object for each built-in objects.
// 2. Initialize built-in objects.
//
#[derive(jsgc_derive::Trace)]
pub(crate) struct Builtins {
    // [[GlobalObject]]
    pub(crate) global_object: HandleMut<Object>,
    // %Object%
    pub(crate) object_constructor: HandleMut<Object>,
    // %Object.prototype%
    pub(crate) object_prototype: HandleMut<Object>,
    // %Function%
    pub(crate) function_constructor: HandleMut<Object>,
    // %Function.prototype%
    pub(crate) function_prototype: HandleMut<Object>,
    // %String%
    pub(crate) string_constructor: HandleMut<Object>,
    // %String.prototype%
    pub(crate) string_prototype: HandleMut<Object>,
    // %Promise%
    pub(crate) promise_constructor: HandleMut<Object>,
    // %Promise.prototype%
    pub(crate) promise_prototype: HandleMut<Object>,
    // %Error%
    pub(crate) error_constructor: HandleMut<Object>,
    // %Error.prototype%
    pub(crate) error_prototype: HandleMut<Object>,
    // %AggregateError%
    pub(crate) aggregate_error_constructor: HandleMut<Object>,
    // %AggregateError.prototype%
    pub(crate) aggregate_error_prototype: HandleMut<Object>,
    // %EvalError%
    pub(crate) eval_error_constructor: HandleMut<Object>,
    // %EvalError.prototype%
    pub(crate) eval_error_prototype: HandleMut<Object>,
    // %InternalError%
    pub(crate) internal_error_constructor: HandleMut<Object>,
    // %InternalError.prototype%
    pub(crate) internal_error_prototype: HandleMut<Object>,
    // %RangeError%
    pub(crate) range_error_constructor: HandleMut<Object>,
    // %RangeError.prototype%
    pub(crate) range_error_prototype: HandleMut<Object>,
    // %ReferenceError%
    pub(crate) reference_error_constructor: HandleMut<Object>,
    // %ReferenceError.prototype%
    pub(crate) reference_error_prototype: HandleMut<Object>,
    // %SyntaxError%
    pub(crate) syntax_error_constructor: HandleMut<Object>,
    // %SyntaxError.prototype%
    pub(crate) syntax_error_prototype: HandleMut<Object>,
    // %TypeError%
    pub(crate) type_error_constructor: HandleMut<Object>,
    // %TypeError.prototype%
    pub(crate) type_error_prototype: HandleMut<Object>,
    // %URIError%
    pub(crate) uri_error_constructor: HandleMut<Object>,
    // %URIError.prototype%
    pub(crate) uri_error_prototype: HandleMut<Object>,
}

impl Builtins {
    // The first phase of the two-phase construction.
    pub(crate) fn new(heap: &mut Heap) -> Self {
        Self {
            global_object: heap.alloc_mut(Object::new()),
            object_constructor: heap.alloc_mut(Object::new()),
            object_prototype: heap.alloc_mut(Object::new()),
            function_constructor: heap.alloc_mut(Object::new()),
            function_prototype: heap.alloc_mut(Object::new()),
            string_constructor: heap.alloc_mut(Object::new()),
            string_prototype: heap.alloc_mut(Object::new()),
            promise_constructor: heap.alloc_mut(Object::new()),
            promise_prototype: heap.alloc_mut(Object::new()),
            error_constructor: heap.alloc_mut(Object::new()),
            error_prototype: heap.alloc_mut(Object::new()),
            aggregate_error_constructor: heap.alloc_mut(Object::new()),
            aggregate_error_prototype: heap.alloc_mut(Object::new()),
            eval_error_constructor: heap.alloc_mut(Object::new()),
            eval_error_prototype: heap.alloc_mut(Object::new()),
            internal_error_constructor: heap.alloc_mut(Object::new()),
            internal_error_prototype: heap.alloc_mut(Object::new()),
            reference_error_constructor: heap.alloc_mut(Object::new()),
            reference_error_prototype: heap.alloc_mut(Object::new()),
            range_error_constructor: heap.alloc_mut(Object::new()),
            range_error_prototype: heap.alloc_mut(Object::new()),
            syntax_error_constructor: heap.alloc_mut(Object::new()),
            syntax_error_prototype: heap.alloc_mut(Object::new()),
            type_error_constructor: heap.alloc_mut(Object::new()),
            type_error_prototype: heap.alloc_mut(Object::new()),
            uri_error_constructor: heap.alloc_mut(Object::new()),
            uri_error_prototype: heap.alloc_mut(Object::new()),
        }
    }
}

impl<X> Runtime<X> {
    // The second phase of the two-phase construction.
    pub(crate) fn init_builtin_objects(&mut self) {
        self.init_intrinsic_objects();
        self.init_global_object();
    }

    fn init_intrinsic_objects(&mut self) {
        self.init_object_constructor();
        self.init_object_prototype();
        self.init_function_constructor();
        self.init_function_prototype();
        self.init_string_constructor();
        self.init_string_prototype();
        self.init_promise_constructor();
        self.init_promise_prototype();
        self.init_error_constructor();
        self.init_error_prototype();
        self.init_aggregate_error_constructor();
        self.init_aggregate_error_prototype();
        self.init_eval_error_constructor();
        self.init_eval_error_prototype();
        self.init_internal_error_constructor();
        self.init_internal_error_prototype();
        self.init_range_error_constructor();
        self.init_range_error_prototype();
        self.init_reference_error_constructor();
        self.init_reference_error_prototype();
        self.init_syntax_error_constructor();
        self.init_syntax_error_prototype();
        self.init_type_error_constructor();
        self.init_type_error_prototype();
        self.init_uri_error_constructor();
        self.init_uri_error_prototype();
    }

    fn create_builtin_function(&mut self, params: &BuiltinFunctionParams<X>) -> HandleMut<Object> {
        logger::debug!(
            event = "create_builtin_function",
            ?params.lambda,
            ?params.name,
            params.length,
            ?params.slots,
        );
        let func = self.create_object();
        self.init_builtin_function(func, params);
        func
    }

    fn init_builtin_function(
        &mut self,
        mut func: HandleMut<Object>,
        params: &BuiltinFunctionParams<X>,
    ) {
        let closure = self.create_closure(params.lambda, LambdaId::HOST, 0);
        func.set_prototype(self.builtins.function_prototype);
        func.slots_mut().extend_from_slice(params.slots);
        func.set_closure(closure);
        self.set_function_length(func, params.length);
        // TODO: prefix
        self.set_function_name(func, params.name);
    }

    //#sec-ordinaryobjectcreate
    fn ordinary_object_create(
        &mut self,
        proto: HandleMut<Object>,
        // additionalInternalSlotList
    ) -> HandleMut<Object> {
        let mut object = self.create_object();
        object.set_prototype(proto);
        object
    }

    //#sec-ordinarycreatefromconstructor
    fn ordinary_create_from_constructor(
        &mut self,
        constructor: HandleMut<Object>,
        intrinsic_default_proto: HandleMut<Object>,
        // internal_slot_list
    ) -> Result<HandleMut<Object>, Error> {
        let proto = self.get_prototype_from_constructor(constructor, intrinsic_default_proto)?;
        Ok(self.ordinary_object_create(proto))
    }

    //#sec-getprototypefromconstructor
    fn get_prototype_from_constructor(
        &self,
        constructor: HandleMut<Object>,
        intrinsic_default_proto: HandleMut<Object>,
    ) -> Result<HandleMut<Object>, Error> {
        match constructor.get_value(&Symbol::PROTOTYPE.into()) {
            Some(Value::Object(proto)) => Ok(*proto),
            _ => {
                // TODO(feat)
                // a. Let realm be ? GetFunctionRealm(constructor).
                // b. Set proto to realm's intrinsic object named intrinsicDefaultProto.
                Ok(intrinsic_default_proto)
            }
        }
    }

    // 10.2.9 SetFunctionName ( F, name [ , prefix ] )
    fn set_function_name(&mut self, mut func: HandleMut<Object>, name: Handle<String>) {
        let result =
            func.define_own_property(Symbol::NAME.into(), Property::data_xxc(Value::String(name)));
        debug_assert!(matches!(result, Ok(true)));
    }

    // 10.2.10 SetFunctionLength ( F, length )
    fn set_function_length(&mut self, mut func: HandleMut<Object>, length: u16) {
        let result = func.define_own_property(
            Symbol::LENGTH.into(),
            Property::data_xxc(Value::Number(length as f64)),
        );
        debug_assert!(matches!(result, Ok(true)));
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
            Value::String(_value) => runtime_todo!(),
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

    // 7.1.20 ToLength ( argument )
    fn value_to_length(&mut self, value: &Value) -> Result<u64, Error> {
        logger::debug!(event = "runtime.value_to_length", ?value);
        let len = self.value_to_integer_or_infinity(value)?;
        if len < 0.0 {
            Ok(0)
        } else {
            Ok(len.min(crate::types::number::MAX_SAFE_INTEGER) as u64)
        }
    }

    // TODO(refactor): code clone, see runtime_concat_strings.
    fn concat_strings(&mut self, a: Handle<String>, b: Handle<String>) -> Handle<String> {
        a.concat(b, &mut self.heap)
    }

    // 7.1.17 ToString ( argument )
    // TODO: code clone, see backend::bridge::runtime_to_string
    pub(crate) fn value_to_string(&mut self, value: &Value) -> Result<Handle<String>, Error> {
        logger::debug!(event = "runtime.value_to_string", ?value);
        match value {
            Value::None => unreachable!("Value::None"),
            Value::Undefined => Ok(const_string_handle!("undefined")),
            Value::Null => Ok(const_string_handle!("null")),
            Value::Boolean(true) => Ok(const_string_handle!("true")),
            Value::Boolean(false) => Ok(const_string_handle!("false")),
            Value::Number(value) => Ok(self.number_to_string(*value)),
            Value::String(value) => Ok(*value),
            // TODO(feat): Value::Symbol(_) => type_error!(),
            Value::Object(value) => self.object_to_string(*value),
        }
    }

    fn object_to_string(&mut self, object: HandleMut<Object>) -> Result<Handle<String>, Error> {
        // TODO(feat): ToPrimitive(object, STRING)
        if self.is_string_object(object) {
            Ok(object.string())
        } else {
            Ok(const_string_handle!("[object Object]"))
        }
    }

    pub(crate) fn create_exception(&mut self, err: Error) -> Value {
        let msg = err.message.map(Handle::from_ref);
        Value::Object(match err.kind {
            ErrorKind::SyntaxError => self.create_syntax_error(msg),
            ErrorKind::TypeError => self.create_type_error(msg),
            ErrorKind::RangeError => self.create_range_error(msg),
            ErrorKind::InternalError => self.create_internal_error(msg),
        })
    }

    fn make_string_filler(&mut self, fill_string: Handle<String>, fill_len: u32) -> Handle<String> {
        debug_assert!(!fill_string.is_empty());

        let fill_string_len = fill_string.len();
        let repetitions = fill_len / fill_string_len;
        let remaining = fill_len % fill_string_len;

        if repetitions == 0 {
            debug_assert!(remaining > 0);
            return fill_string.substring(0, remaining, &mut self.heap);
        }

        let string = fill_string.repeat(repetitions, &mut self.heap);
        if remaining == 0 {
            return string;
        }

        let last = fill_string.substring(0, remaining, &mut self.heap);
        string.concat(last, &mut self.heap)
    }

    fn repeat_string(&mut self, s: Handle<String>, n: u32) -> Handle<String> {
        if s.is_empty() || n == 1 {
            s
        } else {
            s.repeat(n, &mut self.heap)
        }
    }

    //#sec-lengthofarraylike
    fn length_of_array_like(&mut self, obj: HandleMut<Object>) -> Result<f64, Error> {
        logger::debug!(event = "runtime.length_of_array_like", ?obj);
        let value = obj
            .get_value(&Symbol::LENGTH.into())
            .unwrap_or(&Value::Undefined);
        Ok(self.value_to_length(value)? as f64)
    }

    //#sec-createlistfromarraylike
    fn create_vec_from_array_like(&mut self, obj: &Value) -> Result<Vec<Value>, Error> {
        logger::debug!(event = "runtime.create_vec_from_array_like", ?obj);
        // TODO(feat): validElementTypes
        let obj = match obj {
            Value::None => unreachable!(),
            Value::Object(v) => *v,
            _ => return type_error!(),
        };
        let len = self.length_of_array_like(obj)?;
        let mut values = Vec::with_capacity(len as usize);
        let mut index = 0.0;
        while index < len {
            let next = obj.get_value(&index.into()).unwrap_or(&Value::Undefined);
            // TODO(feat): validElementTypes
            values.push(next.clone());
            index += 1.0;
        }
        Ok(values)
    }
}

// 7.2.1 RequireObjectCoercible ( argument )
fn require_object_coercible(value: &Value) -> Result<(), Error> {
    match value {
        Value::None => unreachable!(),
        Value::Undefined | Value::Null => type_error!(),
        _ => Ok(()),
    }
}

struct BuiltinFunctionParams<'a, X> {
    lambda: Lambda<X>,
    #[allow(unused)]
    name: Handle<String>,
    length: u16,
    slots: &'a [Value],
}
