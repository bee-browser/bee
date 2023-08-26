mod lalr;

use crate::lexer::Goal;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenKind;

use lalr::Action;
use lalr::State;

const INITIAL_STACK_SIZE: usize = 512;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    stack: Vec<ParserState>,
    template_depth: usize,
    new_line: bool,
    // TODO: used only for measurement
    max_stack_depth: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            lexer: Lexer::new(src),
            stack: Vec::with_capacity(INITIAL_STACK_SIZE),
            template_depth: 0,
            new_line: false,
            max_stack_depth: 0,
        }
    }

    pub fn parse(&mut self) -> bool {
        self.push_state(Default::default());
        let mut token = self.next_token();
        loop {
            // TODO: no-line-terminator
            match token.kind {
                TokenKind::WhiteSpaceSequence | TokenKind::Comment => {
                    self.lexer.consume_token(token);
                    self.new_line = false;
                    token = self.next_token();
                    continue;
                }
                TokenKind::LineTerminatorSequence => {
                    self.lexer.consume_token(token);
                    self.new_line = true;
                    token = self.next_token();
                    continue;
                }
                _ => {}
            }

            match self.handle_token(&token) {
                ParserResult::Accept => break,
                ParserResult::Reconsume => (),
                ParserResult::NextToken => {
                    self.lexer.consume_token(token);
                    self.new_line = false;
                    token = self.next_token();
                }
                ParserResult::Error => {
                    if self.is_auto_semicolon_allowed(&token) {
                        loop {
                            match self.auto_semicolon() {
                                ParserResult::Accept => return true,
                                ParserResult::Reconsume => (),
                                ParserResult::NextToken => break,
                                ParserResult::Error => {
                                    self.report_error(&token);
                                    return false;
                                }
                            }
                        }
                    } else {
                        self.report_error(&token);
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn max_stack_depth(&self) -> usize {
        self.max_stack_depth
    }

    #[inline(always)]
    fn next_token(&mut self) -> Token<'a> {
        self.lexer.set_goal(self.lexical_goal());
        let token = self.lexer.next_token();
        tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);
        token
    }

    fn lexical_goal(&self) -> Goal {
        let state = self.stack.last().unwrap();
        let template_tail_disallowed = state.block_depth > 0 || self.template_depth == 0;
        match state.lalr_state.lexical_goal() {
            Goal::InputElementRegExpOrTemplateTail if template_tail_disallowed => {
                Goal::InputElementRegExp
            }
            Goal::InputElementTemplateTail if template_tail_disallowed => {
                Goal::InputElementDiv
            }
            goal @ _ => goal,
        }
    }

    fn push_state(&mut self, state: ParserState) {
        tracing::trace!(
            opcode = "push",
            stack.pos = self.stack.len(),
            state.id = state.lalr_state.id(),
            state.label = state.lalr_state.label(),
            state.block_depth,
            template_depth = self.template_depth,
        );
        self.stack.push(state);
        if self.max_stack_depth < self.stack.len() {
            self.max_stack_depth = self.stack.len();
        }
    }

    fn pop_states(&mut self, n: usize) {
        // n may be zero.
        debug_assert!(n <= self.stack.len());
        tracing::trace!(opcode = "pop", num_states = n);
        self.stack.truncate(self.stack.len() - n);
    }

    fn handle_token(&mut self, token: &Token<'_>) -> ParserResult {
        match self.stack.last().unwrap().lalr_state.action(token) {
            Action::Accept => {
                tracing::trace!(opcode = "accept", ?token.kind);
                ParserResult::Accept
            }
            Action::Shift(next) => {
                tracing::trace!(opcode = "shift", ?token.kind);
                let mut state = self.stack.last().unwrap().clone();
                state.lalr_state = next;
                match token.kind {
                    TokenKind::TemplateHead => {
                        self.template_depth += 1;
                        state.block_depth = 0;
                    }
                    TokenKind::TemplateTail => {
                        self.template_depth -= 1;
                        assert_eq!(state.block_depth, 0);
                    }
                    TokenKind::Lbrace => {
                        state.block_depth += 1;
                    }
                    TokenKind::Rbrace => {
                        state.block_depth -= 1;
                    }
                    _ => (),
                }
                self.push_state(state);
                ParserResult::NextToken
            }
            Action::Reduce(non_terminal, n, rule) => {
                tracing::trace!(opcode = "reduce", ?rule, ?token.kind);
                self.pop_states(n as usize);
                let mut state = self.stack.last().unwrap().clone();
                state.lalr_state = state.lalr_state.goto(non_terminal);
                self.push_state(state);
                ParserResult::Reconsume
            }
            Action::Error => ParserResult::Error,
        }
    }

    fn is_auto_semicolon_allowed(&self, token: &Token<'_>) -> bool {
        if self.new_line {
            return true;
        }
        if token.kind == TokenKind::Eof || token.kind == TokenKind::Rbrace {
            return true;
        }
        if self
            .stack
            .last()
            .unwrap()
            .lalr_state
            .is_auto_semicolon_do_while_statement()
        {
            return true;
        }
        // TODO: no-line-terminator
        false
    }

    fn auto_semicolon(&mut self) -> ParserResult {
        const SEMICOLON: Token<'static> = Token {
            kind: TokenKind::SemiColon,
            lexeme: ";",
        };
        match self.stack.last().unwrap().lalr_state.action(&SEMICOLON) {
            Action::Accept => {
                tracing::trace!(opcode = "accept", auto_semicolon = true);
                ParserResult::Accept
            }
            Action::Shift(next) => {
                tracing::trace!(opcode = "shift", auto_semicolon = true);
                if next.is_auto_semicolon_disallowed() {
                    ParserResult::Error
                } else {
                    let mut state = self.stack.last().unwrap().clone();
                    state.lalr_state = next;
                    self.push_state(state);
                    ParserResult::NextToken
                }
            }
            Action::Reduce(non_terminal, n, rule) => {
                tracing::trace!(opcode = "reduce", ?rule, auto_semicolon = true);
                self.pop_states(n as usize);
                let mut state = self.stack.last().unwrap().clone();
                state.lalr_state = state.lalr_state.goto(non_terminal);
                self.push_state(state);
                ParserResult::Reconsume
            }
            _ => ParserResult::Error,
        }
    }

    fn report_error(&self, token: &Token<'_>) {
        let pos = self.lexer.location();
        let src = self.lexer.src();
        let state = self.stack.last().unwrap();
        tracing::error!(
            pos,
            parsed = &src[pos.saturating_sub(10)..pos],
            remaianing = &src[pos..((pos + 10).min(src.len()))],
            ?token.kind,
            ?token.lexeme,
            state.id = state.lalr_state.id(),
            state.label = state.lalr_state.label(),
            state.block_depth,
            template_depth = self.template_depth
        );
    }
}

#[derive(Clone, Default)]
struct ParserState {
    lalr_state: State,
    block_depth: usize,
}

enum ParserResult {
    Accept,
    Reconsume,
    NextToken,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_parse_empty_script() {
        assert!(Parser::new("").parse());
    }

    #[test]
    fn test_parse_auto_semicolon1() {
        assert!(Parser::new("{ 1\n2 } 3").parse());
    }

    #[test]
    fn test_parse_auto_semicolon2() {
        assert!(Parser::new("function x() { return\na + b }").parse());
    }

    #[test]
    fn test_parse_auto_semicolon_variable_statement() {
        assert!(Parser::new("var x = 1").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement1() {
        assert!(!Parser::new("for () {}").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement2() {
        assert!(!Parser::new("for (true) {}").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement3() {
        assert!(!Parser::new("for (;) {}").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement4() {
        assert!(!Parser::new("for (true;) {}").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement5() {
        assert!(!Parser::new("for (;true) {}").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_do_while1() {
        assert!(Parser::new("do {} while (0)").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_do_while2() {
        assert!(Parser::new("do {} while (0) 0;").parse());
    }

    #[test]
    fn test_parser_auto_semicolon_template_literal() {
        assert!(Parser::new("`${x.x(x=>{return()})}`").parse());
    }

    #[test]
    fn test_parser_template_literal() {
        assert!(Parser::new("`${`${a}`}`").parse());
    }
}
