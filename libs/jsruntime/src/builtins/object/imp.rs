//$id object
//$class Object

use jsgc::HandleMut;
use jsparser::Symbol;

use crate::Error;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Property;
use crate::types::PropertyFlags;
use crate::types::PropertyKey;
use crate::types::Value;

use super::logger;

//#sec-object-value constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "object_constructor");

    match cc.new_target() {
        Some(new_target) if new_target != cc.func().unwrap() => Ok(Value::Object(
            runtime
                .ordinary_create_from_constructor(new_target, runtime.builtins.object_prototype)?,
        )),
        _ => match cc.args().first() {
            None | Some(Value::Undefined) | Some(Value::Null) => Ok(Value::Object(
                runtime.ordinary_object_create(runtime.builtins.object_prototype),
            )),
            Some(value) => Ok(Value::Object(runtime.value_to_object(value)?)),
        },
    }
}

//#sec-object.assign constructor.function
pub fn object_assign<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "object_assign");
    let target = cc.arg(0);
    let mut to = runtime.value_to_object(target)?;
    // TODO(feat): `sources` is a rest parameter.
    for arg in cc.args().iter().skip(1) {
        match arg {
            Value::None => unreachable!(),
            Value::Null | Value::Undefined => continue,
            _ => {
                let from = runtime.value_to_object(arg)?;
                for (key, prop) in from.iter_own_properties() {
                    if prop.is_enumerable() {
                        to.set_value(key, prop.value());
                    }
                }
            }
        }
    }
    Ok(Value::Object(to))
}

//#sec-object.create constructor.function
pub fn object_create<X>(runtime: &mut Runtime<X>, cc: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "object_create");
    let proto = cc.arg(0);
    match proto {
        Value::None => unreachable!(),
        Value::Null | Value::Object(_) => (),
        _ => return type_error!("Object prototype may only be an Object or null"),
    }
    let mut obj = runtime.create_object();
    if let Value::Object(proto) = proto {
        obj.set_prototype(*proto);
    }
    let properties = cc.arg(1);
    match properties {
        Value::None => unreachable!(),
        Value::Undefined => Ok(Value::Object(obj)),
        _ => Ok(Value::Object(
            runtime.object_define_properties(obj, properties)?,
        )),
    }
}

//#sec-object.defineproperties constructor.function
pub fn object_define_properties<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_define_properties");
    let obj = match cc.arg(0) {
        Value::None => unreachable!(),
        Value::Object(object) => *object,
        _ => return type_error!("Object.defineProperties called on non-object"),
    };
    Ok(Value::Object(
        runtime.object_define_properties(obj, cc.arg(1))?,
    ))
}

//#sec-object.defineproperty constructor.function
pub fn object_define_property<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_define_property");
    let mut obj = match cc.arg(0) {
        Value::None => unreachable!(),
        Value::Object(object) => *object,
        _ => return type_error!("Object.defineProperty called on non-object"),
    };
    let key = runtime.value_to_property_key(cc.arg(1))?;
    let prop = runtime.value_to_property(cc.arg(2))?;
    obj.define_own_property(key, prop)?;
    Ok(Value::Object(obj))
}

//#sec-object.prototype.hasownproperty prototype.function
pub fn object_prototype_has_own_property<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_prototype_has_own_property");
    let key = runtime.value_to_property_key(cc.arg(0))?;
    let obj = runtime.value_to_object(cc.this())?;
    match obj.get_own_property(&key) {
        Some(_) => Ok(Value::TRUE),
        None => Ok(Value::FALSE),
    }
}

//#sec-object.prototype.isprototypeof prototype.function
pub fn object_prototype_is_prototype_of<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_prototype_is_prototype_of");
    let mut value = match cc.arg(0) {
        Value::Object(v) => *v,
        _ => return Ok(Value::FALSE),
    };
    let obj = runtime.value_to_object(cc.this())?;
    loop {
        value = match value.prototype() {
            Some(v) => v,
            None => return Ok(Value::FALSE),
        };
        if obj == value {
            return Ok(Value::TRUE);
        }
    }
}

//#sec-object.prototype.propertyisenumerable prototype.function
pub fn object_prototype_property_is_enumerable<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_prototype_property_is_enumerable");
    let key = runtime.value_to_property_key(cc.arg(0))?;
    let obj = runtime.value_to_object(cc.this())?;
    match obj.get_own_property(&key) {
        Some(prop) => Ok(Value::Boolean(prop.is_enumerable())),
        None => Ok(Value::FALSE),
    }
}

