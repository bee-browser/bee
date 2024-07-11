use bitflags::bitflags;

use super::Locator;
use super::Reference;
use super::Symbol;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeRef(u16);

impl ScopeRef {
    pub const NONE: Self = Self::new(0);

    const fn new(index: u16) -> Self {
        Self(index)
    }

    fn checked_new(index: usize) -> Option<Self> {
        if index > u16::MAX as usize {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(index as u16))
    }

    fn index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingRef(u16, u16);

impl BindingRef {
    pub const NONE: Self = Self::new(0, 0);

    const fn new(scope_index: u16, binding_index: u16) -> Self {
        Self(scope_index, binding_index)
    }

    fn checked_new(scope_ref: ScopeRef, index: usize) -> Option<Self> {
        if index > u16::MAX as usize {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(scope_ref.0, index as u16))
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
    depth: u16,
}

impl ScopeTreeBuilder {
    pub const MAX_LOCAL_BINDINGS: usize = 0x1000;

    #[inline(always)]
    pub fn current(&self) -> ScopeRef {
        self.current
    }

    pub fn push_function(&mut self) {
        self.push(ScopeFlags::FUNCTION);
    }

    pub fn push_block(&mut self) {
        self.push(ScopeFlags::empty());
    }

    fn push(&mut self, flags: ScopeFlags) {
        let index = self.scopes.len();
        self.scopes.push(Scope {
            bindings: vec![],
            outer: self.current,
            depth: self.depth,
            flags,
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
        scope.flags.insert(ScopeFlags::SORTED);
        self.current = scope.outer;
        self.depth -= 1;
    }

    pub fn add_binding(&mut self, symbol: Symbol, kind: BindingKind) {
        let scope = &mut self.scopes[self.current.index()];
        debug_assert!(!scope.is_sorted());
        scope.bindings.push(Binding {
            symbol,
            kind,
            captured: false,
        });
    }

    pub fn set_immutable(&mut self, n: u32) {
        let scope = &mut self.scopes[self.current.index()];
        debug_assert!(!scope.is_sorted());
        for binding in scope.bindings.iter_mut().rev().take(n as usize) {
            binding.kind = BindingKind::Immutable;
        }
    }

    pub fn is_captured(&self, binding_ref: BindingRef) -> bool {
        let scope = &self.scopes[binding_ref.scope_index()];
        debug_assert!(scope.is_sorted());
        scope.bindings[binding_ref.binding_index()].captured
    }

    pub fn set_captured(&mut self, binding_ref: BindingRef) {
        let scope = &mut self.scopes[binding_ref.scope_index()];
        debug_assert!(scope.is_sorted());
        scope.bindings[binding_ref.binding_index()].captured = true;
    }

    pub fn resolve_reference(&self, reference: &Reference) -> BindingRef {
        let symbol = reference.symbol;
        let mut scope_ref = reference.scope_ref;
        loop {
            let scope = &self.scopes[scope_ref.index()];
            debug_assert!(scope.is_sorted());
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
                    if scope.is_function() {
                        return BindingRef::NONE;
                    }
                }
            }
        }
    }

    pub fn compute_locator(&self, binding_ref: BindingRef, offset: usize) -> Locator {
        let scope = &self.scopes[binding_ref.scope_index()];
        debug_assert!(scope.is_sorted());
        match scope.bindings[binding_ref.binding_index()].kind {
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
        debug_assert!(scope.is_sorted());
        if scope.is_function() {
            return 0;
        }
        let mut offset = 0;
        scope = &self.scopes[scope.outer.index()];
        loop {
            offset += scope.bindings.len();
            if scope.is_function() {
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

pub struct Scope {
    bindings: Vec<Binding>,
    outer: ScopeRef,
    depth: u16,
    flags: ScopeFlags,
}

impl Scope {
    const NONE: Self = Self {
        bindings: vec![],
        outer: ScopeRef::NONE,
        depth: 0,
        flags: ScopeFlags::empty(),
    };

    fn is_function(&self) -> bool {
        self.flags.contains(ScopeFlags::FUNCTION)
    }

    fn is_sorted(&self) -> bool {
        self.flags.contains(ScopeFlags::SORTED)
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:indent$}", "", indent = self.depth as usize)?;
        if !self.is_sorted() {
            write!(f, "*")?;
        }
        if self.is_function() {
            write!(f, "function:")?;
        } else {
            write!(f, "block:")?;
        }
        for binding in self.bindings.iter() {
            write!(f, " {binding}")?;
        }
        Ok(())
    }
}

bitflags! {
    struct ScopeFlags: u8 {
        const FUNCTION = 0b01;
        const SORTED   = 0b10;
    }
}

#[derive(Debug)]
struct Binding {
    symbol: Symbol,
    kind: BindingKind,
    captured: bool,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            BindingKind::FormalParameter(index) => write!(f, "{index}:{:?}", self.symbol)?,
            BindingKind::Mutable => write!(f, "M:{:?}", self.symbol)?,
            BindingKind::Immutable => write!(f, "I:{:?}", self.symbol)?,
        }
        if self.captured {
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
