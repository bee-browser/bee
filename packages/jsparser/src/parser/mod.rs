mod lalr;

use crate::lexer::Lexer;
use crate::lexer::TokenKind;

use lalr::Action;
use lalr::State;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    stack: Vec<State>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            lexer: Lexer::new(src),
            stack: Vec::with_capacity(2048),
        }
    }

    pub fn parse(&mut self) {
        let mut state = State::default();
        self.push_state(state);
        let mut token = self.lexer.next_token();
        tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);
        loop {
            // TODO: no-lime-terminator, auto-semicolon
            match token.kind {
                TokenKind::WhiteSpaceSequence
                | TokenKind::LineTerminatorSequence
                | TokenKind::Comment => {
                    token = self.lexer.next_token();
                    tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);
                    continue;
                }
                _ => {}
            }
            match state.action(&token) {
                Action::Accept => {
                    tracing::trace!(opcode = "accept");
                    break;
                }
                Action::Shift(next) => {
                    tracing::trace!(opcode = "shift");
                    self.push_state(next);
                    state = next;
                    token = self.lexer.next_token();
                    tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);
                }
                Action::Reduce(non_terminal, n, rule) => {
                    tracing::trace!(opcode = "reduce", ?rule);
                    self.pop_states(n as usize);
                    let next = self.stack.last().unwrap().goto(non_terminal);
                    self.push_state(next);
                    state = next;
                }
                Action::Error => {
                    tracing::error!(?token);
                    break;
                }
            }
        }
    }

    fn push_state(&mut self, state: State) {
        tracing::trace!(opcode = "push", state = state.debug_info());
        self.stack.push(state);
        self.lexer.set_goal(state.lexical_goal());
    }

    fn pop_states(&mut self, n: usize) {
        // n may be zero.
        debug_assert!(n <= self.stack.len());
        tracing::trace!(opcode = "pop", num_states = n);
        self.stack.truncate(self.stack.len() - n);
    }
}
