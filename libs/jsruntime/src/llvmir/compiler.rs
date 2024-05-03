use std::ffi::CStr;

use jsparser::Error;
use jsparser::Parser;
use jsparser::Processor;
use jsparser::Symbol;

use super::bridge;
use super::logger;
use super::FunctionId;
use super::Module;
use super::Runtime;

use crate::semantics::Analyzer;
use crate::semantics::CompileCommand;
use crate::semantics::Locator;

impl Runtime {
    pub fn compile_script(&mut self, source: &str) -> Option<Module> {
        let analyzer = Analyzer::new(&mut self.symbol_registry, &self.function_registry);
        let processor = Processor::new(analyzer, false);
        let program = Parser::for_script(source, processor).parse().ok()?;
        // TODO: Deferring the compilation until it's actually called improves the performance.
        // Because the program may contain unused functions.
        let mut compiler = Compiler::new();
        compiler.start_compile();
        for func in program.functions.iter() {
            let (func_id, func_name) = self
                .function_registry
                .create_native_function(func.formal_parameters.clone());
            compiler.start_function(func.symbol, func_id, func_name);
            for command in func.commands.iter() {
                compiler.process_command(command);
            }
            compiler.end_function();
        }
        compiler.end_compile().ok()
    }
}

struct Compiler {
    peer: *mut bridge::Compiler,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            peer: unsafe { bridge::compiler_peer_new() },
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

    fn start_function(&self, symbol: Symbol, func_id: FunctionId, func_name: &CStr) {
        logger::debug!(event = "start_function", ?symbol, ?func_id, ?func_name);
        unsafe {
            bridge::compiler_peer_start_function(self.peer, func_name.as_ptr());
        }
    }

    fn end_function(&self) {
        logger::debug!(event = "end_function");
        unsafe {
            bridge::compiler_peer_end_function(self.peer);
        }
    }

