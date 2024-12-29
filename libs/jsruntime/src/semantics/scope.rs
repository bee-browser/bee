// TODO(feat): Symbols in a scope must be unique
//   10.2.11 FunctionDeclarationInstantiation ( func, argumentsList )
//   16.1.7 GlobalDeclarationInstantiation ( script, env )

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
pub struct VariableRef(u16, u16);

impl VariableRef {
    pub const NONE: Self = Self::new(0, 0);

    pub const fn new(scope_index: u16, variable_index: u16) -> Self {
        Self(scope_index, variable_index)
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

    fn variable_index(&self) -> usize {
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

    pub fn iter_variables(
        &self,
        scope_ref: ScopeRef,
    ) -> impl Iterator<Item = (VariableRef, &Variable)> {
        self.scopes[scope_ref.index()]
            .variables
            .iter()
            .enumerate()
            .map(move |(index, variable)| (VariableRef::new(scope_ref.0, index as u16), variable))
    }

    pub fn get_symbol(&self, variable_ref: VariableRef) -> Symbol {
        let scope = &self.scopes[variable_ref.scope_index()];
        scope.variables[variable_ref.variable_index()].symbol
    }

    pub fn find_variable(&self, scope_ref: ScopeRef, symbol: Symbol) -> VariableRef {
        let mut scope_ref = scope_ref;
        loop {
            let scope = &self.scopes[scope_ref.index()];
            match scope
                .variables
                .binary_search_by_key(&symbol, |variable| variable.symbol)
            {
                Ok(index) => {
                    // TODO: should return an error
                    return VariableRef::checked_new(scope_ref, index).unwrap();
                }
                Err(_) => {
                    if scope.is_function() {
                        // Reference to a free variable.
                        return VariableRef::NONE;
                    }
                    scope_ref = scope.outer;
                    if scope_ref == ScopeRef::NONE {
                        // Reference to a property of the global object.
                        return VariableRef::NONE;
                    }
                }
            }
        }
    }

    pub fn compute_locator(&self, variable_ref: VariableRef) -> Locator {
        let scope = &self.scopes[variable_ref.scope_index()];
        let variable = &scope.variables[variable_ref.variable_index()];
        match variable.kind {
            VariableKind::FormalParameter => Locator::Argument(variable.index),
            VariableKind::Mutable | VariableKind::Immutable => Locator::Local(variable.index),
            VariableKind::Capture => Locator::Capture(variable.index),
            VariableKind::Global => Locator::Global,
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
    pub fn push_function(&mut self) -> ScopeRef {
        self.push(ScopeKind::Function)
    }

    pub fn push_block(&mut self) -> ScopeRef {
        self.push(ScopeKind::Block)
    }

    fn push(&mut self, kind: ScopeKind) -> ScopeRef {
        let index = self.scopes.len();
        self.scopes.push(Scope {
            variables: vec![],
            outer: self.current,
            depth: self.depth,
            max_child_block_depth: self.depth,
            kind,
        });
        // TODO: should return an error
        self.current = ScopeRef::checked_new(index).unwrap();
        self.depth += 1;
        self.current
    }

    pub fn pop(&mut self) {
        let scope = &mut self.scopes[self.current.index()];
        scope
            .variables
            .sort_unstable_by_key(|variable| variable.symbol);
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

    pub fn add_formal_parameter(&mut self, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[self.current.index()];
        scope.variables.push(Variable {
            symbol,
            index,
            kind: VariableKind::FormalParameter,
            flags: VariableFlags::empty(),
        });
    }

    pub fn add_mutable(&mut self, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[self.current.index()];
        scope.variables.push(Variable {
            symbol,
            index,
            kind: VariableKind::Mutable,
            flags: VariableFlags::empty(),
        });
    }

    pub fn add_function_scoped_mutable(&mut self, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[self.current.index()];
        scope.variables.push(Variable {
            symbol,
            index,
            kind: VariableKind::Mutable,
            flags: VariableFlags::FUNCTION_SCOPED,
        });
    }

    pub fn add_immutable(&mut self, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[self.current.index()];
        scope.variables.push(Variable {
            symbol,
            index,
            kind: VariableKind::Immutable,
            flags: VariableFlags::empty(),
        });
    }

    pub fn add_capture(&mut self, scope_ref: ScopeRef, symbol: Symbol, index: u16) {
        let scope = &mut self.scopes[scope_ref.index()];
        debug_assert!(scope.is_function());
        scope.variables.push(Variable {
            symbol,
            index,
            kind: VariableKind::Capture,
            flags: VariableFlags::empty(),
        });
        scope
            .variables
            .sort_unstable_by_key(|variable| variable.symbol); // TODO(perf)
    }

    pub fn add_global(&mut self, scope_ref: ScopeRef, symbol: Symbol) {
        let scope = &mut self.scopes[scope_ref.index()];
        debug_assert!(scope.is_function());
        scope.variables.push(Variable {
            symbol,
            index: 0, // TODO
            kind: VariableKind::Global,
            flags: VariableFlags::empty(),
        });
        scope
            .variables
            .sort_unstable_by_key(|variable| variable.symbol); // TODO(perf)
    }

    pub fn set_captured(&mut self, variable_ref: VariableRef) {
        let scope = &mut self.scopes[variable_ref.scope_index()];
        scope.variables[variable_ref.variable_index()].set_captured();
    }

    #[allow(unused)]
    pub fn max_scope_depth(&self, scope_ref: ScopeRef) -> u16 {
        let scope = &self.scopes[scope_ref.index()];
        debug_assert!(scope.max_child_block_depth >= scope.depth);
        scope.max_child_block_depth - scope.depth + 1
    }

    pub fn resolve_reference(&self, reference: &Reference) -> VariableRef {
        let symbol = reference.symbol;
        let mut scope_ref = reference.scope_ref;
        loop {
            let scope = &self.scopes[scope_ref.index()];
            match scope
                .variables
                .binary_search_by_key(&symbol, |variable| variable.symbol)
            {
                Ok(index) => {
                    // TODO: should return an error
                    return VariableRef::checked_new(scope_ref, index).unwrap();
                }
                Err(_) => {
                    if scope.is_function() {
                        // Reference to a free variable.
                        return VariableRef::NONE;
                    }
                    scope_ref = scope.outer;
                    if scope_ref == ScopeRef::NONE {
                        // Reference to a property of the global object.
                        return VariableRef::NONE;
                    }
                }
            }
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

// TODO(refactor): Currently, the function scope holds variables for free variables (references
// to global variables and captures) in addition to formal parameters and local variables
// (function-scoped variables and top-level lexical variables).  The variables might be better to
// place in separate scopes like this:
//
//   function-scope: formal parameter
//     scope-for-free-variables: free variables
//       scope-for-function-scoped-variables: variables declared with `var`
//         block-scope: top-level lexical variables
//           child-block-scope
//           ...
//
// This change increases the depth, but makes it possible to easily access to particular variables.
// Block scopes hold only lexically-scoped variables.
pub struct Scope {
    pub variables: Vec<Variable>,
    outer: ScopeRef,
    depth: u16,
    max_child_block_depth: u16,
    kind: ScopeKind,
}

impl Scope {
    const NONE: Self = Self {
        variables: vec![],
        outer: ScopeRef::NONE,
        depth: 0,
        max_child_block_depth: 0,
        kind: ScopeKind::Block,
    };

    pub fn is_function(&self) -> bool {
        matches!(self.kind, ScopeKind::Function)
    }

    pub fn count_captures(&self) -> u16 {
        self.variables
            .iter()
            .filter(|variable| variable.is_capture())
            .count() as u16
    }
}

struct ScopePrinter<'a> {
    index: usize,
    scope: &'a Scope,
}

impl std::fmt::Display for ScopePrinter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:indent$}", "", indent = self.scope.depth as usize)?;
        match self.scope.kind {
            ScopeKind::Block => write!(f, "B")?,
            ScopeKind::Function => write!(f, "F")?,
        }
        write!(f, "@{}:", self.index)?;
        for variable in self.scope.variables.iter() {
            write!(f, " {variable}")?;
        }
        Ok(())
    }
}

enum ScopeKind {
    Block,
    Function,
}

#[derive(Debug)]
pub struct Variable {
    pub symbol: Symbol,
    pub index: u16,
    pub kind: VariableKind,
    flags: VariableFlags,
}

impl Variable {
    pub fn is_local(&self) -> bool {
        matches!(self.kind, VariableKind::Immutable | VariableKind::Mutable)
    }

    pub fn is_capture(&self) -> bool {
        matches!(self.kind, VariableKind::Capture)
    }

    pub fn locator(&self) -> Locator {
        match self.kind {
            VariableKind::FormalParameter => Locator::Argument(self.index),
            VariableKind::Mutable | VariableKind::Immutable => Locator::Local(self.index),
            VariableKind::Capture => Locator::Capture(self.index),
            VariableKind::Global => Locator::Global,
        }
    }

    pub fn is_captured(&self) -> bool {
        self.flags.contains(VariableFlags::CAPTURED)
    }

    pub fn is_function_scoped(&self) -> bool {
        self.flags.contains(VariableFlags::FUNCTION_SCOPED)
    }

    fn set_captured(&mut self) {
        self.flags.insert(VariableFlags::CAPTURED)
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_captured() {
            write!(f, "*")?;
        }
        if self.is_function_scoped() {
            write!(f, "^")?;
        }
        match self.kind {
            VariableKind::FormalParameter => write!(f, "P@{}:{}", self.index, self.symbol)?,
            VariableKind::Mutable => write!(f, "M@{}:{}", self.index, self.symbol)?,
            VariableKind::Immutable => write!(f, "I@{}:{}", self.index, self.symbol)?,
            VariableKind::Capture => write!(f, "C@{}:{}", self.index, self.symbol)?,
            VariableKind::Global => write!(f, "G@{}:{}", self.index, self.symbol)?,
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum VariableKind {
    FormalParameter,
    Mutable,
    Immutable,
    Capture,
    Global,
}

bitflags! {
    #[derive(Debug)]
    struct VariableFlags: u8 {
        const CAPTURED        = 1 << 0;
        const FUNCTION_SCOPED = 1 << 1;
    }
}
