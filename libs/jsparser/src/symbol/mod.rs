mod builtins;

use std::ffi::CStr;

use indexmap::IndexSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Symbol(u32);

impl Symbol {
    pub const NONE: Symbol = Symbol(0);

    pub fn id(&self) -> u32 {
        self.0
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<u32> for Symbol {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

pub struct SymbolRegistry {
    symbols: IndexSet<Vec<u16>>,
}

impl SymbolRegistry {
    // TODO: measure the number of symbols used in a typical JavaScript program.
    const INITIAL_CAPACITY: usize = 512;

    fn new() -> Self {
        let mut symbols = IndexSet::with_capacity(Self::INITIAL_CAPACITY);
        symbols.insert(vec![]);
        debug_assert!(symbols.get_index(0).is_some());
        Self { symbols }
    }

    pub fn intern_cstr<T: AsRef<CStr>>(&mut self, s: T) -> Symbol {
        self.intern_str(s.as_ref().to_str().unwrap())
    }

    pub fn intern_str<T: AsRef<str>>(&mut self, s: T) -> Symbol {
        let code_units: Vec<u16> = s.as_ref().encode_utf16().collect();
        self.intern(code_units)
    }

    // TODO: use more efficient memory management such as bump allocation and arena.
    pub fn intern(&mut self, code_units: Vec<u16>) -> Symbol {
        let i = match self.symbols.get_index_of(&code_units) {
            Some(i) => i,
            None => {
                let (i, _) = self.symbols.insert_full(code_units);
                debug_assert!(i <= u32::MAX as usize);
                i
            }
        };
        Symbol(i as u32)
    }

    pub fn lookup(&self, code_units: &[u16]) -> Symbol {
        match self.symbols.get_index_of(code_units) {
            Some(i) => Symbol(i as u32),
            None => Symbol::NONE,
        }
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<&[u16]> {
        self.symbols
            .get_index(symbol.0 as usize)
            .map(|v| v.as_slice())
    }
}

impl Default for SymbolRegistry {
    fn default() -> Self {
        let mut self_ = Self::new();
        self_.register_builtin_symbols();
        self_
    }
}
