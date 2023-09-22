#![doc = include_str!("../README.md")]

mod closure;
mod firstset;
mod grammar;
mod lalr;
mod lr;
mod phrase;
mod preprocess;
mod state;

pub use firstset::collect_first_set;
pub use firstset::FirstSet;
pub use grammar::Grammar;
pub use grammar::Rule;
pub use grammar::Term;
pub use lalr::build_lalr_states;
pub use lalr::build_lookahead_tables;
pub use lalr::LalrAction;
pub use lalr::LalrProblem;
pub use lalr::LalrSpec;
pub use lalr::LalrState;
pub use lalr::LookaheadTable;
pub use preprocess::preprocess;
pub use state::build_lr0_automaton;
pub use state::State;
pub use state::StateId;
