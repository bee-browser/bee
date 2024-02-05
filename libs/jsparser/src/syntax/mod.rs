mod actions;
mod logger;

use super::Error;
use super::Location;
use super::ProductionRule;
use super::SyntaxHandler;
use super::Token;
use super::TokenKind;

pub trait SemanticHandler {
    type Artifact;

    fn start(&mut self);
    fn accept(&mut self) -> Result<Self::Artifact, Error>;
    fn handle_number_literal(&mut self, value: f64) -> Result<(), Error>;
    fn handle_string_literal(&mut self, value: String) -> Result<(), Error>;
    fn handle_addition_expression(&mut self) -> Result<(), Error>;
    fn handle_subtraction_expression(&mut self) -> Result<(), Error>;
    fn handle_multiplication_expression(&mut self) -> Result<(), Error>;
    fn handle_division_expression(&mut self) -> Result<(), Error>;
    fn handle_remainder_expression(&mut self) -> Result<(), Error>;
    fn handle_expression_statement(&mut self) -> Result<(), Error>;
    fn handle_statement(&mut self) -> Result<(), Error>;
}

pub struct Processor<H> {
    handler: H,
    location: Location,
    multiplicative_operator: MultiplicativeOperator,
}

impl<H> Processor<H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            location: Default::default(),
            multiplicative_operator: MultiplicativeOperator::Mul,
        }
    }
}

impl<H> SyntaxHandler for Processor<H>
where
    H: SemanticHandler,
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

    fn shift(&mut self, token: &Token<'_>) -> Result<(), Self::Error> {
        logger::debug!(
            event = "shift",
            ?token.kind,
            inserted_automaticaly = token.inserted_automatically(),
            start = %self.location,
            end = %token.compute_end(&self.location),
        );

        match token.kind {
            TokenKind::NumericLiteral => {
                // TODO: Perform `NumericValue`
                let value = token.lexeme.parse::<f64>().unwrap();
                self.handler.handle_number_literal(value)
            }
            TokenKind::StringLiteral => {
                // TODO: Perform `SV`
                let value = token.lexeme.to_owned();
                self.handler.handle_string_literal(value)
            }
            _ => Ok(()),
        }
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

impl<H> Processor<H>
where
    H: SemanticHandler,
{
    // MultiplicativeOperator -> MUL
    fn handle_multiplication_operator(&mut self) -> Result<(), Error> {
        self.multiplicative_operator = MultiplicativeOperator::Mul;
        Ok(())
    }

    // MultiplicativeOperator -> DIV
    fn handle_division_operator(&mut self) -> Result<(), Error> {
        self.multiplicative_operator = MultiplicativeOperator::Div;
        Ok(())
    }

    // MultiplicativeOperator -> MOD
    fn handle_remainder_operator(&mut self) -> Result<(), Error> {
        self.multiplicative_operator = MultiplicativeOperator::Rem;
        Ok(())
    }

    // AdditiveExpression -> AdditiveExpression ADD MultiplicativeExpression
    fn handle_addition_expression(&mut self) -> Result<(), Error> {
        self.handler.handle_addition_expression()
    }

    // AdditiveExpression -> AdditiveExpression SUB MultiplicativeExpression
    fn handle_subtraction_expression(&mut self) -> Result<(), Error> {
        self.handler.handle_subtraction_expression()
    }

    // MultiplicativeExpression -> MultiplicativeExpression MultiplicativeOperator ExponentiationExpression
    fn handle_multiplicative_expression(&mut self) -> Result<(), Error> {
        match self.multiplicative_operator {
            MultiplicativeOperator::Mul => self.handler.handle_multiplication_expression(),
            MultiplicativeOperator::Div => self.handler.handle_division_expression(),
            MultiplicativeOperator::Rem => self.handler.handle_remainder_expression(),
        }
    }

    // ExpressionStatement -> (?![ASYNC (!LINE_TERMINATOR_SEQUENCE) FUNCTION, CLASS, FUNCTION, LBRACE, LET LBRACK]) Expression_In SEMICOLON
    fn handle_expression_statement(&mut self) -> Result<(), Error> {
        self.handler.handle_expression_statement()
    }

    // Statement -> ExpressionStatement
    fn handle_statement(&mut self) -> Result<(), Error> {
        self.handler.handle_statement()
    }
}

enum MultiplicativeOperator {
    Mul,
    Div,
    Rem,
}

enum Action<H> {
    Undefined,
    Nop,
    Invoke(fn(&mut Processor<H>) -> Result<(), Error>, &'static str),
}