    fn process_command(&mut self, command: &CompileCommand) {
        logger::debug!(event = "process_command", ?command);
        match command {
            CompileCommand::Nop => (),
            CompileCommand::Undefined => unsafe {
                // TODO
                bridge::compiler_peer_number(self.peer, 0.0);
            },
            CompileCommand::Null => {
                // TODO
            }
            CompileCommand::Boolean(_value) => {
                // TODO
            }
            CompileCommand::Number(value) => unsafe {
                bridge::compiler_peer_number(self.peer, *value);
            },
            CompileCommand::String(_value) => {
                // TODO
            }
            CompileCommand::Function(func_id) => unsafe {
                bridge::compiler_peer_function(self.peer, *func_id);
            },
            CompileCommand::Reference(symbol, locator) => match *locator {
                Locator::None => panic!(),
                Locator::Argument(_nest, index) => unsafe {
                    bridge::compiler_peer_argument_ref(self.peer, symbol.id(), index);
                },
                Locator::Local(nest, index) => unsafe {
                    bridge::compiler_peer_local_ref(self.peer, symbol.id(), nest, index);
                },
            },
            CompileCommand::Bindings(_n) => {
                // TODO
            }
            CompileCommand::MutableBinding => unsafe {
                bridge::compiler_peer_declare_mutable(self.peer);
            },
            CompileCommand::ImmutableBinding => unsafe {
                bridge::compiler_peer_declare_immutable(self.peer);
            },
            CompileCommand::DeclareFunction => unsafe {
                bridge::compiler_peer_declare_function(self.peer);
            },
            CompileCommand::Arguments(_nargs) => (),
            CompileCommand::Argument(_index) => unsafe {
                bridge::compiler_peer_push_argument(self.peer);
            },
            CompileCommand::Call(_nargs) => unsafe {
                bridge::compiler_peer_call(self.peer);
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
            CompileCommand::PostfixIncrement => {
                // TODO
            }
            CompileCommand::PostfixDecrement => {
                // TODO
            }
            CompileCommand::PrefixIncrement => {
                // TODO
            }
            CompileCommand::PrefixDecrement => {
                // TODO
            }
            CompileCommand::Delete => {
                // TODO
            }
            CompileCommand::Void => {
                // TODO
            }
            CompileCommand::Typeof => {
                // TODO
            }
            CompileCommand::Plus => {
                // TODO
            }
            CompileCommand::Negation => {
                // TODO
            }
            CompileCommand::BitwiseNot => {
                // TODO
            }
            CompileCommand::LogicalNot => {
                // TODO
            }
            CompileCommand::Equality => unsafe {
                bridge::compiler_peer_eq(self.peer);
            },
            CompileCommand::Inequality => unsafe {
                bridge::compiler_peer_ne(self.peer);
            },
            CompileCommand::StrictEquality => {
                // TODO: check type
                unsafe {
                    bridge::compiler_peer_eq(self.peer);
                }
            }
            CompileCommand::StrictInequality => {
                // TODO: check type
                unsafe {
                    bridge::compiler_peer_ne(self.peer);
                }
            }
            CompileCommand::LessThan => unsafe {
                bridge::compiler_peer_lt(self.peer);
            },
            CompileCommand::LessThanOrEqual => unsafe {
                bridge::compiler_peer_lte(self.peer);
            },
            CompileCommand::GreaterThan => unsafe {
                bridge::compiler_peer_gt(self.peer);
            },
            CompileCommand::GreaterThanOrEqual => unsafe {
                bridge::compiler_peer_gte(self.peer);
            },
            CompileCommand::LeftShift => {
                // TODO
            }
            CompileCommand::RightShift => {
                // TODO
            }
            CompileCommand::UnsignedRightShift => {
                // TODO
            }
            CompileCommand::Addition => unsafe {
                bridge::compiler_peer_add(self.peer);
            },
            CompileCommand::Subtraction => unsafe {
                bridge::compiler_peer_sub(self.peer);
            },
            CompileCommand::Multiplication => unsafe {
                bridge::compiler_peer_mul(self.peer);
            },
            CompileCommand::Division => unsafe {
                bridge::compiler_peer_div(self.peer);
            },
            CompileCommand::Remainder => unsafe {
                bridge::compiler_peer_rem(self.peer);
            },
            CompileCommand::BitwiseOr => {
                // TODO
            }
            CompileCommand::BitwiseXor => {
                // TODO
            }
            CompileCommand::BitwiseAnd => {
                // TODO
            }
            CompileCommand::In => {
                // TODO
            }
            CompileCommand::Instanceof => {
                // TODO
            }
            CompileCommand::Exponentiation => {
                // TODO
            }
            CompileCommand::LogicalAnd => {
                // TODO
            }
            CompileCommand::LogicalOr => {
                // TODO
            }
            CompileCommand::Nullish => {
                // TODO
            }
            CompileCommand::Assignment => unsafe {
                bridge::compiler_peer_set(self.peer);
            },
            CompileCommand::MultiplicationAssignment => {
                // TODO
            }
            CompileCommand::DivisionAssignment => {
                // TODO
            }
            CompileCommand::RemainderAssignment => {
                // TODO
            }
            CompileCommand::AdditionAssignment => {
                // TODO
            }
            CompileCommand::SubtractionAssignment => {
                // TODO
            }
            CompileCommand::LeftShiftAssignment => {
                // TODO
            }
            CompileCommand::RightShiftAssignment => {
                // TODO
            }
            CompileCommand::UnsignedRightShiftAssignment => {
                // TODO
            }
            CompileCommand::BitwiseAndAssignment => {
                // TODO
            }
            CompileCommand::BitwiseXorAssignment => {
                // TODO
            }
            CompileCommand::BitwiseOrAssignment => {
                // TODO
            }
            CompileCommand::ExponentiationAssignment => {
                // TODO
            }
            CompileCommand::LogicalAndAssignment => {
                // TODO
            }
            CompileCommand::LogicalOrAssignment => {
                // TODO
            }
            CompileCommand::NullishCoalescingAssignment => {
                // TODO
            }
            CompileCommand::Test => unsafe {
                bridge::compiler_peer_to_boolean(self.peer);
            },
            CompileCommand::Then => unsafe {
                bridge::compiler_peer_block(self.peer);
            },
            CompileCommand::Else => unsafe {
                bridge::compiler_peer_block(self.peer);
            },
            CompileCommand::ConditionalTernary => unsafe {
                bridge::compiler_peer_conditional_expression(self.peer);
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
                bridge::compiler_peer_void(self.peer);
            },
        }
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            bridge::compiler_peer_delete(self.peer);
        }
    }
}
