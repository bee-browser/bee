use std::ffi::CStr;
use std::io::Write;

use jsparser::syntax::LoopFlags;
use jsparser::Symbol;

use crate::function::FunctionId;
use crate::function::FunctionRegistry;
use crate::logger;
use crate::semantics::CompileCommand;
use crate::semantics::ScopeRef;
use crate::semantics::ScopeTree;
use crate::Program;
use crate::Runtime;

use super::bridge;
use super::control_flow::ControlFlowStack;
use super::Locator;
use super::Module;

impl<X> Runtime<X> {
    pub fn compile(&mut self, program: &Program, optimize: bool) -> Result<Module, CompileError> {
        logger::debug!(event = "compile");
        // TODO: Deferring the compilation until it's actually called improves the performance.
        // Because the program may contain unused functions.
        let mut compiler = Compiler::new(&self.function_registry, &program.scope_tree);
        compiler.start_compile(self.pref.enable_llvmir_labels);
        compiler.set_data_layout(self.executor.get_data_layout());
        compiler.set_target_triple(self.executor.get_target_triple());
        for func in program.functions.iter() {
            compiler.start_function(func.symbol, func.id);
            for command in func.commands.iter() {
                compiler.process_command(command);
            }
            compiler.end_function(optimize);
        }
        Ok(compiler.end_compile())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompileError {}

struct BasicBlockNameStack {
    buffer: Vec<u8>,
    index_stack: Vec<usize>,
}

impl BasicBlockNameStack {
    fn new() -> Self {
        let mut buffer = Vec::with_capacity(1024);
        write!(&mut buffer, "bb").unwrap();
        Self {
            buffer,
            index_stack: Vec::with_capacity(64),
        }
    }

    fn as_name(&self) -> (*const std::ffi::c_char, usize) {
        (self.buffer.as_ptr() as *const std::ffi::c_char, self.buffer.len())
    }

    fn push(&mut self, name: &str) {
        let index = self.buffer.len();
        write!(&mut self.buffer, ".{}", name).unwrap();
        self.index_stack.push(index);
    }

    fn push_with_id<T: std::fmt::Display>(&mut self, name: &str, id: T) {
        let index = self.buffer.len();
        write!(&mut self.buffer, ".{}", name).unwrap();
        write!(&mut self.buffer, ".{}", id).unwrap();
        self.index_stack.push(index);
    }

    fn pop(&mut self) {
        let index = self.index_stack.pop().unwrap();
        self.buffer.truncate(index);
    }
}

impl Default for BasicBlockNameStack {
    fn default() -> Self {
        Self::new()
    }
}

struct Compiler<'r, 's> {
    peer: *mut bridge::Compiler,
    function_registry: &'r FunctionRegistry,
    scope_tree: &'s ScopeTree,
    control_flow_stack: ControlFlowStack,
    basic_block_name_stack: Option<BasicBlockNameStack>,
}

macro_rules! push_bb_name {
    ($compiler:expr, $name:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.push($name);
        }
    };
    ($compiler:expr, $name:expr, $id:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.push_with_id($name, $id);
        }
    };
}

macro_rules! pop_bb_name {
    ($compiler:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.pop();
        }
    };
}

macro_rules! bb_name {
    ($compiler:expr) => {
        if let Some(ref mut stack) = $compiler.basic_block_name_stack {
            stack.as_name()
        } else {
            (c"".as_ptr(), 0)
        }
    };
}

impl<'r, 's> Compiler<'r, 's> {
    pub fn new(function_registry: &'r FunctionRegistry, scope_tree: &'s ScopeTree) -> Self {
        Self {
            peer: unsafe { bridge::compiler_peer_new() },
            function_registry,
            scope_tree,
            control_flow_stack: Default::default(),
            basic_block_name_stack: None,
        }
    }

    fn start_compile(&mut self, enable_labels: bool) {
        logger::debug!(event = "start_compile", enable_labels);
        if enable_labels {
            self.basic_block_name_stack = Some(Default::default());
        }
        unsafe {
            bridge::compiler_peer_start(self.peer, enable_labels);
        }
    }

