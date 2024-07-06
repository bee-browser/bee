use std::ffi::CString;

/// The identifier of a function.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FunctionId(u32);

impl FunctionId {
    const HOST_BIT: u32 = 0x80000000;
    const VALUE_MASK: u32 = !Self::HOST_BIT;
    const MAX_INDEX: usize = Self::VALUE_MASK as usize;

    pub const MAIN: Self = Self::native(0);

    #[inline(always)]
    pub const fn is_native(&self) -> bool {
        (self.0 & Self::HOST_BIT) == 0
    }

    #[inline(always)]
    pub const fn is_host(&self) -> bool {
        (self.0 & Self::HOST_BIT) != 0
    }

    #[inline(always)]
    const fn native(index: usize) -> Self {
        debug_assert!(index <= Self::MAX_INDEX);
        Self(index as u32)
    }

    #[inline(always)]
    const fn host(index: usize) -> Self {
        debug_assert!(index <= Self::MAX_INDEX);
        Self(index as u32 | Self::HOST_BIT)
    }

    #[inline(always)]
    const fn index(&self) -> usize {
        (self.0 & Self::VALUE_MASK) as usize
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
        let index = self.index();
        if self.is_native() {
            write!(f, "FunctionId::Native({index})")
        } else {
            write!(f, "FunctionId::Host({index})")
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

    pub fn get_native(&self, id: FunctionId) -> &NativeFunction {
        debug_assert!(id.is_native());
        &self.native_functions[id.index()]
    }

    pub fn get_native_mut(&mut self, id: FunctionId) -> &mut NativeFunction {
        debug_assert!(id.is_native());
        &mut self.native_functions[id.index()]
    }

    pub fn get_host(&self, id: FunctionId) -> &HostFunction {
        debug_assert!(id.is_host());
        &self.host_functions[id.index()]
    }

    pub fn create_native_function(&mut self) -> FunctionId {
        let index = self.native_functions.len();
        assert!(index <= FunctionId::MAX_INDEX);
        let name = CString::new(format!("fn{index}")).unwrap();
        self.native_functions.push(NativeFunction {
            name,
        });
        FunctionId::native(index)
    }

    pub fn register_host_function(&mut self, name: &str) -> FunctionId {
        let index = self.host_functions.len();
        assert!(index <= FunctionId::MAX_INDEX);
        let name = CString::new(name).unwrap();
        self.host_functions.push(HostFunction { name });
        FunctionId::host(index)
    }

    pub fn enumerate_host_function(&self) -> impl Iterator<Item = (FunctionId, &HostFunction)> {
        self.host_functions.iter().enumerate().map(|(index, func)| {
            debug_assert!(index <= FunctionId::MAX_INDEX);
            (FunctionId::host(index), func)
        })
    }
}

pub struct NativeFunction {
    // [[ECMAScriptCode]]
    pub name: CString,
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
        assert_eq!(FunctionId::MAX_INDEX, (FunctionId::HOST_BIT - 1) as usize);
    }
}
