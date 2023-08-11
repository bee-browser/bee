use std::collections::HashMap;

use serde::Deserialize;

use crate::automaton::Dfa;
use crate::automaton::Nfa;
use crate::automaton::StateId;
use crate::unicode::unicode_set;
use crate::unicode::unicode_span;
use crate::unicode::CodePoint;
use crate::unicode::UnicodeSet;
use crate::unicode::UnicodeSpan;

pub struct Grammar<'a, 'b> {
    pub rules: &'a [Rule],
    pub rule_map: HashMap<&'a str, Vec<&'a Rule>>,
    pub tokens: &'b [String],
}

impl<'a, 'b> Grammar<'a, 'b> {
    pub fn new(rules: &'a [Rule], tokens: &'b [String]) -> Self {
        let mut rule_map: HashMap<&'a str, Vec<&'a Rule>> = Default::default();
        for rule in rules.iter() {
            rule_map.entry(rule.name.as_str()).or_default().push(rule);
        }
        Grammar {
            rules,
            rule_map,
            tokens,
        }
    }

    pub fn compile(&self) -> Dfa {
        tracing::info!("Building an NFA from the lexical grammar in CFG...");
        let nfa = self.build_nfa();
        tracing::info!("#States in NFA: {}", nfa.size());

        tracing::info!("Building DFA from NFA...");
        let dfa = nfa.build_dfa();
        tracing::info!("#States in DFA: {}", dfa.size());

        tracing::info!("Minifying DFA...");
        let minified = dfa.minify(self.tokens);
        tracing::info!("#States in DFA (minified): {}", minified.size());

        minified
    }

    fn build_nfa(&self) -> Nfa {
        let mut builder = NfaBuilder::new(&self.rule_map);
        for token in self.tokens.iter() {
            tracing::debug!("Compiling {token} into NFA...");
            builder.add_token(token);
        }
        builder.build()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rule {
    name: String,
    production: Vec<Term>,
}

struct Label<'a>(&'a Rule, usize);

impl<'a> std::fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ->", self.0.name)?;
        for (i, term) in self.0.production.iter().enumerate() {
            if i == self.1 {
                write!(f, " .")?;
            }
            write!(f, " {term}")?;
        }
        if self.1 == self.0.production.len() {
            write!(f, " .")?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "data")]
pub enum Term {
    Empty,
    Any,
    UnicodeSet(Vec<UnicodePattern>),
    NonTerminal(String),
    Lookahead {
        patterns: Vec<UnicodePattern>,
        negate: bool,
    },
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "(empty)")?,
            Self::Any => write!(f, "(any)")?,
            Self::UnicodeSet(patterns) => {
                write!(f, "[")?;
                if let Some((last, patterns)) = patterns.split_last() {
                    for pattern in patterns.iter() {
                        write!(f, "{pattern} ")?;
                    }
                    write!(f, "{last}")?;
                }
                write!(f, "]")?;
            }
            Self::NonTerminal(non_terminal) => write!(f, "{non_terminal}")?,
            Self::Lookahead { patterns, negate } => {
                if *negate {
                    write!(f, "!")?;
                }
                write!(f, "?[")?;
                if let Some((last, patterns)) = patterns.split_last() {
                    for pattern in patterns.iter() {
                        write!(f, "{pattern} ")?;
                    }
                    write!(f, "{last}")?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "data")]
pub enum UnicodePattern {
    Any,
    BuiltIn(UnicodeBuiltInPattern),
    Char(char),
    Span(char, char),
    Exclude(String),
    NonTerminal(String),
}

impl std::fmt::Display for UnicodePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "(any)"),
            Self::BuiltIn(builtin) => write!(f, "{builtin}"),
            Self::Char(ch) => write!(f, "{}", CodePoint::from(*ch)),
            Self::Span(first, last) => {
                write!(f, "{}", UnicodeSpan::new((*first).into(), (*last).into()))
            }
            Self::Exclude(name) => write!(f, "-{name}"),
            Self::NonTerminal(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
pub enum UnicodeBuiltInPattern {
    TAB,
    VT,
    FF,
    SP,
    USP,
    LF,
    CR,
    LS,
    PS,
    ZWNJ,
    ZWJ,
    ZWNBSP,
}

impl std::fmt::Display for UnicodeBuiltInPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TAB => write!(f, "<TAB>"),
            Self::VT => write!(f, "<VT>"),
            Self::FF => write!(f, "<FF>"),
            Self::SP => write!(f, "<SP>"),
            Self::USP => write!(f, "<USP>"),
            Self::LF => write!(f, "<LF>"),
            Self::CR => write!(f, "<CR>"),
            Self::LS => write!(f, "<LS>"),
            Self::PS => write!(f, "<PS>"),
            Self::ZWNJ => write!(f, "<ZWNJ>"),
            Self::ZWJ => write!(f, "<ZWJ>"),
            Self::ZWNBSP => write!(f, "<ZWNBSP>"),
        }
    }
}

