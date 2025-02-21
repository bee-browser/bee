use std::collections::HashMap;

use crate::grammar::Grammar;
use crate::grammar::NonTerminal;
use crate::grammar::Rule;
use crate::grammar::Term;
use crate::logger;
use crate::phrase::PhraseSet;
use crate::phrase::macros::*;

pub struct FirstSet {
    pub max_tokens: usize,
    pub table: HashMap<NonTerminal, PhraseSet>,
}

pub fn collect_first_set(grammar: &Grammar, max_tokens: usize) -> FirstSet {
    let mut table = Default::default();

    let mut iteration = 0;
    loop {
        let mut num_changed = 0;

        for rule in grammar.rules() {
            if let Some(set) = collect_first_set_of_rule(max_tokens, rule, &table) {
                match table.get_mut(&rule.name) {
                    Some(prev) => {
                        let set = set.merge(prev);
                        if set != *prev {
                            *prev = set;
                            num_changed += 1;
                        }
                    }
                    _ => {
                        table.insert(rule.name.clone(), set);
                        num_changed += 1;
                    }
                }
            }
        }

        logger::debug!(iteration, num_changed);
        if num_changed == 0 {
            break;
        }

        iteration += 1;
    }

    FirstSet { max_tokens, table }
}

fn collect_first_set_of_rule(
    max_tokens: usize,
    rule: &Rule,
    table: &HashMap<NonTerminal, PhraseSet>,
) -> Option<PhraseSet> {
    let mut set = phrase_set![phrase!()]; // epsilon

    for term in rule.production.iter() {
        set = match collect_first_set_of_term(term, table) {
            Some(follower_set) => set.concat(&follower_set),
            None => return None,
        };
        if let Some(min_tokens) = set.min_tokens() {
            if min_tokens >= max_tokens {
                return Some(set);
            }
        }
    }

    Some(set)
}

fn collect_first_set_of_term(
    term: &Term,
    table: &HashMap<NonTerminal, PhraseSet>,
) -> Option<PhraseSet> {
    match term {
        Term::Empty => Some(phrase_set![phrase!()]),
        Term::Token(token) => Some(phrase_set![phrase!(token)]),
        Term::NonTerminal(non_terminal) => table.get(non_terminal).cloned(),
        Term::Lookahead(_) => Some(phrase_set![phrase!()]),
        Term::Disallow(_) => unreachable!(),
    }
}
