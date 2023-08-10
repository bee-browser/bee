use std::collections::HashMap;

use serde::Deserialize;

use crate::automaton::Dfa;
use crate::automaton::Nfa;
use crate::automaton::StateId;
use crate::unicode::unicode_set;
use crate::unicode::unicode_span;
use crate::unicode::UnicodeSet;

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

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type", content = "data")]
pub enum Term {
    Empty,
    Any,
    UnicodeSet(Vec<UnicodePattern>),
    Word(String),
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
        nfa.set_label(start_id, "@start".to_string());

        NfaBuilder {
            rule_map,
            nfa,
            start_id,
            recursion_stack: vec![],
        }
    }

    pub fn add_token(&mut self, token: &str) {
        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("#{token}@start"));

        let accept_id = self.nfa.create_state();
        self.nfa.set_label(accept_id, format!("#{token}@accept"));
        self.nfa.accept(accept_id, token.to_string());

        let (inner_start_id, inner_end_id) = self.build_nfa_for_non_terminal(token);

        self.nfa.epsilon_transition(self.start_id, start_id);
        self.nfa.epsilon_transition(start_id, inner_start_id);
        self.nfa.epsilon_transition(inner_end_id, accept_id);
    }

    pub fn build(self) -> Nfa {
        self.nfa
    }

    fn build_nfa_for_non_terminal(&mut self, name: &str) -> (StateId, StateId) {
        if let Some(endpoints) = self.find_recursion(name) {
            self.mark_recursion();
            return endpoints;
        }

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{name}@start"));

        let end_id = self.nfa.create_state();
        self.nfa.set_label(end_id, format!("{name}@end"));

        let rules = self.rule_map.get(name).unwrap();
        self.recursion_stack.push(RecursionContext {
            label: name.to_string(),
            item: Some((name.to_string(), (start_id, end_id))),
            recursion: false,
        });
        let (inner_start_id, inner_end_id) = self.build_nfa_for_rules(rules);
        self.recursion_stack.pop();

        self.nfa.epsilon_transition(start_id, inner_start_id);
        self.nfa.epsilon_transition(inner_end_id, end_id);

        (start_id, end_id)
    }

    fn build_nfa_for_rules(&mut self, rules: &[&'b Rule]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{label}/rules@start"));

        let end_id = self.nfa.create_state();
        self.nfa.set_label(end_id, format!("{label}/rules@end"));

        for (i, rule) in rules.iter().enumerate() {
            let label = self.recursion_stack.last().unwrap().label.as_str();

            self.recursion_stack.push(RecursionContext {
                label: format!("{label}/rules@{i}"),
                item: None,
                recursion: false,
            });
            let (rule_start_id, rule_end_id) = self.build_nfa_for_rule(rule);
            self.recursion_stack.pop();

            self.nfa.epsilon_transition(start_id, rule_start_id);
            self.nfa.epsilon_transition(rule_end_id, end_id);
        }

        (start_id, end_id)
    }

    fn build_nfa_for_rule(&mut self, rule: &Rule) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{label}@start"));

        let end_id = self.nfa.create_state();
        self.nfa.set_label(end_id, format!("{label}@end"));

        let (prod_start_id, prod_end_id) = self.build_nfa_for_production(&rule.production);

        self.nfa.epsilon_transition(start_id, prod_start_id);
        self.nfa.epsilon_transition(prod_end_id, end_id);

        (start_id, end_id)
    }

    fn build_nfa_for_production(&mut self, production: &[Term]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let lookahead_index = production.iter().position(|term| match term {
            Term::Lookahead { .. } => true,
            _ => false,
        });
        let normal_seq_end = if let Some(i) = lookahead_index {
            i
        } else {
            production.len()
        };

        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/production@start"));

        let end_id = self.nfa.create_state();
        self.nfa
            .set_label(end_id, format!("{label}/production@end"));

        let mut id = start_id;
        for i in 0..normal_seq_end {
            let label = self.recursion_stack.last().unwrap().label.as_str();
            let term = &production[i];

            self.recursion_stack.push(RecursionContext {
                label: format!("{label}/terms@{i}"),
                item: None,
                recursion: false,
            });
            let (term_start_id, term_end_id) = self.build_nfa_for_term(term);
            let context = self.recursion_stack.pop().unwrap();

            // We build a DFA recognizing tokens defined in regular expressions.
            // A production like `A -> aAb` is not allowed.
            // A rule including such productions cannot be represented in a regular
            // expression and a stack is needed for recognizing it.
            if context.recursion {
                assert!(i == 0 || i == production.len() - 1);
            }

            self.nfa.epsilon_transition(id, term_start_id);

            id = term_end_id;
        }

        // Process lookahead items.
        if let Some(i) = lookahead_index {
            let label = self.recursion_stack.last().unwrap().label.as_str();

            self.recursion_stack.push(RecursionContext {
                label: format!("{label}/terms@{i}"),
                item: None,
                recursion: false,
            });
            let (lookahead_start_id, lookahead_end_id) =
                self.build_nfa_for_lookahead(&production[i..]);
            self.recursion_stack.pop();

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
            Term::Word(ref word) => self.build_nfa_for_word(word),
            Term::NonTerminal(ref name) => self.build_nfa_for_non_terminal(name),
            _ => unimplemented!(),
        }
    }

    fn build_nfa_for_empty(&mut self) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa.set_label(start_id, format!("{label}/empty@start"));

        let end_id = self.nfa.create_state();
        self.nfa.set_label(end_id, format!("{label}/empty@end"));

        self.nfa.epsilon_transition(start_id, end_id);

        (start_id, end_id)
    }

    fn build_nfa_for_unicode_set(&mut self, patterns: &[UnicodePattern]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/unicode_set@start"));

        let end_id = self.nfa.create_state();
        self.nfa
            .set_label(end_id, format!("{label}/unicode_set@end"));

        let unicode_set = self.build_unicode_set(patterns);
        self.nfa.transition(start_id, end_id, unicode_set);

        (start_id, end_id)
    }

    fn build_nfa_for_word(&mut self, word: &str) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        tracing::debug!(context.label = label, word);
        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/word({word})@start"));

        let end_id = self.nfa.create_state();
        self.nfa
            .set_label(end_id, format!("{label}/word({word})@end"));

        let mut id = start_id;
        for (i, ch) in word.char_indices() {
            let next_id = self.nfa.create_state();
            self.nfa
                .set_label(next_id, format!("{label}/word({word})@{i}"));
            self.nfa.transition(id, next_id, unicode_set![ch]);
            id = next_id;
        }

        self.nfa.epsilon_transition(id, end_id);

        (start_id, end_id)
    }

    fn build_nfa_for_lookahead(&mut self, rules: &[Term]) -> (StateId, StateId) {
        let label = self.recursion_stack.last().unwrap().label.as_str();

        let start_id = self.nfa.create_state();
        self.nfa
            .set_label(start_id, format!("{label}/lookahead@start"));

        let end_id = self.nfa.create_state();
        self.nfa.set_label(end_id, format!("{label}/lookahead@end"));
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
    label: String,
    item: Option<(String, (StateId, StateId))>,
    recursion: bool,
}
