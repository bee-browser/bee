use bitflags::bitflags;

use super::Locator;
use super::Reference;
use super::Symbol;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeRef(u16);

impl ScopeRef {
    pub const NONE: Self = Self::new(0);

    pub const fn new(index: u16) -> Self {
        Self(index)
    }

    pub const fn id(&self) -> u16 {
        self.0
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

impl Default for ScopeRef {
    fn default() -> Self {
        Self::NONE
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BindingRef(u16, u16);

impl BindingRef {
    pub const NONE: Self = Self::new(0, 0);

    pub const fn new(scope_index: u16, binding_index: u16) -> Self {
        Self(scope_index, binding_index)
    }

    fn checked_new(scope_ref: ScopeRef, index: usize) -> Option<Self> {
        if index > u16::MAX as usize {
            crate::logger::error!(err = "too large", index);
            return None;
        }
        Some(Self::new(scope_ref.0, index as u16))
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
    pub fn scope(&self, scope_ref: ScopeRef) -> &Scope {
        &self.scopes[scope_ref.index()]
    }

    pub fn iter_bindings(
        &self,
        scope_ref: ScopeRef,
    ) -> impl Iterator<Item = (BindingRef, &Binding)> {
        self.scopes[scope_ref.index()]
            .bindings
            .iter()
            .enumerate()
            .map(move |(index, binding)| (BindingRef::new(scope_ref.0, index as u16), binding))
    }

    pub fn get_symbol(&self, binding_ref: BindingRef) -> Symbol {
        let scope = &self.scopes[binding_ref.scope_index()];
        scope.bindings[binding_ref.binding_index()].symbol
    }

    pub fn compute_locator(&self, binding_ref: BindingRef) -> Locator {
        let scope = &self.scopes[binding_ref.scope_index()];
        let binding = &scope.bindings[binding_ref.binding_index()];
        match binding.kind {
            BindingKind::FormalParameter => Locator::Argument(binding.index),
            _ => Locator::Local(binding.index),
        }
    }

    #[allow(unused)]
    pub fn print(&self, indent: &str) {
        for (index, scope) in self.scopes.iter().enumerate().skip(1) {
            println!("{indent}{}", ScopePrinter { index, scope });
        }
    }
}

pub struct ScopeTreeBuilder {
    scopes: Vec<Scope>,
    current: ScopeRef,
    depth: u16,
}

impl ScopeTreeBuilder {
    #[inline(always)]
    pub fn current(&self) -> ScopeRef {
        self.current
    }

    pub fn push_function(&mut self) -> ScopeRef {
        self.push(ScopeFlags::FUNCTION, "")
    }

    pub fn push_block(&mut self, label: &'static str) -> ScopeRef {
        self.push(ScopeFlags::empty(), label)
    }

    fn push(&mut self, flags: ScopeFlags, label: &'static str) -> ScopeRef {
        let index = self.scopes.len();
        self.scopes.push(Scope {
            label,
            bindings: vec![],
            num_formal_parameters: 0,
            num_locals: 0,
            outer: self.current,
            depth: self.depth,
            max_child_block_depth: self.depth,
            flags,
        });
        // TODO: should return an error
        self.current = ScopeRef::checked_new(index).unwrap();
        self.depth += 1;
        self.current
    }

    pub fn pop(&mut self) {
        let scope = &mut self.scopes[self.current.index()];
        scope
            .bindings
            .sort_unstable_by_key(|binding| binding.symbol);
        scope.flags.insert(ScopeFlags::SORTED);
        self.current = scope.outer;
        if !scope.is_function() {
            let max_child_scope_depth = scope.max_child_block_depth;
            let scope = &mut self.scopes[self.current.index()];
            if scope.max_child_block_depth < max_child_scope_depth {
                scope.max_child_block_depth = max_child_scope_depth;
            }
        }
        self.depth -= 1;
    }

    pub fn add_formal_parameter(&mut self, symbol: Symbol, index: usize) {
        let scope = &mut self.scopes[self.current.index()];
        debug_assert!(!scope.is_sorted());
        scope.bindings.push(Binding {
            symbol,
            index: index as u16,
            kind: BindingKind::FormalParameter,
            captured: false,
        });
        scope.num_formal_parameters += 1;
    }

    pub fn add_mutable(&mut self, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[self.current.index()];
        debug_assert!(!scope.is_sorted());
        scope.bindings.push(Binding {
            symbol,
            index,
            kind: BindingKind::Mutable,
            captured: false,
        });
        scope.num_locals += 1;
    }

    pub fn add_immutable(&mut self, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[self.current.index()];
        debug_assert!(!scope.is_sorted());
        scope.bindings.push(Binding {
            symbol,
            index,
            kind: BindingKind::Immutable,
            captured: false,
        });
        scope.num_locals += 1;
    }

    pub fn set_immutable(&mut self, n: u32) {
        let scope = &mut self.scopes[self.current.index()];
        debug_assert!(!scope.is_sorted());
        for binding in scope.bindings.iter_mut().rev().take(n as usize) {
            debug_assert!(matches!(binding.kind, BindingKind::Mutable));
            binding.kind = BindingKind::Immutable;
        }
    }

    pub fn set_captured(&mut self, binding_ref: BindingRef) {
        let scope = &mut self.scopes[binding_ref.scope_index()];
        debug_assert!(scope.is_sorted());
        scope.bindings[binding_ref.binding_index()].captured = true;
    }

    pub fn max_stack_size(&self, scope_ref: ScopeRef) -> u16 {
        let scope = &self.scopes[scope_ref.index()];
        debug_assert!(scope.max_child_block_depth >= scope.depth);
        scope.max_child_block_depth - scope.depth + 1
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

    pub fn compute_locator(&self, binding_ref: BindingRef) -> Locator {
        let scope = &self.scopes[binding_ref.scope_index()];
        let binding = &scope.bindings[binding_ref.binding_index()];
        match binding.kind {
            BindingKind::FormalParameter => Locator::Argument(binding.index),
            _ => Locator::Local(binding.index),
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

// TODO: refactoring
pub struct Scope {
    label: &'static str,
    pub bindings: Vec<Binding>,
    pub num_formal_parameters: u16,
    pub num_locals: u16,
    outer: ScopeRef,
    depth: u16,
    max_child_block_depth: u16,
    flags: ScopeFlags,
}

impl Scope {
    const NONE: Self = Self {
        label: "",
        bindings: vec![],
        num_formal_parameters: 0,
        num_locals: 0,
        outer: ScopeRef::NONE,
        depth: 0,
        max_child_block_depth: 0,
        flags: ScopeFlags::empty(),
    };

    pub fn is_function(&self) -> bool {
        self.flags.contains(ScopeFlags::FUNCTION)
    }

    fn is_sorted(&self) -> bool {
        self.flags.contains(ScopeFlags::SORTED)
    }
}

struct ScopePrinter<'a> {
    index: usize,
    scope: &'a Scope,
}

impl<'a> std::fmt::Display for ScopePrinter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:indent$}", "", indent = self.scope.depth as usize)?;
        if !self.scope.is_sorted() {
            write!(f, "*")?;
        }
        if self.scope.is_function() {
            write!(f, "F")?;
        } else {
            write!(f, "B")?;
        }
        if !self.scope.label.is_empty() {
            write!(f, ".{}", self.scope.label)?;
        }
        write!(f, "@{}:", self.index)?;
        for binding in self.scope.bindings.iter() {
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
pub struct Binding {
    pub symbol: Symbol,
    pub index: u16,
    pub kind: BindingKind,
    pub captured: bool,
}

impl Binding {
    pub fn is_local(&self) -> bool {
        !matches!(self.kind, BindingKind::FormalParameter)
    }

    pub fn locator(&self) -> Locator {
        match self.kind {
            BindingKind::FormalParameter => Locator::Argument(self.index),
            _ => Locator::Local(self.index),
        }
    }
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            BindingKind::FormalParameter => write!(f, "P@{}:{}", self.index, self.symbol)?,
            BindingKind::Mutable => write!(f, "M@{}:{}", self.index, self.symbol)?,
            BindingKind::Immutable => write!(f, "I@{}:{}", self.index, self.symbol)?,
        }
        if self.captured {
            write!(f, "*")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum BindingKind {
    FormalParameter,
    Mutable,
    Immutable,
}
