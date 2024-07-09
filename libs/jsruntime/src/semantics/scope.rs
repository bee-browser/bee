use super::Locator;
use super::Reference;
use super::Symbol;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeRef(u32);

impl ScopeRef {
    pub const NONE: Self = Self::new(0);

    const fn new(index: u32) -> Self {
        Self(index)
    }

    fn checked_new(index: usize) -> Option<Self> {
        if index > u32::MAX as usize {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(index as u32))
    }

    fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingRef(u32, u32);

impl BindingRef {
    pub const NONE: Self = Self::new(0, 0);

    const fn new(scope_index: u32, binding_index: u32) -> Self {
        Self(scope_index, binding_index)
    }

    fn checked_new(scope_ref: ScopeRef, index: usize) -> Option<Self> {
        if index > u32::MAX as usize {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(scope_ref.0, index as u32))
    }

    fn scope_ref(&self) -> ScopeRef {
        ScopeRef::new(self.0)
    }

    fn scope_index(&self) -> usize {
        self.0 as usize
    }

    fn binding_index(&self) -> usize {
        self.1 as usize
    }
}

pub struct ScopeTree {
    scopes: Vec<Scope>,
}

impl ScopeTree {
    #[allow(unused)]
    pub fn print(&self) {
        for scope in self.scopes[1..].iter() {
            println!("{scope}");
        }
    }
}

pub struct ScopeTreeBuilder {
    scopes: Vec<Scope>,
    current: ScopeRef,
    depth: u32,
}

impl ScopeTreeBuilder {
    pub const MAX_LOCAL_BINDINGS: usize = 0x1000;

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
        // TODO: should return an error
        self.current = ScopeRef::checked_new(index).unwrap();
        self.depth += 1;
    }

    pub fn pop(&mut self) {
        let scope = &mut self.scopes[self.current.index()];
        scope
            .bindings
            .sort_unstable_by_key(|binding| binding.symbol);
        self.current = scope.outer;
        self.depth -= 1;
    }

    pub fn add_binding(&mut self, symbol: Symbol, kind: BindingKind) {
        self.scopes[self.current.index()]
            .bindings
            .push(Binding {
                symbol,
                kind,
                closed_over: false,
            });
    }

    pub fn set_immutable(&mut self, n: u32) {
        for binding in self.scopes[self.current.index()]
            .bindings
            .iter_mut()
            .rev()
            .take(n as usize)
        {
            binding.kind = BindingKind::Immutable;
        }
    }

    pub fn set_closed_over(&mut self, binding_ref: BindingRef) {
        self.scopes[binding_ref.scope_index()].bindings[binding_ref.binding_index()].closed_over = true;
    }

    pub fn resolve_reference(&self, reference: &Reference) -> BindingRef {
        let symbol = reference.symbol;
        let mut scope_ref = reference.scope_ref;
        loop {
            let scope = &self.scopes[scope_ref.index()];
            match scope
                .bindings
                .binary_search_by_key(&symbol, |binding| binding.symbol)
            {
                Ok(index) => {
                    // TODO: should return an error
                    return BindingRef::checked_new(scope_ref, index).unwrap();
                }
                Err(_) => {
                    scope_ref = scope.outer;
                    if scope_ref == ScopeRef::NONE {
                        panic!("{reference:?}");
                    }
                    if matches!(scope.kind, ScopeKind::Function) {
                        return BindingRef::NONE;
                    }
                }
            }
        }
    }

    pub fn compute_locator(&self, binding_ref: BindingRef, offset: usize) -> Locator {
        match self.scopes[binding_ref.scope_index()].bindings[binding_ref.binding_index()].kind {
            BindingKind::FormalParameter(index) => {
                // TODO: the compilation should fail if `None` is returned.
                Locator::checked_argument(offset, index).unwrap()
            }
            _ => {
                let base = self.compute_offset(binding_ref.scope_ref());
                // TODO: the compilation should fail if `None` is returned.
                Locator::checked_local(offset, base + binding_ref.binding_index()).unwrap()
            }
        }
    }

    fn compute_offset(&self, scope_ref: ScopeRef) -> usize {
        let mut scope = &self.scopes[scope_ref.index()];
        if matches!(scope.kind, ScopeKind::Function) {
            return 0;
        }
        let mut offset = 0;
        scope = &self.scopes[scope.outer.index()];
        loop {
            offset += scope.bindings.len();
            if matches!(scope.kind, ScopeKind::Function) {
                return offset;
            }
            debug_assert_ne!(scope.outer, ScopeRef::NONE);
            scope = &self.scopes[scope.outer.index()];
        }
    }

    pub fn build(&mut self) -> ScopeTree {
        ScopeTree {
            scopes: std::mem::take(&mut self.scopes),
        }
    }
}

impl Default for ScopeTreeBuilder {
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
    closed_over: bool,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            BindingKind::FormalParameter(index) => write!(f, "{index}:{:?}", self.symbol)?,
            BindingKind::Mutable => write!(f, "M:{:?}", self.symbol)?,
            BindingKind::Immutable => write!(f, "I:{:?}", self.symbol)?,
        }
        if self.closed_over {
            write!(f, "*")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum BindingKind {
    FormalParameter(usize),
    Mutable,
    Immutable,
}
