#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bridge.rs"));

impl Locator {
    pub(crate) const NONE: Self = Self::new(LocatorKind_None, 0, 0);

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

impl Value {
    pub const UNDEFINED: Self = Self {
        kind: ValueKind_Undefined,
        holder: ValueHolder { opaque: 0 },
    };

    pub const NULL: Self = Self {
        kind: ValueKind_Null,
        holder: ValueHolder { opaque: 0 },
    };

    pub const TRUE: Self = Self::boolean(true);
    pub const FALSE: Self = Self::boolean(false);

    pub const fn boolean(boolean: bool) -> Self {
        Self {
            kind: ValueKind_Boolean,
            holder: ValueHolder { boolean },
        }
    }

    pub const fn number(number: f64) -> Self {
        Self {
            kind: ValueKind_Number,
            holder: ValueHolder { number },
        }
    }

    pub const fn function(function: FuncPtr) -> Self {
        Self {
            kind: ValueKind_Function,
            holder: ValueHolder { function },
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::UNDEFINED
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::number(value)
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

impl From<FuncPtr> for Value {
    fn from(value: FuncPtr) -> Self {
        Self::function(value)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `unsafe` is needed for accessing the `holder` field.
        unsafe {
            match self.kind {
                ValueKind_Undefined => write!(f, "undefined"),
                ValueKind_Null => write!(f, "null"),
                ValueKind_Boolean if self.holder.boolean => write!(f, "true"),
                ValueKind_Boolean => write!(f, "false"),
                ValueKind_Number => write!(f, "{}", self.holder.number),
                ValueKind_Function => write!(f, "{:?}", self.holder.function),
                _ => unreachable!(),
            }
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            to_boolean: Some(runtime_to_boolean),
            to_numeric: Some(runtime_to_numeric),
            to_int32: Some(runtime_to_int32),
            to_uint32: Some(runtime_to_uint32),
            is_loosely_equal: Some(runtime_is_loosely_equal),
            is_strictly_equal: Some(runtime_is_strictly_equal),
        }
    }
}

// 7.1.2 ToBoolean ( argument )
unsafe extern "C" fn runtime_to_boolean(_: usize, value: *const Value) -> bool {
    let value = &*value;
    match value.kind {
        ValueKind_Undefined => false,
        ValueKind_Null => false,
        ValueKind_Boolean => value.holder.boolean,
        ValueKind_Number if value.holder.number == 0.0 => false,
        ValueKind_Number if value.holder.number.is_nan() => false,
        ValueKind_Number => true,
        ValueKind_Function => true,
        _ => unreachable!(),
    }
}

// 7.1.3 ToNumeric ( value )
// 7.1.4 ToNumber ( argument )
unsafe extern "C" fn runtime_to_numeric(_: usize, value: *const Value) -> f64 {
    let value = &*value;
    match value.kind {
        ValueKind_Undefined => f64::NAN,
        ValueKind_Null => 0.0,
        ValueKind_Boolean if value.holder.boolean => 1.0,
        ValueKind_Boolean => 0.0,
        ValueKind_Number => value.holder.number,
        ValueKind_Function => f64::NAN,
        _ => unreachable!(),
    }
}

// 7.1.6 ToInt32 ( argument )
unsafe extern "C" fn runtime_to_int32(_: usize, value: f64) -> i32 {
    const EXP2_31: f64 = (2u64 << 31) as f64;
    const EXP2_32: f64 = (2u64 << 32) as f64;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !value.is_finite() || value == 0.0 {
        return 0;
    }

    // 3. Let int be truncate(ℝ(number)).
    let int_ = value.trunc();

    // 4. Let int32bit be int modulo 2**32.
    let int32bit = int_ % EXP2_32;
    // int32bit may be negative.

    // 5. If int32bit ≥ 2**31, return 𝔽(int32bit - 2**32); otherwise return 𝔽(int32bit).
    if int32bit >= EXP2_31 {
        (int32bit - EXP2_32) as i32
    } else {
        int32bit as i32
    }
}

// 7.1.7 ToUint32 ( argument )
unsafe extern "C" fn runtime_to_uint32(_: usize, value: f64) -> u32 {
    const EXP2_31: f64 = (2u64 << 31) as f64;
    const EXP2_32: f64 = (2u64 << 32) as f64;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !value.is_finite() || value == 0.0 {
        return 0;
    }

    // 3. Let int be truncate(ℝ(number)).
    let int_ = dbg!(value.trunc());

    // 4. Let int32bit be int modulo 2**32.
    let int32bit = dbg!(int_ % EXP2_32);
    // int32bit may be negative.

    // 5. Return 𝔽(int32bit).
    if int32bit < 0.0 {
        dbg!((int32bit + EXP2_31) as u32)
    } else {
        dbg!(int32bit as u32)
    }
}

// 7.2.13 IsLooselyEqual ( x, y )
unsafe extern "C" fn runtime_is_loosely_equal(
    runtime: usize,
    a: *const Value,
    b: *const Value,
) -> bool {
    let x = &*a;
    let y = &*b;
    // 1. If Type(x) is Type(y)
    if x.kind == y.kind {
        // a. Return IsStrictlyEqual(x, y).
        return runtime_is_strictly_equal(runtime, a, b);
    }
    // 2. If x is null and y is undefined, return true.
    if x.kind == ValueKind_Null && y.kind == ValueKind_Undefined {
        return true;
    }
    // 3. If x is undefined and y is null, return true.
    if x.kind == ValueKind_Undefined && y.kind == ValueKind_Null {
        return true;
    }
    // TODO: 4. NOTE: This step is replaced in section B.3.6.2.
    // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
    // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
    // TODO: 7. If x is a BigInt and y is a String, then
    // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
    // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
    // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
    // ...
    let xnum = runtime_to_numeric(runtime, x);
    let ynum = runtime_to_numeric(runtime, y);
    if xnum.is_nan() || ynum.is_nan() {
        return false;
    }
    xnum == ynum
}

// 7.2.14 IsStrictlyEqual ( x, y )
unsafe extern "C" fn runtime_is_strictly_equal(_: usize, a: *const Value, b: *const Value) -> bool {
    let x = &*a;
    let y = &*b;
    if x.kind != y.kind {
        return false;
    }
    match x.kind {
        ValueKind_Undefined => true,
        ValueKind_Null => true,
        ValueKind_Boolean => x.holder.boolean == y.holder.boolean,
        ValueKind_Number => x.holder.number == y.holder.number,
        ValueKind_Function => x.holder.function == y.holder.function,
        _ => unreachable!(),
    }
}
