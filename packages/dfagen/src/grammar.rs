use std::collections::HashMap;

use serde::Deserialize;

use crate::automaton::Dfa;
use crate::automaton::Nfa;
use crate::automaton::StateId;
use crate::unicode::unicode_set;
use crate::unicode::unicode_span;
use crate::unicode::UnicodeSet;

pub struct Grammar<'a, 'b> {
    pub rules: &'a HashMap<String, Rule>,
    pub tokens: &'b [String],
}

impl<'a, 'b> Grammar<'a, 'b> {
    pub fn new(rules: &'a HashMap<String, Rule>, tokens: &'b [String]) -> Self {
        Grammar { rules, tokens }
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
        let mut builder = NfaBuilder::new(self.rules);
        for token in self.tokens.iter() {
            tracing::debug!("Compiling {token} into NFA...");
            builder.add_token(token);
        }
        builder.build()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "data")]
pub enum Rule {
    Empty,
    Any,
    UnicodeSet(Vec<UnicodePattern>),
    Word(String),
    Sequence(Vec<Rule>),
    OneOf(Vec<Rule>),
    NonTerminal(String),
    Lookahead {
        patterns: Vec<UnicodePattern>,
        negate: bool,
    },
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

pub struct NfaBuilder<'a> {
    rules: &'a HashMap<String, Rule>,
    nfa: Nfa,
    start_id: StateId,
    recursion_stack: Vec<RecursionContext>,
}

impl<'a> NfaBuilder<'a> {
    pub fn new(rules: &'a HashMap<String, Rule>) -> Self {
        let mut nfa = Nfa::new();
        let start_id = nfa.create_state();
        nfa.set_label(start_id, "@start".to_string());

        NfaBuilder {
            rules,
            nfa,
            start_id,
            recursion_stack: vec![],
        }
    }

    pub fn add_token(&mut self, token: &str) {
        let rule = self.rules.get(token).unwrap();

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{token}@start"));

        let accept_id = self.nfa.create_state();
        self.nfa.set_label(accept_id, format!("{token}@accept"));
        self.nfa.accept(accept_id, token.to_string());

        // RecursionContexts constitutes a stack used for detecting a recursion in production
        // rules.
        self.recursion_stack.push(RecursionContext {
            label: token.to_string(),
            item: Some((token.to_string(), (start_id, accept_id))),
            recursion: false,
        });
        let (inner_start_id, inner_accept_id) = self.build_nfa(rule);
        self.recursion_stack.pop();

        self.nfa.epsilon_transition(start_id, inner_start_id);
        self.nfa.epsilon_transition(inner_accept_id, accept_id);
        self.nfa.epsilon_transition(self.start_id, start_id);
    }

    pub fn build(self) -> Nfa {
        self.nfa
    }

    fn build_nfa(&mut self, rule: &Rule) -> (StateId, StateId) {
        match rule {
            Rule::Empty => self.build_empty_nfa(),
            Rule::UnicodeSet(ref patterns) => self.build_unicode_set_nfa(patterns),
            Rule::Word(ref word) => self.build_word_nfa(word),
            Rule::NonTerminal(ref name) => self.build_non_terminal_nfa(name),
            Rule::Sequence(ref rules) => self.build_sequence_nfa(rules),
            Rule::OneOf(ref rules) => self.build_unified_nfa(rules),
            _ => unimplemented!(),
        }
    }

    fn build_empty_nfa(&mut self) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{label}/empty@start"));

        let accept_id = self.nfa.create_state();
        self.nfa
            .set_label(accept_id, format!("{label}/empty@accept"));

        self.nfa.epsilon_transition(start_id, accept_id);

