mod lalr;

pub use lalr::GoalSymbol;
pub use lalr::ProductionRule;

use crate::lexer::Goal;
use crate::lexer::Lexer;
use crate::lexer::Location;
use crate::lexer::Token;
use crate::lexer::TokenKind;
use crate::Error;

use lalr::Action;
use lalr::State;

const INITIAL_STATE_STACK_SIZE: usize = 512;
const INITIAL_BLOCK_STACK_SIZE: usize = 32;

pub struct Parser<'s, H>
where
    H: SyntaxHandler,
{
    handler: H,
    goal_symbol: GoalSymbol,
    lexer: Lexer<'s>,
    state_stack: Vec<State>,
    block_stack: Vec<BlockContext>,
    new_line: bool,
    // TODO: used only for measurement
    max_stack_depth: usize,
    max_template_literal_depth: usize,
}

impl<'s, H> Parser<'s, H>
where
    H: SyntaxHandler,
{
    /// Creates a parser recognizing a `Script`.
    pub fn for_script(src: &'s str, handler: H) -> Self {
        Self::new(src, handler, GoalSymbol::Script)
    }

    /// Creates a parser recognizing a `Module`.
    pub fn for_module(src: &'s str, handler: H) -> Self {
        Self::new(src, handler, GoalSymbol::Module)
    }

    fn new(src: &'s str, handler: H, goal_symbol: GoalSymbol) -> Self {
        Self {
            handler,
            goal_symbol,
            lexer: Lexer::new(src),
            state_stack: Vec::with_capacity(INITIAL_STATE_STACK_SIZE),
            block_stack: Vec::with_capacity(INITIAL_BLOCK_STACK_SIZE),
            new_line: false,
            max_stack_depth: 0,
            max_template_literal_depth: 0,
        }
    }

    pub fn parse(&mut self) -> Result<H::Artifact, Error> {
        self.handler.start();
        self.push_state(self.goal_symbol.start_state_id());
        self.push_block_context();
        let mut token = self.next_token()?;
        tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);
        loop {
            match self.handle_token(&token) {
                ParserResult::Accept(artifact) => return Ok(artifact),
                ParserResult::Reconsume => (),
                ParserResult::NextToken => {
                    self.consume_token(token);
                    token = self.next_token()?;
                    tracing::trace!(opcode = "token", ?token.kind, ?token.lexeme);
                }
                ParserResult::Error => {
                    if self.is_auto_semicolon_allowed(&token) {
                        loop {
                            match self.auto_semicolon() {
                                ParserResult::Accept(artifact) => return Ok(artifact),
                                ParserResult::Reconsume => (),
                                ParserResult::NextToken => break,
                                ParserResult::Error => {
                                    self.report_error(&token);
                                    return Err(Error::SyntaxError);
                                }
                            }
                        }
                    } else {
                        self.report_error(&token);
                        return Err(Error::SyntaxError);
                    }
                }
            }
        }
    }

    pub fn max_stack_depth(&self) -> usize {
        self.max_stack_depth
    }

    pub fn max_template_literal_depth(&self) -> usize {
        self.max_template_literal_depth
    }

    #[inline(always)]
    fn next_token(&mut self) -> Result<Token<'s>, Error> {
        self.lexer.set_goal(self.lexical_goal());
        self.lexer.next_token()
    }

    #[inline(always)]
    fn consume_token(&mut self, token: Token<'s>) {
        self.new_line = match token.kind {
            TokenKind::LineTerminatorSequence => true,
            TokenKind::WhiteSpaceSequence | TokenKind::Comment => self.new_line,
            _ => false,
        };
        tracing::trace!(new_line = self.new_line, ?token.kind);
        self.lexer.consume_token(token);
    }

    fn lexical_goal(&self) -> Goal {
        let template_tail_disallowed = self.template_literal_depth() == 0 || self.block_depth() > 0;
        match self.state().lexical_goal() {
            Goal::InputElementRegExpOrTemplateTail if template_tail_disallowed => {
                Goal::InputElementRegExp
            }
            Goal::InputElementTemplateTail if template_tail_disallowed => Goal::InputElementDiv,
            goal @ _ => goal,
        }
    }

    fn push_state(&mut self, state: State) {
        tracing::trace!(
            opcode = "push-state",
            stack.pos = self.state_stack.len(),
            state.id = state.id(),
            state.label = state.label(),
        );
        self.state_stack.push(state);
        if self.max_stack_depth < self.state_stack.len() {
            self.max_stack_depth = self.state_stack.len();
        }
    }

    fn pop_states(&mut self, n: usize) {
        // n may be zero.
        debug_assert!(n <= self.state_stack.len());
        tracing::trace!(opcode = "pop-state", num_states = n);
        self.state_stack.truncate(self.state_stack.len() - n);
    }

    fn replace_state(&mut self, state: State) {
        tracing::trace!(
            opcode = "replace-state",
            state.id = state.id(),
            state.label = state.label()
        );
        *self.state_stack.last_mut().unwrap() = state;
    }

    fn push_block_context(&mut self) {
        tracing::trace!(opcode = "push-block-context");
        self.block_stack.push(Default::default());
        let template_literal_depth = self.template_literal_depth();
        if self.max_template_literal_depth < template_literal_depth {
            self.max_template_literal_depth = template_literal_depth;
        }
    }

    fn pop_block_context(&mut self) {
        tracing::trace!(opcode = "pop-block-context");
        debug_assert_eq!(self.block_stack.last().unwrap().depth, 0);
        self.block_stack.pop();
    }

    fn push_block(&mut self) {
        tracing::trace!(opcode = "push-block");
        self.block_stack.last_mut().unwrap().depth += 1;
    }

    fn pop_block(&mut self) {
        tracing::trace!(opcode = "pop-block");
        debug_assert!(self.block_stack.last().unwrap().depth > 0);
        self.block_stack.last_mut().unwrap().depth -= 1;
    }

    fn handle_token(&mut self, token: &Token<'_>) -> ParserResult<H::Artifact> {
        let result = match self.state().action(token) {
            Action::Accept => {
                tracing::trace!(opcode = "accept", ?token.kind);
                self.handler.location(self.lexer.location());
                match self.handler.accept() {
                    Ok(artifact) => ParserResult::Accept(artifact),
                    Err(_err) => ParserResult::Error, // TODO: error reporting
                }
            }
            Action::Shift(next) => {
                tracing::trace!(opcode = "shift", ?token.kind);
                self.handler.location(self.lexer.location());
                match self.handler.shift(token) {
                    Ok(_) => {
                        match token.kind {
                            TokenKind::TemplateHead => self.push_block_context(),
                            TokenKind::TemplateTail => self.pop_block_context(),
                            TokenKind::Lbrace => self.push_block(),
                            TokenKind::Rbrace => self.pop_block(),
                            _ => (),
                        }
                        self.push_state(next);
                        ParserResult::NextToken
                    }
                    Err(_err) => ParserResult::Error, // TODO: error reporting
                }
            }
            Action::Reduce(non_terminal, n, rule) => {
                tracing::trace!(opcode = "reduce", %rule, ?token.kind);
                self.handler.location(self.lexer.location());
                match self.handler.reduce(rule) {
                    Ok(_) => {
                        self.pop_states(n as usize);
                        let mut next = self.state().goto(non_terminal);
                        if self.new_line {
                            if let Some(state) = next.can_replace() {
                                next = state;
                            }
                        }
                        self.push_state(next);
                        ParserResult::Reconsume
                    }
                    Err(_err) => ParserResult::Error, // TODO: error reporting
                }
            }
            Action::Replace(next) => {
                tracing::trace!(opcode = "replace", ?token.kind);
                self.replace_state(next);
                ParserResult::Reconsume
            }
            Action::Ignore => {
                tracing::trace!(opcode = "ignore", ?token.kind);
                ParserResult::NextToken
            }
            Action::Error => ParserResult::Error,
        };

        result
    }

    fn is_auto_semicolon_allowed(&self, token: &Token<'_>) -> bool {
        if self.new_line {
            return true;
        }
        if token.kind == TokenKind::Eof || token.kind == TokenKind::Rbrace {
            return true;
        }
        if self.state().is_auto_semicolon_do_while_statement() {
            return true;
        }
        false
    }

    fn auto_semicolon(&mut self) -> ParserResult<H::Artifact> {
        match self.state().action(&Token::AUTO_SEMICOLON) {
            Action::Accept => {
                tracing::trace!(opcode = "accept", auto_semicolon = true);
                match self.handler.accept() {
                    Ok(artifact) => ParserResult::Accept(artifact),
                    Err(_err) => ParserResult::Error, // TODO: error reporting
                }
            }
            Action::Shift(next) => {
                tracing::trace!(opcode = "shift", auto_semicolon = true);
                if next.is_auto_semicolon_disallowed() {
                    ParserResult::Error
                } else {
                    self.handler.location(self.lexer.location());
                    match self.handler.shift(&Token::AUTO_SEMICOLON) {
                        Ok(_) => {
                            self.push_state(next);
                            ParserResult::NextToken
                        }
                        Err(_err) => ParserResult::Error, // TODO: error reporting
                    }
                }
            }
            Action::Reduce(non_terminal, n, rule) => {
                tracing::trace!(opcode = "reduce", ?rule, auto_semicolon = true);
                self.handler.location(self.lexer.location());
                match self.handler.reduce(rule) {
                    Ok(_) => {
                        self.pop_states(n as usize);
                        let state = self.state();
                        let next = state.goto(non_terminal);
                        self.push_state(next);
                        ParserResult::Reconsume
                    }
                    Err(_err) => ParserResult::Error, // TODO: error reporting
                }
            }
            Action::Replace(_) => unreachable!(),
            Action::Ignore => unreachable!(),
            Action::Error => ParserResult::Error,
        }
    }

    fn report_error(&self, token: &Token<'_>) {
        let pos = self.lexer.pos();
        let src = self.lexer.src();
        let state = self.state();
        tracing::error!(
            pos,
            parsed = &src[pos.saturating_sub(10)..pos],
            remaianing = &src[pos..((pos + 10).min(src.len()))],
            ?token.kind,
            ?token.lexeme,
            state.id = state.id(),
            state.label = state.label(),
            template_literal_depth = self.template_literal_depth(),
            block_depth = self.block_depth(),
        );
    }

    #[inline(always)]
    fn state(&self) -> State {
        self.state_stack.last().unwrap().clone()
    }

    #[inline(always)]
    fn block(&self) -> &BlockContext {
        self.block_stack.last().unwrap()
    }

    #[inline(always)]
    fn template_literal_depth(&self) -> usize {
        debug_assert!(self.block_stack.len() > 0);
        self.block_stack.len() - 1
    }

    #[inline(always)]
    fn block_depth(&self) -> usize {
        self.block().depth
    }
}

