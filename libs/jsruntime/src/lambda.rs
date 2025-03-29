/// The identifier of a lambda function.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct LambdaId(u32);

impl LambdaId {
    const fn new(index: usize) -> Self {
        debug_assert!(index <= u32::MAX as usize);
        Self(index as u32)
    }

    const fn index(&self) -> usize {
        self.0 as usize
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

#[derive(Default)]
pub struct LambdaRegistry {
    entries: Vec<LambdaInfo>,
}

impl LambdaRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, is_coroutine: bool) -> LambdaId {
        let index = self.entries.len();
        self.entries.push(LambdaInfo {
            scratch_buffer_len: 0,
            is_coroutine,
        });
        LambdaId::new(index)
    }

    pub fn get(&self, id: LambdaId) -> &LambdaInfo {
        self.entries.get(id.index()).unwrap()
    }

    pub fn get_mut(&mut self, id: LambdaId) -> &mut LambdaInfo {
        self.entries.get_mut(id.index()).unwrap()
    }
}

pub struct LambdaInfo {
    // [[ECMAScriptCode]]
    pub scratch_buffer_len: u32,
    pub is_coroutine: bool,
}
