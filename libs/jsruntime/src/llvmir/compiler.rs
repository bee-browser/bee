use std::marker::PhantomData;

use jsparser::Identifier;
use jsparser::NumericLiteral;
use jsparser::SemanticHandler;
use jsparser::StringLiteral;
use jsparser::SymbolTable;

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
    symbol_table: SymbolTable,
}

impl Compiler {
    fn new(runtime: *mut bridge::Runtime, compiler: *mut bridge::Compiler) -> Self {
        Self {
            runtime,
            compiler,
            symbol_table: SymbolTable::with_builtin_symbols(),
        }
    }

    fn populate_module(&self) {
        unsafe {
            bridge::runtime_populate_module(self.runtime, self.compiler);
        }
    }
}

impl<'s> SemanticHandler<'s> for Compiler {
    type Artifact = ();

    fn symbol_table(&mut self) -> &SymbolTable {
        &self.symbol_table
    }

    fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    fn start(&mut self) {
        logger::debug!(event = "start");
    }

    fn accept(&mut self) -> Result<Self::Artifact, jsparser::Error> {
        logger::debug!(event = "accept");
        self.populate_module();
        Ok(())
    }

    fn handle_numeric_literal(
        &mut self,
        literal: NumericLiteral<'s>,
    ) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_numeric_literal", literal.value);
        unsafe {
            bridge::compiler_number(self.compiler, literal.value);
        }
        Ok(())
    }

    fn handle_string_literal(&mut self, literal: StringLiteral<'s>) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_string_literal", literal.raw);
        unsafe {
            // TODO: use utf-16 string
            let data = literal.raw.as_ptr() as *const i8;
            bridge::compiler_string(self.compiler, data, literal.raw.len());
        }
        Ok(())
    }

    fn handle_identifier(&mut self, identifier: Identifier) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_identifier", identifier.raw);
        // TODO
        Ok(())
    }

    fn handle_addition_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_addition_expression");
        unsafe {
            bridge::compiler_add(self.compiler);
        }
        Ok(())
    }

    fn handle_subtraction_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_subtraction_expression");
        unsafe {
            bridge::compiler_sub(self.compiler);
        }
        Ok(())
    }

    fn handle_multiplication_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_multiplication_expression");
        unsafe {
            bridge::compiler_mul(self.compiler);
        }
        Ok(())
    }

    fn handle_division_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_division_expression");
        unsafe {
            bridge::compiler_div(self.compiler);
        }
        Ok(())
    }

    fn handle_remainder_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_remainder_expression");
        unsafe {
            bridge::compiler_rem(self.compiler);
        }
        Ok(())
    }

    fn handle_lt_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_lt_expression");
        unsafe {
            bridge::compiler_lt(self.compiler);
        }
        Ok(())
    }

    fn handle_gt_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_gt_expression");
        unsafe {
            bridge::compiler_gt(self.compiler);
        }
        Ok(())
    }

    fn handle_lte_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_lte_expression");
        unsafe {
            bridge::compiler_lte(self.compiler);
        }
        Ok(())
    }

    fn handle_gte_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_gte_expression");
        unsafe {
            bridge::compiler_gte(self.compiler);
        }
        Ok(())
    }

    fn handle_eq_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_eq_expression");
        unsafe {
            bridge::compiler_eq(self.compiler);
        }
        Ok(())
    }

    fn handle_ne_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_ne_expression");
        unsafe {
            bridge::compiler_ne(self.compiler);
        }
        Ok(())
    }

    fn handle_strict_eq_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_strict_eq_expression");
        // TODO: check type
        unsafe {
            bridge::compiler_eq(self.compiler);
        }
        Ok(())
    }

    fn handle_strict_ne_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_strict_ne_expression");
        // TODO: check type
        unsafe {
            bridge::compiler_ne(self.compiler);
        }
        Ok(())
    }

    fn handle_expression_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_expression_statement");
        // TODO
        unsafe {
            bridge::compiler_print(self.compiler);
        }
        Ok(())
    }

    fn handle_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_statement");
        // TODO
        Ok(())
    }
}