    fn end_compile(&self) -> Module {
        logger::debug!(event = "end_compile");
        let peer = unsafe { bridge::compiler_peer_end(self.peer) };
        Module { peer }
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

    fn start_function(&mut self, symbol: Symbol, func_id: FunctionId) {
        logger::debug!(event = "start_function", ?symbol, ?func_id);
        let native = self.function_registry.get_native(func_id);
        unsafe {
            bridge::compiler_peer_start_function(self.peer, native.name.as_ptr());
        }

        let locals_block = unsafe {
            bridge::compiler_peer_get_locals_block(self.peer)
        };
        let args_block = unsafe {
            bridge::compiler_peer_get_args_block(self.peer)
        };
        let body_block = unsafe {
            bridge::compiler_peer_get_body_block(self.peer)
        };
        let return_block = unsafe {
            bridge::compiler_peer_get_return_block(self.peer)
        };
        self.control_flow_stack.push_function_flow(locals_block, args_block, body_block, return_block);
    }

    fn end_function(&mut self, optimize: bool) {
        logger::debug!(event = "end_function", optimize);
        self.control_flow_stack.pop_function_flow();
        unsafe {
            bridge::compiler_peer_end_function(self.peer, optimize);
        }
        debug_assert!(self.control_flow_stack.is_empty());
        self.control_flow_stack.clear();
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
                let func_id = *func_id;
                let name = if func_id.is_native() {
                    &self.function_registry.get_native(func_id).name
                } else {
                    &self.function_registry.get_host(func_id).name
                };
                unsafe {
                    bridge::compiler_peer_function(self.peer, func_id.into(), name.as_ptr());
                }
            }
            CompileCommand::Closure(prologue, num_captures) => {
                let block = if *prologue {
                    self.control_flow_stack.scope_flow().hoisted_block
                } else {
                    0
                };
                unsafe {
                    // `*num_captures` may be 0.
                    bridge::compiler_peer_closure(self.peer, block, *num_captures);
                }
            }
            CompileCommand::Reference(symbol, locator) => unsafe {
                debug_assert_ne!(*locator, Locator::NONE);
                bridge::compiler_peer_reference(self.peer, symbol.id(), *locator);
            },
            CompileCommand::Exception => unsafe {
                bridge::compiler_peer_exception(self.peer);
            },
            CompileCommand::AllocateLocals(num_locals) => unsafe {
                debug_assert!(*num_locals > 0);
                bridge::compiler_peer_allocate_locals(self.peer, *num_locals);
            },
            CompileCommand::MutableBinding => unsafe {
                bridge::compiler_peer_declare_mutable(self.peer);
            },
            CompileCommand::ImmutableBinding => unsafe {
                bridge::compiler_peer_declare_immutable(self.peer);
            },
            CompileCommand::DeclareFunction => {
                let block = self.control_flow_stack.scope_flow().hoisted_block;
                unsafe {
                    bridge::compiler_peer_declare_function(self.peer, block);
                }
            }
            CompileCommand::DeclareClosure => {
                let block = self.control_flow_stack.scope_flow().hoisted_block;
                unsafe {
                    bridge::compiler_peer_declare_closure(self.peer, block);
                }
            }
            CompileCommand::Arguments(nargs) => unsafe {
                if *nargs > 0 {
                    bridge::compiler_peer_arguments(self.peer, *nargs);
                }
            },
            CompileCommand::Argument(index) => unsafe {
                bridge::compiler_peer_argument(self.peer, *index);
            },
            CompileCommand::Call(nargs) => {
                let block = self.control_flow_stack.exception_block();
                unsafe {
                    bridge::compiler_peer_call(self.peer, *nargs, block);
                }
                // The function may throw an exception.
                self.control_flow_stack.set_thrown();
            }
            CompileCommand::PushScope(scope_ref) => {
                // TODO(issue#234)
                let scope_ref = *scope_ref;
                debug_assert_ne!(scope_ref, ScopeRef::NONE);
                self.start_scope(scope_ref);
                let scope = self.scope_tree.scope(scope_ref);
                for binding in scope.bindings.iter() {
                    if binding.is_local() {
                        let flow = self.control_flow_stack.scope_flow();
                        unsafe {
                            bridge::compiler_peer_init_local(self.peer, binding.locator(), flow.init_block);
                        }
                    }
                    if binding.captured {
                        let flow = self.control_flow_stack.scope_flow();
                        unsafe {
                            bridge::compiler_peer_create_capture(self.peer, binding.locator(), flow.init_block);
                        }
                    }
                }
            }
            CompileCommand::PopScope(scope_ref) => {
                // TODO(issue#234)
                let scope_ref = *scope_ref;
                debug_assert_ne!(scope_ref, ScopeRef::NONE);
                let scope = self.scope_tree.scope(scope_ref);
                for binding in scope.bindings.iter() {
                    if binding.captured {
                        let block = self.control_flow_stack.scope_flow().cleanup_block;
                        unsafe {
                            bridge::compiler_peer_escape_variable(self.peer, binding.locator(), block);
                        }
                    }
                    if binding.is_local() {
                        unsafe {
                            bridge::compiler_peer_tidy_local(self.peer, binding.locator());
                        }
                    }
                }
                self.end_scope(scope_ref);
            }
            CompileCommand::CaptureVariable(declaration) => {
                let block = if *declaration {
                    self.control_flow_stack.scope_flow().hoisted_block
                } else {
                    0
                };
                unsafe {
                    bridge::compiler_peer_capture_variable(self.peer, block);
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
            CompileCommand::Ternary => {
                let else_branch = self.control_flow_stack.pop_branch_flow();
                let then_branch = self.control_flow_stack.pop_branch_flow();
                unsafe {
                    bridge::compiler_peer_ternary(self.peer, then_branch.before_block, then_branch.after_block, else_branch.before_block, else_branch.after_block);
                }
            }
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
            CompileCommand::FalsyShortCircuit => {
                unsafe {
                    bridge::compiler_peer_falsy_short_circuit(self.peer);
                }
                self.branch(); // then
                self.branch(); // else
            }
            CompileCommand::TruthyShortCircuit => {
                unsafe {
                    bridge::compiler_peer_truthy_short_circuit(self.peer);
                }
                self.branch(); // then
                self.branch(); // else
            }
            CompileCommand::NullishShortCircuit => {
                unsafe {
                    bridge::compiler_peer_nullish_short_circuit(self.peer);
                }
                self.branch(); // then
                self.branch(); // else
            }
            CompileCommand::FalsyShortCircuitAssignment => {
                unsafe {
                    bridge::compiler_peer_falsy_short_circuit_assignment(self.peer);
                }
                self.branch(); // then
                self.branch(); // else
            }
            CompileCommand::TruthyShortCircuitAssignment => {
                unsafe {
                    bridge::compiler_peer_truthy_short_circuit_assignment(self.peer);
                }
                self.branch(); // then
                self.branch(); // else
            }
            CompileCommand::NullishShortCircuitAssignment => {
                unsafe {
                    bridge::compiler_peer_nullish_short_circuit_assignment(self.peer);
                }
                self.branch(); // then
                self.branch(); // else
            }
            CompileCommand::Then => self.branch(),
            CompileCommand::Else => self.branch(),
            CompileCommand::IfElseStatement => {
                let else_branch = self.control_flow_stack.pop_branch_flow();
                let then_branch = self.control_flow_stack.pop_branch_flow();
                unsafe {
                    bridge::compiler_peer_if_else_statement(self.peer, then_branch.before_block, then_branch.after_block, else_branch.before_block, else_branch.after_block);
                }
            }
            CompileCommand::IfStatement => {
                let branch = self.control_flow_stack.pop_branch_flow();
                unsafe {
                    bridge::compiler_peer_if_statement(self.peer, branch.before_block, branch.after_block);
                }
            }
            CompileCommand::DoWhileLoop(id) => {
                push_bb_name!(self, "do-while", id);

                let loop_body = self.create_basic_block("loop-body");
                let loop_test = self.create_basic_block("loop-test");
                let loop_end = self.create_basic_block("loop-end");
                let loop_start = loop_body;
                let loop_continue = loop_test;
                let loop_break = loop_end;
                self.control_flow_stack.push_loop_test_flow(loop_body, loop_end, loop_end);
                self.control_flow_stack.push_loop_body_flow(loop_test, loop_test);
                self.control_flow_stack.set_continue_target(loop_continue);
                self.control_flow_stack.push_break_target(loop_break, Symbol::NONE);
                self.control_flow_stack.push_continue_target(loop_continue, Symbol::NONE);
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, loop_start);
                    bridge::compiler_peer_set_basic_block(self.peer, loop_start);
                }
            }
            CompileCommand::WhileLoop(id) => {
                push_bb_name!(self, "while", id);

                let loop_test = self.create_basic_block("loop-test");
                let loop_body = self.create_basic_block("loop-body");
                let loop_end = self.create_basic_block("loop-end");

                let loop_start = loop_test;
                let loop_continue = loop_test;
                let loop_break = loop_end;

                self.control_flow_stack.push_loop_body_flow(loop_test, loop_end);
                self.control_flow_stack.push_loop_test_flow(loop_body, loop_end, loop_body);
                self.control_flow_stack.set_continue_target(loop_continue);
                self.control_flow_stack.push_break_target(loop_break, Symbol::NONE);
                self.control_flow_stack.push_continue_target(loop_continue, Symbol::NONE);
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, loop_start);
                    bridge::compiler_peer_set_basic_block(self.peer, loop_start);
                }
            }
            // TODO: rewrite using if and break
            CompileCommand::ForLoop(id, flags) => {
                push_bb_name!(self, "for", id);

                let has_init = flags.contains(LoopFlags::HAS_INIT);
                let has_test = flags.contains(LoopFlags::HAS_TEST);
                let has_next = flags.contains(LoopFlags::HAS_NEXT);
                let loop_init = if has_init {
                    self.create_basic_block("loop-init")
                } else {
                    0
                };
                let loop_test = if has_test {
                    self.create_basic_block("loop-test")
                } else {
                    0
                };
                let loop_body = self.create_basic_block("loop-body");
                let loop_next = if has_next {
                    self.create_basic_block("loop-next")
                } else {
                    0
                };
                let loop_end = self.create_basic_block("loop-end");

                let mut loop_start = loop_body;
                let mut loop_continue = loop_body;
                let loop_break = loop_end;
                let mut insert_point = loop_body;

                if has_next {
                    self.control_flow_stack.push_loop_body_flow(loop_next, loop_end);
                } else if has_test {
                    self.control_flow_stack.push_loop_body_flow(loop_test, loop_end);
                } else {
                    self.control_flow_stack.push_loop_body_flow(loop_body, loop_end);
                }

                if has_next {
                    if has_test {
                        self.control_flow_stack.push_loop_next_flow(loop_test, loop_body);
                    } else {
                        self.control_flow_stack.push_loop_next_flow(loop_body, loop_body);
                    }
                    loop_continue = loop_next;
                    insert_point = loop_next;
                }

                if has_test {
                    if has_next {
                        self.control_flow_stack.push_loop_test_flow(loop_body, loop_end, loop_next);
                    } else {
                        self.control_flow_stack.push_loop_test_flow(loop_body, loop_end, loop_body);
                    }
                    loop_start = loop_test;
                    if !has_next {
                        loop_continue = loop_test;
                    }
                    insert_point = loop_test;
                }

                if has_init {
                    if has_test {
                        self.control_flow_stack.push_loop_init_flow(loop_test, loop_test);
                    } else if has_next {
                        self.control_flow_stack.push_loop_init_flow(loop_body, loop_next);
                    } else {
                        self.control_flow_stack.push_loop_init_flow(loop_body, loop_body);
                    }
                    loop_start = loop_init;
                    insert_point = loop_init;
                }
                self.control_flow_stack.set_continue_target(loop_continue);
                self.control_flow_stack.push_break_target(loop_break, Symbol::NONE);
                self.control_flow_stack.push_continue_target(loop_continue, Symbol::NONE);
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, loop_start);
                    bridge::compiler_peer_set_basic_block(self.peer, insert_point);
                }
            }
            CompileCommand::LoopInit => {
                let loop_init = self.control_flow_stack.pop_loop_init_flow();
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, loop_init.branch_block);
                    bridge::compiler_peer_set_basic_block(self.peer, loop_init.insert_point);
                }
            }
            CompileCommand::LoopTest => {
                let loop_test = self.control_flow_stack.pop_loop_test_flow();
                unsafe {
                    // TODO: refactoring
                    bridge::compiler_peer_loop_test(self.peer, loop_test.then_block, loop_test.else_block, loop_test.insert_point);
                }
            }
            CompileCommand::LoopNext => {
                let loop_next = self.control_flow_stack.pop_loop_next_flow();
                unsafe {
                    // Discard the evaluation result.
                    bridge::compiler_peer_discard(self.peer);
                    bridge::compiler_peer_create_br(self.peer, loop_next.branch_block);
                    bridge::compiler_peer_set_basic_block(self.peer, loop_next.insert_point);
                }
            }
            CompileCommand::LoopBody => {
                let loop_body = self.control_flow_stack.pop_loop_body_flow();
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, loop_body.branch_block);
                    bridge::compiler_peer_move_basic_block_after(self.peer, loop_body.insert_point);
                    bridge::compiler_peer_set_basic_block(self.peer, loop_body.insert_point);
                }
            }
            CompileCommand::LoopEnd => {
                pop_bb_name!(self);

                self.control_flow_stack.pop_break_target();
                self.control_flow_stack.pop_continue_target();
            }
            CompileCommand::CaseBlock(id, num_cases) => {
                push_bb_name!(self, "switch", id);
                unsafe {
                    debug_assert!(*num_cases > 0);
                    // TODO: refactoring
                    bridge::compiler_peer_case_block(self.peer, *id, *num_cases);
                }
                let end_block = self.create_basic_block("end");
                self.control_flow_stack.push_switch_flow(end_block);
                self.control_flow_stack.push_break_target(end_block, Symbol::NONE);
            }
            CompileCommand::CaseClause(has_statement) => {
                let branch = self.control_flow_stack.pop_branch_flow();
                let case_end_block = unsafe {
                    bridge::compiler_peer_get_basic_block(self.peer)
                };
                unsafe {
                    bridge::compiler_peer_case_clause(self.peer, *has_statement, branch.before_block, branch.after_block);
                }
                self.control_flow_stack.push_case_banch_flow(case_end_block, branch.after_block);
            }
            CompileCommand::DefaultClause(has_statement) => {
                let branch = self.control_flow_stack.pop_branch_flow();
                let case_end_block = unsafe {
                    bridge::compiler_peer_get_basic_block(self.peer)
                };
                unsafe {
                    bridge::compiler_peer_default_clause(self.peer, *has_statement, branch.before_block);
                }
                self.control_flow_stack.push_case_banch_flow(case_end_block, branch.after_block);
                self.control_flow_stack.set_default_case_block(branch.after_block);
            }
            CompileCommand::Switch(_id, num_cases, _default_index) => {
                let num_cases = *num_cases;

                pop_bb_name!(self);

                self.control_flow_stack.pop_break_target();
                let case_block = unsafe {
                    bridge::compiler_peer_get_basic_block(self.peer)
                };

                // Discard the switch-values
                unsafe {
                    bridge::compiler_peer_discard(self.peer);
                    bridge::compiler_peer_discard(self.peer);
                }

                // Connect the last basic blocks of each case/default clause to the first basic block of the
                // statement lists of the next case/default clause if it's not terminated.
                //
                // The last basic blocks has been stored in the control flow stack in reverse order.
                let mut fall_through_block = self.control_flow_stack.switch_flow().end_block;
                for _ in 0..num_cases {
                    let case_branch = self.control_flow_stack.pop_case_branch_flow();
                    let terminated = unsafe {
                        bridge::compiler_peer_is_basic_block_terminated(self.peer, case_branch.before_block)
                    };
                    if !terminated {
                        unsafe {
                            bridge::compiler_peer_set_basic_block(self.peer, case_branch.before_block);
                            bridge::compiler_peer_create_br(self.peer, fall_through_block);
                            bridge::compiler_peer_move_basic_block_after(self.peer, fall_through_block);
                        }
                    }
                    fall_through_block = case_branch.after_block;
                }

                let switch = self.control_flow_stack.pop_switch_flow();

                // Create an unconditional jump to the statement of the default clause if it
                // exists.  Otherwise, jump to the end block.
                unsafe {
                    bridge::compiler_peer_set_basic_block(self.peer, case_block);
                    bridge::compiler_peer_create_br(self.peer, if switch.default_block != 0 {
                        switch.default_block
                    } else {
                        switch.end_block
                    });
                }

                unsafe {
                    bridge::compiler_peer_move_basic_block_after(self.peer, switch.end_block);
                }
                unsafe {
                    bridge::compiler_peer_set_basic_block(self.peer, switch.end_block);
                }
            }
            CompileCommand::Try => {
                let try_block = self.create_basic_block("try");
                let catch_block = self.create_basic_block("catch");
                let finally_block = self.create_basic_block("finally");
                let end_block = self.create_basic_block("try-end");
                self.control_flow_stack.push_exception_flow(try_block, catch_block, finally_block, end_block);
                unsafe {
                    // Jump from the end of previous block to the beginning of the try block.
                    bridge::compiler_peer_create_br(self.peer, try_block);

                    bridge::compiler_peer_set_basic_block(self.peer, try_block);
                }
                push_bb_name!(self, "try");
            }
            CompileCommand::Catch(nominal) => {
                let nominal = *nominal;

                pop_bb_name!(self);

                self.control_flow_stack.set_in_catch(nominal);
                let flow = self.control_flow_stack.exception_flow();

                // Jump from the end of the try block to the beginning of the finally block.
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, flow.finally_block);
                    bridge::compiler_peer_move_basic_block_after(self.peer, flow.finally_block);

                    bridge::compiler_peer_move_basic_block_after(self.peer, flow.catch_block);
                    bridge::compiler_peer_set_basic_block(self.peer, flow.catch_block);

                    if !nominal {
                        // TODO: Reset the status to Status::Normal.
                        bridge::compiler_peer_create_store_normal_status(self.peer);
                    }
                }

                push_bb_name!(self, "catch");
            }
            CompileCommand::Finally(_nominal) => {
                pop_bb_name!(self);

                self.control_flow_stack.set_in_finally();
                let flow = self.control_flow_stack.exception_flow();

                unsafe {
                    // Jump from the end of the catch block to the beginning of the finally block.
                    bridge::compiler_peer_create_br(self.peer, flow.finally_block);

                    bridge::compiler_peer_move_basic_block_after(self.peer, flow.finally_block);
                    bridge::compiler_peer_set_basic_block(self.peer, flow.finally_block);
                }

                push_bb_name!(self, "finally");
            }
            CompileCommand::TryEnd => {
                pop_bb_name!(self);

                let flow = self.control_flow_stack.pop_exception_flow();
                let exception_block = self.control_flow_stack.exception_block();

                unsafe {
                    bridge::compiler_peer_try_end(self.peer, exception_block, flow.end_block);

                    bridge::compiler_peer_move_basic_block_after(self.peer, flow.end_block);
                    bridge::compiler_peer_set_basic_block(self.peer, flow.end_block);
                }
            }
            CompileCommand::LabelStart(symbol, is_iteration_statement) => {
                debug_assert_ne!(*symbol, Symbol::NONE);
                let start_block = self.create_basic_block("start");
                let end_block = self.create_basic_block("end");
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, start_block);
                    bridge::compiler_peer_move_basic_block_after(self.peer, end_block);
                    bridge::compiler_peer_set_basic_block(self.peer, start_block);
                }
                self.control_flow_stack.push_break_target(end_block, *symbol);
                if *is_iteration_statement {
                    // The `block` member variable will be updated in the method to handle the loop start of the
                    // labeled iteration statement.
                    self.control_flow_stack.push_continue_target(0, *symbol);
                }
            }
            CompileCommand::LabelEnd(symbol, is_iteration_statement) => {
                debug_assert_ne!(*symbol, Symbol::NONE);
                if *is_iteration_statement {
                    self.control_flow_stack.pop_continue_target();
                }
                let break_target = self.control_flow_stack.pop_break_target();
                debug_assert_eq!(break_target.symbol, *symbol);
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, break_target.block);
                    bridge::compiler_peer_move_basic_block_after(self.peer, break_target.block);
                    bridge::compiler_peer_set_basic_block(self.peer, break_target.block);
                }
            }
            CompileCommand::Continue(symbol) => {
                let target_block = self.control_flow_stack.continue_target(*symbol);
                debug_assert_ne!(target_block, 0);
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, target_block);
                }
                // TODO(issue#234)
                self.create_basic_block_for_deadcode();
            }
            CompileCommand::Break(symbol) => {
                let target_block = self.control_flow_stack.break_target(*symbol);
                debug_assert_ne!(target_block, 0);
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, target_block);
                }
                // TODO(issue#234)
                self.create_basic_block_for_deadcode();
            }
            CompileCommand::Return(n) => {
                unsafe {
                    bridge::compiler_peer_return(self.peer, *n as usize);
                }
                self.control_flow_stack.set_returned();
                let next_block = self.control_flow_stack.cleanup_block();
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, next_block);
                }
                // TODO(issue#234)
                self.create_basic_block_for_deadcode();
            }
            CompileCommand::Throw => {
                unsafe {
                    bridge::compiler_peer_throw(self.peer);
                }
                self.control_flow_stack.set_thrown();
                let next_block = self.control_flow_stack.exception_block();
                unsafe {
                    bridge::compiler_peer_create_br(self.peer, next_block);
                    bridge::compiler_peer_move_basic_block_after(self.peer, next_block);
                }
                // TODO(issue#234)
                self.create_basic_block_for_deadcode();
            }
            CompileCommand::Discard => unsafe {
                // TODO: the stack should be managed in the Rust side.
                bridge::compiler_peer_discard(self.peer);
            },
            CompileCommand::Swap => unsafe {
                // TODO: the stack should be managed in the Rust side.
                bridge::compiler_peer_swap(self.peer);
            },
            CompileCommand::PrepareScopeCleanupChecker(stack_size) => unsafe {
                debug_assert!(*stack_size > 0);
                bridge::compiler_peer_prepare_scope_cleanup_checker(self.peer, *stack_size);
            },
        }

        if cfg!(debug_assertions)
            && std::env::var_os("BEE_DEBUG_JSRUNTIME_COMPILER_DUMP_STACK").is_some()
        {
            unsafe {
                bridge::compiler_peer_dump_stack(self.peer);
            }
            self.control_flow_stack.print();
        }
    }

    fn start_scope(&mut self, scope_ref: ScopeRef) {
        push_bb_name!(self, "scope", scope_ref.id());

        let init_block = self.create_basic_block("init");
        let hoisted_block = self.create_basic_block("hoisted");
        let body_block = self.create_basic_block("body");
        let cleanup_block = self.create_basic_block("cleanup");

        self.control_flow_stack.push_scope_flow(init_block, hoisted_block, body_block, cleanup_block);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, init_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, init_block);
            bridge::compiler_peer_set_basic_block(self.peer, body_block);

            bridge::compiler_peer_start_scope_cleanup_checker(self.peer, scope_ref.id());
        }
    }

    fn end_scope(&mut self, scope_ref: ScopeRef) {
        pop_bb_name!(self);

        let flow = self.control_flow_stack.pop_scope_flow();

        unsafe {
            bridge::compiler_peer_create_br(self.peer, flow.cleanup_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, flow.cleanup_block);

            bridge::compiler_peer_set_basic_block(self.peer, flow.init_block);
            bridge::compiler_peer_create_br(self.peer, flow.hoisted_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, flow.hoisted_block);

            bridge::compiler_peer_set_basic_block(self.peer, flow.hoisted_block);
            bridge::compiler_peer_create_br(self.peer, flow.body_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, flow.body_block);

            bridge::compiler_peer_set_basic_block(self.peer, flow.cleanup_block);
        }

        unsafe {
            bridge::compiler_peer_end_scope_cleanup_checker(self.peer, scope_ref.id());
        }

        let block = self.create_basic_block("block");

        let cleanup_block = if flow.returned {
            self.control_flow_stack.cleanup_block()
        } else {
            0
        };
        let exception_block = if flow.thrown && !self.control_flow_stack.in_finally_block() {
            self.control_flow_stack.exception_block()
        } else {
            0
        };
        unsafe {
            bridge::compiler_peer_handle_returned_thrown(self.peer, flow.returned, flow.thrown, block, cleanup_block, exception_block);
        }

        unsafe {
            bridge::compiler_peer_move_basic_block_after(self.peer, block);
            bridge::compiler_peer_set_basic_block(self.peer, block);
        }
    }

    fn branch(&mut self) {
        let before_block = unsafe {
            bridge::compiler_peer_get_basic_block(self.peer)
        };

        // Push a newly created block.
        // This will be used in ConditionalExpression() in order to build a branch instruction.
        let after_block = self.create_basic_block("block");

        unsafe {
            bridge::compiler_peer_set_basic_block(self.peer, after_block);
        }

        self.control_flow_stack.push_branch_flow(before_block, after_block);
    }

    fn create_basic_block(&mut self, name: &str) -> bridge::BasicBlock {
        push_bb_name!(self, name);
        let (name, name_len) = bb_name!(self);
        let block = unsafe {
            bridge::compiler_peer_create_basic_block(self.peer, name, name_len)
        };
        pop_bb_name!(self);
        block
    }

    fn create_basic_block_for_deadcode(&mut self) {
        let block = self.create_basic_block("deadcode");
        unsafe {
            bridge::compiler_peer_set_basic_block(self.peer, block);
        }
    }
}

impl<'r, 's> Drop for Compiler<'r, 's> {
    fn drop(&mut self) {
        unsafe {
            bridge::compiler_peer_delete(self.peer);
        }
    }
}
