use super::Locator;
use super::Reference;
use super::Symbol;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeRef(usize);

impl ScopeRef {
    pub const NONE: Self = Self(0);
}

pub struct ScopeManager {
    scopes: Vec<Scope>,
    current: ScopeRef,
    depth: u32,
}

impl ScopeManager {
    pub const MAX_LOCAL_BINDINGS: usize = 0x1000;
    const MAX_LOCAL_INDEX: usize = 0x0FFF;

    #[inline(always)]
    pub fn current(&self) -> ScopeRef {
        self.current
    }

    pub fn push(&mut self, kind: ScopeKind) {
        let index = self.scopes.len();
        self.scopes.push(Scope {
            outer: self.current,
            bindings: vec![],
            depth: self.depth,
            kind,
        });
        self.current = ScopeRef(index);
        self.depth += 1;
    }

    pub fn pop(&mut self) {
        let scope = &mut self.scopes[self.current.0];
        scope
            .bindings
            .sort_unstable_by_key(|binding| binding.symbol);
        self.current = scope.outer;
        self.depth -= 1;
    }

    pub fn add_binding(&mut self, symbol: Symbol, kind: BindingKind) {
        self.scopes[self.current.0]
            .bindings
            .push(Binding { symbol, kind });
    }

    pub fn set_immutable(&mut self, n: u32) {
        for binding in self.scopes[self.current.0]
            .bindings
            .iter_mut()
            .rev()
            .take(n as usize)
        {
            binding.kind = BindingKind::Immutable;
        }
    }

    pub fn compute_locator(&self, reference: &Reference) -> Locator {
        let symbol = reference.symbol;
        let mut func_offset = 0;
        let mut scope_ref = reference.scope_ref;
        loop {
            let scope = &self.scopes[scope_ref.0];
            match scope
                .bindings
                .binary_search_by_key(&symbol, |binding| binding.symbol)
            {
                Ok(index) => match scope.bindings[index].kind {
                    BindingKind::FormalParameter(index) => {
                        return Locator::argument(func_offset, index);
                    }
                    _ => {
                        let base = self.compute_offset(scope_ref);
                        // TODO: the compilation should fail if the following condition is unmet.
                        assert!(base + index <= Self::MAX_LOCAL_INDEX);
                        return Locator::local(func_offset, (base + index) as u16);
                    }
                },
                Err(_) => {
                    scope_ref = scope.outer;
                    if scope_ref == ScopeRef::NONE {
                        panic!("{reference:?}");
                    }
                    if matches!(scope.kind, ScopeKind::Function) {
                        func_offset += 1;
                    }
                }
            }
        }
    }

    fn compute_offset(&self, scope_ref: ScopeRef) -> usize {
        let mut scope = &self.scopes[scope_ref.0];
        if matches!(scope.kind, ScopeKind::Function) {
            return 0;
        }
        let mut offset = 0;
        scope = &self.scopes[scope.outer.0];
        loop {
            offset += scope.bindings.len();
            if matches!(scope.kind, ScopeKind::Function) {
                return offset;
            }
            debug_assert_ne!(scope.outer, ScopeRef::NONE);
            scope = &self.scopes[scope.outer.0];
        }
    }

    #[allow(unused)]
    pub fn dump(&self, root: ScopeRef) {
        eprintln!("{}", self.scopes[root.0]);
        for scope in self.scopes[root.0 + 1..].iter() {
            if scope.outer == ScopeRef::NONE {
                break;
            }
            eprintln!("{scope}");
        }
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::NONE],
            current: ScopeRef::NONE,
            depth: 0,
        }
    }
}

pub enum ScopeKind {
    Function,
    Block,
}

pub struct Scope {
    outer: ScopeRef,
    bindings: Vec<Binding>,
    depth: u32,
    kind: ScopeKind,
}

impl Scope {
    const NONE: Self = Self {
        outer: ScopeRef::NONE,
        bindings: vec![],
        depth: 0,
        kind: ScopeKind::Function,
    };
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:indent$}", "", indent = self.depth as usize)?;
        match self.kind {
            ScopeKind::Function => write!(f, "function-scope:")?,
            ScopeKind::Block => write!(f, "block-scope:")?,
        }
        for binding in self.bindings.iter() {
            write!(f, " {binding}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Binding {
    symbol: Symbol,
    kind: BindingKind,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            BindingKind::FormalParameter(index) => write!(f, "{index}:{:?}", self.symbol),
            BindingKind::Mutable => write!(f, "M:{:?}", self.symbol),
            BindingKind::Immutable => write!(f, "I:{:?}", self.symbol),
        }
    }
}

#[derive(Debug)]
pub enum BindingKind {
    FormalParameter(u16),
    Mutable,
    Immutable,
}
