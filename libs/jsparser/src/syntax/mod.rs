mod actions;
mod logger;

use std::collections::VecDeque;

use super::Error;
use super::Location;
use super::ProductionRule;
use super::Symbol;
use super::SymbolTable;
use super::SyntaxHandler;
use super::Token;
use super::TokenKind;

pub trait SemanticHandler<'s> {
    type Artifact;

    fn symbol_table(&mut self) -> &SymbolTable;
    fn symbol_table_mut(&mut self) -> &mut SymbolTable;

    fn start(&mut self);
    fn accept(&mut self) -> Result<Self::Artifact, Error>;
    fn handle_numeric_literal(&mut self, literal: NumericLiteral<'s>) -> Result<(), Error>;
    fn handle_string_literal(&mut self, literal: StringLiteral<'s>) -> Result<(), Error>;
    fn handle_identifier(&mut self, identifier: Identifier<'s>) -> Result<(), Error>;
    fn handle_addition_expression(&mut self) -> Result<(), Error>;
    fn handle_subtraction_expression(&mut self) -> Result<(), Error>;
    fn handle_multiplication_expression(&mut self) -> Result<(), Error>;
    fn handle_division_expression(&mut self) -> Result<(), Error>;
    fn handle_remainder_expression(&mut self) -> Result<(), Error>;
    fn handle_lt_expression(&mut self) -> Result<(), Error>;
    fn handle_gt_expression(&mut self) -> Result<(), Error>;
    fn handle_lte_expression(&mut self) -> Result<(), Error>;
    fn handle_gte_expression(&mut self) -> Result<(), Error>;
    fn handle_eq_expression(&mut self) -> Result<(), Error>;
    fn handle_ne_expression(&mut self) -> Result<(), Error>;
    fn handle_strict_eq_expression(&mut self) -> Result<(), Error>;
    fn handle_strict_ne_expression(&mut self) -> Result<(), Error>;
    fn handle_call_expression(&mut self) -> Result<(), Error>;
    fn handle_assignment_expression(&mut self) -> Result<(), Error>;
    fn handle_then_block(&mut self) -> Result<(), Error>;
    fn handle_else_block(&mut self) -> Result<(), Error>;
    fn handle_conditional_expression(&mut self) -> Result<(), Error>;
    fn handle_expression_statement(&mut self) -> Result<(), Error>;
    fn handle_if_else_statement(&mut self) -> Result<(), Error>;
    fn handle_if_statement(&mut self) -> Result<(), Error>;
    fn handle_return_statement(&mut self, n: usize) -> Result<(), Error>;
    fn handle_statement(&mut self) -> Result<(), Error>;
    fn handle_function_signature(&mut self, symbol: Symbol, formal_parameters: Vec<Symbol>) -> Result<(), Error>;
    fn handle_function_declaration(&mut self) -> Result<(), Error>;

    fn handle_start_let_declaration(&mut self) -> Result<(), Error>;
    fn handle_let_binding(&mut self, with_init: bool) -> Result<(), Error>;
    fn handle_end_let_declaration(&mut self) -> Result<(), Error>;

    fn handle_start_const_declaration(&mut self) -> Result<(), Error>;
    fn handle_const_binding(&mut self) -> Result<(), Error>;
    fn handle_end_const_declaration(&mut self) -> Result<(), Error>;

    fn handle_start_scope(&mut self) -> Result<(), Error>;
    fn handle_end_scope(&mut self) -> Result<(), Error>;

    fn handle_argument_list(&mut self, empty: bool) -> Result<(), Error>;
    fn handle_argument_list_item(&mut self) -> Result<(), Error>;
}

pub struct Processor<'s, H> {
    handler: H,
    location: Location,
    queue: VecDeque<Syntax<'s>>,
    strict_mode: bool,
    module: bool,
}

#[derive(Debug)]
enum Syntax<'s> {
    NumericLiteral(NumericLiteral<'s>),
    StringLiteral(StringLiteral<'s>),
    Identifier(Identifier<'s>),
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equality,
    Inequality,
    StrictEquality,
    StrictInequality,
    Assignment,
    Cpeaapl,
    CallExpressionOrAsyncArrowHead,
    MaybeArrowFormalParameters,
    MaybeArrowFormalParametersEmpty,
    MaybeArrowFormalParametersRestParameter,
    MaybeArrowFormalParametersRestPattern,
    MaybeArrowFormalParametersWithRestParameter,
    MaybeArrowFormalParametersWithRestPattern,
    FormalParameters(Vec<Symbol>), // TODO: Vec<BindingElement>
    LexicalBinding,
    LexicalBindingWithInitializer,
    EmptyList,
    ArgumentList(bool),
    ArgumentListItem,

    // Internals
    FormalParameter(Symbol), // TODO: BindingElement
}

#[derive(Debug)]
pub struct NumericLiteral<'s> {
    pub value: f64,
    pub raw: &'s str,
}

#[derive(Debug)]
pub struct StringLiteral<'s> {
    pub value: Vec<u16>,
    pub raw: &'s str,
}

#[derive(Debug)]
pub struct Identifier<'s> {
    pub symbol: Symbol,
    pub raw: &'s str,
}

