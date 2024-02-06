mod builtins;

use indexmap::IndexSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Symbol(usize);

pub struct SymbolTable {
    symbols: IndexSet<Vec<u16>>,
}

impl SymbolTable {
    // TODO: measure the number of symbols used in a typical JavaScript program.
    const INITIAL_CAPACITY: usize = 512;

    pub fn new() -> Self {
        Self {
            symbols: IndexSet::with_capacity(Self::INITIAL_CAPACITY),
        }
    }

    pub fn with_builtin_symbols() -> Self {
        let mut self_ = Self::new();
        self_.register_builtin_symbols();
        self_
    }

    // TODO: use more efficient memory management such as bump allocation and arena.
    pub fn intern(&mut self, code_units: Vec<u16>) -> Symbol {
        Symbol(match self.symbols.get_index_of(&code_units) {
            Some(index) => index,
            None => self.symbols.insert_full(code_units).0,
        })
    }

    pub fn resolve(&self, symbol: Symbol) -> Option<&[u16]> {
        self.symbols.get_index(symbol.0).map(|v| v.as_slice())
    }
}
