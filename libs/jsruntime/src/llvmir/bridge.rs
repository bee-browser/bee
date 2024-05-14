#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

impl Default for Runtime {
    fn default() -> Self {
        Self {
            to_boolean: Some(runtime_to_boolean),
            to_numeric: Some(runtime_to_numeric),
        }
    }
}

unsafe extern "C" fn runtime_to_boolean(_: usize, value: *const Value) -> bool {
    let value = &*value;
    match value.kind {
        ValueKind_Undefined => false,
        ValueKind_Boolean => value.holder.boolean,
        ValueKind_Number if value.holder.number == 0.0 => false,
        ValueKind_Number if value.holder.number.is_nan() => false,
        ValueKind_Number => true,
        _ => panic!(),
    }
}

unsafe extern "C" fn runtime_to_numeric(_: usize, value: *const Value) -> f64 {
    let value = &*value;
    match value.kind {
        ValueKind_Undefined => f64::NAN,
        ValueKind_Boolean => {
            if value.holder.boolean {
                1.0
            } else {
                0.0
            }
        }
        ValueKind_Number => value.holder.number,
        _ => panic!(),
    }
}

impl crate::Value {
    pub fn load(value: *const Value) -> Self {
        unsafe {
            let value = &*value;
            match value.kind {
                ValueKind_Undefined => Self::Undefined,
                ValueKind_Boolean => Self::Boolean(value.holder.boolean),
                ValueKind_Number => Self::Number(value.holder.number),
                //ValueKind_Closure => Self::Closure(value.holder.closure.into()),
                _ => unreachable!("{}", value.kind),
            }
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self {
            kind: ValueKind_Undefined,
            holder: ValueHolder { opaque: 0 },
        }
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Self {
            kind: ValueKind_Boolean,
            holder: ValueHolder { boolean },
        }
    }
}

impl From<f64> for Value {
    fn from(number: f64) -> Self {
        Self {
            kind: ValueKind_Boolean,
            holder: ValueHolder { number },
        }
    }
}
