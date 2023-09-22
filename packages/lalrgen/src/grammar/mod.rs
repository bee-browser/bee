#[cfg(test)]
pub(crate) mod macros;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::phrase::MatchStatus;
use crate::phrase::PhraseSet;

// grammar

#[derive(Debug)]
pub struct Grammar {
    rules: Vec<Arc<Rule>>,
    non_terminals: HashMap<NonTerminal, Vec<Arc<Rule>>>,
}

impl Grammar {
    pub fn new(rules: Vec<Arc<Rule>>) -> Self {
        let mut non_terminals: HashMap<_, Vec<_>> = HashMap::new();
        for rule in rules.iter() {
            non_terminals
                .entry(rule.name.clone())
                .or_default()
                .push(rule.clone());
        }
        Grammar {
            rules,
            non_terminals,
        }
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn rules(&self) -> &[Arc<Rule>] {
        self.rules.as_slice()
    }

    pub fn non_terminals(&self) -> impl Iterator<Item = &NonTerminal> {
        self.non_terminals.keys()
    }

    pub fn non_terminal_rules(&self, non_terminal: &NonTerminal) -> &[Arc<Rule>] {
        self.non_terminals.get(non_terminal).unwrap()
    }

    pub fn create_augmented_grammar(&self, goal_symbol: &str) -> Self {
        let goal_symbol = NonTerminal::from(goal_symbol);

        // Allocate enough space in order to avoid re-allocation.
        let mut rules = Vec::with_capacity(1 + self.rules.len());

        // Add the production rule for the start symbol of the augmented grammar.
        rules.push(Arc::new(Rule::new(
            NonTerminal::GoalOfAugmentedGrammar,
            vec![Term::NonTerminal(goal_symbol.clone())],
        )));

        // Collect only rules actually used.
        let mut collected: HashSet<NonTerminal> = Default::default();
        let mut remaining: VecDeque<NonTerminal> = Default::default();
        remaining.push_back(goal_symbol);
        while let Some(non_terminal) = remaining.pop_front() {
            // `remaining` may contain a non-terminal already collected.
            if collected.contains(&non_terminal) {
                continue;
            }
            collected.insert(non_terminal.clone());
            for rule in self.non_terminal_rules(&non_terminal).iter() {
                rules.push(rule.clone());
                for term in rule.production.iter() {
                    if let Term::NonTerminal(non_terminal) = term {
                        if !collected.contains(non_terminal) {
                            // `remaining` may already contain the same non-terminal.
                            remaining.push_back(non_terminal.clone());
                        }
                    }
                }
            }
        }

        // Report removed non-terminals.
        for non_terminal in self.non_terminals() {
            if !collected.contains(non_terminal) {
                tracing::debug!(removed = %non_terminal);
            }
        }

        // Shrink the allocated space before creating a grammar object.
        rules.shrink_to_fit();
        Grammar::new(rules)
    }

    pub fn validate(&self) -> bool {
        let mut valid = true;
        for rule in self.rules.iter() {
            rule.production
                .iter()
                .filter_map(|term| match term {
                    Term::NonTerminal(non_terminal) => Some(non_terminal),
                    _ => None,
                })
                .for_each(|non_terminal| {
                    if let None = self.non_terminals.get(non_terminal) {
                        tracing::error!("{non_terminal} is not defined");
                        valid = false;
                    }
                });
        }
        valid
    }

    pub fn max_lookahead_tokens(&self) -> usize {
        self.rules
            .iter()
            .map(|rule| {
                rule.production
                    .iter()
                    .map(|term| match term {
                        Term::Lookahead(lookahead) => lookahead.max_tokens().unwrap_or(0),
                        _ => 0,
                    })
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }
}

impl PartialEq for Grammar {
    fn eq(&self, other: &Self) -> bool {
        self.rules.len() == other.rules.len() && self.non_terminals == other.non_terminals
    }
}

// rule

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Rule {
    pub name: NonTerminal,
    pub production: Vec<Term>,
    // TODO: remove this property and use an external map.
    #[serde(default)]
    pub derived_from: Option<Arc<Rule>>,
}

impl Rule {
    pub fn new(name: NonTerminal, production: Vec<Term>) -> Self {
        Rule {
            name,
            production,
            derived_from: None,
        }
    }

    pub fn with_rule(name: NonTerminal, production: Vec<Term>, rule: Arc<Rule>) -> Self {
        Rule {
            name,
            production,
            derived_from: Some(rule),
        }
    }

    pub fn count_symbols(&self) -> usize {
        self.production
            .iter()
            .filter(|term| term.is_symbol())
            .count()
    }

    pub fn to_original(&self) -> Self {
        if let Some(ref rule) = self.derived_from {
            rule.to_original()
        } else {
            self.clone()
        }
    }

    pub fn is_goal_of_augmented_grammar(&self) -> bool {
        self.name.is_goal_of_augmented_grammar()
    }

    pub fn has_tail_lookahead(&self) -> bool {
        match self.production.last() {
            Some(Term::Lookahead(_)) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ->", self.name)?;
        for term in self.production.iter() {
            write!(f, " {term}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Symbol {
    Token(String),
    NonTerminal(String),
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token(token) => write!(f, "{token}"),
            Self::NonTerminal(non_terminal) => write!(f, "{non_terminal}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum NonTerminal {
    Original(String),
    Variant(String, usize),
    GoalOfAugmentedGrammar,
}

impl NonTerminal {
    /// Returns a symbol without a variant ID.
    pub fn symbol(&self) -> &str {
        match self {
            Self::Original(symbol) => symbol,
            Self::Variant(symbol, _) => symbol,
            _ => "^",
        }
    }

    pub fn with_variant(&self, variant: usize) -> Self {
        let symbol = match self {
            Self::Original(symbol) => symbol,
            Self::Variant(symbol, _) => symbol,
            _ => unreachable!(),
        };
        NonTerminal::Variant(symbol.clone(), variant)
    }

    pub fn is_variant(&self) -> bool {
        match self {
            Self::Variant(..) => true,
            _ => false,
        }
    }

    pub fn is_goal_of_augmented_grammar(&self) -> bool {
        match self {
            Self::GoalOfAugmentedGrammar => true,
            _ => false,
        }
    }
}

impl From<&str> for NonTerminal {
    fn from(symbol: &str) -> Self {
        NonTerminal::Original(symbol.to_owned())
    }
}

impl std::fmt::Display for NonTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Original(symbol) => write!(f, "{symbol}"),
            Self::Variant(symbol, variant) => write!(f, "{symbol}.{variant}"),
            Self::GoalOfAugmentedGrammar => write!(f, "^"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum Term {
    Empty,
    Token(String),
    NonTerminal(NonTerminal),
    Lookahead(Arc<Lookahead>),
    Disallow(String),
}

impl Term {
    pub fn is_symbol(&self) -> bool {
        match self {
            Self::Token(_) | Self::NonTerminal(_) => true,
            _ => false,
        }
    }

    pub fn is_token(&self) -> bool {
        match self {
            Self::Token(_) => true,
            _ => false,
        }
    }

    pub fn is_lookahead(&self) -> bool {
        match self {
            Self::Lookahead(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "(empty)"),
            Self::Token(token) => write!(f, "{token}"),
            Self::NonTerminal(non_terminal) => write!(f, "{non_terminal}"),
            Self::Lookahead(lookahead) => write!(f, "{lookahead}"),
            Self::Disallow(token) => write!(f, "(!{token})"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum Lookahead {
    Include(PhraseSet),
    Exclude(PhraseSet),
}

impl Lookahead {
    fn max_tokens(&self) -> Option<usize> {
        match self {
            Self::Include(set) => set.max_tokens(),
            Self::Exclude(set) => set.max_tokens(),
        }
    }

    pub fn process_token(&self, token: &str) -> MatchStatus<Arc<Lookahead>> {
        match self {
            Self::Include(set) => match set.includes(token) {
                MatchStatus::Matched => MatchStatus::Matched,
                MatchStatus::Unmatched => MatchStatus::Unmatched,
                MatchStatus::Remaining(set) => {
                    MatchStatus::Remaining(Arc::new(Lookahead::Include(set)))
                }
            },
            Self::Exclude(set) => match set.excludes(token) {
                MatchStatus::Matched => MatchStatus::Matched,
                MatchStatus::Unmatched => MatchStatus::Unmatched,
                MatchStatus::Remaining(set) => {
                    MatchStatus::Remaining(Arc::new(Lookahead::Exclude(set)))
                }
            },
        }
    }
}

impl std::fmt::Display for Lookahead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let set = match self {
            Self::Include(set) => {
                write!(f, "(?=")?;
                set
            }
            Self::Exclude(set) => {
                write!(f, "(?!")?;
                set
            }
        };
        write!(f, "{set})")
    }
}

#[cfg(test)]
mod tests {
    use super::macros::*;
    use crate::phrase::macros::*;

    #[test]
    fn test_rule_format() {
        let rule =
            rule!("A" -> token!("a") non_terminal!("B") lookahead!(x: phrase_set![phrase!("c")]));
        assert_eq!(format!("{}", rule), "A -> a B (?![c])");

        let rule = rule!("A" ->);
        assert_eq!(format!("{}", rule), "A ->");
    }

    #[test]
    fn test_term_format() {
        assert_eq!(format!("{}", empty!()), "(empty)");
        assert_eq!(format!("{}", token!("a")), "a");
        assert_eq!(format!("{}", non_terminal!("A")), "A");
        assert_eq!(
            format!("{}", lookahead!(phrase_set![phrase!("a")])),
            "(?=[a])"
        );
        assert_eq!(format!("{}", disallow!("a")), "(!a)");
    }
}
