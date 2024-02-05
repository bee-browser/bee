use std::collections::VecDeque;
use std::marker::PhantomData;

use jsparser::SemanticHandler;

use super::bridge;
use super::Runtime;
use crate::logger;

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
    instructions: VecDeque<Instruction>,
}

impl Compiler {
    fn new(runtime: *mut bridge::Runtime, compiler: *mut bridge::Compiler) -> Self {
        Self {
            runtime,
            compiler,
            instructions: Default::default(),
        }
    }

    fn populate_module(&self) {
        unsafe {
            bridge::runtime_populate_module(self.runtime, self.compiler);
        }
    }
}

impl SemanticHandler for Compiler {
    type Artifact = ();

    fn start(&mut self) {
        logger::debug!(event = "start");
    }

    fn accept(&mut self) -> Result<Self::Artifact, jsparser::Error> {
        logger::debug!(event = "accept");
        self.populate_module();
        Ok(())
    }

    fn handle_number_literal(&mut self, value: f64) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_number_literal", value);
        self.instructions.push_back(Instruction::Number(value));
        Ok(())
    }

    fn handle_string_literal(&mut self, value: String) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_string_literal", value);
        self.instructions.push_back(Instruction::String(value));
        Ok(())
    }

    fn handle_addition_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_addition_expression");
        self.instructions.push_back(Instruction::Add);
        Ok(())
    }

    fn handle_subtraction_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_subtraction_expression");
        self.instructions.push_back(Instruction::Sub);
        Ok(())
    }

    fn handle_multiplication_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_multiplication_expression");
        self.instructions.push_back(Instruction::Mul);
        Ok(())
    }

    fn handle_division_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_division_expression");
        self.instructions.push_back(Instruction::Div);
        Ok(())
    }

    fn handle_remainder_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_remainder_expression");
        self.instructions.push_back(Instruction::Rem);
        Ok(())
    }

    fn handle_expression_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_expression_statement");
        self.instructions.push_back(Instruction::Print);
        Ok(())
    }

    fn handle_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_statement");
        while let Some(instruction) = self.instructions.pop_front() {
            logger::debug!(event = "compile", ?instruction);
            match instruction {
                Instruction::Number(value) => unsafe {
                    bridge::compiler_number(self.compiler, value);
                },
                Instruction::String(value) => unsafe {
                    let data = value.as_ptr() as *const i8;
                    bridge::compiler_string(self.compiler, data, value.len());
                },
                Instruction::Add => unsafe {
                    bridge::compiler_add(self.compiler);
                },
                Instruction::Sub => unsafe {
                    bridge::compiler_sub(self.compiler);
                },
                Instruction::Mul => unsafe {
                    bridge::compiler_mul(self.compiler);
                },
                Instruction::Div => unsafe {
                    bridge::compiler_div(self.compiler);
                },
                Instruction::Rem => unsafe {
                    bridge::compiler_rem(self.compiler);
                },
                Instruction::Print => unsafe {
                    bridge::compiler_print(self.compiler);
                },
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Instruction {
    Number(f64),
    String(String),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Print,
}
