use std::ffi::c_void;

use crate::Runtime;
use crate::U16String;
use crate::types::Status;
use crate::types::Value;

pub unsafe extern "C" fn constructor<X>(
    runtime: *mut c_void,
    _context: *mut c_void,
    this: *mut Value,
    argc: u16,
    argv: *mut Value,
    retv: *mut Value,
) -> Status {
    let runtime = unsafe { &mut *(runtime as *mut Runtime<X>) };
    let this = unsafe { &mut *this };
    let args = if argc == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(argv as *const Value, argc as usize) }
    };
    let retv = unsafe { &mut *retv };
    match runtime.string_constructor(this, args) {
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
    fn string_constructor(&mut self, _this: &mut Value, args: &[Value]) -> Result<Value, Value> {
        let s = match args.first() {
            Some(v) => self.perform_to_string(v),
            None => U16String::EMPTY,
        };
        // TODO(feat): NewTarget
        Ok(Value::String(s))
    }
}
