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

impl Runtime {
    pub fn compile_script(&mut self, source: &str) -> Option<Module> {
        let analyzer = Analyzer::new(&mut self.symbol_registry);
        let processor = Processor::new(analyzer, false);
        let program = Parser::for_script(source, processor).parse().ok()?;
        let mut compiler = Compiler::new();
        compiler.start_compile();
        // main
        for command in program.functions[0].commands.iter() {
            compiler.process_command(command);
        }
        // functions
        for func in &program.functions[1..] {
            let (func_id, func_name) = self.create_native_function(func.formal_parameters.clone());
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
    scope_stack: Vec<ScopeState>,
}

#[derive(Default)]
struct ScopeState {
    returned: bool,
}

impl Compiler {
    pub fn new() -> Self {
        let peer = unsafe { bridge::compiler_peer_new() };
        Self {
            peer,
            scope_stack: vec![],
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
        logger::debug!(event = "start_function", ?func_id, ?func_name);
        let name = func_name.as_ptr();
        unsafe {
            bridge::compiler_peer_declare_function(self.peer, symbol.id(), func_id.0);
            bridge::compiler_peer_start_function(self.peer, name);
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
            CompileCommand::Reference(symbol, _locator) => unsafe {
                // TODO: use the locator
                bridge::compiler_peer_symbol(self.peer, symbol.id());
            },
            CompileCommand::Bindings(_n) => {
                // TODO
            }
            CompileCommand::MutableBinding => unsafe {
                bridge::compiler_peer_declare_variable(self.peer);
            },
            CompileCommand::ImmutableBinding => unsafe {
                bridge::compiler_peer_declare_const(self.peer);
            },
            CompileCommand::Arguments(_nargs) => unsafe {
                bridge::compiler_peer_push_args(self.peer);
            },
            CompileCommand::Argument(_index) => unsafe {
                bridge::compiler_peer_push_arg(self.peer);
            },
            CompileCommand::Call(_nargs) => unsafe {
                bridge::compiler_peer_call(self.peer);
            },
            CompileCommand::StartScope(_n) => {
                self.scope_stack.push(Default::default());
                unsafe {
                    bridge::compiler_peer_start_scope(self.peer);
                }
            }
            CompileCommand::EndScope(_n) => {
                if self.scope_stack.last().unwrap().returned {
                    // The scope will be removed from the stack in `llvmir::call()`.
                } else {
                    unsafe {
                        bridge::compiler_peer_end_scope(self.peer);
                    }
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
                bridge::compiler_peer_dump_stack(self.peer);
                bridge::compiler_peer_if_else_statement(self.peer);
            },
            CompileCommand::IfStatement => unsafe {
                bridge::compiler_peer_if_statement(self.peer);
            },
            CompileCommand::Return(n) => {
                self.scope_stack.last_mut().unwrap().returned = true;
                unsafe {
                    bridge::compiler_peer_return(self.peer, *n as usize);
                }
            }
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
