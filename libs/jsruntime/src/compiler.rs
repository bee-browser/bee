use std::ffi::CStr;

use jsparser::syntax::LoopFlags;
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
            CompileCommand::Exception => unsafe {
                bridge::compiler_peer_exception(self.peer);
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
                bridge::compiler_peer_conditional_ternary(self.peer);
            },
            CompileCommand::Assignment => unsafe {
                bridge::compiler_peer_assignment(self.peer);
            },
            CompileCommand::ExponentiationAssignment => {
                unimplemented!("**= operator");
            }
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
            CompileCommand::Truthy => unsafe {
                bridge::compiler_peer_truthy(self.peer);
            },
            CompileCommand::FalsyShortCircuit => unsafe {
                bridge::compiler_peer_falsy_short_circuit(self.peer);
            },
            CompileCommand::TruthyShortCircuit => unsafe {
                bridge::compiler_peer_truthy_short_circuit(self.peer);
            },
            CompileCommand::NullishShortCircuit => unsafe {
                bridge::compiler_peer_nullish_short_circuit(self.peer);
            },
            CompileCommand::FalsyShortCircuitAssignment => unsafe {
                bridge::compiler_peer_falsy_short_circuit_assignment(self.peer);
            },
            CompileCommand::TruthyShortCircuitAssignment => unsafe {
                bridge::compiler_peer_truthy_short_circuit_assignment(self.peer);
            },
            CompileCommand::NullishShortCircuitAssignment => unsafe {
                bridge::compiler_peer_nullish_short_circuit_assignment(self.peer);
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
            CompileCommand::DoWhileLoop => unsafe {
                bridge::compiler_peer_do_while_loop(self.peer);
            },
            CompileCommand::WhileLoop => unsafe {
                bridge::compiler_peer_while_loop(self.peer);
            },
            // TODO: rewrite using if and break
            CompileCommand::ForLoop(flags) => unsafe {
                bridge::compiler_peer_for_loop(
                    self.peer,
                    flags.contains(LoopFlags::HAS_INIT),
                    flags.contains(LoopFlags::HAS_TEST),
                    flags.contains(LoopFlags::HAS_NEXT),
                );
            },
            CompileCommand::LoopInit => unsafe {
                bridge::compiler_peer_loop_init(self.peer);
            },
            CompileCommand::LoopTest => unsafe {
                bridge::compiler_peer_loop_test(self.peer);
            },
            CompileCommand::LoopNext => unsafe {
                bridge::compiler_peer_loop_next(self.peer);
            },
            CompileCommand::LoopBody => unsafe {
                bridge::compiler_peer_loop_body(self.peer);
            },
            CompileCommand::LoopEnd => unsafe {
                bridge::compiler_peer_loop_end(self.peer);
            },
            CompileCommand::CaseBlock(n) => unsafe {
                debug_assert!(*n > 0);
                // TODO: refactoring
                bridge::compiler_peer_case_block(self.peer, *n);
            },
            CompileCommand::CaseClause(has_statement) => unsafe {
                bridge::compiler_peer_case_clause(self.peer, *has_statement);
            },
            CompileCommand::DefaultClause(has_statement) => unsafe {
                bridge::compiler_peer_default_clause(self.peer, *has_statement);
            },
            CompileCommand::Switch(n, default_index) => unsafe {
                let i = default_index.unwrap_or(*n);
                bridge::compiler_peer_switch(self.peer, *n, i);
            },
            CompileCommand::Try => unsafe {
                bridge::compiler_peer_try(self.peer);
            },
            CompileCommand::Catch(nominal) => unsafe {
                bridge::compiler_peer_catch(self.peer, *nominal);
            },
            CompileCommand::Finally(nominal) => unsafe {
                bridge::compiler_peer_finally(self.peer, *nominal);
            },
            CompileCommand::TryEnd => unsafe {
                bridge::compiler_peer_try_end(self.peer);
            },
            CompileCommand::LabelStart(symbol, is_iteration_statement) => unsafe {
                bridge::compiler_peer_label_start(self.peer, symbol.id(), *is_iteration_statement);
            },
            CompileCommand::LabelEnd(symbol, is_iteration_statement) => unsafe {
                bridge::compiler_peer_label_end(self.peer, symbol.id(), *is_iteration_statement);
            },
            CompileCommand::Continue(symbol) => unsafe {
                bridge::compiler_peer_continue(self.peer, symbol.id());
            },
            CompileCommand::Break(symbol) => unsafe {
                bridge::compiler_peer_break(self.peer, symbol.id());
            },
            CompileCommand::Return(n) => unsafe {
                bridge::compiler_peer_return(self.peer, *n as usize);
            },
            CompileCommand::Throw => unsafe {
                bridge::compiler_peer_throw(self.peer);
            },
            CompileCommand::Discard => unsafe {
                // TODO: the stack should be managed in the Rust side.
                bridge::compiler_peer_discard(self.peer);
            },
            CompileCommand::Swap => unsafe {
                // TODO: the stack should be managed in the Rust side.
                bridge::compiler_peer_swap(self.peer);
            },
        }

        if cfg!(debug_assertions)
            && std::env::var_os("BEE_DEBUG_JSRUNTIME_COMPILER_DUMP_STACK").is_some()
        {
            unsafe {
                bridge::compiler_peer_dump_stack(self.peer);
            }
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
