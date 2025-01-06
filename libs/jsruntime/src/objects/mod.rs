use bitflags::bitflags;
use rustc_hash::FxHashMap;

use jsparser::Symbol;

use crate::types::Value;

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
    properties: FxHashMap<Symbol, Property>,
}

impl Object {
    pub fn get(&self, name: Symbol) -> Value {
        match self.properties.get(&name) {
            Some(Property::Data { ref value, .. }) => value.clone(),
            Some(Property::Accessor { .. }) => todo!(),
            None => Value::Undefined,
        }
    }

    // TODO(feat): strict, writable
    pub fn set(&mut self, name: Symbol, value: &Value) {
        self.properties
            .entry(name)
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

    pub fn get_own_property(&self, key: Symbol) -> Option<&Property> {
        self.properties.get(&key)
    }

    // TODO(feat): 10.1.6.3 ValidateAndApplyPropertyDescriptor ( O, P, extensible, Desc, current )
    pub fn define_own_property(&mut self, name: Symbol, prop: Property) -> Result<bool, Value> {
        self.properties.insert(name, prop);
        Ok(true)
    }

    pub fn iter_own_properties(&self) -> impl Iterator<Item = (Symbol, &Property)> {
        self.properties.iter().map(|(symbol, desc)| (*symbol, desc))
    }
}

// 19 The Global Object
impl Object {
    pub fn define_builtin_global_properties(&mut self) {
        macro_rules! define {
            ($name:expr, $value:expr) => {
                let result = self.define_own_property(
                    $name,
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
