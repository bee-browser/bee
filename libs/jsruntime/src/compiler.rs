use std::ffi::CStr;

use jsparser::Error;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::Symbol;

use crate::bridge;
use crate::bridge::Locator;
use crate::function::FunctionId;
use crate::function::FunctionRegistry;
use crate::logger;
use crate::semantics::Analyzer;
use crate::semantics::CompileCommand;
use crate::Module;
use crate::Runtime;

impl Runtime {
    pub fn compile_script(&mut self, source: &str, optimize: bool) -> Option<Module> {
        let mut analyzer = Analyzer::new(&mut self.symbol_registry, &mut self.function_registry);
        analyzer.use_global_bindings();
        let processor = Processor::new(analyzer, false);
        let program = Parser::for_script(source, processor).parse().ok()?;
        // TODO: Deferring the compilation until it's actually called improves the performance.
        // Because the program may contain unused functions.
        let mut compiler = Compiler::new(&self.function_registry);
        compiler.start_compile();
        compiler.set_data_layout(self.executor.get_data_layout());
        compiler.set_target_triple(self.executor.get_target_triple());
        compiler.set_runtime(self);
        for func in program.functions.iter() {
            compiler.start_function(func.symbol, func.id);
            for command in func.commands.iter() {
                compiler.process_command(command);
            }
            compiler.end_function(optimize);
        }
        compiler.end_compile().ok()
    }
}

struct Compiler<'a> {
    peer: *mut bridge::Compiler,
    function_registry: &'a FunctionRegistry,
}

impl<'a> Compiler<'a> {
    pub fn new(function_registry: &'a FunctionRegistry) -> Self {
        Self {
            peer: unsafe { bridge::compiler_peer_new() },
            function_registry,
        }
    }

    fn start_compile(&self) {
        logger::debug!(event = "start_compile");
        unsafe {
            bridge::compiler_peer_start(self.peer);
        }
    }

    fn end_compile(&self) -> Result<Module, Error> {
        logger::debug!(event = "end_compile");
        let peer = unsafe { bridge::compiler_peer_end(self.peer) };
        Ok(Module { peer })
    }

    fn set_data_layout(&self, data_layout: &CStr) {
        logger::debug!(event = "set_data_layout", ?data_layout);
        unsafe {
            bridge::compiler_peer_set_data_layout(self.peer, data_layout.as_ptr());
        }
    }

    fn set_target_triple(&self, triple: &CStr) {
        logger::debug!(event = "set_target_triple", ?triple);
        unsafe {
            bridge::compiler_peer_set_target_triple(self.peer, triple.as_ptr());
        }
    }

    fn set_runtime(&self, runtime: &Runtime) {
        let runtime = runtime as *const Runtime as usize;
        logger::debug!(event = "set_runtime", ?runtime);
        unsafe {
            bridge::compiler_peer_set_runtime(self.peer, runtime);
        }
    }

    fn start_function(&self, symbol: Symbol, func_id: FunctionId) {
        logger::debug!(event = "start_function", ?symbol, ?func_id);
        let native = self.function_registry.get_native(func_id.value());
        unsafe {
            bridge::compiler_peer_start_function(self.peer, native.name.as_ptr());
        }
    }

    fn end_function(&self, optimize: bool) {
        logger::debug!(event = "end_function", optimize);
        unsafe {
            bridge::compiler_peer_end_function(self.peer, optimize);
        }
    }

