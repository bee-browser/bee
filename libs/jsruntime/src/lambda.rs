use crate::ProgramId;

/// The identifier of a lambda function.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct LambdaId(u32);

impl LambdaId {
    pub(crate) const HOST: Self = Self(u32::MAX);

    const fn new(index: usize) -> Self {
        debug_assert!(index < u32::MAX as usize);
        Self(index as u32)
    }

    const fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for LambdaId {
    fn from(value: u32) -> Self {
        debug_assert!(value < u32::MAX);
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

    pub fn register(&mut self, kind: LambdaKind) -> LambdaId {
        // TODO(fix): index < u32::MAX
        let index = self.entries.len();
        self.entries.push(LambdaInfo {
            program_id: ProgramId::INVALID,
            function_index: u32::MAX,
            scratch_buffer_len: 0,
            kind,
        });
        LambdaId::new(index)
    }

    pub fn get(&self, id: LambdaId) -> &LambdaInfo {
        debug_assert_ne!(id, LambdaId::HOST);
        self.entries.get(id.index()).unwrap()
    }

    pub fn get_mut(&mut self, id: LambdaId) -> &mut LambdaInfo {
        debug_assert_ne!(id, LambdaId::HOST);
        self.entries.get_mut(id.index()).unwrap()
    }
}

pub struct LambdaInfo {
    // [[ECMAScriptCode]]
    pub program_id: ProgramId,
    pub function_index: u32,
    pub scratch_buffer_len: u32,
    pub kind: LambdaKind,
}

#[derive(Clone, Copy, Debug)]
pub enum LambdaKind {
    Normal,
    Ramp,
    Coroutine,
}