impl<'s, H> Processor<'s, H> {
    const INITIAL_STACK_CAPACITY: usize = 64;

    pub fn new(handler: H, module: bool) -> Self {
        Self {
            handler,
            location: Default::default(),
            queue: VecDeque::with_capacity(Self::INITIAL_STACK_CAPACITY),
            strict_mode: false,
            module,
        }
    }

    #[inline]
    fn last(&self) -> &Syntax<'s> {
        self.queue.back().unwrap()
    }

    #[inline]
    fn last_mut(&mut self) -> &mut Syntax<'s> {
        self.queue.back_mut().unwrap()
    }

    #[inline]
    fn push(&mut self, syntax: Syntax<'s>) {
        logger::debug!(event = "push", ?syntax);
        self.queue.push_back(syntax);
    }

    #[inline]
    fn pop(&mut self) -> Option<Syntax<'s>> {
        self.queue.pop_back()
    }

    #[inline]
    fn swap_pop(&mut self) -> Option<Syntax<'s>> {
        debug_assert!(self.queue.len() > 1);
        let i = self.queue.len() - 2;
        self.queue.swap_remove_back(i)
    }

    #[inline]
    fn fetch(&mut self) -> Option<Syntax<'s>> {
        self.queue.pop_front()
    }

    #[inline]
    fn split(&mut self, n: usize) -> VecDeque<Syntax<'s>> {
        debug_assert!(n > 0);
        debug_assert!(self.queue.len() >= n);
        let i = self.queue.len() - n;
        self.queue.split_off(i)
    }
}

impl<'s, H> SyntaxHandler<'s> for Processor<'s, H>
where
    H: SemanticHandler<'s>,
{
    type Artifact = H::Artifact;
    type Error = Error;

    fn start(&mut self) {
        logger::debug!(event = "start");
        self.handler.start();
    }

    fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
        logger::debug!(event = "accept");
        self.handler.accept()
    }

    fn shift(&mut self, token: &Token<'s>) -> Result<(), Self::Error> {
        logger::debug!(
            event = "shift",
            ?token.kind,
            inserted_automaticaly = token.inserted_automatically(),
            start = %self.location,
            end = %token.compute_end(&self.location),
        );

        match token.kind {
            TokenKind::NumericLiteral => {
                // TODO: perform `NumericValue`
                let value = token.lexeme.parse::<f64>().unwrap();
                self.push(Syntax::NumericLiteral(NumericLiteral {
                    value,
                    raw: token.lexeme,
                }));
            }
            TokenKind::StringLiteral => {
                // TODO: perform `SV`
                let value = token.lexeme.encode_utf16().collect();
                self.push(Syntax::StringLiteral(StringLiteral {
                    value,
                    raw: token.lexeme,
                }));
            }
            TokenKind::IdentifierName => {
                // TODO: perform `StringValue`
                let value = token.lexeme.encode_utf16().collect();
                let symbol_table = self.handler.symbol_table_mut();
                let symbol = symbol_table.intern(value);
                self.push(Syntax::Identifier(Identifier {
                    symbol,
                    raw: token.lexeme,
                }));
            }
            _ => (),
        }
        Ok(())
    }

    fn reduce(&mut self, rule: ProductionRule) -> Result<(), Self::Error> {
        match Self::ACTIONS[rule.id() as usize] {
            Action::Undefined => unimplemented!("No action defined for: {rule}"),
            Action::Nop => {
                logger::debug!(event = "reduce", action = "nop", %rule);
                Ok(())
            }
            Action::Invoke(action, name) => {
                logger::debug!(event = "reduce", action = name, %rule);
                action(self)
            }
        }
    }

    fn location(&mut self, location: &Location) {
        logger::debug!(event = "location", %location);
        self.location = location.clone();
    }
}

