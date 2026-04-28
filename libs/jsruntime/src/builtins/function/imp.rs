//$id function
//$class Function
//$inherits object

use crate::Error;
use crate::Runtime;
use crate::logger;
use crate::types::CallContext;
use crate::types::Value;

//#sec-function-p1-p2-pn-body constructor
pub fn constructor<X>(
    _runtime: &mut Runtime<X>,
    _context: &mut CallContext,
) -> Result<Value, Error> {
    logger::debug!(event = "function");
    runtime_todo!("TODO: Function constructor")
}
