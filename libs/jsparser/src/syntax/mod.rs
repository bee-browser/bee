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
    fn handle_expression_statement(&mut self) -> Result<(), Error>;
    fn handle_return_statement(&mut self, n: usize) -> Result<(), Error>;
    fn handle_statement(&mut self) -> Result<(), Error>;
    fn handle_formal_parameters(&mut self, nargs: usize) -> Result<(), Error>;
    fn handle_scope(&mut self) -> Result<(), Error>;
    fn handle_function_declaration(&mut self) -> Result<(), Error>;
}

pub struct Processor<'s, H> {
    handler: H,
    location: Location,
    queue: VecDeque<Item<'s>>,
    strict_mode: bool,
    module: bool,
}

#[derive(Debug)]
enum Item<'s> {
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
    Cpeaapl,
    CallExpressionOrAsyncArrowHead,
    MaybeArrowFormalParameters,
    MaybeArrowFormalParametersEmpty,
    MaybeArrowFormalParametersRestParameter,
    MaybeArrowFormalParametersRestPattern,
    MaybeArrowFormalParametersWithRestParameter,
    MaybeArrowFormalParametersWithRestPattern,
    FormalParameters(usize),
    EmptyList,
    ListHead,
    ListItem,
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
                self.queue.push_back(Item::NumericLiteral(NumericLiteral {
                    value,
                    raw: token.lexeme,
                }));
            }
            TokenKind::StringLiteral => {
                // TODO: perform `SV`
                let value = token.lexeme.encode_utf16().collect();
                self.queue.push_back(Item::StringLiteral(StringLiteral {
                    value,
                    raw: token.lexeme,
                }));
            }
            TokenKind::IdentifierName => {
                // TODO: perform `StringValue`
                let value = token.lexeme.encode_utf16().collect();
                let symbol_table = self.handler.symbol_table_mut();
                let symbol = symbol_table.intern(value);
                self.queue.push_back(Item::Identifier(Identifier {
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
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
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
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
        };
        match identifier.symbol {
            SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_yield(&mut self) -> Result<(), Error> {
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
        };
        match identifier.symbol {
            SymbolTable::YIELD => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_yield_or_await(&mut self) -> Result<(), Error> {
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
        };
        match identifier.symbol {
            SymbolTable::YIELD | SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn syntax_error_if_arguments_or_eval(&mut self) -> Result<(), Error> {
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
        };
        match identifier.symbol {
            SymbolTable::ARGUMENTS | SymbolTable::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            _ => Ok(()),
        }
    }

    fn syntax_error_if_arguments_or_eval_or_await(&mut self) -> Result<(), Error> {
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
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
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
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
        let identifier = match self.queue.back().unwrap() {
            Item::Identifier(identifier) => identifier,
            _ => panic!(),
        };
        match identifier.symbol {
            SymbolTable::ARGUMENTS | SymbolTable::EVAL if self.strict_mode => {
                Err(Error::SyntaxError)
            }
            SymbolTable::YIELD | SymbolTable::AWAIT => Err(Error::SyntaxError),
            _ => Ok(()),
        }
    }

    fn handle_addition_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Addition);
        Ok(())
    }

    fn handle_subtraction_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Subtraction);
        Ok(())
    }

    fn handle_multiplication_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Multiplication);
        Ok(())
    }

    fn handle_division_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Division);
        Ok(())
    }

    fn handle_remainder_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Remainder);
        Ok(())
    }

    fn handle_lt_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::LessThan);
        Ok(())
    }

    fn handle_gt_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::GreaterThan);
        Ok(())
    }

    fn handle_lte_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::LessThanOrEqual);
        Ok(())
    }

    fn handle_gte_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::GreaterThanOrEqual);
        Ok(())
    }

    fn handle_eq_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Equality);
        Ok(())
    }

    fn handle_ne_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Inequality);
        Ok(())
    }

    fn handle_strict_eq_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::StrictEquality);
        Ok(())
    }

    fn handle_strict_ne_expression(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::StrictInequality);
        Ok(())
    }

    fn handle_cpeaapl(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::Cpeaapl);
        Ok(())
    }

    fn handle_call_expression(&mut self) -> Result<(), Error> {
        self.queue.pop_back();
        self.flush()?;
        self.handler.handle_call_expression()
    }

    fn handle_call_expression_or_async_arrow_head(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::CallExpressionOrAsyncArrowHead);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::MaybeArrowFormalParameters);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters_empty(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::MaybeArrowFormalParametersEmpty);
        Ok(())
    }

    fn handle_maybe_arrow_formal_rest_parameter(&mut self) -> Result<(), Error> {
        self.queue
            .push_back(Item::MaybeArrowFormalParametersRestParameter);
        Ok(())
    }

    fn handle_maybe_arrow_formal_rest_pattern(&mut self) -> Result<(), Error> {
        self.queue
            .push_back(Item::MaybeArrowFormalParametersRestPattern);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters_with_rest_parameter(&mut self) -> Result<(), Error> {
        self.queue
            .push_back(Item::MaybeArrowFormalParametersWithRestParameter);
        Ok(())
    }

    fn handle_maybe_arrow_formal_parameters_with_rest_pattern(&mut self) -> Result<(), Error> {
        self.queue
            .push_back(Item::MaybeArrowFormalParametersWithRestPattern);
        Ok(())
    }

    fn handle_group_expression(&mut self) -> Result<(), Error> {
        debug_assert!(matches!(self.queue.back(), Some(Item::Cpeaapl)));
        self.queue.pop_back();
        self.flush()
    }

    fn handle_expression_statement(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_expression_statement()
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

    fn handle_function_declaration(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_function_declaration()
    }

    fn handle_formal_parameters_empty(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::FormalParameters(0));
        Ok(())
    }

    fn handle_scope(&mut self) -> Result<(), Error> {
        self.flush()?;
        self.handler.handle_scope()
    }

    fn handle_function_body(&mut self) -> Result<(), Error> {
        // TODO: early errors
        Ok(())
    }

    fn handle_empty_list(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::EmptyList);
        Ok(())
    }

    fn handle_list_head(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::ListHead);
        Ok(())
    }

    fn handle_list_item(&mut self) -> Result<(), Error> {
        self.queue.push_back(Item::ListItem);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        logger::debug!(?self.queue);
        while let Some(item) = self.queue.pop_front() {
            match item {
                Item::NumericLiteral(literal) => self.handler.handle_numeric_literal(literal)?,
                Item::StringLiteral(literal) => self.handler.handle_string_literal(literal)?,
                Item::Identifier(identifier) => self.handler.handle_identifier(identifier)?,
                Item::Addition => self.handler.handle_addition_expression()?,
                Item::Subtraction => self.handler.handle_subtraction_expression()?,
                Item::Multiplication => self.handler.handle_multiplication_expression()?,
                Item::Division => self.handler.handle_division_expression()?,
                Item::Remainder => self.handler.handle_remainder_expression()?,
                Item::LessThan => self.handler.handle_lt_expression()?,
                Item::GreaterThan => self.handler.handle_gt_expression()?,
                Item::LessThanOrEqual => self.handler.handle_lte_expression()?,
                Item::GreaterThanOrEqual => self.handler.handle_gte_expression()?,
                Item::Equality => self.handler.handle_eq_expression()?,
                Item::Inequality => self.handler.handle_ne_expression()?,
                Item::StrictEquality => self.handler.handle_strict_eq_expression()?,
                Item::StrictInequality => self.handler.handle_strict_ne_expression()?,
                Item::FormalParameters(nargs) => self.handler.handle_formal_parameters(nargs)?,
                Item::EmptyList => (),
                Item::ListHead => (),
                _ => unreachable!("{item:?}"),
            }
        }
        Ok(())
    }
}

enum Action<'s, H> {
    Undefined,
    Nop,
    Invoke(fn(&mut Processor<'s, H>) -> Result<(), Error>, &'static str),
}
