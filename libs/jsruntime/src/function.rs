use jsparser::Symbol;

/// The identifier of a function.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct FunctionId(u32);

impl FunctionId {
    const HOST_BIT: u32 = 1 << 31;
    const COROUTINE_BIT: u32 = 1 << 30;

    const VALUE_MASK: u32 = !(Self::HOST_BIT | Self::COROUTINE_BIT);
    const MAX_INDEX: usize = Self::VALUE_MASK as usize;

    pub const MAIN: Self = Self::native(0, false); // TODO: modules including await expression.

    pub const fn is_native(&self) -> bool {
        (self.0 & Self::HOST_BIT) == 0
    }

    pub const fn is_host(&self) -> bool {
        (self.0 & Self::HOST_BIT) != 0
    }

    pub const fn is_coroutine(&self) -> bool {
        (self.0 & Self::COROUTINE_BIT) != 0
    }

    const fn native(index: usize, coroutine: bool) -> Self {
        debug_assert!(index <= Self::MAX_INDEX);
        Self(if coroutine {
            index as u32 | Self::COROUTINE_BIT
        } else {
            index as u32
        })
    }

    const fn host(index: usize) -> Self {
        debug_assert!(index <= Self::MAX_INDEX);
        Self(index as u32 | Self::HOST_BIT)
    }

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
            write!(f, "FunctionId::Native")?;
        } else {
            write!(f, "FunctionId::Host")?;
        }
        if self.is_coroutine() {
            write!(f, "Coroutine")?;
        }
        write!(f, "({index})")
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

    pub fn create_native_function(&mut self, coroutine: bool) -> FunctionId {
        let index = self.native_functions.len();
        assert!(index <= FunctionId::MAX_INDEX);
        self.native_functions.push(NativeFunction {});
        FunctionId::native(index, coroutine)
    }

    pub fn register_host_function(&mut self, symbol: Symbol) -> FunctionId {
        let index = self.host_functions.len();
        assert!(index <= FunctionId::MAX_INDEX);
        self.host_functions.push(HostFunction { symbol });
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
}

pub struct HostFunction {
    pub symbol: Symbol,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_id_max() {
        // TODO: checking at compile time is better.
        assert_eq!(
            FunctionId::MAX_INDEX,
            (FunctionId::COROUTINE_BIT - 1) as usize
        );
    }
}
