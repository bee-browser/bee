mod lalr;

use crate::lexer::Lexer;
use crate::lexer::TokenKind;

use lalr::State;
use lalr::Action;

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
        self.stack.push(state);
        let mut token = self.lexer.next_token();
        loop {
            match token.kind {
                TokenKind::WhiteSpaceSequence | TokenKind::LineTerminatorSequence => {
                    token = self.lexer.next_token();
                    continue;
                }
                _ => {}
            }
            tracing::info!(?token);
            match state.action(&token) {
                Action::Accept => {
                    tracing::info!(action = "accept");
                    break;
                }
                Action::Shift(next) => {
                    self.stack.push(next);
                    state = next;
                    tracing::info!(action = "shift", ?state);
                    token = self.lexer.next_token();
                }
                Action::Reduce(non_terminal, n, rule) => {
                    self.stack.truncate(self.stack.len() - n as usize);
                    let next = self.stack.last().unwrap().goto(non_terminal);
                    self.stack.push(next);
                    state = next;
                    tracing::info!(action = "reduce", rule, ?state);
                }
                Action::Error => {
                    tracing::error!(?state, ?token);
                    break;
                }
            }
        }
    }
}