    fn process_command(&mut self, command: &CompileCommand) {
        logger::debug!(event = "process_command", ?command);
        match command {
            CompileCommand::Nop => (),
            CompileCommand::Undefined => unsafe {
                bridge::compiler_peer_undefined(self.peer);
            },
            CompileCommand::Null => unsafe {
                bridge::compiler_peer_null(self.peer);
            },
            CompileCommand::Boolean(value) => unsafe {
                bridge::compiler_peer_boolean(self.peer, *value);
            },
            CompileCommand::Number(value) => unsafe {
                bridge::compiler_peer_number(self.peer, *value);
            },
            CompileCommand::String(_value) => {
                unimplemented!("string literal");
            }
            CompileCommand::Function(func_id) => {
                let name = if func_id.is_native() {
                    &self.function_registry.get_native(func_id.value()).name
                } else {
                    &self.function_registry.get_host(func_id.value()).name
                };
                unsafe {
                    bridge::compiler_peer_function(self.peer, (*func_id).into(), name.as_ptr());
                }
            }
            CompileCommand::Reference(symbol, locator) => unsafe {
                assert_ne!(*locator, Locator::NONE);
                bridge::compiler_peer_reference(self.peer, symbol.id(), *locator);
            },
            CompileCommand::Bindings(n) => unsafe {
                bridge::compiler_peer_bindings(self.peer, *n);
            },
            CompileCommand::MutableBinding => unsafe {
                bridge::compiler_peer_declare_mutable(self.peer);
            },
            CompileCommand::ImmutableBinding => unsafe {
                bridge::compiler_peer_declare_immutable(self.peer);
            },
            CompileCommand::DeclareFunction => unsafe {
                bridge::compiler_peer_declare_function(self.peer);
            },
            CompileCommand::Arguments(nargs) => unsafe {
                if *nargs > 0 {
                    bridge::compiler_peer_arguments(self.peer, *nargs);
                }
            },
            CompileCommand::Argument(index) => unsafe {
                bridge::compiler_peer_argument(self.peer, *index);
            },
            CompileCommand::Call(nargs) => unsafe {
                bridge::compiler_peer_call(self.peer, *nargs);
            },
            CompileCommand::AllocateBindings(n, prologue) => {
                debug_assert!(*n > 0);
                unsafe {
                    bridge::compiler_peer_allocate_bindings(self.peer, *n, *prologue);
                }
            }
            CompileCommand::ReleaseBindings(n) => {
                debug_assert!(*n > 0);
                unsafe {
                    // `runtime_pop_scope()` call will not be added if the basic block already has
                    // a terminator instruction.
                    bridge::compiler_peer_release_bindings(self.peer, *n);
                }
            }
            CompileCommand::PostfixIncrement => unsafe {
                bridge::compiler_peer_postfix_increment(self.peer);
            },
            CompileCommand::PostfixDecrement => unsafe {
                bridge::compiler_peer_postfix_decrement(self.peer);
            },
            CompileCommand::PrefixIncrement => unsafe {
                bridge::compiler_peer_prefix_increment(self.peer);
            },
            CompileCommand::PrefixDecrement => unsafe {
                bridge::compiler_peer_prefix_decrement(self.peer);
            },
            CompileCommand::Delete => {
                unimplemented!("delete operator");
            }
            CompileCommand::Void => unsafe {
                bridge::compiler_peer_void(self.peer);
            },
            CompileCommand::Typeof => {
                // TODO: implement String before this
                unimplemented!("typeof operator");
            }
            CompileCommand::UnaryPlus => unsafe {
                bridge::compiler_peer_unary_plus(self.peer);
            },
            CompileCommand::UnaryMinus => unsafe {
                bridge::compiler_peer_unary_minus(self.peer);
            },
            CompileCommand::BitwiseNot => unsafe {
                bridge::compiler_peer_bitwise_not(self.peer);
            },
            CompileCommand::LogicalNot => unsafe {
                bridge::compiler_peer_logical_not(self.peer);
            },
            CompileCommand::Exponentiation => {
                unimplemented!("** operator");
            }
            CompileCommand::Multiplication => unsafe {
                bridge::compiler_peer_multiplication(self.peer);
            },
            CompileCommand::Division => unsafe {
                bridge::compiler_peer_division(self.peer);
            },
            CompileCommand::Remainder => unsafe {
                bridge::compiler_peer_remainder(self.peer);
            },
            CompileCommand::Addition => unsafe {
                bridge::compiler_peer_addition(self.peer);
            },
            CompileCommand::Subtraction => unsafe {
                bridge::compiler_peer_subtraction(self.peer);
            },
            CompileCommand::LeftShift => unsafe {
                bridge::compiler_peer_left_shift(self.peer);
            },
            CompileCommand::SignedRightShift => unsafe {
                bridge::compiler_peer_signed_right_shift(self.peer);
            },
            CompileCommand::UnsignedRightShift => unsafe {
                bridge::compiler_peer_unsigned_right_shift(self.peer);
            },
            CompileCommand::LessThan => unsafe {
                bridge::compiler_peer_less_than(self.peer);
            },
            CompileCommand::GreaterThan => unsafe {
                bridge::compiler_peer_greater_than(self.peer);
            },
            CompileCommand::LessThanOrEqual => unsafe {
                bridge::compiler_peer_less_than_or_equal(self.peer);
            },
            CompileCommand::GreaterThanOrEqual => unsafe {
                bridge::compiler_peer_greater_than_or_equal(self.peer);
            },
            CompileCommand::Instanceof => {
                unimplemented!("instanceof operator");
            }
            CompileCommand::In => {
                unimplemented!("in operator");
            }
            CompileCommand::Equality => unsafe {
                bridge::compiler_peer_equality(self.peer);
            },
            CompileCommand::Inequality => unsafe {
                bridge::compiler_peer_inequality(self.peer);
            },
            CompileCommand::StrictEquality => unsafe {
                bridge::compiler_peer_strict_equality(self.peer);
            },
            CompileCommand::StrictInequality => unsafe {
                bridge::compiler_peer_strict_inequality(self.peer);
            },
            CompileCommand::BitwiseAnd => unsafe {
                bridge::compiler_peer_bitwise_and(self.peer);
            },
            CompileCommand::BitwiseXor => unsafe {
                bridge::compiler_peer_bitwise_xor(self.peer);
            },
            CompileCommand::BitwiseOr => unsafe {
                bridge::compiler_peer_bitwise_or(self.peer);
            },
            CompileCommand::ConditionalTernary => unsafe {
                bridge::compiler_peer_conditional_expression(self.peer);
            },
            CompileCommand::Assignment => unsafe {
                bridge::compiler_peer_assignment(self.peer);
            },
            CompileCommand::ExponentiationAssignment => {
                unimplemented!("**= operator");
            },
            CompileCommand::MultiplicationAssignment => unsafe {
                bridge::compiler_peer_multiplication_assignment(self.peer);
            },
            CompileCommand::DivisionAssignment => unsafe {
                bridge::compiler_peer_division_assignment(self.peer);
            },
            CompileCommand::RemainderAssignment => unsafe {
                bridge::compiler_peer_remainder_assignment(self.peer);
            },
            CompileCommand::AdditionAssignment => unsafe {
                bridge::compiler_peer_addition_assignment(self.peer);
            },
            CompileCommand::SubtractionAssignment => unsafe {
                bridge::compiler_peer_subtraction_assignment(self.peer);
            },
            CompileCommand::LeftShiftAssignment => unsafe {
                bridge::compiler_peer_left_shift_assignment(self.peer);
            },
            CompileCommand::SignedRightShiftAssignment => unsafe {
                bridge::compiler_peer_signed_right_shift_assignment(self.peer);
            },
            CompileCommand::UnsignedRightShiftAssignment => unsafe {
                bridge::compiler_peer_unsigned_right_shift_assignment(self.peer);
            },
            CompileCommand::BitwiseAndAssignment => unsafe {
                bridge::compiler_peer_bitwise_and_assignment(self.peer);
            },
            CompileCommand::BitwiseXorAssignment => unsafe {
                bridge::compiler_peer_bitwise_xor_assignment(self.peer);
            },
            CompileCommand::BitwiseOrAssignment => unsafe {
                bridge::compiler_peer_bitwise_or_assignment(self.peer);
            },
            CompileCommand::LogicalAndAssignment => unsafe {
                bridge::compiler_peer_logical_and_assignment(self.peer);
            },
            CompileCommand::LogicalOrAssignment => unsafe {
                bridge::compiler_peer_logical_or_assignment(self.peer);
            },
            CompileCommand::NullishCoalescingAssignment => {
                unimplemented!("??= operator");
            }
            CompileCommand::Truthy => unsafe {
                bridge::compiler_peer_to_boolean(self.peer);
            },
            CompileCommand::Then => unsafe {
                bridge::compiler_peer_block(self.peer);
            },
            CompileCommand::Else => unsafe {
                bridge::compiler_peer_block(self.peer);
            },
            CompileCommand::IfElseStatement => unsafe {
                bridge::compiler_peer_if_else_statement(self.peer);
            },
            CompileCommand::IfStatement => unsafe {
                bridge::compiler_peer_if_statement(self.peer);
            },
            CompileCommand::Return(n) => unsafe {
                bridge::compiler_peer_return(self.peer, *n as usize);
            },
            CompileCommand::Discard => unsafe {
                bridge::compiler_peer_discard(self.peer);
            },
        }
    }
}

impl<'a> Drop for Compiler<'a> {
    fn drop(&mut self) {
        unsafe {
            bridge::compiler_peer_delete(self.peer);
        }
    }
}
