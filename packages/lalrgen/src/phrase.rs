use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use bee_macros::delegate_all;

/// An immutable object representing a sequence of tokens.
///
/// A `Phrase` object may not be a phrase in the linguistic sense.  It can be an empty or contain
/// only a single token in order to provide functions needed for implementing the LALR parsing
/// tables generator.  For example, an empty phrase is used for representing an epsilon transition
/// in an automaton.
#[derive(Clone, Debug, Deserialize, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Phrase(Arc<Vec<String>>);

delegate_all! {Phrase => Arc<Vec<String>>}

impl Phrase {
    pub fn new(tokens: Vec<String>) -> Self {
        Phrase(Arc::new(tokens))
    }

    pub fn empty() -> Self {
        Default::default()
    }

    pub fn starts_with(&self, token: &str) -> bool {
        self.0.first().filter(|&tok| token == tok).is_some()
    }

    pub fn remove_first(&self) -> Option<Self> {
        self.split_first()
            .and_then(|(_, remaining)| {
                if remaining.is_empty() {
                    None
                } else {
                    Some(remaining)
                }
            })
            .map(|remaining| Self::new(remaining.to_vec()))
    }

    pub fn concat(&self, other: &Self) -> Self {
        let capacity = self.len() + other.len();
        let mut tokens = Vec::with_capacity(capacity);
        for token in self.iter() {
            tokens.push(token.clone());
        }
        for token in other.iter() {
            tokens.push(token.clone());
        }
        Self::new(tokens)
    }

    pub fn shorten(&self, n: usize) -> Self {
        if n == 0 {
            Default::default()
        } else {
            let mut tokens = Vec::with_capacity(n);
            for token in self.iter().take(n) {
                tokens.push(token.clone());
            }
            Self::new(tokens)
        }
    }

    pub fn count_tokens(&self) -> usize {
        // A token used for restrictions doesn't increase the number of lookahead tokens.
        self.iter().filter(|token| !token.starts_with("(!")).count()
    }
}

impl std::fmt::Display for Phrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.split_last() {
            Some((last, precedings)) => {
                for token in precedings {
                    write!(f, "{token} ")?;
                }
                write!(f, "{last}")
            }
            None => write!(f, "()"),
        }
    }
}

/// An immutable object representing a set of phrases.
///
/// `BTreeSet` is internally used for representing this type and supporting the hash calculation.
#[derive(Clone, Debug, Deserialize, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PhraseSet(Arc<BTreeSet<Phrase>>);

delegate_all! {PhraseSet => Arc<BTreeSet<Phrase>>}

impl PhraseSet {
    pub fn new(set: BTreeSet<Phrase>) -> Self {
        PhraseSet(Arc::new(set))
    }

    pub fn max_tokens(&self) -> Option<usize> {
        self.iter().map(Phrase::count_tokens).max()
    }

    pub fn min_tokens(&self) -> Option<usize> {
        self.iter().map(Phrase::count_tokens).min()
    }

    pub fn concat(&self, other: &Self) -> Self {
        let mut set = BTreeSet::default();
        for a in self.iter() {
            for b in other.iter() {
                set.insert(a.concat(b));
            }
        }
        Self::new(set)
    }

    pub fn concat_phrase(&self, phrase: &Phrase) -> Self {
        let mut set = BTreeSet::default();
        for p in self.iter() {
            set.insert(p.concat(phrase));
        }
        Self::new(set)
    }

    pub fn shorten(&self, n: usize) -> Self {
        let mut set = BTreeSet::default();
        for phrase in self.iter() {
            set.insert(phrase.shorten(n));
        }
        Self::new(set)
    }

    pub fn merge(&self, other: &Self) -> Self {
        if self.eq(other) {
            return self.clone();
        }
        let mut set = self.0.deref().clone();
        for phrase in other.iter() {
            set.insert(phrase.clone());
        }
        Self::new(set)
    }

    pub fn includes(&self, token: &str) -> MatchStatus<Self> {
        let matches = self.iter().filter(|seq| seq.starts_with(token));

        if matches.clone().count() == 0 {
            // The condition has been unmet.
            return MatchStatus::Unmatched;
        }

        // Create a new phrase set for the subsequent look-ahead by removing the token from each
        // phrase in `matches`.
        let mut set = BTreeSet::default();
        for phrase in matches {
            if let Some(phrase) = phrase.remove_first() {
                set.insert(phrase);
            }
        }

        if set.is_empty() {
            // The condition has been met.
            MatchStatus::Matched
        } else {
            // Need to continue checking the remaining condition.
            MatchStatus::Remaining(Self::new(set))
        }
    }

    pub fn excludes(&self, token: &str) -> MatchStatus<Self> {
        let matches = self.iter().filter(|seq| seq.starts_with(token));

        if matches.clone().count() == 0 {
            // The condition has been met.
            return MatchStatus::Matched;
        }

        // Create a new phrase set for the subsequent look-ahead by removing the token from each
        // phrase in `matches`.
        let mut set = BTreeSet::default();
        for phrase in matches {
            if let Some(phrase) = phrase.remove_first() {
                set.insert(phrase);
            }
        }

        if set.is_empty() {
            // The condition has been unmet.
            MatchStatus::Unmatched
        } else {
            // Need to continue checking the remaining condition.
            MatchStatus::Remaining(Self::new(set))
        }
    }
}

