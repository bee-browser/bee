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

bitflags! {
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
    // TODO(perf): Which one is better?  `Option::None` or `&Value::None`.
    // In JIT-compiled code, we need a `nullptr` check if we choose `Option::None`.
    // If we choose `&Value::None`, we always need a memory access for the discriminant check of
    // the value but no `nullptr` access happens.
    pub fn get(&self, name: Symbol) -> Option<&Value> {
        match self.properties.get(&name) {
            Some(Property::Data { ref value, .. }) => Some(value),
            Some(Property::Accessor { .. }) => todo!(),
            None => None,
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

    // TODO(feat): 10.1.6.3 ValidateAndApplyPropertyDescriptor ( O, P, extensible, Desc, current )
    pub fn define_own_property(&mut self, name: Symbol, prop: Property) {
        self.properties.insert(name, prop);
    }
}

// 19 The Global Object
impl Object {
    pub fn define_builtin_global_properties(&mut self) {
        // TODO: 19.1.1 globalThis

        // 19.1.2 Infinity
        self.define_own_property(
            Symbol::INFINITY,
            Property::Data {
                value: Value::Number(f64::INFINITY),
                // { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: false }
                flags: PropertyFlags::empty(),
            },
        );

        // 19.1.3 NaN
        self.define_own_property(
            Symbol::NAN,
            Property::Data {
                value: Value::Number(f64::NAN),
                // { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: false }
                flags: PropertyFlags::empty(),
            },
        );

        // 19.1.4 undefined
        self.define_own_property(
            Symbol::UNDEFINED,
            Property::Data {
                value: Value::Undefined,
                // { [[Writable]]: false, [[Enumerable]]: false, [[Configurable]]: false }
                flags: PropertyFlags::empty(),
            },
        );
    }
}
