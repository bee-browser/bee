#[cfg(test)]
mod macros;

use std::collections::BTreeSet;
use std::sync::Arc;

use bee_macros::delegate_all;

use crate::grammar::Rule;
use crate::grammar::Term;
use crate::phrase::Phrase;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LrItem {
    pub rule: Arc<Rule>,
    pub dot: usize,
    pub lookahead: Phrase,
}

impl LrItem {
    pub fn to_gramatical(&self) -> Self {
        LrItem {
            rule: Arc::new(self.rule.to_grammatical()),
            dot: self.dot,
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
        LrItem {
            rule: self.rule.clone(),
            dot: self.dot + 1,
            lookahead: self.lookahead.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rule.production.is_empty()
            || self.rule.production.iter().all(|term| match term {
                Term::Empty | Term::Lookahead(_) => true,
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

impl LrItemSet {
    pub fn merge(&self, other: &Self) -> Self {
        let mut set = self.clone();
        if set.eq(other) {
            return set;
        }
        for item in other.iter() {
            set.insert(item.clone());
        }
        set
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
