use std::hash::Hash;
use std::hash::Hasher;

use bitflags::bitflags;
use rustc_hash::FxHashMap;

use jsparser::Symbol;

use crate::types::Value;

#[derive(Clone, Debug)]
pub enum PropertyKey {
    Symbol(u32),
    Number(f64),
}

impl Eq for PropertyKey {}

impl From<Symbol> for PropertyKey {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value.id())
    }
}

impl From<f64> for PropertyKey {
    fn from(value: f64) -> Self {
        if value.is_nan() {
            Symbol::NAN.into()
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                Symbol::INFINITY.into()
            } else {
                Symbol::NEG_INFINITY.into()
            }
        } else if value == 0. {
            Self::Number(0.) // convert `-0.` to `0.`
        } else {
            Self::Number(value)
        }
    }
}

impl Hash for PropertyKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Symbol(v) => state.write_u32(*v),
            Self::Number(v) => state.write_u64(v.to_bits()),
        }
    }
}

impl PartialEq for PropertyKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::Number(a), Self::Number(b)) => a == b,
            _ => false,
        }
    }
}

// 6.1.7.1 Property Attributes

pub enum Property {
    /// A data property.
    Data {
        /// The `[[Value]]` attribute.
        value: Value,

        /// Flags for boolean attributes.
        flags: PropertyFlags,
    },

    /// A accessor property
    #[allow(unused)]
    Accessor {
        /// Flags for boolean attributes.
        flags: PropertyFlags,
    },
}

impl Property {
    pub fn is_enumerable(&self) -> bool {
        self.flags().contains(PropertyFlags::ENUMERABLE)
    }

    pub fn flags(&self) -> PropertyFlags {
        match self {
            Self::Data { flags, .. } => *flags,
            Self::Accessor { flags } => *flags,
        }
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    pub struct PropertyFlags: u8 {
        /// The `[[Writable]]` attribute.
        ///
        /// Available only for the data property.
        const WRITABLE     = 1 << 0;

        /// The `[[Enumerable]]` attribute.
        const ENUMERABLE   = 1 << 1;

        /// The `[[Configurable]]` attribute.
        const CONFIGURABLE = 1 << 2;
    }
}

// 10 Ordinary and Exotic Objects Behaviours

// 10.1 Ordinary Object Internal Methods and Internal Slots

#[derive(Default)]
pub struct Object {
    properties: FxHashMap<PropertyKey, Property>,
}

impl Object {
    // TODO(perf): Which one is better?  `Option::None` or `&Value::None`.
    // In JIT-compiled code, we need a `nullptr` check if we choose `Option::None`.
    // If we choose `&Value::None`, we always need a memory access for the discriminant check of
    // the value but no `nullptr` access happens.
    //
    // TODO(perf): Returning a `Value` degrades the performance.
    // Returning the reference to the value improves the performance.  But it doesn't work in the
    // case of `Property::Accessor` if we don't use a *scratch* memory area in the object in order
    // to store the computation result temporarily and return it from this method as the return
    // value.  Returning the reference to the value works properly if and only if the value is used
    // before it's overwritten.  At this point, we are not sure whether or not it's always works in
    // any expression.
    pub fn get_value(&self, key: &PropertyKey) -> Option<&Value> {
        match self.properties.get(key) {
            Some(Property::Data { ref value, .. }) => Some(value),
            Some(Property::Accessor { .. }) => todo!(),
            None => None,
        }
    }

    // TODO(feat): strict, writable
    pub fn set_value(&mut self, key: &PropertyKey, value: &Value) {
        self.properties
            .entry(key.clone())
            .and_modify(|prop| match prop {
                Property::Data {
                    // The variable name `value` is already used in the arguments.
                    //
                    // NOTE: Be careful.  `clippy` does NOT detect mistakes like this:
                    //
                    // ```rust
                    // *value = value.clone();
                    // ```
                    //
                    // This does NOT match the `self_assignment` lint that is denied by default.
                    //
                    // If `Value` implements `Copy`, this kind of self-assignment can be detected
                    // if the `assigning_clones` lint is denied (but it's allowed by default).
                    value: ref mut value_ref,
                    flags,
                } => {
                    debug_assert!(flags.contains(PropertyFlags::WRITABLE));
                    *value_ref = value.clone();
                }
                Property::Accessor { flags } => {
                    debug_assert!(flags.contains(PropertyFlags::WRITABLE));
                    *prop = Property::Data {
                        value: value.clone(),
                        flags: PropertyFlags::empty(),
                    }
                }
            })
            .or_insert(Property::Data {
                value: value.clone(),
                flags: PropertyFlags::empty(),
            });
    }

    pub fn get_own_property(&self, key: &PropertyKey) -> Option<&Property> {
        self.properties.get(key)
    }

    // TODO(feat): 10.1.6.3 ValidateAndApplyPropertyDescriptor ( O, P, extensible, Desc, current )
    pub fn define_own_property(&mut self, key: PropertyKey, prop: Property) -> Result<bool, Value> {
        self.properties.insert(key, prop);
        Ok(true)
    }

    pub fn iter_own_properties(&self) -> impl Iterator<Item = (&PropertyKey, &Property)> {
        self.properties.iter()
    }
}

// 19 The Global Object
impl Object {
    pub fn define_builtin_global_properties(&mut self) {
        macro_rules! define {
            ($name:expr, $value:expr) => {
                let result = self.define_own_property(
                    $name.into(),
                    Property::Data {
                        value: $value,
                        // { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: false }
                        flags: PropertyFlags::empty(),
                    },
                );
                debug_assert!(matches!(result, Ok(true)));
            };
        }

        // TODO: 19.1.1 globalThis

        // 19.1.2 Infinity
        define! {Symbol::INFINITY, Value::Number(f64::INFINITY)}

        // 19.1.3 NaN
        define! {Symbol::NAN, Value::Number(f64::NAN)}

        // 19.1.4 undefined
        define! {Symbol::UNDEFINED, Value::Undefined}
    }
}