pub struct NfaBuilder<'a, 'b> {
    rule_map: &'a HashMap<&'b str, Vec<&'b Rule>>,
    nfa: Nfa,
    start_id: StateId,
    recursion_stack: Vec<RecursionContext>,
}

impl<'a, 'b> NfaBuilder<'a, 'b> {
    pub fn new(rule_map: &'a HashMap<&'b str, Vec<&'b Rule>>) -> Self {
        let mut nfa = Nfa::new();
        let start_id = nfa.create_state();
        nfa.add_label(start_id, "@start".to_string());

        NfaBuilder {
            rule_map,
            nfa,
            start_id,
            recursion_stack: vec![],
        }
    }

    pub fn add_token(&mut self, token: &str) {
        let start_id = self.nfa.create_state();

        let accept_id = self.nfa.create_state();
        self.nfa.accept(accept_id, token.to_string());

        let (inner_start_id, inner_end_id) = self.build_nfa_for_non_terminal(token, true);

        self.nfa.epsilon_transition(self.start_id, start_id);
        self.nfa.epsilon_transition(start_id, inner_start_id);
        self.nfa.epsilon_transition(inner_end_id, accept_id);
    }

    pub fn build(self) -> Nfa {
        self.nfa
    }

    fn build_nfa_for_non_terminal(&mut self, name: &str, accept: bool) -> (StateId, StateId) {
        if let Some(endpoints) = self.find_recursion(name) {
            self.mark_recursion();
            return endpoints;
        }

        let start_id = self.nfa.create_state();
        let end_id = self.nfa.create_state();

        let rules = self.rule_map.get(name).unwrap();
        self.recursion_stack.push(RecursionContext {
            item: Some((name.to_string(), (start_id, end_id))),
            recursion: false,
        });
        let (inner_start_id, inner_end_id) = self.build_nfa_for_rules(rules, accept);
        self.recursion_stack.pop();

        self.nfa.epsilon_transition(start_id, inner_start_id);
        self.nfa.epsilon_transition(inner_end_id, end_id);

        (start_id, end_id)
    }

