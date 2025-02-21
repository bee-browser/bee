use std::collections::BTreeSet;

use dashmap::DashMap;

use crate::firstset::FirstSet;
use crate::grammar::Grammar;
use crate::grammar::Term;
use crate::lr::LrItem;
use crate::lr::LrItemSet;
use crate::phrase::MatchStatus;
use crate::phrase::Phrase;
use crate::phrase::PhraseSet;
use crate::phrase::macros::*;

pub type ClosureCache = DashMap<LrItem, LrItemSet>;

pub struct ClosureContext<'g, 'f> {
    grammar: &'g Grammar,
    first_set: &'f FirstSet,
    recursion: im::HashSet<LrItem>,
}

impl<'g, 'f> ClosureContext<'g, 'f> {
    pub fn new(grammar: &'g Grammar, first_set: &'f FirstSet) -> Self {
        ClosureContext {
            grammar,
            first_set,
            recursion: Default::default(),
        }
    }

    pub fn compute_closure(&self, items: &[LrItem], cache: &ClosureCache) -> LrItemSet {
        items
            .iter()
            .map(|item| self.compute_closure_of_item(item, cache))
            .fold(LrItemSet::default(), |set, closure| set.merge(&closure))
    }

    pub fn compute_closure_of_item(&self, item: &LrItem, cache: &ClosureCache) -> LrItemSet {
        if let Some(item_set) = cache.get(item) {
            return item_set.clone();
        }

        let mut item_set = LrItemSet::default();

        // Theoretically, lookahead restrictions in a production rule is processed in the closure
        // computation of an LR item because an LR automaton for a given grammar can recognizes
        // only the symbols in the grammar.  However, we've processed all non-tail lookahead
        // restrictions in the grammar and created a new grammar including *variant* production
        // rules before the closure computation in order to simplify the closure computation.
        //
        // The closure computation here is performed using variant production rules.  However,
        // the LR(0) automaton and LARL parsing tables will be built with the original symbols in
        // the grammar.
        //
        // `item_set` contains variant symbols.  So, we have to convert them to corresponding
        //  *original* symbols before building LR(0) states and LALR parsing tables.
        item_set.insert(item.clone());

        if self.recursion.contains(item) {
            return item_set;
        }

        let context = self.update(item.clone());

        if let Some((next_term, followers)) = item.follower_terms().split_first() {
            match next_term {
                Term::Empty | Term::Lookahead(_) | Term::Disallow(_) => {
                    item_set =
                        item_set.merge(&context.compute_closure_of_item(&item.shift(), cache));
                }
                Term::Token(_) => {
                    // Nothing to do.
                }
                Term::NonTerminal(non_terminal) => {
                    let lookahead_set = if item.k() > 0 {
                        context.compute_first_set_of_followers(item.k(), followers, &item.lookahead)
                    } else {
                        phrase_set![phrase!()]
                    };
                    for rule in context.grammar.non_terminal_rules(non_terminal) {
                        'next_lookahead: for lookahead in lookahead_set.iter() {
                            if let Some(Term::Lookahead(condition)) = rule.production.last() {
                                let mut condition = condition.clone();
                                for token in lookahead.iter() {
                                    match condition.process_token(token) {
                                        MatchStatus::Matched => break,
                                        MatchStatus::Unmatched => continue 'next_lookahead,
                                        MatchStatus::Remaining(next_condition) => {
                                            condition = next_condition
                                        }
                                    }
                                }
                            }
                            let non_kernel_item = LrItem {
                                rule: rule.clone(),
                                dot: 0,
                                lookahead: lookahead.shorten(item.k()),
                            };
                            item_set = item_set
                                .merge(&context.compute_closure_of_item(&non_kernel_item, cache));
                        }
                    }
                }
            }
        }

        cache.insert(item.clone(), item_set.clone());
        item_set
    }

    fn compute_first_set_of_followers(
        &self,
        k: usize,
        terms: &[Term],
        lookahead: &Phrase,
    ) -> PhraseSet {
        let (terms, cond) = match terms.split_last() {
            Some((Term::Lookahead(cond), terms)) => (terms, Some(cond)),
            _ => (terms, None),
        };

        let mut first_set = phrase_set![phrase!()];
        for term in terms {
            first_set = match term {
                Term::Empty | Term::Disallow(_) => first_set,
                Term::Token(token) => first_set.concat(&phrase_set![phrase!(token)]),
                Term::NonTerminal(non_terminal) => {
                    first_set.concat(self.first_set.table.get(non_terminal).unwrap())
                }
                _ => unreachable!(),
            };
            match first_set.min_tokens() {
                Some(n) if n >= k => break,
                _ => (),
            }
        }

        first_set = first_set.concat_phrase(lookahead);

        if let Some(cond) = cond {
            let mut set = BTreeSet::default();
            'next_phrase: for phrase in first_set.iter() {
                let mut cond = cond.clone();
                for token in phrase.iter() {
                    match cond.process_token(token) {
                        MatchStatus::Matched => {
                            set.insert(phrase.clone());
                            continue 'next_phrase;
                        }
                        MatchStatus::Unmatched => {
                            continue 'next_phrase;
                        }
                        MatchStatus::Remaining(next_cond) => {
                            cond = next_cond;
                        }
                    }
                }
                unimplemented!();
            }
            first_set = PhraseSet::new(set);
        }

        first_set
    }

    fn update(&self, item: LrItem) -> Self {
        ClosureContext {
            grammar: self.grammar,
            first_set: self.first_set,
            recursion: self.recursion.update(item),
        }
    }
}
