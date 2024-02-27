use jsparser::Identifier;
use jsparser::NumericLiteral;
use jsparser::SemanticHandler;
use jsparser::StringLiteral;
use jsparser::Symbol;
use jsparser::SymbolTable;

use super::bridge;
use super::Runtime;

use crate::logger;

pub struct Compiler<'r> {
    runtime: &'r mut Runtime,
    peer: *mut bridge::Compiler,
    scope_stack: Vec<ScopeState>,
}

#[derive(Default)]
struct ScopeState {
    returned: bool,
}

impl<'r> Compiler<'r> {
    pub fn new(runtime: &'r mut Runtime) -> Self {
        let peer = unsafe { bridge::runtime_peer_start_compilation(runtime.peer) };
        Self {
            runtime,
            peer,
            scope_stack: vec![],
        }
    }

    fn populate_module(&self) {
        unsafe {
            bridge::runtime_peer_populate_module(self.runtime.peer, self.peer);
        }
    }
}

impl<'r> Drop for Compiler<'r> {
    fn drop(&mut self) {
        unsafe {
            bridge::runtime_peer_end_compilation(self.runtime.peer, self.peer);
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
        unsafe {
            bridge::compiler_peer_print(self.peer);
        }
        self.populate_module();
        Ok(())
    }

    fn handle_numeric_literal(
        &mut self,
        literal: NumericLiteral<'s>,
    ) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_numeric_literal", literal.value);
        unsafe {
            bridge::compiler_peer_number(self.peer, literal.value);
        }
        Ok(())
    }

    fn handle_string_literal(&mut self, literal: StringLiteral<'s>) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_string_literal", literal.raw);
        unsafe {
            // TODO: use utf-16 string
            let data = literal.raw.as_ptr() as *const i8;
            bridge::compiler_peer_string(self.peer, data, literal.raw.len());
        }
        Ok(())
    }

