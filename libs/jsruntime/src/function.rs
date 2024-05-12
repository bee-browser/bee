use std::ffi::CString;

use jsparser::Symbol;

use crate::llvmir::bridge::NativeFuncPtr;
use crate::Runtime;
use crate::Value;

pub type HostFn = fn(&mut Runtime, &[Value]);

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FunctionId(u32);

impl FunctionId {
    const HOST_BIT: u32 = 0x80000000;
    const VALUE_MASK: u32 = !Self::HOST_BIT;
    const MAX: u32 = Self::VALUE_MASK;

    #[inline(always)]
    pub fn native(value: u32) -> Self {
        debug_assert!(value <= Self::MAX);
        Self(value)
    }

    #[inline(always)]
    pub fn host(value: u32) -> Self {
        debug_assert!(value <= Self::MAX);
        Self(value | Self::HOST_BIT)
    }

    #[inline(always)]
    pub fn value(&self) -> u32 {
        self.0 & Self::VALUE_MASK
    }

    #[inline(always)]
    pub fn is_native(&self) -> bool {
        (self.0 & Self::HOST_BIT) == 0
    }
}

impl From<u32> for FunctionId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<FunctionId> for u32 {
    fn from(value: FunctionId) -> Self {
        value.0
    }
}

impl std::fmt::Debug for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_native() {
            write!(f, "FunctionId::Native({})", self.value())
        } else {
            write!(f, "FunctionId::Host({})", self.value())
        }
    }
}

pub struct FunctionRegistry {
    native_functions: Vec<NativeFunction>,
    host_functions: Vec<HostFunction>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            native_functions: vec![],
            host_functions: vec![],
        }
    }

    pub fn get_native(&self, func_id: u32) -> &NativeFunction {
        let func_id = func_id as usize;
        &self.native_functions[func_id]
    }

    pub fn get_native_mut(&mut self, func_id: u32) -> &mut NativeFunction {
        let func_id = func_id as usize;
        &mut self.native_functions[func_id]
    }

    pub fn get_host(&self, func_id: u32) -> &HostFunction {
        let func_id = func_id as usize;
        &self.host_functions[func_id]
    }

    pub fn create_native_function(&mut self, formal_parameters: Vec<Symbol>) -> FunctionId {
        let value = self.native_functions.len();
        debug_assert!(value <= FunctionId::MAX as usize);
        let name = CString::new(format!("fn{value}")).unwrap();
        self.native_functions.push(NativeFunction {
            formal_parameters,
            name,
            func: None,
        });
        FunctionId::native(value as u32)
    }

    pub fn register_host_function(&mut self, name: &str) -> FunctionId {
        let value = self.host_functions.len();
        debug_assert!(value <= FunctionId::MAX as usize);
        let name = CString::new(name).unwrap();
        self.host_functions.push(HostFunction { name });
        FunctionId::host(value as u32)
    }

    pub fn enumerate_host_function(&self) -> impl Iterator<Item = (FunctionId, &HostFunction)> {
        self.host_functions.iter().enumerate().map(|(i, func)| {
            debug_assert!(i <= FunctionId::MAX as usize);
            (FunctionId::host(i as u32), func)
        })
    }
}

pub struct NativeFunction {
    // [[FormalParameters]]
    // TODO: Vec<BindingElement>
    pub formal_parameters: Vec<Symbol>,

    // [[ECMAScriptCode]]
    pub name: CString,
    pub func: NativeFuncPtr,
}

pub struct HostFunction {
    pub name: CString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_id_max() {
        // TODO: checking at compile time is better.
        assert_eq!(FunctionId::MAX, FunctionId::HOST_BIT - 1);
    }
}
