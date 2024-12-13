/// The identifier of a function.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct FunctionId(u32);

impl FunctionId {
    const COROUTINE_BIT: u32 = 1 << 31;

    const VALUE_MASK: u32 = !(Self::COROUTINE_BIT);
    const MAX_INDEX: usize = Self::VALUE_MASK as usize;

    pub const MAIN: Self = Self::native(0, false);

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
        write!(f, "FunctionId::Native")?;
        if self.is_coroutine() {
            write!(f, "Coroutine")?;
        }
        write!(f, "({index})")
    }
}

pub struct FunctionRegistry {
    functions: Vec<Function>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }

    pub fn create_native_function(&mut self, coroutine: bool) -> FunctionId {
        let index = self.functions.len();
        assert!(index <= FunctionId::MAX_INDEX);
        self.functions.push(Function::Native(NativeFunction {
            scratch_buffer_len: 0,
        }));
        FunctionId::native(index, coroutine)
    }

    pub fn get_native(&self, func_id: FunctionId) -> &NativeFunction {
        match self.functions.get(func_id.index()) {
            Some(Function::Native(func)) => func,
            _ => unreachable!(),
        }
    }

    pub fn get_native_mut(&mut self, func_id: FunctionId) -> &mut NativeFunction {
        match self.functions.get_mut(func_id.index()) {
            Some(Function::Native(func)) => func,
            _ => unreachable!(),
        }
    }
}

enum Function {
    Native(NativeFunction),
}

pub struct NativeFunction {
    // [[ECMAScriptCode]]
    pub scratch_buffer_len: u32,
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