impl std::fmt::Display for PhraseSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.len();
        write!(f, "[")?;
        for (i, phrase) in self.iter().enumerate() {
            write!(f, "{phrase}")?;
            if i != n - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
pub enum MatchStatus<T> {
    Matched,
    Unmatched,
    Remaining(T),
}

// macros

pub(crate) mod macros {
    macro_rules! phrase {
        () => {
            crate::phrase::Phrase::default()
        };
        ($($tokens:expr,)+) => {
            phrase!($($tokens),+)
        };
        ($($tokens:expr),+) => {
            crate::phrase::Phrase::new([$($tokens),+].iter().map(|token| token.to_string()).collect())
        };
    }

    macro_rules! phrase_set {
        () => {
            crate::phrase::PhraseSet::default()
        };
        ($($phrases:expr,)+) => {
            phrase_set![$($phrases),+]
        };
        ($($phrases:expr),+) => {
            crate::phrase::PhraseSet::new([$($phrases),+].into())
        };
    }

    pub(crate) use phrase;
    pub(crate) use phrase_set;
}

#[cfg(test)]
mod tests {
    use super::macros::*;
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_phrase_starts_with() {
        assert!(!phrase!().starts_with("a"));
        assert!(phrase!("a").starts_with("a"));
        assert!(!phrase!("a").starts_with("b"));
        assert!(phrase!("a", "b").starts_with("a"));
        assert!(!phrase!("a", "b").starts_with("b"));
    }

    #[test]
    fn test_phrase_concat() {
        assert_eq!(phrase!().concat(&phrase!()), phrase!());
        assert_eq!(phrase!().concat(&phrase!("a")), phrase!("a"));
        assert_eq!(phrase!("a").concat(&phrase!()), phrase!("a"));
        assert_eq!(phrase!("a").concat(&phrase!("b")), phrase!("a", "b"));
    }

    #[test]
    fn test_phrase_format() {
        assert_eq!(format!("{}", phrase!()), "()");
        assert_eq!(format!("{}", phrase!("a")), "a");
        assert_eq!(format!("{}", phrase!("a", "b")), "a b");
    }

    #[test]
    fn test_phrase_min_tokens() {
        assert_eq!(phrase_set![].min_tokens(), None);
        assert_eq!(phrase_set![phrase!(), phrase!("a")].min_tokens(), Some(0));
    }

    #[test]
    fn test_phrase_set_concat() {
        let empty = phrase_set![];
        let epsilon = phrase_set![phrase!()];
        let a = phrase_set![phrase!("a")];
        let ab = phrase_set![phrase!("a"), phrase!("b")];

        assert_eq!(empty.concat(&empty), empty);
        assert_eq!(empty.concat(&epsilon), empty);
        assert_eq!(empty.concat(&a), empty);
        assert_eq!(empty.concat(&ab), empty);

        assert_eq!(epsilon.concat(&empty), empty);
        assert_eq!(epsilon.concat(&epsilon), epsilon);
        assert_eq!(epsilon.concat(&a), a);
        assert_eq!(epsilon.concat(&ab), ab);

        assert_eq!(a.concat(&empty), empty);
        assert_eq!(a.concat(&epsilon), a);
        assert_eq!(a.concat(&a), phrase_set![phrase!("a", "a")]);
        assert_eq!(
            a.concat(&ab),
            phrase_set![phrase!("a", "a"), phrase!("a", "b")]
        );

        assert_eq!(ab.concat(&empty), empty);
        assert_eq!(ab.concat(&epsilon), ab);
        assert_eq!(
            ab.concat(&a),
            phrase_set![phrase!("a", "a"), phrase!("b", "a")]
        );
        assert_eq!(
            ab.concat(&ab),
            phrase_set![
                phrase!("a", "a"),
                phrase!("a", "b"),
                phrase!("b", "a"),
                phrase!("b", "b")
            ]
        );
    }

    #[test]
    fn test_phrase_set_merge() {
        let ab = phrase_set![phrase!("a"), phrase!("b")];
        let bc = phrase_set![phrase!("b"), phrase!("c")];
        assert_eq!(ab.merge(&ab), ab);
        assert_eq!(
            ab.merge(&bc),
            phrase_set![phrase!("a"), phrase!("b"), phrase!("c")]
        );
    }

    #[test]
    fn test_phrase_set_includes() {
        let set = phrase_set![phrase!("a", "b"), phrase!("a", "c"), phrase!("x")];
        assert_matches!(set.includes("0"), MatchStatus::Unmatched);
        assert_matches!(set.includes("a"), MatchStatus::Remaining(set) => {
            assert_eq!(set, phrase_set![phrase!("b"), phrase!("c")]);
        });
        assert_matches!(set.includes("x"), MatchStatus::Matched);
    }

    #[test]
    fn test_phrase_set_excludes() {
        let set = phrase_set![phrase!("a", "b"), phrase!("a", "c"), phrase!("x")];
        assert_matches!(set.excludes("0"), MatchStatus::Matched);
        assert_matches!(set.excludes("a"), MatchStatus::Remaining(set) => {
            assert_eq!(set, phrase_set![phrase!("b"), phrase!("c")]);
        });
        assert_matches!(set.excludes("x"), MatchStatus::Unmatched);
    }

    #[test]
    fn test_phrase_set_format() {
        assert_eq!(format!("{}", phrase_set![]), "[]");
        assert_eq!(format!("{}", phrase_set![phrase!()]), "[()]");
        assert_eq!(format!("{}", phrase_set![phrase!("a")]), "[a]");
        assert_eq!(
            format!("{}", phrase_set![phrase!("a"), phrase!("b")]),
            "[a, b]"
        )
    }
}
