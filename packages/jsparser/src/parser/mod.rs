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
        // TODO: switch the lexical goal symbol according to the first set of the follower symbol.
        //
        // InputElementRegExpOrTemplateTail
        //   In syntactic grammar contexts where a RegularExpressionLiteral, a TemplateMiddle, or a
        //   TemplateTail is permitted.
        //
        // InputElementRegExp
        //   In syntactic grammar contexts where a RegularExpressionLiteral is permitted but
        //   neither a TemplateMiddle, nor a TemplateTail is permitted.
        //
        // InputElementTemplateTail
        //   In syntactic grammar contexts where a TemplateMiddle or a TemplateTail is permitted
        //   but a RegularExpressionLiteral is not permitted.
        //
        // InputElementDiv
        //   In all other contexts.
        //
        // For implementing this function, we have to put the first set table into lalr.json.  Or,
        // we have to compute the next lexical goal symbol for each shift and goto actions and
        // output these to lalr.json.
        let mut token = self.lexer.next_token();
        loop {
            // TODO: no-lime-terminator, auto-semicolon
            match token.kind {
                TokenKind::WhiteSpaceSequence
                | TokenKind::LineTerminatorSequence
                | TokenKind::Comment => {
                    token = self.lexer.next_token();
                    continue;
                }
                _ => {}
            }
            match state.action(&token) {
                Action::Accept => {
                    tracing::info!(action = "accept");
                    break;
                }
                Action::Shift(next) => {
                    tracing::info!(action = "shift", ?token);
                    self.push_state(next);
                    state = next;
                    token = self.lexer.next_token();
                }
                Action::Reduce(non_terminal, n, rule) => {
                    tracing::info!(action = "reduce", ?rule);
                    self.stack.truncate(self.stack.len() - n as usize);
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
        tracing::info!(state = state.debug_info());
        self.stack.push(state);
    }
}
