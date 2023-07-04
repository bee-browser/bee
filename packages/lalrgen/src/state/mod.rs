use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::closure::ClosureCache;
use crate::closure::ClosureContext;
use crate::firstset::FirstSet;
use crate::grammar::Grammar;
use crate::grammar::NonTerminal;
use crate::grammar::Symbol;
use crate::grammar::Term;
use crate::lr::LrItem;
use crate::lr::LrItemSet;
use crate::phrase::macros::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateId(usize);

impl StateId {
    pub fn index(&self) -> usize {
        self.0
    }
}

impl From<usize> for StateId {
    fn from(value: usize) -> Self {
        StateId(value)
    }
}

#[derive(Debug)]
pub struct State {
    pub id: StateId,
    pub item_set: LrItemSet,
    pub transitions: HashMap<Symbol, StateId>,
}

impl State {
    fn new(id: StateId, item_set: LrItemSet) -> Self {
        State {
            id,
            item_set,
            transitions: Default::default(),
        }
    }

    pub fn kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.item_set.iter().filter(|item| item.is_kernel())
    }
}

pub fn build_lr0_states(grammar: &Grammar, first_set: &FirstSet) -> Vec<State> {
    let mut builder = StateBuilder::default();

    assert_eq!(
        grammar
            .non_terminal_rules(&NonTerminal::GoalOfAugmentedGrammar)
            .len(),
        1
    );

    let item = LrItem {
        rule: grammar
            .non_terminal_rules(&NonTerminal::GoalOfAugmentedGrammar)
            .first()
            .unwrap()
            .clone(),
        dot: 0,
        lookahead: phrase!(),
    };

    let cache = ClosureCache::default();
    let context = ClosureContext::new(grammar, first_set);

    let item_set = context.compute_closure(&[item], &cache);
    let state_id = builder.create_state(item_set);

    let mut remaining = VecDeque::default();
    remaining.push_back(state_id);

    let mut processed: HashSet<StateId> = HashSet::default();

    while let Some(state_id) = remaining.pop_front() {
        if processed.contains(&state_id) {
            continue;
        }
        processed.insert(state_id);

        tracing::debug!(?state_id, remaining = remaining.len());

        // Use BTreeMap instead of HashMap in order to reproduce the same order of states in the
        // resulting vector.  If HashMap is used, the order (i.e. state.id) will change randomly
        // even if the grammar doesn't change.
        let mut next_kernel_table: BTreeMap<Symbol, Vec<LrItem>> = Default::default();
        for item in builder.state(state_id).item_set.iter() {
            let symbol = match item.next_term() {
                Some(Term::Empty) | Some(Term::Lookahead(_)) => continue,
                Some(Term::Token(token)) => Symbol::Token(token.clone()),
                Some(Term::NonTerminal(non_terminal)) => {
                    assert!(!non_terminal.is_variant());
                    Symbol::NonTerminal(non_terminal.symbol().to_owned())
                }
                None => continue,
            };
            next_kernel_table
                .entry(symbol)
                .or_default()
                .push(item.shift());
        }

        for (symbol, items) in next_kernel_table.into_iter() {
            let item_set = context.compute_closure(&items, &cache);
            let next_id = builder.create_state(item_set);
            tracing::trace!(transition = %symbol, from = ?state_id, to = ?next_id);
            builder
                .state_mut(state_id)
                .transitions
                .insert(symbol, next_id);
            if !processed.contains(&next_id) {
                remaining.push_back(next_id);
            }
        }
    }

    builder.build()
}

#[derive(Default)]
struct StateBuilder {
    states: Vec<State>,
    item_set_map: HashMap<LrItemSet, StateId>,
}

impl StateBuilder {
    fn state(&self, id: StateId) -> &State {
        &self.states[id.0]
    }

    fn state_mut(&mut self, id: StateId) -> &mut State {
        &mut self.states[id.0]
    }

    fn create_state(&mut self, item_set: LrItemSet) -> StateId {
        match self.item_set_map.get(&item_set) {
            Some(&state_id) => state_id,
            None => {
                let state_id = StateId(self.states.len());
                tracing::trace!(created = ?state_id, %item_set);
                self.states.push(State::new(state_id, item_set.clone()));
                self.item_set_map.insert(item_set, state_id);
                state_id
            }
        }
    }

    fn build(self) -> Vec<State> {
        self.states
    }
}