    fn handle_identifier(&mut self, identifier: Identifier) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_identifier", identifier.raw);
        unsafe {
            bridge::compiler_peer_symbol(self.peer, identifier.symbol.id());
        }
        Ok(())
    }

    fn handle_addition_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_addition_expression");
        unsafe {
            bridge::compiler_peer_add(self.peer);
        }
        Ok(())
    }

    fn handle_subtraction_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_subtraction_expression");
        unsafe {
            bridge::compiler_peer_sub(self.peer);
        }
        Ok(())
    }

    fn handle_multiplication_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_multiplication_expression");
        unsafe {
            bridge::compiler_peer_mul(self.peer);
        }
        Ok(())
    }

    fn handle_division_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_division_expression");
        unsafe {
            bridge::compiler_peer_div(self.peer);
        }
        Ok(())
    }

    fn handle_remainder_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_remainder_expression");
        unsafe {
            bridge::compiler_peer_rem(self.peer);
        }
        Ok(())
    }

    fn handle_lt_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_lt_expression");
        unsafe {
            bridge::compiler_peer_lt(self.peer);
        }
        Ok(())
    }

    fn handle_gt_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_gt_expression");
        unsafe {
            bridge::compiler_peer_gt(self.peer);
        }
        Ok(())
    }

    fn handle_lte_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_lte_expression");
        unsafe {
            bridge::compiler_peer_lte(self.peer);
        }
        Ok(())
    }

    fn handle_gte_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_gte_expression");
        unsafe {
            bridge::compiler_peer_gte(self.peer);
        }
        Ok(())
    }

    fn handle_eq_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_eq_expression");
        unsafe {
            bridge::compiler_peer_eq(self.peer);
        }
        Ok(())
    }

    fn handle_ne_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_ne_expression");
        unsafe {
            bridge::compiler_peer_ne(self.peer);
        }
        Ok(())
    }

    fn handle_strict_eq_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_strict_eq_expression");
        // TODO: check type
        unsafe {
            bridge::compiler_peer_eq(self.peer);
        }
        Ok(())
    }

    fn handle_strict_ne_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_strict_ne_expression");
        // TODO: check type
        unsafe {
            bridge::compiler_peer_ne(self.peer);
        }
        Ok(())
    }

    fn handle_call_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_call_expression");

        unsafe {
            bridge::compiler_peer_call(self.peer, 0);
        }

        Ok(())
    }

    fn handle_assignment_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_assignment_expression");

        unsafe {
            bridge::compiler_peer_set(self.peer);
        }

        Ok(())
    }

    fn handle_then_block(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_then_block");

        unsafe {
            bridge::compiler_peer_to_boolean(self.peer);
            bridge::compiler_peer_block(self.peer);
        }

        Ok(())
    }

    fn handle_else_block(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_else_block");

        unsafe {
            bridge::compiler_peer_block(self.peer);
        }

        Ok(())
    }

    fn handle_conditional_expression(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_conditional_expression");

        unsafe {
            bridge::compiler_peer_conditional_expression(self.peer);
        }

        Ok(())
    }

    fn handle_expression_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_expression_statement");
        // TODO
        Ok(())
    }

    fn handle_return_statement(&mut self, n: usize) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_return_statement", n);
        self.scope_stack.last_mut().unwrap().returned = true;
        unsafe {
            bridge::compiler_peer_return(self.peer, n);
        }
        Ok(())
    }

    fn handle_if_else_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_if_else_statement");

        unsafe {
            bridge::compiler_peer_if_else_statement(self.peer);
        }

        Ok(())
    }

    fn handle_if_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_if_statement");

        unsafe {
            bridge::compiler_peer_if_statement(self.peer);
        }

        Ok(())
    }

    fn handle_statement(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_statement");
        // TODO
        Ok(())
    }

    fn handle_formal_parameters(&mut self, argc: usize) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_formal_parameters", argc);
        // TODO
        Ok(())
    }

    fn handle_function_signature(&mut self, symbol: Symbol) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_function_signature");

        let name = self.runtime.next_func_name();

        unsafe {
            let len = name.len();
            let name = name.as_ptr() as *const i8;
            bridge::compiler_peer_start_function(self.peer, name, len);
        }

        self.runtime.set_function(symbol, name);

        Ok(())
    }

    fn handle_function_declaration(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_function_declaration");

        unsafe {
            bridge::compiler_peer_end_function(self.peer);
        }

        Ok(())
    }

    #[inline(always)]
    fn handle_start_let_declaration(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_start_let_declaration");
        Ok(())
    }

    fn handle_let_binding(&mut self, with_init: bool) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_let_binding");

        if with_init {
            unsafe {
                bridge::compiler_peer_declare(self.peer);
            }
        } else {
            unsafe {
                bridge::compiler_peer_set_undefined(self.peer);
            }
        }

        Ok(())
    }

    #[inline(always)]
    fn handle_end_let_declaration(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_end_let_declaration");
        Ok(())
    }

    #[inline(always)]
    fn handle_start_const_declaration(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_start_const_declaration");
        Ok(())
    }

    fn handle_const_binding(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_const_binding");

        unsafe {
            bridge::compiler_peer_declare(self.peer);
        }

        Ok(())
    }

    #[inline(always)]
    fn handle_end_const_declaration(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_end_const_declaration");
        Ok(())
    }

    fn handle_start_scope(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_start_scope");
        self.scope_stack.push(Default::default());
        unsafe {
            bridge::compiler_peer_start_scope(self.peer);
        }
        Ok(())
    }

    fn handle_end_scope(&mut self) -> Result<(), jsparser::Error> {
        logger::debug!(event = "handle_end_scope");
        if self.scope_stack.last().unwrap().returned {
            // The scope will be removed from the stack in `llvmir::call()`.
        } else {
            unsafe {
                bridge::compiler_peer_end_scope(self.peer);
            }
        }
        Ok(())
    }
}
