use jsparser::Identifier;
use jsparser::NumericLiteral;
use jsparser::SemanticHandler;
use jsparser::StringLiteral;
use jsparser::Symbol;
use jsparser::SymbolTable;

use super::bridge;
use super::Runtime;
use super::Value;

use crate::logger;

pub struct Compiler<'r> {
    stack: Vec<Item>,
    runtime: &'r mut Runtime,
    imp: *mut bridge::Compiler,
}

enum Item {
    Identifier(Symbol),
}

impl<'r> Compiler<'r> {
    const INITIAL_CAPACITY: usize = 32;

    pub fn new(runtime: &'r mut Runtime) -> Self {
        let imp = unsafe { bridge::runtime_start_compilation(runtime.imp) };
        Self {
            stack: Vec::with_capacity(Self::INITIAL_CAPACITY),
            runtime,
            imp,
        }
    }

    fn populate_module(&self) {
        unsafe {
            bridge::runtime_populate_module(self.runtime.imp, self.imp);
        }
    }
}

impl<'r> Drop for Compiler<'r> {
    fn drop(&mut self) {
        unsafe {
            bridge::runtime_end_compilation(self.runtime.imp, self.imp);
        }
    }
}

impl<'r, 's> SemanticHandler<'s> for Compiler<'r> {
    type Artifact = ();

    fn symbol_table(&mut self) -> &SymbolTable {
        &self.runtime.symbol_table
    }

    fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.runtime.symbol_table
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
            bridge::compiler_number(self.imp, literal.value);
        }
        Ok(())
    }

    fn handle_string_literal(&mut self, literal: StringLiteral<'s>) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_string_literal", literal.raw);
        unsafe {
            // TODO: use utf-16 string
            let data = literal.raw.as_ptr() as *const i8;
            bridge::compiler_string(self.imp, data, literal.raw.len());
        }
        Ok(())
    }

    fn handle_identifier(&mut self, identifier: Identifier) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_identifier", identifier.raw);
        self.stack.push(Item::Identifier(identifier.symbol));
        Ok(())
    }

    fn handle_addition_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_addition_expression");
        unsafe {
            bridge::compiler_add(self.imp);
        }
        Ok(())
    }

    fn handle_subtraction_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_subtraction_expression");
        unsafe {
            bridge::compiler_sub(self.imp);
        }
        Ok(())
    }

    fn handle_multiplication_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_multiplication_expression");
        unsafe {
            bridge::compiler_mul(self.imp);
        }
        Ok(())
    }

    fn handle_division_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_division_expression");
        unsafe {
            bridge::compiler_div(self.imp);
        }
        Ok(())
    }

    fn handle_remainder_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_remainder_expression");
        unsafe {
            bridge::compiler_rem(self.imp);
        }
        Ok(())
    }

    fn handle_lt_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_lt_expression");
        unsafe {
            bridge::compiler_lt(self.imp);
        }
        Ok(())
    }

    fn handle_gt_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_gt_expression");
        unsafe {
            bridge::compiler_gt(self.imp);
        }
        Ok(())
    }

    fn handle_lte_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_lte_expression");
        unsafe {
            bridge::compiler_lte(self.imp);
        }
        Ok(())
    }

    fn handle_gte_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_gte_expression");
        unsafe {
            bridge::compiler_gte(self.imp);
        }
        Ok(())
    }

    fn handle_eq_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_eq_expression");
        unsafe {
            bridge::compiler_eq(self.imp);
        }
        Ok(())
    }

    fn handle_ne_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_ne_expression");
        unsafe {
            bridge::compiler_ne(self.imp);
        }
        Ok(())
    }

    fn handle_strict_eq_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_strict_eq_expression");
        // TODO: check type
        unsafe {
            bridge::compiler_eq(self.imp);
        }
        Ok(())
    }

    fn handle_strict_ne_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_strict_ne_expression");
        // TODO: check type
        unsafe {
            bridge::compiler_ne(self.imp);
        }
        Ok(())
    }

    fn handle_call_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_call_expression");

        let symbol = match self.stack.pop() {
            Some(Item::Identifier(symbol)) => symbol,
            _ => panic!(),
        };

        let func_id = match self.runtime.global_scope.bindings.get(&symbol) {
            Some(Value::Function(func_id)) => func_id,
            _ => panic!(),
        };

        unsafe {
            bridge::compiler_call(self.imp, func_id.0, 0);
        }

        Ok(())
    }

    fn handle_expression_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_expression_statement");
        // TODO
        unsafe {
            bridge::compiler_print(self.imp);
        }
        Ok(())
    }

    fn handle_return_statement(&mut self, n: usize) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_return_statement", n);
        unsafe {
            bridge::compiler_return(self.imp, n);
        }
        Ok(())
    }

    fn handle_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_statement");
        // TODO
        Ok(())
    }

    fn handle_formal_parameters(&mut self, nargs: usize) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_formal_parameters", nargs);
        // TODO
        Ok(())
    }

    fn handle_scope(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_scope");

        let symbol = match self.stack.pop() {
            Some(Item::Identifier(symbol)) => symbol,
            _ => panic!(),
        };

        let func_id = self.runtime.next_func_id();
        self.runtime.global_scope.bindings.insert(symbol, Value::Function(func_id));

        // TODO: Should be kept while the function is alive.
        let func_name = format!("fn{}", func_id.0);
        let data = func_name.as_ptr() as *const i8;
        unsafe {
            bridge::compiler_start_function(self.imp, func_id.0, data, func_name.len());
        }

        Ok(())
    }

    fn handle_function_declaration(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_function_declaration");

        unsafe {
            bridge::compiler_end_function(self.imp);
        }

        Ok(())
    }
}
