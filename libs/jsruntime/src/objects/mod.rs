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
    pub fn get(&self, symbol: Symbol) -> Option<&Value> {
        match self.properties.get(&symbol) {
            Some(Property::Data { ref value, .. }) => Some(value),
            Some(Property::Accessor { .. }) => todo!(),
            None => None,
        }
    }

    // TODO(feat): strict, writable
    pub fn set(&mut self, symbol: Symbol, value: &Value) {
        self.properties
            .entry(symbol)
            .and_modify(|e| {
                *e = Property::Data {
                    value: value.clone(),
                    flags: PropertyFlags::empty(),
                }
            })
            .or_insert(Property::Data {
                value: value.clone(),
                flags: PropertyFlags::empty(),
            });
    }
}