    fn build_nfa_for_rules(&mut self, rules: &[&'b Rule], accept: bool) -> (StateId, StateId) {
        let start_id = self.nfa.create_state();
        let end_id = self.nfa.create_state();

        for &rule in rules.iter() {
            let (rule_start_id, rule_end_id) = self.build_nfa_for_rule(rule, accept);
            self.nfa.epsilon_transition(start_id, rule_start_id);
            self.nfa.epsilon_transition(rule_end_id, end_id);
        }

        (start_id, end_id)
    }

    fn build_nfa_for_rule(&mut self, rule: &Rule, accept: bool) -> (StateId, StateId) {
        let lookahead_index = rule.production.iter().position(|term| match term {
            Term::Lookahead { .. } => true,
            _ => false,
        });
        let normal_seq_end = if let Some(i) = lookahead_index {
            i
        } else {
            rule.production.len()
        };

        let start_id = self.nfa.create_state();

        let end_id = self.nfa.create_state();
        if accept {
            let label = Label(rule, rule.production.len());
            self.nfa.add_label(end_id, format!("{label}"));
        }

        let mut id = start_id;
        for i in 0..normal_seq_end {
            let term = &rule.production[i];

            self.recursion_stack.push(RecursionContext {
                item: None,
                recursion: false,
            });
            let (term_start_id, term_end_id) = self.build_nfa_for_term(term);
            let context = self.recursion_stack.pop().unwrap();

            // We build a DFA recognizing tokens that can be defined in regular expressions.
            // A production like `A -> aAb` is not allowed.
            // A rule including such productions cannot be represented in a regular
            // expression and a stack is needed for recognizing it.
            if context.recursion {
                assert!(i == 0 || i == rule.production.len() - 1);
            }

            let label = Label(rule, i);
            self.nfa.add_label(term_start_id, format!("{label}"));

            self.nfa.epsilon_transition(id, term_start_id);

            id = term_end_id;
        }

        // Process lookahead items.
        if let Some(i) = lookahead_index {
            let (lookahead_start_id, lookahead_end_id) =
                self.build_nfa_for_lookahead(&rule.production[i..]);

            let label = Label(rule, i);
            self.nfa.add_label(lookahead_end_id, format!("{label}"));

            self.nfa.epsilon_transition(id, lookahead_start_id);

            id = lookahead_end_id;
        }

        self.nfa.epsilon_transition(id, end_id);

        (start_id, end_id)
    }

    fn build_nfa_for_term(&mut self, term: &Term) -> (StateId, StateId) {
        match term {
            Term::Empty => self.build_nfa_for_empty(),
            Term::UnicodeSet(ref patterns) => self.build_nfa_for_unicode_set(patterns),
            Term::NonTerminal(ref name) => self.build_nfa_for_non_terminal(name, false),
            _ => unimplemented!(),
        }
    }

    fn build_nfa_for_empty(&mut self) -> (StateId, StateId) {
        let start_id = self.nfa.create_state();
        let end_id = self.nfa.create_state();
        self.nfa.epsilon_transition(start_id, end_id);
        (start_id, end_id)
    }

    fn build_nfa_for_unicode_set(&mut self, patterns: &[UnicodePattern]) -> (StateId, StateId) {
        let start_id = self.nfa.create_state();
        let end_id = self.nfa.create_state();
        let unicode_set = self.build_unicode_set(patterns);
        self.nfa.transition(start_id, end_id, unicode_set);
        (start_id, end_id)
    }

    fn build_nfa_for_lookahead(&mut self, rules: &[Term]) -> (StateId, StateId) {
        let start_id = self.nfa.create_state();
        let end_id = self.nfa.create_state();
        self.nfa.lookahead(end_id);

        let mut unicode_set = UnicodeSet::any();
        for rule in rules {
            match rule {
                Term::Lookahead { patterns, negate } => {
                    let mut us = self.build_unicode_set(patterns);
                    if *negate {
                        us = UnicodeSet::any().exclude(&us);
                    }
                    unicode_set = unicode_set.intersect(&us);
                }
                _ => unreachable!(),
            };
        }

        self.nfa.transition(start_id, end_id, unicode_set);

        (start_id, end_id)
    }

    fn build_unicode_set(&self, patterns: &[UnicodePattern]) -> UnicodeSet {
        let mut unicode_set = UnicodeSet::empty();
        for pattern in patterns {
            match pattern {
                UnicodePattern::Any => {
                    unicode_set = unicode_set.merge(&UnicodeSet::any());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::TAB) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::tab());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::VT) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::vt());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::FF) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::ff());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::SP) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::sp());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::USP) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::usp());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::LF) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::lf());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::CR) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::cr());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::LS) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::ls());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::PS) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::ps());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::ZWNJ) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::zwnj());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::ZWJ) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::zwj());
                }
                UnicodePattern::BuiltIn(UnicodeBuiltInPattern::ZWNBSP) => {
                    unicode_set = unicode_set.merge(&UnicodeSet::zwnbsp());
                }
                UnicodePattern::Char(ch) => {
                    unicode_set = unicode_set.merge(&unicode_set![*ch]);
                }
                UnicodePattern::Span(first, last) => {
                    unicode_set = unicode_set.merge(&unicode_set![*first..=*last]);
                }
                UnicodePattern::Exclude(value) => {
                    assert!(!value.is_empty());
                    if value.chars().count() == 1 {
                        let ch = value.chars().next().unwrap();
                        unicode_set = unicode_set.exclude_span(&unicode_span!(ch));
                    } else {
                        let us = self.build_unicode_set_for_non_terminal(value);
                        unicode_set = unicode_set.exclude(&us);
                    }
                }
                UnicodePattern::NonTerminal(name) => {
                    let us = self.build_unicode_set_for_non_terminal(name);
                    unicode_set = unicode_set.merge(&us);
                }
            };
        }
        unicode_set
    }

    fn build_unicode_set_for_non_terminal(&self, name: &str) -> UnicodeSet {
        let rules = self.rule_map.get(name).unwrap();
        self.build_unicode_set_for_rules(rules)
    }

    fn build_unicode_set_for_rules(&self, rules: &[&'b Rule]) -> UnicodeSet {
        let mut unicode_set = UnicodeSet::empty();
        for &rule in rules {
            let us = self.build_unicode_set_for_rule(rule);
            unicode_set = unicode_set.merge(&us);
        }
        unicode_set
    }

    fn build_unicode_set_for_rule(&self, rule: &Rule) -> UnicodeSet {
        assert_eq!(rule.production.len(), 1);
        self.build_unicode_set_for_term(&rule.production[0])
    }

    fn build_unicode_set_for_term(&self, term: &Term) -> UnicodeSet {
        match term {
            Term::Any => UnicodeSet::any(),
            Term::NonTerminal(ref name) => self.build_unicode_set_for_non_terminal(name),
            Term::UnicodeSet(ref patterns) => self.build_unicode_set(patterns),
            _ => unimplemented!(),
        }
    }

    fn find_recursion(&self, name: &str) -> Option<(StateId, StateId)> {
        self.recursion_stack
            .iter()
            .rev()
            .filter_map(|context| context.item.as_ref())
            .find(|item| item.0 == name)
            .map(|item| item.1.clone())
    }

    fn mark_recursion(&mut self) {
        self.recursion_stack.last_mut().unwrap().recursion = true;
    }
}

struct RecursionContext {
    item: Option<(String, (StateId, StateId))>,
    recursion: bool,
}
