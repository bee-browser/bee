//$id function
//$class Function
//$inherits object

use crate::Error;
use crate::Runtime;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

use super::logger;

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
    let func = match context.this() {
        Value::Object(v) if v.is_callable() => *v,
        _ => {
            *retv = Value::Object(runtime.create_type_error(None));
            return Status::Exception;
        }
    };
    let this = context.arg(0);
    match context.arg(1) {
        Value::None => unreachable!(),
        Value::Undefined | Value::Null => {
            // TODO: PrepareForTailCall()
            runtime.call(context, func, this, &mut [], retv)
        }
        args => {
            let mut args = match runtime.create_vec_from_array_like(args) {
                Ok(args) => args,
                Err(err) => {
                    *retv = runtime.create_exception(err);
                    return Status::Exception;
                }
            };
            // TODO: PrepareForTailCall()
            runtime.call(context, func, this, &mut args, retv)
        }
    }
}
