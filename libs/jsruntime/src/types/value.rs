use std::ops::Deref;

use jsgc::Handle;
use jsgc::HandleMut;

use crate::logger;
use crate::types::Object;
use crate::types::Status;
use crate::types::String;

/// A data type to hold a JavaScript value.
//
// DO NOT CHANGE THE ORDER OF THE VARIANTS.
// Some operations heavily rely on the order.
#[repr(C, u8)]
#[derive(Clone, Debug)]
pub enum Value {
    None = Self::KIND_NONE,
    Undefined = Self::KIND_UNDEFINED,
    Null = Self::KIND_NULL,
    Boolean(bool) = Self::KIND_BOOLEAN,
    Number(f64) = Self::KIND_NUMBER,
    String(Handle<String>) = Self::KIND_STRING,
    Object(HandleMut<Object>) = Self::KIND_OBJECT,
}

base::static_assert_eq!(size_of::<Value>(), 16);
base::static_assert_eq!(align_of::<Value>(), 8);

impl Value {
    // There is no way to define const function to extract the discriminant of each variant.
    pub(crate) const KIND_NONE: u8 = 0;
    pub(crate) const KIND_UNDEFINED: u8 = 1;
    pub(crate) const KIND_NULL: u8 = 2;
    pub(crate) const KIND_BOOLEAN: u8 = 3;
    pub(crate) const KIND_NUMBER: u8 = 4;
    pub(crate) const KIND_STRING: u8 = 5;
    pub(crate) const KIND_OBJECT: u8 = 6;

    pub(crate) const SIZE: usize = size_of::<Self>();
    pub(crate) const ALIGNMENT: usize = align_of::<Self>();
    pub(crate) const HOLDER_SIZE: usize = size_of::<u64>();
    pub(crate) const KIND_OFFSET: usize = 0;
    pub(crate) const HOLDER_OFFSET: usize = size_of::<u64>();

    pub fn is_valid(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn is_callable(&self) -> bool {
        debug_assert!(self.is_valid());
        match self {
            Self::Object(object) => object.is_callable(),
            _ => false,
        }
    }

    // 7.1.18 ToObject ( argument )
    pub fn to_object(&self) -> Result<HandleMut<Object>, Value> {
        match self {
            Self::Undefined | Self::Null => Err(1001.into()), // TODO: TypeError
            Self::Boolean(_value) => unimplemented!("new Boolean(value)"),
            Self::Number(_value) => unimplemented!("new Number(value)"),
            Self::String(_value) => unimplemented!("new String(value)"),
            Self::Object(value) => Ok(*value),
            Self::None => unreachable!(),
        }
    }

    pub fn into_result(self, status: Status) -> Result<Value, Value> {
        logger::debug!(event = "into_result", ?status);
        match status {
            Status::Normal => Ok(self),
            Status::Exception => Err(self),
            _ => unreachable!("{status:?}"),
        }
    }

    // 13.5.3.1 Runtime Semantics: Evaluation
    pub fn get_typeof(&self) -> Handle<String> {
        match self {
            Self::None => unreachable!(),
            Self::Undefined => const_string!("undefined"),
            Self::Boolean(_) => const_string!("boolean"),
            Self::Number(_) => const_string!("number"),
            Self::String(_) => const_string!("string"),
            Self::Object(object) if object.is_callable() => const_string!("function"),
            Self::Null | Self::Object(_) => const_string!("object"),
        }
    }

    pub fn dummy_object() -> Self {
        Self::Object(HandleMut::dummy_for_testing())
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Undefined
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::from(value as f64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::from(value as f64)
    }
}

impl From<HandleMut<Object>> for Value {
    fn from(value: HandleMut<Object>) -> Self {
        Self::Object(value)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undefined, Self::Undefined) => true,
            (Self::Null, Self::Null) => true,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::String(a), Self::String(b)) => {
                debug_assert_eq!(
                    std::any::type_name_of_val(a.deref()),
                    std::any::type_name::<String>()
                );
                debug_assert_eq!(
                    std::any::type_name_of_val(b.deref()),
                    std::any::type_name::<String>()
                );
                a.deref() == b.deref()
            }
            (Self::Object(a), Self::Object(b)) => a == b,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Undefined => write!(f, "undefined"),
            Self::Null => write!(f, "null"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Object(value) => write!(f, "object({value:?})"),
        }
    }
}
