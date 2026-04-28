//$id object
//$class Object

use jsgc::HandleMut;
use jsparser::Symbol;

use crate::Error;
use crate::Runtime;
use crate::logger;
use crate::types::CallContext;
use crate::types::Object;
use crate::types::Property;
use crate::types::PropertyFlags;
use crate::types::Value;

//#sec-object-value constructor
pub fn constructor<X>(runtime: &mut Runtime<X>, context: &mut CallContext) -> Result<Value, Error> {
    logger::debug!(event = "object");
    let this = context.this();
    let args = context.args();
    let new = context.is_new();
    runtime.create_object_object(Some(this), args, new)
}

//#sec-object.assign constructor.function
pub fn object_assign<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_assign");
    let target = context.arg(0);
    let mut to = match runtime.value_to_object(target)? {
        Value::Object(object) => object,
        _ => unreachable!(),
    };
    // TODO(feat): `sources` is a rest parameter.
    for arg in context.args().iter().skip(1) {
        match arg {
            Value::None => unreachable!(),
            Value::Null | Value::Undefined => continue,
            _ => {
                let from = match runtime.value_to_object(arg)? {
                    Value::Object(object) => object,
                    _ => unreachable!(),
                };
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
pub fn object_create<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_create");
    let proto = context.arg(0);
    match proto {
        Value::None => unreachable!(),
        Value::Null | Value::Object(_) => (),
        _ => return type_error!("Object prototype may only be an Object or null"),
    }
    let mut obj = runtime.create_object();
    if let Value::Object(proto) = proto {
        obj.set_prototype(*proto);
    }
    let properties = context.arg(1);
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
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "object_define_properties");
    let obj = match context.arg(0) {
        Value::None => unreachable!(),
        Value::Object(object) => *object,
        _ => return type_error!("Object.defineProperties called on non-object"),
    };
    Ok(Value::Object(
        runtime.object_define_properties(obj, context.arg(1))?,
    ))
}

// helpers

impl<X> Runtime<X> {
    // 20.1.2.3.1 ObjectDefineProperties ( obj, properties )
    fn object_define_properties(
        &mut self,
        mut obj: HandleMut<Object>,
        properties: &Value,
    ) -> Result<HandleMut<Object>, Error> {
        let props = match self.value_to_object(properties)? {
            Value::Object(object) => object,
            _ => unreachable!(),
        };
        for (key, prop) in props.iter_own_properties() {
            let desc_obj = prop.value();
            if !matches!(desc_obj, Value::Undefined) && prop.is_enumerable() {
                let new_prop = self.property_descriptor_to_property(desc_obj)?;
                obj.define_own_property(key.clone(), new_prop)?;
            }
        }
        Ok(obj)
    }

    // 6.2.6.5 ToPropertyDescriptor ( obj )
    fn property_descriptor_to_property(&mut self, desc: &Value) -> Result<Property, Error> {
        let obj = match desc {
            Value::None => unreachable!(),
            Value::Object(obj) => *obj,
            _ => return type_error!(),
        };

        let mut flags = PropertyFlags::empty();
        let mut value = Value::Undefined;

        if let Some(v) = obj.get_value(&Symbol::ENUMERABLE.into()) {
            if self.value_to_boolean(v) {
                flags |= PropertyFlags::ENUMERABLE;
            }
        }

        if let Some(v) = obj.get_value(&Symbol::CONFIGURABLE.into()) {
            if self.value_to_boolean(v) {
                flags |= PropertyFlags::CONFIGURABLE;
            }
        }

        if let Some(v) = obj.get_value(&Symbol::VALUE.into()) {
            value = v.clone();
        }

        if let Some(v) = obj.get_value(&Symbol::WRITABLE.into()) {
            if self.value_to_boolean(v) {
                flags |= PropertyFlags::WRITABLE;
            }
        }

        if let Some(_v) = obj.get_value(&Symbol::GET.into()) {
            return runtime_todo!("TODO: get");
        }

        if let Some(_v) = obj.get_value(&Symbol::SET.into()) {
            return runtime_todo!("TODO: get");
        }

        Ok(Property::data(value, flags))
    }

    pub(crate) fn create_object_object(
        &mut self,
        this: Option<&Value>,
        args: &[Value],
        new: bool,
    ) -> Result<Value, Error> {
        // TODO(feat): NewTarget
        if new {
            let object = if let Some(&Value::Object(this)) = this {
                this
            } else {
                // 10.4.3.4 StringCreate ( value, prototype )
                let mut object = self.create_object();
                object.set_prototype(self.builtins.string_prototype);
                object
            };
            Ok(Value::Object(object))
        } else {
            match args.first() {
                None | Some(Value::Undefined) | Some(Value::Null) => {
                    let mut object = self.create_object();
                    object.set_prototype(self.builtins.object_prototype);
                    // TODO(feat): NewTarget
                    Ok(Value::Object(object))
                }
                Some(value) => self.value_to_object(value),
            }
        }
    }
}
