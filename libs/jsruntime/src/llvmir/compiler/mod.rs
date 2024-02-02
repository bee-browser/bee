mod actions;

use std::collections::VecDeque;
use std::marker::PhantomData;

use jsparser::Location;
use jsparser::ProductionRule;
use jsparser::SyntaxHandler;
use jsparser::Token;
use jsparser::TokenKind;

use super::bridge;
use super::Runtime;
use crate::logger;
use actions::Action;
use actions::ACTIONS;

/// Represents a compilation session of a runtime.
///
/// This type is introduced in order to separate the lifetime management from [`Compiler`].  The
/// [`Compiler`] type is used as a formal parameter type in the [`action::ACTIONS`] table and a
/// build error occurs if the formal parameter type has lifetime parameters.  This separation is
/// needed for avoiding this situation.
pub struct Session<'r> {
    runtime: *mut bridge::Runtime,
    compiler: *mut bridge::Compiler,
    phantom: PhantomData<&'r Runtime>,
}

impl<'r> Drop for Session<'r> {
    fn drop(&mut self) {
        unsafe {
            bridge::runtime_end_compilation(self.runtime, self.compiler);
        }
    }
}

impl<'r> Session<'r> {
    pub fn new(runtime: &'r Runtime) -> Self {
        Self {
            runtime: runtime.0,
            compiler: unsafe { bridge::runtime_start_compilation(runtime.0) },
            phantom: PhantomData,
        }
    }

    pub fn compiler(&self) -> Compiler {
        Compiler::new(self.runtime, self.compiler)
    }
}

pub struct Compiler {
    runtime: *mut bridge::Runtime,
    compiler: *mut bridge::Compiler,
    operations: VecDeque<Operation>,
    location: Location,
}

impl Compiler {
    fn new(runtime: *mut bridge::Runtime, compiler: *mut bridge::Compiler) -> Self {
        Self {
            runtime,
            compiler,
            operations: Default::default(),
            location: Default::default(),
        }
    }

    fn populate_module(&self) {
        unsafe {
            bridge::runtime_populate_module(self.runtime, self.compiler);
        }
    }

    // semantic actions

    // ExpressionStatement -> (?![ASYNC (!LINE_TERMINATOR_SEQUENCE) FUNCTION, CLASS, FUNCTION, LBRACE, LET LBRACK]) Expression_In SEMICOLON
    fn handle_expression_statement(&mut self) -> Result<(), <Self as SyntaxHandler>::Error> {
        self.process_expression_statement()
    }

    // Statement -> ExpressionStatement
    fn handle_statement(&mut self) -> Result<(), <Self as SyntaxHandler>::Error> {
        self.process_statement()
    }
}

pub trait SemanticAction {
    type Artifact;
    type Error;

    fn start(&mut self);
    fn accept(&mut self) -> Result<Self::Artifact, Self::Error>;
    fn process_number_literal(&mut self, value: f64) -> Result<(), Self::Error>;
    fn process_string_literal(&mut self, value: String) -> Result<(), Self::Error>;
    fn process_expression_statement(&mut self) -> Result<(), Self::Error>;
    fn process_statement(&mut self) -> Result<(), Self::Error>;
}

impl SemanticAction for Compiler {
    type Artifact = ();
    type Error = String;

    fn start(&mut self) {
        logger::debug!(event = "semantic.start");
    }

    fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
        logger::debug!(event = "semantic.accept");
        self.populate_module();
        Ok(())
    }

    fn process_number_literal(&mut self, value: f64) -> Result<(), Self::Error> {
        logger::debug!(event = "semantic.process_number_literal", value);
        self.operations.push_back(Operation::PushNumber(value));
        Ok(())
    }

    fn process_string_literal(&mut self, value: String) -> Result<(), Self::Error> {
        logger::debug!(event = "semantic.process_string_literal", value);
        self.operations.push_back(Operation::PushString(value));
        Ok(())
    }

    fn process_expression_statement(&mut self) -> Result<(), Self::Error> {
        logger::debug!(event = "semantic.process_expression_statement");
        self.operations.push_back(Operation::Print);
        Ok(())
    }

    fn process_statement(&mut self) -> Result<(), Self::Error> {
        logger::debug!(event = "semantic.process_statement");
        while let Some(op) = self.operations.pop_front() {
            match op {
                Operation::PushNumber(value) => unsafe {
                    logger::debug!(event = "push_number", value);
                    bridge::compiler_push_number(self.compiler, value);
                },
                Operation::PushString(value) => unsafe {
                    let data = value.as_ptr() as *const i8;
                    bridge::compiler_push_string(self.compiler, data, value.len());
                },
                Operation::Print => unsafe {
                    logger::debug!(event = "print");
                    bridge::compiler_print(self.compiler);
                },
            }
        }
        Ok(())
    }
}

enum Operation {
    PushNumber(f64),
    PushString(String),
    Print,
}

impl SyntaxHandler for Compiler {
    type Artifact = <Self as SemanticAction>::Artifact;
    type Error = <Self as SemanticAction>::Error;

    fn start(&mut self) {
        logger::debug!(event = "syntax.start");
        SemanticAction::start(self)
    }

    fn accept(&mut self) -> Result<Self::Artifact, Self::Error> {
        logger::debug!(event = "syntax.accept");
        SemanticAction::accept(self)
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
                self.process_number_literal(value)
            }
            TokenKind::StringLiteral => {
                // TODO: Perform `SV`
                let value = token.lexeme.to_owned();
                self.process_string_literal(value)
            }
            _ => Ok(()),
        }
    }

    fn reduce(&mut self, rule: ProductionRule) -> Result<(), Self::Error> {
        match ACTIONS[rule.id() as usize] {
            Action::Undefined => {
                logger::error!("No action defined for: {rule}");
                Err(format!("No action defined for: {rule}"))
            }
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
