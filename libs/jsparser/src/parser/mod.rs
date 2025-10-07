logging::define_logger! {"bee::jsparser::parser"}

mod lalr;

pub use lalr::GoalSymbol;
pub use lalr::ProductionRule;

use crate::Error;
use crate::lexer::Goal;
use crate::lexer::Lexer;
use crate::lexer::Location;
use crate::lexer::Token;
use crate::lexer::TokenKind;

use lalr::Action;
use lalr::State;

const INITIAL_STATE_STACK_SIZE: usize = 512;
const INITIAL_BLOCK_STACK_SIZE: usize = 32;

pub struct Parser<'s, H>
where
    H: SyntaxHandler<'s>,
{
    handler: H,
    goal_symbol: GoalSymbol,
    lexer: Lexer<'s>,
    state_stack: Vec<State>,
    block_stack: Vec<BlockContext>,
    auto_semicolon_insertion_point: Location,
    new_line: bool,
    // TODO: used only for measurement
    max_stack_depth: usize,
    max_template_literal_depth: usize,
}

impl<'s, H> Parser<'s, H>
where
    H: SyntaxHandler<'s>,
{
    /// Creates a parser recognizing a `Script`.
    pub fn for_script(src: &'s str, handler: H) -> Self {
        Self::new(src, handler, GoalSymbol::Script)
    }

    /// Creates a parser recognizing a `Module`.
    pub fn for_module(src: &'s str, handler: H) -> Self {
        Self::new(src, handler, GoalSymbol::Module)
    }

    pub(crate) fn new(src: &'s str, handler: H, goal_symbol: GoalSymbol) -> Self {
        Self {
            handler,
            goal_symbol,
            lexer: Lexer::new(src),
            state_stack: Vec::with_capacity(INITIAL_STATE_STACK_SIZE),
            block_stack: Vec::with_capacity(INITIAL_BLOCK_STACK_SIZE),
            auto_semicolon_insertion_point: Default::default(),
            new_line: false,
            max_stack_depth: 0,
            max_template_literal_depth: 0,
        }
    }

    pub fn parse(&mut self) -> Result<H::Artifact, Error> {
        self.handler.start();
        self.handler.source(self.lexer.src());
        self.push_state(self.goal_symbol.start_state_id());
        self.push_block_context();
        let mut token = self.next_token()?;
        let mut auto_semicolon_inserted = false;
        logger::trace!(opcode = "token", ?token.kind, token.lexeme);
        loop {
            match self.handle_token(&token) {
                ParserResult::Accept(artifact) => return Ok(artifact),
                ParserResult::Reconsume => (),
                ParserResult::NextToken => {
                    auto_semicolon_inserted = false;
                    self.consume_token(token);
                    token = self.next_token()?;
                    logger::trace!(opcode = "token", ?token.kind, token.lexeme);
                }
                ParserResult::Error
                    if !auto_semicolon_inserted && self.is_auto_semicolon_allowed(&token) =>
                {
                    loop {
                        match self.auto_semicolon() {
                            ParserResult::Accept(artifact) => return Ok(artifact),
                            ParserResult::Reconsume => (),
                            ParserResult::NextToken => break,
                            _ => {
                                let err = Error::SyntaxError;
                                self.report_error(&err, &token);
                                return Err(err);
                            }
                        }
                    }
                    auto_semicolon_inserted = true;
                }
                ParserResult::Error => {
                    let err = Error::SyntaxError;
                    self.report_error(&err, &token);
                    return Err(err);
                }
                ParserResult::SyntaxError(err) => {
                    self.report_error(&err, &token);
                    return Err(err);
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
        let update_auto_semicolon_insertion_point = match token.kind {
            TokenKind::LineTerminatorSequence => {
                self.new_line = true;
                false
            }
            TokenKind::WhiteSpaceSequence | TokenKind::HashbangComment => false,
            // A comment having line terminators affects the new_line state as described in
            // "5.1.2 The Lexical and RegExp Grammars".
            TokenKind::Comment => {
                if token.has_line_terminators() {
                    self.new_line = true;
                }
                false
            }
            _ => {
                self.new_line = false;
                true
            }
        };
        logger::trace!(opcode = "consume", new_line = self.new_line, ?token.kind);
        self.lexer.consume_token(token);
        if update_auto_semicolon_insertion_point {
            self.auto_semicolon_insertion_point = self.lexer.location().clone();
        }
    }

    fn lexical_goal(&self) -> Goal {
        let template_tail_disallowed = self.template_literal_depth() == 0 || self.block_depth() > 0;
        match self.state().lexical_goal() {
            Goal::RegExpOrTemplateTail if template_tail_disallowed => Goal::RegExp,
            Goal::TemplateTail if template_tail_disallowed => Goal::Div,
            goal => goal,
        }
    }

    fn push_state(&mut self, state: State) {
        logger::trace!(
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
        logger::trace!(opcode = "pop-state", num_states = n);
        self.state_stack.truncate(self.state_stack.len() - n);
    }

    fn replace_state(&mut self, state: State) {
        logger::trace!(
            opcode = "replace-state",
            state.id = state.id(),
            state.label = state.label()
        );
        *self.state_stack.last_mut().unwrap() = state;
    }

    fn push_block_context(&mut self) {
        logger::trace!(opcode = "push-block-context");
        self.block_stack.push(Default::default());
        let template_literal_depth = self.template_literal_depth();
        if self.max_template_literal_depth < template_literal_depth {
            self.max_template_literal_depth = template_literal_depth;
        }
    }

    fn pop_block_context(&mut self) {
        logger::trace!(opcode = "pop-block-context");
        debug_assert_eq!(self.block_stack.last().unwrap().depth, 0);
        self.block_stack.pop();
    }

    fn push_block(&mut self) {
        logger::trace!(opcode = "push-block");
        self.block_stack.last_mut().unwrap().depth += 1;
    }

    fn pop_block(&mut self) {
        logger::trace!(opcode = "pop-block");
        debug_assert!(self.block_stack.last().unwrap().depth > 0);
        self.block_stack.last_mut().unwrap().depth -= 1;
    }

    fn handle_token(&mut self, token: &Token<'s>) -> ParserResult<H::Artifact> {
        // An comment having line terminators is treated as a single line terminator in the
        // grammar as described in "5.1.2 The Lexical and RegExp Grammars".
        let token_for_grammar = match token.kind {
            TokenKind::Comment if token.has_line_terminators() => &Token::SINGLE_LINE_TERMINATOR,
            _ => token,
        };
        match self.state().action(token_for_grammar) {
            Action::Accept => {
                logger::trace!(opcode = "accept", ?token.kind);
                self.handler.location(self.lexer.location());
                match self.handler.accept() {
                    Ok(artifact) => ParserResult::Accept(artifact),
                    Err(_err) => ParserResult::Error, // TODO: error reporting
                }
            }
            Action::Shift(next) => {
                logger::trace!(opcode = "shift", ?token.kind);
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
                    Err(err) => ParserResult::SyntaxError(err),
                }
            }
            Action::Reduce(non_terminal, n, rule) => {
                logger::trace!(opcode = "reduce", %rule, ?token.kind);
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
                    Err(err) => ParserResult::SyntaxError(err),
                }
            }
            Action::Replace(next) => {
                logger::trace!(opcode = "replace", ?token.kind);
                self.replace_state(next);
                ParserResult::Reconsume
            }
            Action::Ignore => {
                logger::trace!(opcode = "ignore", ?token.kind);
                ParserResult::NextToken
            }
            Action::Error => ParserResult::Error,
        }
    }

    fn is_auto_semicolon_allowed(&self, token: &Token<'s>) -> bool {
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
                logger::trace!(opcode = "accept", auto_semicolon = true);
                self.handler.location(self.lexer.location());
                match self.handler.accept() {
                    Ok(artifact) => ParserResult::Accept(artifact),
                    Err(err) => ParserResult::SyntaxError(err),
                }
            }
            Action::Shift(next) => {
                logger::trace!(opcode = "shift", auto_semicolon = true);
                if next.is_auto_semicolon_disallowed() {
                    ParserResult::Error
                } else {
                    self.handler.location(&self.auto_semicolon_insertion_point);
                    match self.handler.shift(&Token::AUTO_SEMICOLON) {
                        Ok(_) => {
                            self.push_state(next);
                            ParserResult::NextToken
                        }
                        Err(err) => ParserResult::SyntaxError(err),
                    }
                }
            }
            Action::Reduce(non_terminal, n, rule) => {
                logger::trace!(opcode = "reduce", %rule, auto_semicolon = true);
                self.handler.location(&self.auto_semicolon_insertion_point);
                match self.handler.reduce(rule) {
                    Ok(_) => {
                        self.pop_states(n as usize);
                        let state = self.state();
                        let next = state.goto(non_terminal);
                        self.push_state(next);
                        ParserResult::Reconsume
                    }
                    Err(err) => ParserResult::SyntaxError(err),
                }
            }
            Action::Replace(_) => unreachable!(),
            Action::Ignore => unreachable!(),
            Action::Error => ParserResult::SyntaxError(Error::SyntaxError),
        }
    }

    fn report_error(&self, err: &Error, token: &Token<'s>) {
        let pos = self.lexer.pos();
        let src = self.lexer.src();
        let state = self.state();
        let next_10_chars: String = src[pos..].chars().take(10).collect();
        logger::error!(
            ?err,
            pos,
            next_10_chars,
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
        *self.state_stack.last().unwrap()
    }

    #[inline(always)]
    fn block(&self) -> &BlockContext {
        self.block_stack.last().unwrap()
    }

    #[inline(always)]
    fn template_literal_depth(&self) -> usize {
        debug_assert!(!self.block_stack.is_empty());
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
    SyntaxError(Error),
}

pub trait SyntaxHandler<'s> {
    type Artifact;

    /// Called before parsing.
    fn start(&mut self) {}

    /// Called once just after `start()` is called.
    #[allow(unused_variables)]
    fn source(&mut self, src: &'s str) {}

    /// Called when the accept state has been reached.
    fn accept(&mut self) -> Result<Self::Artifact, Error>;

    /// Called when a shift action has been performed.
    fn shift(&mut self, token: &Token<'s>) -> Result<(), Error>;

    /// Called when a reduce action has been performed.
    fn reduce(&mut self, rule: ProductionRule) -> Result<(), Error>;

    /// Called before calling other methods in order to inform the location in the source text
    /// where the event occurs.
    #[allow(unused_variables)]
    fn location(&mut self, location: &Location) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    // TODO: use a mock.
    struct NullHandler;

    impl SyntaxHandler<'_> for NullHandler {
        type Artifact = ();
        fn start(&mut self) {}
        fn accept(&mut self) -> Result<Self::Artifact, Error> {
            Ok(())
        }
        fn shift(&mut self, _token: &Token<'_>) -> Result<(), Error> {
            Ok(())
        }
        fn reduce(&mut self, _rule: ProductionRule) -> Result<(), Error> {
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

    #[test]
    fn test_inc_no_line_terminator_multi_line_comment() {
        parse!("x/**/++");
    }

    #[test]
    fn test_inc_no_line_terminator_multi_line_comment_lf() {
        parse_fail!("x/*\n*/++");
    }

    #[test]
    fn test_dec_no_line_terminator_multi_line_comment() {
        parse!("x/**/--");
    }

    #[test]
    fn test_dec_no_line_terminator_multi_line_comment_lf() {
        parse_fail!("x/*\n*/--");
    }

    #[test]
    fn test_continue_no_line_terminator_multi_line_comment() {
        parse!("x: for(;;){continue/**/x}");
    }

    #[test]
    fn test_continue_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "x: for(;;){continue; x;}".
        parse!("x: for(;;){continue/*\n*/x}");
    }

    #[test]
    fn test_break_no_line_terminator_multi_line_comment() {
        parse!("x: for(;;){break/**/x}");
    }

    #[test]
    fn test_break_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "x: for(;;){break; x;}".
        parse!("x: for(;;){break/*\n*/x}");
    }

    #[test]
    fn test_return_no_line_terminator_multi_line_comment() {
        parse!("function x(){return/**/0}");
    }

    #[test]
    fn test_return_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "function x() { return; 0; }".
        parse!("function x(){return/*\n*/0}");
    }

    #[test]
    fn test_throw_no_line_terminator_multi_line_comment() {
        parse!("throw/**/0");
    }

    #[test]
    fn test_throw_no_line_terminator_multi_line_comment_lf() {
        parse_fail!("throw/*\n*/0");
    }

    #[test]
    fn test_arrow_function_no_line_terminator_multi_line_comment() {
        parse!("()/**/=>{}");
    }

    #[test]
    fn test_arrow_function_no_line_terminator_multi_line_comment_lf() {
        parse_fail!("()/*\n*/=>{}");
    }

    #[test]
    fn test_yield_no_line_terminator_multi_line_comment() {
        parse!("function* x(){yield/**/0}");
    }

    #[test]
    fn test_yield_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "function* x() { yield; 0; }".
        parse!("function* x(){yield/*\n*/0}");
    }

    #[test]
    fn test_async_generator_declaration_no_line_terminator_multi_line_comment() {
        parse!("async/**/function* x(){}");
    }

    #[test]
    fn test_async_generator_declaration_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "async; function* x() {}".
        parse!("async/*\n*/function* x(){}");
    }

    #[test]
    fn test_async_generator_expression_no_line_terminator_multi_line_comment() {
        parse!("x=async/**/function*(){}");
    }

    #[test]
    fn test_async_generator_expression_no_line_terminator_multi_line_comment_lf() {
        parse_fail!("x=async/*\n*/function*(){}");
    }

    #[test]
    fn test_async_generator_method_no_line_terminator_multi_line_comment() {
        parse!("class X{async/**/*x(){}}");
    }

    #[test]
    fn test_async_generator_method_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "class X { async; *x() {} }".
        parse!("class X{async/*\n*/*x(){}}");
    }

    #[test]
    fn test_async_function_declaration_no_line_terminator_multi_line_comment() {
        parse!("async/**/function x(){}");
    }

    #[test]
    fn test_async_function_declaration_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "async; function x() {}"
        parse!("async/*\n*/function x(){}");
    }

    #[test]
    fn test_async_function_expression_no_line_terminator_multi_line_comment() {
        parse!("x=async/**/function(){}");
    }

    #[test]
    fn test_async_function_expression_no_line_terminator_multi_line_comment_lf() {
        parse_fail!("x=async/*\n*/function(){}");
    }

    #[test]
    fn test_async_method_no_line_terminator_multi_line_comment() {
        parse!("class X{async/**/x(){}}");
    }

    #[test]
    fn test_async_method_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "class X { async; x() {} }".
        parse!("class X{async/*\n*/x(){}}");
    }

    #[test]
    fn test_async_arrow_function_no_line_terminator_multi_line_comment() {
        parse!("async/**/()=>{}");
    }

    #[test]
    fn test_async_arrow_function_no_line_terminator_multi_line_comment_lf() {
        // This case can be parsed as "async; ()=>{}".
        // Because the second production rule is met.
        //
        //   AsyncArrowFunction[In, Yield, Await] :
        //     `async` [no LineTerminator here] AsyncArrowBindingIdentifier[?Yield] [no LineTerminator here] `=>` AsyncConciseBody[?In]
        //     CoverCallExpressionAndAsyncArrowHead[?Yield, ?Await] [no LineTerminator here] `=>` AsyncConciseBody[?In] #callcover
        //
        // However, Acorn reports an unexpected token error.  Probably, Acorn is right and our
        // implementation is wrong...
        //
        // TODO: Use the supplemental syntax defined in 15.9 Async Arrow Function Definitions.
        parse!("async/*\n*/()=>{}");
    }
}
