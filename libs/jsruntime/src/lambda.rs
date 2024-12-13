/// The identifier of a lambda function.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LambdaId(u32);

impl LambdaId {
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

impl From<u32> for LambdaId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<LambdaId> for u32 {
    fn from(value: LambdaId) -> Self {
        value.0
    }
}

impl std::fmt::Debug for LambdaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = self.index();
        write!(f, "LambdaId::Native")?;
        if self.is_coroutine() {
            write!(f, "Coroutine")?;
        }
        write!(f, "({index})")
    }
}

pub struct LambdaRegistry {
    functions: Vec<Function>,
}

impl LambdaRegistry {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }

    pub fn create_native_function(&mut self, coroutine: bool) -> LambdaId {
        let index = self.functions.len();
        assert!(index <= LambdaId::MAX_INDEX);
        self.functions.push(Function::Native(NativeFunction {
            scratch_buffer_len: 0,
        }));
        LambdaId::native(index, coroutine)
    }

    pub fn get_native(&self, func_id: LambdaId) -> &NativeFunction {
        match self.functions.get(func_id.index()) {
            Some(Function::Native(func)) => func,
            _ => unreachable!(),
        }
    }

    pub fn get_native_mut(&mut self, func_id: LambdaId) -> &mut NativeFunction {
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
        assert_eq!(LambdaId::MAX_INDEX, (LambdaId::COROUTINE_BIT - 1) as usize);
    }
}