#[derive(Default)]
struct BlockContext {
    depth: usize,
}

enum ParserResult<T> {
    Accept(T),
    Reconsume,
    NextToken,
    Error,
}

pub trait SyntaxHandler {
    type Artifact;
    type Error: std::fmt::Debug + std::fmt::Display;

    /// Called before parsing.
    fn start(&mut self);

    /// Called when the accept state has been reached.
    fn accept(&mut self) -> Result<Self::Artifact, Self::Error>;

    /// Called when a shift action has been performed.
    fn shift<'a>(&mut self, token: &Token<'a>) -> Result<(), Self::Error>;

    /// Called when a reduce action has been performed.
    fn reduce(&mut self, rule: ProductionRule) -> Result<(), Self::Error>;

    /// Called before calling other methods in order to inform the location in the source text
    /// where the event occurs.
    #[allow(unused_variables)]
    #[inline(always)]
    fn location(&mut self, location: &Location) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use test_log::test;

    // TODO: use a mock.
    struct NullHandler;

    impl SyntaxHandler for NullHandler {
        type Artifact = ();
        type Error = std::convert::Infallible;
        fn start(&mut self) {}
        fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
            Ok(())
        }
        fn shift<'a>(&mut self, _token: &Token<'a>) -> Result<(), Self::Error> {
            Ok(())
        }
        fn reduce(&mut self, _rule: ProductionRule) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    macro_rules! parse {
        ($src:literal) => {
            assert_matches!(Parser::for_script($src, NullHandler).parse(), Ok(_));
        };
    }

    macro_rules! parse_fail {
        ($src:literal) => {
            assert_matches!(Parser::for_script($src, NullHandler).parse(), Err(_));
        };
    }

    #[test]
    fn test_parse_empty_script() {
        parse!("");
    }

    #[test]
    fn test_parse_auto_semicolon1() {
        parse!("{ 1\n2 } 3");
    }

    #[test]
    fn test_parse_auto_semicolon2() {
        parse!("function x() { return\na + b }");
    }

    #[test]
    fn test_parse_auto_semicolon_variable_statement() {
        parse!("var x = 1");
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement1() {
        parse_fail!("for () {}");
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement2() {
        parse_fail!("for (true) {}");
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement3() {
        parse_fail!("for (;) {}");
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement4() {
        parse_fail!("for (true;) {}");
    }

    #[test]
    fn test_parser_auto_semicolon_for_statement5() {
        parse_fail!("for (;true) {}");
    }

    #[test]
    fn test_parser_auto_semicolon_do_while1() {
        parse!("do {} while (0)");
    }

    #[test]
    fn test_parser_auto_semicolon_do_while2() {
        parse!("do {} while (0) 0;");
    }

    #[test]
    fn test_parser_auto_semicolon_template_literal() {
        parse!("`${x.x(x=>{return()})}`");
    }

    #[test]
    fn test_parser_template_literal() {
        parse!("`${`${a}`}`");
    }

    #[test]
    fn test_parser_arrow_function() {
        parse!("()=>{}");
    }

    #[test]
    fn test_parser_async_arrow_function() {
        parse!("async()=>{}");
    }
}