//#sec-object.prototype.tostring prototype.function
pub fn object_prototype_to_string<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_prototype_to_string");
    match cc.this() {
        Value::None => unreachable!(),
        Value::Undefined => Ok(Value::String(const_string_handle!("[object Undefined]"))),
        Value::Null => Ok(Value::String(const_string_handle!("[object Null]"))),
        this => {
            let obj = runtime.value_to_object(this)?;
            // TODO(feat): IsArray(obj), "[object Array]"
            // TODO(feat): "[object Arguments]"
            if obj.is_callable() {
                Ok(Value::String(const_string_handle!("[object Function]")))
            } else if obj.is_error() {
                Ok(Value::String(const_string_handle!("[object Error]")))
            } else {
                // TODO(feat): "[object Boolean]"
                // TODO(feat): "[object Number]"
                // TODO(feat): "[object String]"
                // TODO(feat): "[object Date]"
                // TODO(feat): "[object RegExp]"
                // TODO(feat): Get(obj, %Symbol.toStringTag%)
                Ok(Value::String(const_string_handle!("[object Object]")))
            }
        }
    }
}

//#sec-object.prototype.valueof prototype.function
pub fn object_prototype_value_of<X>(
    runtime: &mut Runtime<X>,
    cc: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_prototype_value_of");
    Ok(Value::Object(runtime.value_to_object(cc.this())?))
}

// helpers

impl<X> Runtime<X> {
    // 20.1.2.3.1 ObjectDefineProperties ( obj, properties )
    fn object_define_properties(
        &mut self,
        mut obj: HandleMut<Object>,
        properties: &Value,
    ) -> Result<HandleMut<Object>, Error> {
        let props = self.value_to_object(properties)?;
        for (key, prop) in props.iter_own_properties() {
            let desc_obj = prop.value();
            if !matches!(desc_obj, Value::Undefined) && prop.is_enumerable() {
                let new_prop = self.value_to_property(desc_obj)?;
                obj.define_own_property(key.clone(), new_prop)?;
            }
        }
        Ok(obj)
    }

    // 7.1.19 ToPropertyKey ( argument )
    fn value_to_property_key(&mut self, value: &Value) -> Result<PropertyKey, Error> {
        // TODO: ToPrimitive(value, STRING)
        let string = self.value_to_string(value)?;
        // TODO(perf): inefficient
        let symbol = self.symbol_registry.intern_utf16(string.make_utf16());
        Ok(symbol.into())
    }

    // 6.2.6.5 ToPropertyDescriptor ( obj )
    fn value_to_property(&mut self, desc: &Value) -> Result<Property, Error> {
        let obj = match desc {
            Value::None => unreachable!(),
            Value::Object(obj) => *obj,
            _ => return type_error!(),
        };

        let mut flags = PropertyFlags::empty();
        let mut value = Value::None;

        if let Some(v) = obj.get_value(&Symbol::ENUMERABLE.into()) {
            if v.to_boolean() {
                flags |= PropertyFlags::ENUMERABLE;
            }
        }

        if let Some(v) = obj.get_value(&Symbol::CONFIGURABLE.into()) {
            if v.to_boolean() {
                flags |= PropertyFlags::CONFIGURABLE;
            }
        }

        if let Some(v) = obj.get_value(&Symbol::VALUE.into()) {
            value = v.clone();
        }

        if let Some(v) = obj.get_value(&Symbol::WRITABLE.into()) {
            if v.to_boolean() {
                flags |= PropertyFlags::WRITABLE;
            }
        }

        let getter = if let Some(v) = obj.get_value(&Symbol::GET.into()) {
            match v {
                Value::None => unreachable!(),
                Value::Undefined => None,
                Value::Object(getter) if getter.is_callable() => Some(*getter),
                _ => return type_error!("Getter must be callable"),
            }
        } else {
            None
        };

        let setter = if let Some(v) = obj.get_value(&Symbol::SET.into()) {
            match v {
                Value::None => unreachable!(),
                Value::Undefined => None,
                Value::Object(setter) if setter.is_callable() => Some(*setter),
                _ => return type_error!("Setter must be callable"),
            }
        } else {
            None
        };

        if getter.is_some() || setter.is_some() {
            if value.is_valid() || flags.contains(PropertyFlags::WRITABLE) {
                return type_error!();
            }
            return runtime_todo!("TODO: accessor");
        }

        if !value.is_valid() {
            value = Value::Undefined;
        }

        Ok(Property::data(value, flags))
    }
}
