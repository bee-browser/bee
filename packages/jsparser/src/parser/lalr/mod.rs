mod action;
mod debug;
mod goto;
mod lexical_goal;
mod non_terminals;

use crate::lexer::Goal;
use crate::lexer::Token;

pub use non_terminals::NonTerminal;

#[derive(Clone, Copy, Debug, Default)]
pub struct State(u16);

impl State {
    #[inline(always)]
    pub fn action<'a>(&self, token: &Token<'a>) -> Action {
        let token = token.kind as u8;
        action::TABLE[self.0 as usize].get(&token).cloned().unwrap_or(Action::Error)
    }

    #[inline(always)]
    pub fn goto(&self, non_terminal: NonTerminal) -> State {
        let non_terminal = non_terminal as u16;
        goto::TABLE[self.0 as usize].get(&non_terminal).unwrap().clone()
    }

    #[inline(always)]
    pub fn lexical_goal(&self) -> Goal {
        lexical_goal::TABLE[self.0 as usize]
    }

    pub fn debug_info(&self) -> &'static str {
        debug::LABELS[self.0 as usize]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Accept,
    Shift(State),
    Reduce(NonTerminal, u8, &'static str),
    Error,
}
