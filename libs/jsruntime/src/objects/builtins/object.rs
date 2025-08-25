use crate::Runtime;
use crate::types::CallContext;
use crate::types::Status;
use crate::types::Value;

pub extern "C" fn constructor<X>(
    runtime: &mut Runtime<X>,
    context: &mut CallContext,
    argc: u16,
    argv: *mut Value,
    retv: &mut Value,
) -> Status {
    let args = if argc == 0 {
        &[]
    } else {
        // SAFETY: `argv` is always non-null and a valid pointer to an array of `Value`s.
        unsafe {
            debug_assert!(!argv.is_null());
            debug_assert!(argv.is_aligned());
            std::slice::from_raw_parts(argv as *const Value, argc as usize)
        }
    };
    match runtime.object_constructor(context, args) {
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
    fn object_constructor(
        &mut self,
        _context: &mut CallContext,
        args: &[Value],
    ) -> Result<Value, Value> {
        let o = match args.first() {
            None | Some(Value::Undefined) | Some(Value::Null) => {
                self.create_object(self.object_prototype)
            }
            Some(_v) => todo!(), // TODO(feat): ToObject()
        };
        // TODO(feat): NewTarget
        Ok(Value::Object(o.as_ptr()))
    }
}
