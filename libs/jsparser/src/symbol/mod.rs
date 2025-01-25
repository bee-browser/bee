mod builtin;

use std::hash::Hash;
use std::hash::Hasher;

use indexmap::Equivalent;
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
    symbols: IndexSet<SymbolName>,
}

impl SymbolRegistry {
    // TODO: measure the number of symbols used in a typical JavaScript program.
    const INITIAL_CAPACITY: usize = 512;

    fn new() -> Self {
        let mut symbols = IndexSet::with_capacity(Self::INITIAL_CAPACITY);
        let (i, _) = symbols.insert_full(SymbolName::NO_NAME);
        debug_assert_eq!(i, Symbol::NONE.id() as usize);
        Self { symbols }
    }

    pub fn intern_str<T: AsRef<str>>(&mut self, s: T) -> Symbol {
        // TODO: use more efficient memory management such as bump allocation and arena.
        let name: Vec<u16> = s.as_ref().encode_utf16().collect();
        self.intern(SymbolName::Dynamic(name))
    }

    fn intern(&mut self, name: SymbolName) -> Symbol {
        let i = match self.symbols.get_index_of(&name) {
            Some(i) => i,
            None => {
                let (i, _) = self.symbols.insert_full(name);
                debug_assert!(i <= u32::MAX as usize);
                i
            }
        };
        Symbol(i as u32)
    }

    pub fn lookup(&self, name: &[u16]) -> Symbol {
        match self.symbols.get_index_of(&Query(name)) {
            Some(i) => Symbol(i as u32),
            None => Symbol::NONE,
        }
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<&[u16]> {
        self.symbols
            .get_index(symbol.0 as usize)
            .map(|v| v.as_ref())
    }
}

impl Default for SymbolRegistry {
    fn default() -> Self {
        let mut self_ = Self::new();
        self_.register_builtin_symbols();
        self_
    }
}

#[derive(Debug, Eq)]
enum SymbolName {
    Static(&'static [u16]),
    Dynamic(Vec<u16>),
}

impl SymbolName {
    const NO_NAME: Self = SymbolName::Static(&[]);
}

impl AsRef<[u16]> for SymbolName {
    fn as_ref(&self) -> &[u16] {
        match self {
            Self::Static(name) => name,
            Self::Dynamic(ref name) => name.as_slice(),
        }
    }
}

impl Hash for SymbolName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl PartialEq for SymbolName {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

// Use a wrapper in order to implement `Equivalent<SymbolName>`.
#[derive(Hash)]
struct Query<'a>(&'a [u16]);

impl Equivalent<SymbolName> for Query<'_> {
    fn equivalent(&self, key: &SymbolName) -> bool {
        self.0 == key.as_ref()
    }
}
