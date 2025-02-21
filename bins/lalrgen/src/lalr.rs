use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::closure::ClosureCache;
use crate::closure::ClosureContext;
use crate::firstset::FirstSet;
use crate::grammar::Grammar;
use crate::grammar::Symbol;
use crate::grammar::Term;
use crate::logger;
use crate::lr::LrItem;
use crate::phrase::MatchStatus;
use crate::phrase::PhraseSet;
use crate::phrase::macros::*;
use crate::state::Automaton;
use crate::state::StateId;

pub type LookaheadTable = HashMap<LrItem, PhraseSet>;

pub fn build_lookahead_tables(
    grammar: &Grammar,
    first_set: &FirstSet,
    automaton: &Automaton,
) -> Vec<LookaheadTable> {
    let mut lookahead_tables = Vec::with_capacity(automaton.size());
    for _ in 0..automaton.size() {
        lookahead_tables.push(LookaheadTable::default());
    }

    for (_, start_state) in automaton.start_states() {
        for item in start_state.kernel_items() {
            lookahead_tables[start_state.id.index()]
                .insert(item.clone(), phrase_set![phrase!("$")]);
        }
    }

    let closure_cache = ClosureCache::default();

    let mut iteration = 0;
    loop {
        logger::debug!(iteration, phase = "collect");
        let mut operations = automaton
            .states
            .par_iter()
            .map(|state| {
                state
                    .internal_kernel_items()
                    .map(move |item| (state, item))
                    .collect_vec()
            })
            .flatten()
            .filter_map(|(state, item)| {
                if item.is_reducible() {
                    return None;
                }
                let temp_kernel_item = item.with_lookahead(phrase!("#"));
                let closure_context = ClosureContext::new(grammar, first_set);
                let temp_item_set =
                    closure_context.compute_closure_of_item(&temp_kernel_item, &closure_cache);
                Some(
                    temp_item_set
                        .iter()
                        .map(|temp_item| (state, item, temp_item.clone()))
                        .collect_vec(),
                )
            })
            .flatten()
            .filter_map(|(state, item, temp_item)| {
                let (target_state, target_item) = if temp_item.is_reducible() {
                    let kernel_item = temp_item.without_lookahead().to_original(grammar);
                    assert!(state.item_set.contains(&kernel_item));
                    (state.id, kernel_item)
                } else {
                    let kernel_item = temp_item.without_lookahead().shift().to_original(grammar);
                    let next_symbol = match temp_item.next_term() {
                        Some(Term::Token(token)) => Some(Symbol::Token(token.clone())),
                        Some(Term::NonTerminal(non_terminal)) => {
                            // `non_terminal` may be a variant symbol.
                            Some(Symbol::NonTerminal(non_terminal.symbol().to_owned()))
                        }
                        _ => None,
                    };
                    match next_symbol {
                        Some(next_symbol) => {
                            let next_id = state.transitions.get(&next_symbol).cloned().unwrap();
                            let next_state = automaton.state(next_id);
                            assert!(next_state.item_set.contains(&kernel_item));
                            (next_state.id, kernel_item)
                        }
                        None => (state.id, kernel_item),
                    }
                };

                if temp_item.lookahead.iter().all(|token| token == "#") {
                    lookahead_tables[state.id.index()]
                        .get(&item.to_original(grammar))
                        .cloned()
                        .map(|lookahead_set| {
                            (
                                target_state.index(),
                                Operation::Propagate(OperationData {
                                    source_state: state.id,
                                    source_item: item.to_original(grammar),
                                    target_state,
                                    target_item,
                                    lookahead_set,
                                }),
                            )
                        })
                } else {
                    Some((
                        target_state.index(),
                        Operation::Generate(OperationData {
                            source_state: state.id,
                            source_item: item.clone(),
                            target_state,
                            target_item,
                            lookahead_set: phrase_set![temp_item.lookahead.clone()],
                        }),
                    ))
                }
            })
            .collect::<Vec<_>>();

        // Process transitions for restricted tokens.
        //
        // The item set of a next state for each restricted token must not contain restricted
        // items.  So, we have to re-compute the closure for the next state.
        for state in automaton
            .states
            .iter()
            .filter(|state| state.is_restricted())
        {
            let closure_context = ClosureContext::new(grammar, first_set);
            let disallowed_tokens = state.collect_disallowed_tokens();
            for token in disallowed_tokens.into_iter() {
                // Remove restricted items from the item set of the state.
                let kernel_items = state
                    .internal_kernel_items()
                    .filter(|item| !item.is_disallowed(&token))
                    .cloned()
                    .collect_vec();
                if kernel_items.is_empty() {
                    continue;
                }
                // Then, re-compute the closure.
                let item_set = closure_context.compute_closure(&kernel_items, &closure_cache);
                let symbol = Symbol::Token(token);
                let next_id = state.transitions.get(&symbol).cloned().unwrap();
                let next_state = automaton.state(next_id);
                // Iterate over *original* items.  Because the lookahead table is built for the
                // LR(0) automaton.  Variant symbols in items should be converted to corresponding
                // symbols in the original grammar before updating the lookahead table with the
                // items.
                for item in item_set.to_original(grammar).iter() {
                    assert!(state.item_set.contains(item));
                    assert!(next_state.item_set.contains(item));
                    if let Some(lookahead_set) =
                        lookahead_tables[state.id.index()].get(item).cloned()
                    {
                        operations.push((
                            next_id.index(),
                            Operation::Propagate(OperationData {
                                source_state: state.id,
                                source_item: item.clone(),
                                target_state: next_id,
                                target_item: item.clone(),
                                lookahead_set,
                            }),
                        ));
                    }
                }
            }
        }

        logger::debug!(iteration, phase = "collect", operations = operations.len());

        // Collect operations for each state.
        let operations = operations.into_iter().fold(
            Default::default(),
            |mut map: HashMap<_, Vec<_>>, (i, op)| {
                map.entry(i).or_default().push(op);
                map
            },
        );

        // Then edit the lookahead table for each state in parallel.
        logger::debug!(iteration, phase = "edit");
        let changes = lookahead_tables
            .par_iter_mut()
            .enumerate()
            .map(|(i, lookahead_table)| {
                let ops = match operations.get(&i) {
                    Some(ops) => ops,
                    None => return 0,
                };
                let mut changes = 0;
                for op in ops.iter() {
                    let data = match op {
                        Operation::Propagate(data) => {
                            logger::trace!(
                                iteration,
                                propagate = %data.lookahead_set,
                                source.state = data.source_state.index(),
                                source.item = %data.source_item,
                                target.state = data.target_state.index(),
                                target.item = %data.target_item,
                            );
                            data
                        }
                        Operation::Generate(data) => {
                            logger::trace!(
                                iteration,
                                generate = %data.lookahead_set,
                                source.state = data.source_state.index(),
                                source.item = %data.source_item,
                                target.state = data.target_state.index(),
                                target.item = %data.target_item,
                            );
                            data
                        }
                    };

                    let lookahead_set = match data.target_item.rule.production.last() {
                        Some(Term::Lookahead(cond)) => {
                            // Apply the tail lookahead restriction.
                            let mut cond = cond.clone();
                            let mut set = BTreeSet::default();
                            'next_lookahead: for lookahead in data.lookahead_set.iter() {
                                for token in lookahead.iter() {
                                    match cond.process_token(token) {
                                        MatchStatus::Matched => {
                                            set.insert(lookahead.clone());
                                            continue 'next_lookahead;
                                        }
                                        MatchStatus::Unmatched => {
                                            continue 'next_lookahead;
                                        }
                                        MatchStatus::Remaining(next_cond) => {
                                            cond = next_cond;
                                        }
                                    }
                                }
                                unimplemented!();
                            }
                            PhraseSet::new(set)
                        }
                        _ => data.lookahead_set.clone(),
                    };

                    let lookahead_set = match lookahead_table.get(&data.target_item) {
                        Some(old) => {
                            let new = old.merge(&lookahead_set);
                            if new.ne(old) {
                                changes += 1;
                            }
                            new
                        }
                        None => {
                            changes += 1;
                            lookahead_set.clone()
                        }
                    };
                    lookahead_table.insert(data.target_item.clone(), lookahead_set);
                }
                changes
            })
            .reduce(|| 0, |a, b| a + b);
        logger::debug!(iteration, phase = "edit", changes);
        if changes == 0 {
            break;
        }
        iteration += 1;
    }

    lookahead_tables
}

