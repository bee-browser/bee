use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

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

/// Represents the identifier of a state.
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

impl std::fmt::Display for StateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State({})", self.0)
    }
}

/// Represents a state of an LR(0) automaton.
#[derive(Debug)]
pub struct State {
    /// The identifier of the state.
    ///
    /// The value of the identifier is used as the index of the state in the state list returned
    /// from `build_lr0_states()`.
    pub id: StateId,

    /// An LR item set used for distinguishing the state from others.
    ///
    /// Every LR item in this item set doesn't contain any variant symbols.  Therefore, this item
    /// set must not be used for the closure computation.
    pub item_set: LrItemSet,

    /// An LR item set used for the closure computation.
    ///
    /// Some of LR items in this item set may contain variant symbols.  Therefore, this item set
    /// must not be used for identifying the state.
    pub internal_item_set: LrItemSet,

    /// A transition table of the state.  Every symbol used as a key of the map is not a variant
    /// symbol.
    pub transitions: HashMap<Symbol, StateId>,
}

impl State {
    fn new(id: StateId, internal_item_set: LrItemSet) -> Self {
        let item_set = internal_item_set.to_original();
        State {
            id,
            item_set,
            internal_item_set,
            transitions: Default::default(),
        }
    }

    /// Returns an iterator over *original* kernel items in `item_set`.
    pub fn kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.item_set.kernel_items()
    }

    /// Returns an iterator over *original* non-kernel items in `item_set`.
    pub fn non_kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.item_set.non_kernel_items()
    }

    /// Returns an iterator over kernel items in `internal_item_set`.
    pub fn internal_kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.internal_item_set.kernel_items()
    }

    /// Returns an iterator over non-kernel items in `internal_item_set`.
    pub fn internal_non_kernel_items(&self) -> impl Iterator<Item = &LrItem> {
        self.internal_item_set.non_kernel_items()
    }

    pub fn is_restricted(&self) -> bool {
        self.kernel_items().any(LrItem::is_restricted)
    }

    pub fn collect_disallowed_tokens(&self) -> HashSet<String> {
        let mut tokens: HashSet<String> = Default::default();
        for item in self.kernel_items().filter(|item| item.is_restricted()) {
            match item.next_term().unwrap() {
                Term::Disallow(token) => {
                    tokens.insert(token.clone());
                }
                _ => unreachable!(),
            }
        }
        tokens
    }
}

/// Build the LR(0) automaton for a given grammar.
pub fn build_lr0_automaton(grammar: &Grammar, first_set: &FirstSet) -> Vec<State> {
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

        // Iterate over items in `internal_item_set` for the closure computations.
        for item in builder.state(state_id).internal_item_set.iter() {
            let term = match item.next_term() {
                Some(term) => term,
                None => continue,
            };
            let symbol = match term {
                Term::Empty | Term::Lookahead(_) | Term::Disallow(_) => continue,
                Term::Token(token) => Symbol::Token(token.clone()),
                Term::NonTerminal(non_terminal) => {
                    // `non_terminal` may a variant symbol.
                    Symbol::NonTerminal(non_terminal.symbol().to_owned())
                }
            };
            next_kernel_table
                .entry(symbol)
                .or_default()
                .push(item.shift());
        }

        // Add transitions for normal tokens.
        for (symbol, items) in next_kernel_table.into_iter() {
            let item_set = context.compute_closure(&items, &cache);
            let next_id = builder.create_state(item_set);
            tracing::trace!(transition = %symbol, from = %state_id, to = %next_id);
            builder
                .state_mut(state_id)
                .transitions
                .insert(symbol, next_id);
            if !processed.contains(&next_id) {
                remaining.push_back(next_id);
            }
        }

        // Add special transitions for restricted tokens.
        //
        // The item set of a next state for each restricted token must not contain restricted
        // items.  So, we have to re-compute the closure for the next state.
        let disallowed_tokens = builder.state(state_id).collect_disallowed_tokens();
        for token in disallowed_tokens.into_iter() {
            // Remove restricted items from the item set of the state.
            let kernel_items = builder
                .state(state_id)
                .internal_kernel_items()
                .filter(|item| !item.is_disallowed(&token))
                .cloned()
                .collect_vec();
            if kernel_items.is_empty() {
                continue;
            }
            // Then, re-compute its closure.
            let item_set = context.compute_closure(&kernel_items, &cache);
            let symbol = Symbol::Token(token);
            let next_id = builder.create_state(item_set);
            tracing::trace!(transition = %symbol, from = %state_id, to = %next_id);
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
    /// A list of states.
    states: Vec<State>,

    /// A map to identify a state with the *original* item set of an item set.
    ///
    /// Each state should be identified by its *original* item set.
    ///
    /// A non-terminal symbol may appear in multiple production rules.  This may cause multiple
    /// variants of the same non-terminal symbols.  However, the variants should be processed as
    /// the same non-terminal symbol in the original grammar in order to avoid conflicts when
    /// generating the LALR parsing tables.
    ///
    /// Similarly, the LR(0) automaton should be built for the original grammar.  Therefore, we
    /// identify item sets with their *original* item sets in the LR(0) automaton.
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
        let original = item_set.to_original();
        match self.item_set_map.get(&original) {
            Some(&state_id) => state_id,
            None => {
                let state_id = StateId(self.states.len());
                tracing::trace!(created = %state_id, %item_set);
                self.states.push(State::new(state_id, item_set));
                self.item_set_map.insert(original, state_id);
                state_id
            }
        }
    }

    fn build(self) -> Vec<State> {
        self.states
    }
}
