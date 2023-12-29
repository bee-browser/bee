use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

use crate::closure::ClosureCache;
use crate::closure::ClosureContext;
use crate::firstset::FirstSet;
use crate::grammar::Grammar;
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
    fn new(id: StateId, item_set: LrItemSet, internal_item_set: LrItemSet) -> Self {
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
pub fn build_lr0_automaton(grammar: &Grammar, first_set: &FirstSet) -> Automaton {
    let mut builder = StateBuilder::new(grammar);

    // The grammar may have multiple goal symbols.
    assert!(grammar.augmented_rules().count() > 0);

    let cache = ClosureCache::default();
    let context = ClosureContext::new(grammar, first_set);

    let mut remaining = VecDeque::default();

    for (symbol, rule) in grammar.augmented_rules() {
        let item = LrItem {
            rule: rule.clone(),
            dot: 0,
            lookahead: phrase!(),
        };

        let item_set = context.compute_closure(&[item], &cache);
        let state_id = builder.create_state(item_set);
        builder.add_start(symbol, state_id);

        remaining.push_back(state_id);
    }

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
            let symbol = Symbol::Token(token);
            let (item_set, valid_state) = if kernel_items.is_empty() {
                // All items in the state are restricted.  In this case, the state must move to a
                // *dead* state when the disallowed token is received.  The default action of some
                // tokens such as line terminators is simple ignoring the token.  So, it's
                // necessary to add the transition to the dead state in order to avoid the default
                // action.
                (Default::default(), false)
            } else {
                // Then, re-compute its closure.
                (context.compute_closure(&kernel_items, &cache), true)
            };
            // A dead state will be created if the item set has no item.
            let next_id = builder.create_state(item_set);
            tracing::trace!(transition = %symbol, from = %state_id, to = %next_id);
            builder
                .state_mut(state_id)
                .transitions
                .insert(symbol, next_id);
            if valid_state && !processed.contains(&next_id) {
                remaining.push_back(next_id);
            }
        }
    }

    builder.build()
}

struct StateBuilder<'g> {
    grammar: &'g Grammar,

    /// A list of start state IDs.
    starts: Vec<(String, StateId)>,

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

impl<'g> StateBuilder<'g> {
    fn new(grammar: &'g Grammar) -> Self {
        StateBuilder {
            grammar,
            starts: Default::default(),
            states: Default::default(),
            item_set_map: Default::default(),
        }
    }

    fn state(&self, id: StateId) -> &State {
        &self.states[id.0]
    }

    fn state_mut(&mut self, id: StateId) -> &mut State {
        &mut self.states[id.0]
    }

    fn create_state(&mut self, internal_item_set: LrItemSet) -> StateId {
        let item_set = internal_item_set.to_original(self.grammar);
        match self.item_set_map.get(&item_set) {
            Some(&state_id) => state_id,
            None => {
                let state_id = StateId(self.states.len());
                self.item_set_map.insert(item_set.clone(), state_id);
                tracing::trace!(created = %state_id, %internal_item_set);
                self.states
                    .push(State::new(state_id, item_set, internal_item_set));
                state_id
            }
        }
    }

    fn add_start(&mut self, symbol: &str, id: StateId) {
        self.starts.push((symbol.to_string(), id));
    }

    fn build(self) -> Automaton {
        Automaton {
            starts: self.starts,
            states: self.states,
        }
    }
}

/// Represents an LR(0) automaton built from a grammar.
pub struct Automaton {
    /// A list of start state IDs.
    pub starts: Vec<(String, StateId)>,

    /// A list of states.
    pub states: Vec<State>,
}

impl Automaton {
    /// Returns the size of the automaton.
    pub fn size(&self) -> usize {
        self.states.len()
    }

    /// Returns a state identified by a specified ID.
    pub fn state(&self, id: StateId) -> &State {
        self.states.get(id.index()).unwrap()
    }

    /// Returns an iterator over start states.
    pub fn start_states(&self) -> impl Iterator<Item = (&str, &State)> {
        self.starts
            .iter()
            .map(|(symbol, id)| (symbol.as_str(), self.state(*id)))
    }
}