        (start_id, accept_id)
    }

    fn build_unicode_set_nfa(&mut self, patterns: &[UnicodePattern]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/unicode_set@start"));

        let accept_id = self.nfa.create_state();
        self.nfa
            .set_label(accept_id, format!("{label}/unicode_set@accept"));

        let unicode_set = self.build_unicode_set(patterns);
        self.nfa.transition(start_id, accept_id, unicode_set);

        (start_id, accept_id)
    }

    fn build_word_nfa(&mut self, word: &str) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        tracing::debug!(context.label = label, word);
        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/word({word})@start"));

        let mut id = start_id;
        for (i, ch) in word.char_indices() {
            let next_id = self.nfa.create_state();
            self.nfa
                .set_label(next_id, format!("{label}/word({word})@{i}"));
            self.nfa.transition(id, next_id, unicode_set![ch]);
            id = next_id;
        }

        (start_id, id)
    }

    fn build_non_terminal_nfa(&mut self, name: &str) -> (StateId, StateId) {
        if let Some(endpoints) = self.find_recursion(name) {
            self.mark_recursion();
            return endpoints;
        }

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{name}@start"));

        let accept_id = self.nfa.create_state();
        self.nfa.set_label(accept_id, format!("{name}@accept"));

        let rule = self.rules.get(name).unwrap();
        self.recursion_stack.push(RecursionContext {
            label: name.to_string(),
            item: Some((name.to_string(), (start_id, accept_id))),
            recursion: false,
        });
        let (inner_start_id, inner_accept_id) = self.build_nfa(rule);
        self.recursion_stack.pop();

        self.nfa.epsilon_transition(start_id, inner_start_id);
        self.nfa.epsilon_transition(inner_accept_id, accept_id);

        (start_id, accept_id)
    }

    fn build_sequence_nfa(&mut self, rules: &[Rule]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let lookahead_index = rules.iter().position(|rule| match rule {
            Rule::Lookahead { .. } => true,
            _ => false,
        });
        let normal_seq_end = if let Some(i) = lookahead_index {
            i
        } else {
            rules.len()
        };

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{label}/seq@start"));

        let mut id = start_id;
        for i in 0..normal_seq_end {
            let label = self.recursion_stack.last().unwrap().label.as_str();
            let rule = &rules[i];

            self.recursion_stack.push(RecursionContext {
                label: format!("{label}/seq@{i}"),
                item: None,
                recursion: false,
            });
            let (inner_start_id, inner_accept_id) = self.build_nfa(rule);
            let context = self.recursion_stack.pop().unwrap();

            // We build a DFA recognizing tokens defined in regular expressions.
            // A production like `A -> aAb` is not allowed.
            // A rule including such productions cannot be represented in a regular
            // expression and a stack is needed for recognizing it.
            if context.recursion {
                assert!(i == 0 || i == rules.len() - 1);
            }

            self.nfa.epsilon_transition(id, inner_start_id);

            id = inner_accept_id;
        }

        // Process lookahead items.
        if let Some(i) = lookahead_index {
            let label = self.recursion_stack.last().unwrap().label.as_str();

            self.recursion_stack.push(RecursionContext {
                label: format!("{label}/seq@{i}"),
                item: None,
                recursion: false,
            });
            let (inner_start_id, inner_accept_id) = self.build_lookahead_nfa(&rules[i..]);
            self.recursion_stack.pop();

            self.nfa.epsilon_transition(id, inner_start_id);

            id = inner_accept_id;
        }

        (start_id, id)
    }

    fn build_unified_nfa(&mut self, rules: &[Rule]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/one-of@start"));

        let accept_id = self.nfa.create_state();
        self.nfa
            .set_label(accept_id, format!("{label}/one-of@accept"));

        for (i, rule) in rules.iter().enumerate() {
            let label = self.recursion_stack.last().unwrap().label.as_str();

            self.recursion_stack.push(RecursionContext {
                label: format!("{label}/one-of@{i}"),
                item: None,
                recursion: false,
            });
            let (inner_start_id, inner_accept_id) = self.build_nfa(rule);
            self.recursion_stack.pop();

            self.nfa.epsilon_transition(start_id, inner_start_id);
            self.nfa.epsilon_transition(inner_accept_id, accept_id);
        }

        (start_id, accept_id)
    }

    fn build_lookahead_nfa(&mut self, rules: &[Rule]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/lookahead@start"));

        let accept_id = self.nfa.create_state();
        self.nfa
            .set_label(accept_id, format!("{label}/lookahead@accept"));
        self.nfa.lookahead(accept_id);

        let mut unicode_set = UnicodeSet::any();
        for rule in rules {
            match rule {
                Rule::Lookahead { patterns, negate } => {
                    let mut us = self.build_unicode_set(patterns);
                    if *negate {
                        us = UnicodeSet::any().exclude(&us);
                    }
                    unicode_set = unicode_set.intersect(&us);
                }
                _ => unreachable!(),
            };
        }

        self.nfa.transition(start_id, accept_id, unicode_set);

        (start_id, accept_id)
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
                        let us = self.build_non_terminal_unicode_set(value);
                        unicode_set = unicode_set.exclude(&us);
                    }
                }
                UnicodePattern::NonTerminal(name) => {
                    let us = self.build_non_terminal_unicode_set(name);
                    unicode_set = unicode_set.merge(&us);
                }
            };
        }
        unicode_set
    }

    fn build_non_terminal_unicode_set(&self, name: &str) -> UnicodeSet {
        let rule = self.rules.get(name).unwrap();
        match rule {
            Rule::Any => UnicodeSet::any(),
            Rule::NonTerminal(ref name) => self.build_non_terminal_unicode_set(name),
            Rule::OneOf(ref rules) => self.build_unified_unicode_set(rules),
            Rule::UnicodeSet(ref patterns) => self.build_unicode_set(patterns),
            _ => unimplemented!(),
        }
    }

    fn build_unified_unicode_set(&self, rules: &[Rule]) -> UnicodeSet {
        let mut unicode_set = UnicodeSet::empty();
        for rule in rules {
            let us = match rule {
                Rule::NonTerminal(ref name) => self.build_non_terminal_unicode_set(name),
                Rule::UnicodeSet(ref patterns) => self.build_unicode_set(patterns),
                _ => unimplemented!(),
            };
            unicode_set = unicode_set.merge(&us);
        }
        unicode_set
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
    label: String,
    item: Option<(String, (StateId, StateId))>,
    recursion: bool,
}
