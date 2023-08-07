mod lalr;

use crate::lexer::Goal;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenKind;

use lalr::Action;
use lalr::State;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    stack: Vec<State>,
    template_depth: usize,
    new_line: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            lexer: Lexer::new(src),
            stack: Vec::with_capacity(2048),
            template_depth: 0,
            new_line: false,
        }
    }

    pub fn parse(&mut self) -> bool {
        self.push_state(State::default());
        loop {
            // TODO: Currently, we simply tokenize at the beginning of every loop.
            // This is inefficient.
            // Cache the token and invalidate when it's consumed or the lexical goal changes.
            let token = self.lexer.next_token();
            tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);

            // TODO: no-line-terminator
            match token.kind {
                TokenKind::WhiteSpaceSequence | TokenKind::Comment => {
                    self.lexer.consume_token(token);
                    self.new_line = false;
                    continue;
                }
                TokenKind::LineTerminatorSequence => {
                    self.lexer.consume_token(token);
                    self.new_line = true;
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

    fn push_state(&mut self, state: State) {
        tracing::trace!(opcode = "push", state = state.debug_info());
        self.stack.push(state);
        self.lexer.set_goal(match state.lexical_goal() {
            Goal::InputElementRegExpOrTemplateTail if self.template_depth == 0 => {
                Goal::InputElementRegExp
            }
            Goal::InputElementTemplateTail if self.template_depth == 0 => Goal::InputElementDiv,
            goal @ _ => goal,
        });
    }

    fn pop_states(&mut self, n: usize) {
        // n may be zero.
        debug_assert!(n <= self.stack.len());
        tracing::trace!(opcode = "pop", num_states = n);
        self.stack.truncate(self.stack.len() - n);
    }

    fn handle_token(&mut self, token: &Token<'_>) -> ParserResult {
        match self.stack.last().unwrap().action(token) {
            Action::Accept => {
                tracing::trace!(opcode = "accept");
                ParserResult::Accept
            }
            Action::Shift(next) => {
                tracing::trace!(opcode = "shift");
                self.push_state(next);
                match token.kind {
                    TokenKind::TemplateHead => self.template_depth += 1,
                    TokenKind::TemplateTail => self.template_depth -= 1,
                    _ => (),
                }
                ParserResult::NextToken
            }
            Action::Reduce(non_terminal, n, rule) => {
                tracing::trace!(opcode = "reduce", ?rule);
                self.pop_states(n as usize);
                let next = self.stack.last().unwrap().goto(non_terminal);
                self.push_state(next);
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
        match self.stack.last().unwrap().action(&SEMICOLON) {
            Action::Accept => {
                tracing::trace!(opcode = "accept", auto_semicolon = true);
                ParserResult::Accept
            }
            Action::Shift(next) => {
                tracing::trace!(opcode = "shift", auto_semicolon = true);
                if next.is_auto_semicolon_disallowed() {
                    ParserResult::Error
                } else {
                    self.push_state(next);
                    ParserResult::NextToken
                }
            }
            Action::Reduce(non_terminal, n, rule) => {
                tracing::trace!(opcode = "reduce", ?rule, auto_semicolon = true);
                self.pop_states(n as usize);
                let next = self.stack.last().unwrap().goto(non_terminal);
                self.push_state(next);
                ParserResult::Reconsume
            }
            _ => ParserResult::Error,
        }
    }

    fn report_error(&self, token: &Token<'_>) {
        let pos = self.lexer.location();
        let src = self.lexer.src();
        tracing::error!(
            pos,
            src = &src[pos - 10..pos + 10],
            ?token,
            state = self.stack.last().unwrap().debug_info(),
        );
    }
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
}
