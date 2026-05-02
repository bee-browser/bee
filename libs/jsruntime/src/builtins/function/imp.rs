//$id function
//$class Function
//$inherits object

use jsgc::HandleMut;
use jsparser::Symbol;

use crate::Error;
use crate::LambdaId;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Closure;
use crate::types::Object;
use crate::types::Status;
use crate::types::Value;

use super::logger;

macro_rules! catch {
    ($result:expr; $runtime:expr, $retv:expr) => {
        match $result {
            Ok(v) => v,
            Err(err) => {
                *$retv = $runtime.create_exception(err);
                return Status::Exception;
            }
        }
    };
}

//#sec-function-p1-p2-pn-body constructor
pub fn constructor<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "function");
    runtime_todo!("TODO: Function constructor")
}

//#sec-function.prototype.apply prototype.function { "no_adapter": true }
pub fn function_prototype_apply<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "function_prototype_apply");
    let func = catch!(runtime.this_func(context.this()); runtime, retv);
    let this = context.arg(0);
    match context.arg(1) {
        Value::None => unreachable!(),
        Value::Undefined | Value::Null => {
            // TODO: PrepareForTailCall()
            runtime.call(context, func, this, &[], retv)
        }
        args => {
            let args = catch!(runtime.create_vec_from_array_like(args); runtime, retv);
            // TODO: PrepareForTailCall()
            runtime.call(context, func, this, &args, retv)
        }
    }
}

//#sec-function.prototype.bind prototype.function
pub fn function_prototype_bind<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "function_prototype_bind");
    let target = runtime.this_func(context.this())?;
    let this = context.arg(0);
    let args = if context.args().len() > 1 {
        &context.args()[1..]
    } else {
        &[]
    };
    let bound_func = runtime.bound_function_create(target, this, args)?;
    let length = match target.get_own_property(&Symbol::LENGTH.into()) {
        Some(prop) => {
            let target_length = runtime.value_to_length(prop.value())?;
            if target_length > args.len() as u64 {
                target_length - args.len() as u64
            } else {
                0
            }
        }
        None => 0,
    };
    debug_assert!(length <= u16::MAX as u64);
    runtime.set_function_length(bound_func, length as u16);
    let target_name = target
        .get_value(&Symbol::NAME.into())
        .unwrap_or(&Value::Undefined);
    if let Value::String(name) = target_name {
        runtime.set_function_name(bound_func, *name);
    }
    Ok(Value::Object(bound_func))
}

//#sec-function.prototype.call prototype.function { "no_adapter": true }
pub fn function_prototype_call<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    retv: &mut Value,
) -> Status {
    logger::debug!(event = "function_prototype_call");
    let func = catch!(runtime.this_func(context.this()); runtime, retv);
    let this = context.arg(0);
    let args = if context.args().len() > 1 {
        &context.args()[1..]
    } else {
        &[]
    };
    // TODO: PrepareForTailCall()
    runtime.call(context, func, this, args, retv)
}

//#sec-function.prototype.tostring prototype.function
pub fn function_prototype_to_string<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "function_prototype_to_string");
    runtime_todo!("TODO: Function.prototype.toString()")
}

impl<X> Runtime<X> {
    fn this_func(&mut self, this: &Value) -> Result<HandleMut<Object>, Error> {
        match this {
            Value::None => unreachable!(),
            Value::Object(v) if v.is_callable() => Ok(*v),
            _ => type_error!(),
        }
    }

    // #sec-boundfunctioncreate
    fn bound_function_create(
        &mut self,
        func: HandleMut<Object>,
        this: &Value,
        args: &[Value],
    ) -> Result<HandleMut<Object>, Error> {
        extern "C" fn bound_function<X>(
            runtime: &mut Runtime<X>,
            context: &mut CallContext,
            retv: &mut Value,
        ) -> Status {
            let closure = context.closure();
            let (func, this, mut args) = closure.get_bound_function_params();
            args.extend_from_slice(context.args());
            // TODO(feat): [[Construct]], newTarget
            runtime.call(context, func, this, &args, retv)
        }

        let prototype = func.prototype();
        let mut obj = self.create_object();
        if let Some(prototype) = prototype {
            obj.set_prototype(prototype);
        }

        let num_captures = 2 + args.len() as u16;
        let mut closure = self.create_closure(bound_function::<X>, LambdaId::HOST, num_captures);

        let mut func = Value::Object(func);
        let mut capture = self.create_capture(&mut func as *mut Value);
        capture.escape();
        closure.put_capture(0, capture);

        let mut this = this.clone();
        let mut capture = self.create_capture(&mut this as *mut Value);
        capture.escape();
        closure.put_capture(1, capture);

        for (i, arg) in args.iter().enumerate() {
            let mut arg = arg.clone();
            let mut capture = self.create_capture(&mut arg as *mut Value);
            capture.escape();
            closure.put_capture(2 + i, capture);
        }

        obj.set_closure(closure);

        Ok(obj)
    }
}

impl Closure {
    fn get_bound_function_params(&self) -> (HandleMut<Object>, &Value, Vec<Value>) {
        let captures = self.captures();
        debug_assert!(captures.len() >= 2);
        let func = match captures.first().expect("[[BoundTargetFunction]]").value() {
            Value::Object(v) => *v,
            _ => unreachable!(),
        };
        let this = captures.get(1).expect("[[BoundThis]]").value();
        let args: Vec<Value> = captures[2..]
            .iter()
            .map(|capture| capture.value().clone())
            .collect();
        (func, this, args)
    }
}