enum Operation {
    Propagate(OperationData),
    Generate(OperationData),
}

struct OperationData {
    source_state: StateId,
    source_item: LrItem,
    target_state: StateId,
    target_item: LrItem,
    lookahead_set: PhraseSet,
}

pub fn build_lalr_states(
    automaton: &Automaton,
    lookahead_tables: &[LookaheadTable],
) -> (Vec<LalrState>, Vec<LalrProblem>) {
    let mut lalr_states: Vec<LalrState> = Vec::with_capacity(automaton.size());
    let mut problems: Vec<LalrProblem> = vec![];

    for (i, state) in automaton.states.iter().enumerate() {
        let disallowed_tokens = state.collect_disallowed_tokens();
        // Use BTreeMap instead of HashMap in order to keep the order of keys in serialized data.
        let mut actions: BTreeMap<String, LalrAction> = Default::default();
        let mut gotos: BTreeMap<String, usize> = Default::default();
        for (symbol, &next_id) in state.transitions.iter() {
            match symbol {
                Symbol::Token(token) => {
                    assert!(!actions.contains_key(token));
                    let action = if disallowed_tokens.contains(token) {
                        LalrAction::Replace(LalrReplace {
                            next_id: next_id.index(),
                        })
                    } else {
                        LalrAction::Shift(LalrShift {
                            next_id: next_id.index(),
                        })
                    };
                    logger::trace!(state.id = i, token, %action);
                    actions.insert(token.clone(), action);
                }
                Symbol::NonTerminal(non_terminal) => {
                    assert!(!gotos.contains_key(non_terminal));
                    logger::trace!(state.id = i, %non_terminal, goto = next_id.index());
                    gotos.insert(non_terminal.clone(), next_id.index());
                }
            }
        }
        for item in state.item_set.iter().filter(|item| item.is_reducible()) {
            let lookahead_set = match lookahead_tables[i].get(item) {
                Some(lookahead_set) => lookahead_set,
                None => {
                    problems.push(LalrProblem::NoLookahead {
                        state: format!("{}", state.id),
                        kernel_items: state
                            .internal_kernel_items()
                            .map(|item| format!("{item}"))
                            .collect_vec(),
                        non_kernel_items: state
                            .internal_non_kernel_items()
                            .map(|item| format!("{item}"))
                            .collect_vec(),
                        item: format!("{item}"),
                    });
                    continue;
                }
            };
            for lookahead in lookahead_set.iter() {
                assert_eq!(lookahead.len(), 1);
                let token = &lookahead[0];
                match actions.get(token) {
                    Some(LalrAction::Shift(_)) => {
                        problems.push(LalrProblem::ShiftReduceConflict {
                            state: format!("{}", state.id),
                            kernel_items: state
                                .internal_kernel_items()
                                .map(|item| format!("{item}"))
                                .collect_vec(),
                            non_kernel_items: state
                                .internal_non_kernel_items()
                                .map(|item| format!("{item}"))
                                .collect_vec(),
                            token: token.clone(),
                            item: format!("{item}"),
                        });
                    }
                    Some(LalrAction::Reduce(reduce)) => {
                        problems.push(LalrProblem::ReduceReduceConflict {
                            state: format!("{}", state.id),
                            kernel_items: state
                                .internal_kernel_items()
                                .map(|item| format!("{item}"))
                                .collect_vec(),
                            non_kernel_items: state
                                .internal_non_kernel_items()
                                .map(|item| format!("{item}"))
                                .collect_vec(),
                            token: token.clone(),
                            item: format!("{item}"),
                            rule: reduce.rule.clone(),
                        });
                    }
                    _ => {}
                }
                let action = if token == "$" && item.rule.is_goal_of_augmented_grammar() {
                    LalrAction::Accept
                } else {
                    LalrAction::Reduce(LalrReduce {
                        non_terminal: item.rule.name.symbol().to_owned(),
                        num_pops: item.rule.count_symbols(),
                        rule: format!("{}", item.rule),
                    })
                };
                logger::trace!(state.id = i, %token, %action);
                actions.insert(token.clone(), action);
            }
        }

        lalr_states.push(LalrState {
            actions: actions.into_iter().collect_vec(),
            gotos: gotos.into_iter().collect_vec(),
            kernel_items: state
                .kernel_items()
                .map(|item| format!("{item}"))
                .collect_vec(),
            closure: state
                .item_set
                .iter()
                .map(|item| format!("{item}"))
                .collect_vec(),
        });
    }

    (lalr_states, problems)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LalrSpec {
    pub goal_symbols: Vec<String>,
    pub non_terminals: Vec<String>,
    pub production_rules: Vec<String>,
    pub starts: Vec<(String, usize)>,
    pub states: Vec<LalrState>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LalrState {
    pub actions: Vec<(String, LalrAction)>,
    pub gotos: Vec<(String, usize)>,
    pub kernel_items: Vec<String>,
    pub closure: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum LalrAction {
    Accept,
    Shift(LalrShift),
    Reduce(LalrReduce),
    Replace(LalrReplace),
}

impl std::fmt::Display for LalrAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Accept => write!(f, "accept"),
            Self::Shift(shift) => write!(f, "{shift}"),
            Self::Reduce(reduce) => write!(f, "{reduce}"),
            Self::Replace(replace) => write!(f, "{replace}"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LalrShift {
    pub next_id: usize,
}

impl std::fmt::Display for LalrShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "shift({})", self.next_id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LalrReduce {
    pub non_terminal: String,
    pub num_pops: usize,
    pub rule: String, // for debugging purposes
}

impl std::fmt::Display for LalrReduce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reduce({})", self.rule)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LalrReplace {
    pub next_id: usize,
}

impl std::fmt::Display for LalrReplace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "replace({})", self.next_id)
    }
}

#[derive(Debug, Serialize)]
pub enum LalrProblem {
    NoLookahead {
        state: String,
        kernel_items: Vec<String>,
        non_kernel_items: Vec<String>,
        item: String,
    },
    ShiftReduceConflict {
        state: String,
        kernel_items: Vec<String>,
        non_kernel_items: Vec<String>,
        token: String,
        item: String,
    },
    ReduceReduceConflict {
        state: String,
        kernel_items: Vec<String>,
        non_kernel_items: Vec<String>,
        token: String,
        item: String,
        rule: String,
    },
}
