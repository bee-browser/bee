use std::collections::BTreeSet;
use std::sync::Arc;

use base::macros::delegate_all;

use crate::grammar::Grammar;
use crate::grammar::Rule;
use crate::grammar::Term;
use crate::phrase::Phrase;

/// Represents an immutable LR item.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LrItem {
    pub rule: Arc<Rule>,
    pub dot: usize,
    pub lookahead: Phrase,
}

impl LrItem {
    pub fn to_original(&self, grammar: &Grammar) -> Self {
        let original = grammar.to_original_rule(self.rule.clone());
        let dot = if self.dot > 0 {
            // Re-compute the cursor position.
            let mut i = 0;
            let mut dot = 0;
            while i < self.dot && dot < original.production.len() {
                if !original.production[dot].is_lookahead() {
                    i += 1;
                }
                dot += 1;
            }
            dot
        } else {
            self.dot
        };
        LrItem {
            rule: original,
            dot,
            lookahead: self.lookahead.clone(),
        }
    }

    pub fn with_lookahead(&self, lookahead: Phrase) -> Self {
        LrItem {
            rule: self.rule.clone(),
            dot: self.dot,
            lookahead,
        }
    }

    pub fn without_lookahead(&self) -> Self {
        LrItem {
            rule: self.rule.clone(),
            dot: self.dot,
            lookahead: Phrase::empty(),
        }
    }

    pub fn shift(&self) -> Self {
        assert!(self.dot < self.rule.production.len(), "{self}");
        LrItem {
            rule: self.rule.clone(),
            dot: self.dot + 1,
            lookahead: self.lookahead.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.rule.production.is_empty() {
            return true;
        }
        self.rule.production.iter().all(|term| match term {
            Term::Empty | Term::Lookahead(_) | Term::Disallow(_) => true,
            _ => false,
        })
    }

    pub fn k(&self) -> usize {
        self.lookahead.len()
    }

    pub fn is_kernel(&self) -> bool {
        self.rule.is_goal_of_augmented_grammar() || self.dot > 0
    }

    pub fn is_reducible(&self) -> bool {
        self.next_term().is_none()
    }

    pub fn is_disallowed(&self, disallowed: &str) -> bool {
        match self.next_term() {
            Some(Term::Disallow(token)) if token == disallowed => return true,
            _ => (),
        }
        match self.prev_term() {
            Some(Term::Disallow(token)) if token == disallowed => return true,
            _ => (),
        }
        false
    }

    pub fn is_restricted(&self) -> bool {
        match self.next_term() {
            Some(Term::Disallow(_)) => true,
            _ => false,
        }
    }

    pub fn prev_term(&self) -> Option<&Term> {
        if self.dot == 0 {
            return None;
        }
        self.rule.production.get(self.dot - 1)
    }

    pub fn next_term(&self) -> Option<&Term> {
        self.rule.production.get(self.dot)
    }

    pub fn follower_terms(&self) -> &[Term] {
        &self.rule.production[self.dot..]
    }
}

impl std::fmt::Display for LrItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} ->", self.rule.name)?;
        for (i, term) in self.rule.production.iter().enumerate() {
            if i == self.dot {
                write!(f, " .")?;
            }
            write!(f, " {}", term)?;
        }
        if self.dot == self.rule.production.len() {
            write!(f, " .")?;
        }
        if !self.lookahead.is_empty() {
            write!(f, ", {}", self.lookahead)?;
        }
        write!(f, "]")?;
        if self.is_kernel() {
            write!(f, "*")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct LrItemSet(BTreeSet<LrItem>);
delegate_all! {LrItemSet => BTreeSet<LrItem>}

/// Represents an immutable LR item set.
impl LrItemSet {
    pub fn to_original(&self, grammar: &Grammar) -> Self {
        let mut set: BTreeSet<LrItem> = Default::default();
        for item in self.iter() {
            set.insert(item.to_original(grammar));
        }
        LrItemSet(set)
    }

    /// Returns an iterator over kernel items.
    pub fn kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.iter().filter(|item| item.is_kernel())
    }

    /// Returns an iterator over non-kernel items.
    pub fn non_kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.iter().filter(|item| !item.is_kernel())
    }

    /// Returns the kernel item set.
    pub fn kernel_set(&self) -> Self {
        let mut set: BTreeSet<LrItem> = Default::default();
        for item in self.kernel_items().cloned() {
            set.insert(item);
        }
        LrItemSet(set)
    }

    /// Returns the non-kernel item set.
    pub fn non_kernel_set(&self) -> Self {
        let mut set: BTreeSet<LrItem> = Default::default();
        for item in self.non_kernel_items().cloned() {
            set.insert(item);
        }
        LrItemSet(set)
    }

    /// Merges two item sets.
    pub fn merge(&self, other: &Self) -> Self {
        let mut set = self.0.clone();
        if set.eq(other) {
            return LrItemSet(set);
        }
        for item in other.iter() {
            set.insert(item.clone());
        }
        LrItemSet(set)
    }
}

impl std::fmt::Display for LrItemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        // Show only kernel items.
        let mut items = self.iter().filter(|item| item.is_kernel());
        if let Some(item) = items.next() {
            write!(f, "{item}")?;
        }
        for item in items {
            write!(f, ",{item}")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod macros {
    macro_rules! lr_item {
        ($rule:expr, $dot:expr, $lookahead:expr) => {
            crate::lr::LrItem {
                rule: std::sync::Arc::new($rule),
                dot: $dot,
                lookahead: $lookahead,
            }
        };
    }

    macro_rules! lr0_item {
        ($rule:expr, $dot:expr) => {
            lr_item!($rule, $dot, crate::phrase::Phrase::empty())
        };
    }

    pub(crate) use lr0_item;
    pub(crate) use lr_item;
}

#[cfg(test)]
mod tests {
    use super::macros::*;
    use crate::grammar::macros::*;
    use crate::phrase::macros::*;

    #[test]
    fn test_lr_item_is_empty() {
        assert!(!lr0_item!(rule!("A" -> token!("a")), 0).is_empty());
        assert!(lr0_item!(rule!("A" ->), 0).is_empty())
    }

    #[test]
    fn test_lr_item_format() {
        assert_eq!(
            format!("{}", lr0_item!(rule!("A" -> token!("a") token!("b")), 0)),
            "[A -> . a b]"
        );
        assert_eq!(
            format!(
                "{}",
                lr_item!(rule!("A" -> token!("a") token!("b")), 1, phrase!["c"])
            ),
            "[A -> a . b, c]*"
        );
    }
}
