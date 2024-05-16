#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

impl Locator {
    pub const NONE: Self = Self::new(LocatorKind_None, 0, 0);

    const MAX_OFFSET: usize = u8::MAX as usize;
    const MAX_INDEX: usize = u16::MAX as usize;

    pub(crate) fn checked_argument(offset: usize, index: usize) -> Option<Self> {
        Self::checked_new(LocatorKind_Argument, offset, index)
    }

    pub(crate) fn checked_local(offset: usize, index: usize) -> Option<Self> {
        Self::checked_new(LocatorKind_Local, offset, index)
    }

    pub(crate) const fn local(offset: u8, index: u16) -> Self {
        Self::new(LocatorKind_Local, offset, index)
    }

    const fn new(kind: LocatorKind, offset: u8, index: u16) -> Self {
        Self {
            offset,
            kind,
            index,
        }
    }

    fn checked_new(kind: LocatorKind, offset: usize, index: usize) -> Option<Self> {
        if offset > Self::MAX_OFFSET {
            crate::logger::error!(err = "too large", offset);
            return None;
        }
        if index > Self::MAX_INDEX {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(kind, offset as u8, index as u16))
    }
}

impl std::fmt::Debug for Locator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let offset = self.offset;
        let index = self.index;
        match self.kind {
            LocatorKind_None => write!(f, "Locator::None"),
            LocatorKind_Argument => write!(f, "Locator::Argument({offset}, {index})"),
            LocatorKind_Local => write!(f, "Locator::Local({offset}, {index})"),
            _ => unreachable!(),
        }
    }
}

impl crate::Value {
    pub(crate) unsafe fn load(value: *const Value) -> Self {
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