impl<'s, H> Processor<'s, H>
where
    H: SemanticHandler<'s>,
{
    fn syntax_error(&mut self) -> Result<(), Error> {
        Err(Error::SyntaxError)
    }

    // 13.1.1 Static Semantics: Early Errors
    //
    // TODO: improve performance
    // introduce a symbol table to intern identifier strings.

    fn syntax_error_in_module(&mut self) -> Result<(), Error> {
        if self.module {
            Err(Error::SyntaxError)
        } else {
            Ok(())
        }
    }

    fn syntax_error_in_strict_mode(&mut self) -> Result<(), Error> {
        if self.strict_mode {
            Err(Error::SyntaxError)
        } else {
            Ok(())
        }
    }

    fn syntax_error_if_string_value_is_keyword_in_strict_mode(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::IMPLEMENTS
            | SymbolTable::LET
            | SymbolTable::PACKAGE
            | SymbolTable::PRIVATE
            | SymbolTable::PROTECTED
            | SymbolTable::PUBLIC
            | SymbolTable::STATIC
            | SymbolTable::YIELD
                if self.strict_mode =>
            {
                Err(Error::SyntaxError)
            }
            _ => Ok(()),
        }
    }

    fn syntax_error_if_await(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_yield(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::YIELD => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_yield_or_await(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::YIELD | SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_arguments_or_eval(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::ARGUMENTS | SymbolTable::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            _ => Ok(()),
        }
    }

    fn syntax_error_if_arguments_or_eval_or_await(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::ARGUMENTS | SymbolTable::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_arguments_or_eval_or_yield(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::ARGUMENTS | SymbolTable::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            SymbolTable::YIELD => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_arguments_or_eval_or_yield_or_await(&mut self) -> Result<(), Error> {
        let identifier = match self.last() {
            Syntax::Identifier(identifier) => identifier,
            _ => unreachable!(),
        };
        match identifier.symbol {
            SymbolTable::ARGUMENTS | SymbolTable::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            SymbolTable::YIELD | SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn handle_identifier_reference(&mut self) -> Result<(), Error> {
        let identifier = match self.pop() {
            Some(Syntax::Identifier(identifier)) => identifier,
            _ => unreachable!(),
        };
        self.push(Syntax::Identifier(identifier));
        Ok(())
    }

    fn handle_identifier_reference_except_for_await(&mut self) -> Result<(), Error> {
        let identifier = match self.pop() {
            Some(Syntax::Identifier(identifier)) => identifier,
            _ => unreachable!(),
        };
        if matches!(identifier.symbol, SymbolTable::AWAIT) {
            return Err(Error::SyntaxError);
        }
        self.push(Syntax::Identifier(identifier));
        Ok(())
    }

    fn handle_identifier_reference_except_for_yield(&mut self) -> Result<(), Error> {
        let identifier = match self.pop() {
            Some(Syntax::Identifier(identifier)) => identifier,
            _ => unreachable!(),
        };
        if matches!(identifier.symbol, SymbolTable::AWAIT) {
            return Err(Error::SyntaxError);
        }
        self.push(Syntax::Identifier(identifier));
        Ok(())
    }

    fn handle_identifier_reference_except_for_yield_await(&mut self) -> Result<(), Error> {
        let identifier = match self.pop() {
            Some(Syntax::Identifier(identifier)) => identifier,
            _ => unreachable!(),
        };
        if matches!(identifier.symbol, SymbolTable::YIELD | SymbolTable::AWAIT) {
            return Err(Error::SyntaxError);
        }
        self.push(Syntax::Identifier(identifier));
        Ok(())
    }

    fn handle_await_as_identifier_reference_in_script(&mut self) -> Result<(), Error> {
        if self.module {
            return Err(Error::SyntaxError);
        }
        self.handle_await_as_identifier_reference()
    }

    fn handle_yield_as_identifier_reference_in_non_strict_code(&mut self) -> Result<(), Error> {
        if self.strict_mode {
            return Err(Error::SyntaxError);
        }
        self.handle_yield_as_identifier_reference()
    }

    fn handle_await_as_identifier_reference(&mut self) -> Result<(), Error> {
        self.push(Syntax::Identifier(Identifier {
            symbol: SymbolTable::AWAIT,
            raw: "await",
        }));
        Ok(())
    }

    fn handle_yield_as_identifier_reference(&mut self) -> Result<(), Error> {
        self.push(Syntax::Identifier(Identifier {
            symbol: SymbolTable::YIELD,
            raw: "yield",
        }));
        Ok(())
    }

    fn handle_addition_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Addition);
        Ok(())
    }

    fn handle_subtraction_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Subtraction);
        Ok(())
    }

    fn handle_multiplication_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Multiplication);
        Ok(())
    }

    fn handle_division_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Division);
        Ok(())
    }

    fn handle_remainder_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Remainder);
        Ok(())
    }

    fn handle_lt_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::LessThan);
        Ok(())
    }

    fn handle_gt_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::GreaterThan);
        Ok(())
    }

    fn handle_lte_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::LessThanOrEqual);
        Ok(())
    }

    fn handle_gte_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::GreaterThanOrEqual);
        Ok(())
    }

    fn handle_eq_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Equality);
        Ok(())
    }

    fn handle_ne_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::Inequality);
        Ok(())
    }

    fn handle_strict_eq_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::StrictEquality);
        Ok(())
    }

    fn handle_strict_ne_expression(&mut self) -> Result<(), Error> {
        self.push(Syntax::StrictInequality);
        Ok(())
    }

    fn handle_cpeaapl(&mut self) -> Result<(), Error> {
        self.push(Syntax::Cpeaapl);
        Ok(())
    }

    fn handle_call_expression(&mut self) -> Result<(), Error> {
        self.pop(); // removes the cover syntax
        logger::debug!(?self.queue);
        while let Some(syntax) = self.fetch() {
            self.dispatch(syntax)?;
        }
        self.handler.handle_call_expression()
    }

    fn handle_call_expression_or_async_arrow_head(&mut self) -> Result<(), Error> {
        self.push(Syntax::CallExpressionOrAsyncArrowHead);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters(&mut self) -> Result<(), Error> {
        self.push(Syntax::MaybeArrowFormalParameters);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters_empty(&mut self) -> Result<(), Error> {
        self.push(Syntax::MaybeArrowFormalParametersEmpty);
        Ok(())
    }

    fn handle_maybe_arrow_formal_rest_parameter(&mut self) -> Result<(), Error> {
        self.push(Syntax::MaybeArrowFormalParametersRestParameter);
        Ok(())
    }

    fn handle_maybe_arrow_formal_rest_pattern(&mut self) -> Result<(), Error> {
        self.push(Syntax::MaybeArrowFormalParametersRestPattern);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters_with_rest_parameter(&mut self) -> Result<(), Error> {
        self.push(Syntax::MaybeArrowFormalParametersWithRestParameter);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters_with_rest_pattern(&mut self) -> Result<(), Error> {
        self.push(Syntax::MaybeArrowFormalParametersWithRestPattern);
        Ok(())
    }

    fn handle_group_expression(&mut self) -> Result<(), Error> {
        debug_assert!(matches!(self.last(), Syntax::Cpeaapl));
        self.pop();
        self.flush()
    }

    fn handle_assignment_expression(&mut self) -> Result<(), Error> {
        // TODO: EE
        self.push(Syntax::Assignment);
        Ok(())
    }

    fn handle_then_block(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_then_block()
    }

    fn handle_else_block(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_else_block()
    }

    fn handle_conditional_expression(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_conditional_expression()
    }

    fn handle_expression_statement(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_expression_statement()
    }

    fn handle_block_statement(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    fn handle_if_else_statement(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_if_else_statement()
    }

    fn handle_if_statement(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_if_statement()
    }

    fn handle_statement(&mut self) -> Result<(), Error> {
        self.handler.handle_statement()
    }

    fn handle_return_statement_0(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_return_statement(0)
    }

    fn handle_return_statement_1(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_return_statement(1)
    }

    fn handle_let_declaration(&mut self) -> Result<(), Error> {
        logger::debug!(?self.queue);
        // TODO: Check uniqueness of identifiers
        self.handler.handle_start_let_declaration()?;
        while let Some(syntax) = self.fetch() {
            match syntax {
                Syntax::Identifier(identifier) => {
                    if identifier.symbol == SymbolTable::LET {
                        return Err(Error::SyntaxError);
                    }
                    self.handler.handle_identifier(identifier)?;
                }
                Syntax::LexicalBinding => self.handler.handle_let_binding(false)?,
                Syntax::LexicalBindingWithInitializer => self.handler.handle_let_binding(true)?,
                _ => self.dispatch(syntax)?,
            }
        }
        self.handler.handle_end_let_declaration()
    }

    fn handle_const_declaration(&mut self) -> Result<(), Error> {
        logger::debug!(?self.queue);
        // TODO: Check uniqueness of identifiers
        self.handler.handle_start_const_declaration()?;
        while let Some(syntax) = self.fetch() {
            match syntax {
                Syntax::Identifier(identifier) => {
                    if identifier.symbol == SymbolTable::LET {
                        return Err(Error::SyntaxError);
                    }
                    self.handler.handle_identifier(identifier)?;
                }
                Syntax::LexicalBinding => return Err(Error::SyntaxError),
                Syntax::LexicalBindingWithInitializer => self.handler.handle_const_binding()?,
                _ => self.dispatch(syntax)?,
            }
        }
        self.handler.handle_end_const_declaration()
    }

    fn handle_lexical_binding(&mut self) -> Result<(), Error> {
        self.push(Syntax::LexicalBinding);
        Ok(())
    }

    fn handle_lexical_binding_with_initializer(&mut self) -> Result<(), Error> {
        self.push(Syntax::LexicalBindingWithInitializer);
        Ok(())
    }

    fn handle_function_declaration(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_function_declaration()
    }

    fn handle_formal_parameter(&mut self) -> Result<(), Error> {
        // TODO: count items
        let identifier = match self.pop() {
            Some(Syntax::Identifier(identifier)) => identifier,
            _ => unreachable!(),
        };
        self.push(Syntax::FormalParameter(identifier.symbol));
        Ok(())
    }

    fn handle_formal_parameter_list_head(&mut self) -> Result<(), Error> {
        let symbol = match self.pop() {
            Some(Syntax::FormalParameter(symbol)) => symbol,
            _ => unreachable!(),
        };
        self.push(Syntax::FormalParameters(vec![symbol]));
        Ok(())
    }

    fn handle_formal_parameter_list_item(&mut self) -> Result<(), Error> {
        let symbol = match self.swap_pop() {
            Some(Syntax::FormalParameter(symbol)) => symbol,
            _ => unreachable!(),
        };
        match self.last_mut() {
            Syntax::FormalParameters(formal_parameters) => formal_parameters.push(symbol),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn handle_formal_parameters_empty(&mut self) -> Result<(), Error> {
        self.push(Syntax::FormalParameters(vec![]));
        Ok(())
    }

    fn handle_function_signature(&mut self) -> Result<(), Error> {
        logger::debug!(?self.queue);

        let formal_parameters = match self.pop() {
            Some(Syntax::FormalParameters(formal_parameters)) => formal_parameters,
            _ => unreachable!(),
        };

        let identifier = match self.pop() {
            Some(Syntax::Identifier(identifier)) => identifier,
            _ => unreachable!(),
        };

        self.handler.handle_function_signature(identifier.symbol, formal_parameters)
    }

    fn handle_function_body(&mut self) -> Result<(), Error> {
        // TODO: early errors
        Ok(())
    }

    fn handle_scope(&mut self) -> Result<(), Error> {
        self.handler.handle_start_scope()
    }

    fn handle_block(&mut self) -> Result<(), Error> {
        self.handler.handle_end_scope()
    }

    fn handle_arguments_empty(&mut self) -> Result<(), Error> {
        self.push(Syntax::ArgumentList(true));
        Ok(())
    }

    fn handle_argument_list_head(&mut self) -> Result<(), Error> {
        self.push(Syntax::ArgumentList(false));
        Ok(())
    }

    fn handle_argument_list_item(&mut self) -> Result<(), Error> {
        self.push(Syntax::ArgumentListItem);
        Ok(())
    }

    fn handle_empty_list(&mut self) -> Result<(), Error> {
        self.push(Syntax::EmptyList);
        Ok(())
    }

    fn handle_list_head(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    fn handle_list_item(&mut self) -> Result<(), Error> {
        // TODO
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        logger::debug!(?self.queue);
        while let Some(syntax) = self.fetch() {
            self.dispatch(syntax)?;
        }
        Ok(())
    }

    fn dispatch(&mut self, syntax: Syntax<'s>) -> Result<(), Error> {
        match syntax {
            Syntax::NumericLiteral(literal) => self.handler.handle_numeric_literal(literal),
            Syntax::StringLiteral(literal) => self.handler.handle_string_literal(literal),
            Syntax::Identifier(identifier) => self.handler.handle_identifier(identifier),
            Syntax::Addition => self.handler.handle_addition_expression(),
            Syntax::Subtraction => self.handler.handle_subtraction_expression(),
            Syntax::Multiplication => self.handler.handle_multiplication_expression(),
            Syntax::Division => self.handler.handle_division_expression(),
            Syntax::Remainder => self.handler.handle_remainder_expression(),
            Syntax::LessThan => self.handler.handle_lt_expression(),
            Syntax::GreaterThan => self.handler.handle_gt_expression(),
            Syntax::LessThanOrEqual => self.handler.handle_lte_expression(),
            Syntax::GreaterThanOrEqual => self.handler.handle_gte_expression(),
            Syntax::Equality => self.handler.handle_eq_expression(),
            Syntax::Inequality => self.handler.handle_ne_expression(),
            Syntax::StrictEquality => self.handler.handle_strict_eq_expression(),
            Syntax::StrictInequality => self.handler.handle_strict_ne_expression(),
            Syntax::Assignment => self.handler.handle_assignment_expression(),
            Syntax::EmptyList => Ok(()),
            Syntax::ArgumentList(empty) => self.handler.handle_argument_list(empty),
            Syntax::ArgumentListItem => self.handler.handle_argument_list_item(),
            _ => unreachable!("{syntax:?}"),
        }
    }
}

enum Action<'s, H> {
    Undefined,
    Nop,
    Invoke(fn(&mut Processor<'s, H>) -> Result<(), Error>, &'static str),
}
