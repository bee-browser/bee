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

macro_rules! stack_dump_enabled {
    () => {
        cfg!(debug_assertions)
            && std::env::var_os("BEE_DEBUG_JSRUNTIME_COMPILER_DUMP_STACK").is_some()
    };
}

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

        let locals_block = unsafe { bridge::compiler_peer_get_locals_block(self.peer) };
        let args_block = unsafe { bridge::compiler_peer_get_args_block(self.peer) };
        let body_block = unsafe { bridge::compiler_peer_get_body_block(self.peer) };
        let return_block = unsafe { bridge::compiler_peer_get_return_block(self.peer) };
        self.control_flow_stack.push_function_flow(
            locals_block,
            args_block,
            body_block,
            return_block,
        );
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
            CompileCommand::Undefined => self.process_undefined(),
            CompileCommand::Null => self.process_null(),
            CompileCommand::Boolean(value) => self.process_boolean(*value),
            CompileCommand::Number(value) => self.process_number(*value),
            CompileCommand::String(value) => self.process_string(value),
            CompileCommand::Function(func_id) => self.process_function(*func_id),
            CompileCommand::Closure(prologue, num_captures) => {
                self.process_closure(*prologue, *num_captures)
            }
            CompileCommand::Reference(symbol, locator) => self.process_reference(*symbol, *locator),
            CompileCommand::Exception => self.process_exception(),
            CompileCommand::AllocateLocals(num_locals) => self.process_allocate_locals(*num_locals),
            CompileCommand::MutableBinding => self.process_mutable_binding(),
            CompileCommand::ImmutableBinding => self.process_immutable_binding(),
            CompileCommand::DeclareFunction => self.process_declare_function(),
            CompileCommand::DeclareClosure => self.process_declare_closure(),
            CompileCommand::Arguments(nargs) => self.process_arguments(*nargs),
            CompileCommand::Argument(index) => self.process_argument(*index),
            CompileCommand::Call(nargs) => self.process_call(*nargs),
            CompileCommand::PushScope(scope_ref) => self.process_push_scope(*scope_ref),
            CompileCommand::PopScope(scope_ref) => self.process_pop_scope(*scope_ref),
            CompileCommand::CaptureVariable(declaration) => {
                self.process_capture_variable(*declaration)
            }
            CompileCommand::PostfixIncrement => self.process_postfix_increment(),
            CompileCommand::PostfixDecrement => self.process_postfix_decrement(),
            CompileCommand::PrefixIncrement => self.process_prefix_increment(),
            CompileCommand::PrefixDecrement => self.process_prefix_decrement(),
            CompileCommand::Delete => self.process_delete(),
            CompileCommand::Void => self.process_void(),
            CompileCommand::Typeof => self.process_typeof(),
            CompileCommand::UnaryPlus => self.process_unary_plus(),
            CompileCommand::UnaryMinus => self.process_unary_minus(),
            CompileCommand::BitwiseNot => self.process_bitwise_not(),
            CompileCommand::LogicalNot => self.process_logical_not(),
            CompileCommand::Exponentiation => self.process_exponentiation(),
            CompileCommand::Multiplication => self.process_multiplication(),
            CompileCommand::Division => self.process_division(),
            CompileCommand::Remainder => self.process_remainder(),
            CompileCommand::Addition => self.process_addition(),
            CompileCommand::Subtraction => self.process_subtraction(),
            CompileCommand::LeftShift => self.process_left_shift(),
            CompileCommand::SignedRightShift => self.process_signed_right_shift(),
            CompileCommand::UnsignedRightShift => self.process_unsigned_right_shift(),
            CompileCommand::LessThan => self.process_less_than(),
            CompileCommand::GreaterThan => self.process_greater_than(),
            CompileCommand::LessThanOrEqual => self.process_less_than_or_equal(),
            CompileCommand::GreaterThanOrEqual => self.process_greater_than_or_equal(),
            CompileCommand::Instanceof => self.process_instanceof(),
            CompileCommand::In => self.process_in(),
            CompileCommand::Equality => self.process_equality(),
            CompileCommand::Inequality => self.process_inequality(),
            CompileCommand::StrictEquality => self.process_strict_equality(),
            CompileCommand::StrictInequality => self.process_strict_inequality(),
            CompileCommand::BitwiseAnd => self.process_bitwise_and(),
            CompileCommand::BitwiseXor => self.process_bitwise_xor(),
            CompileCommand::BitwiseOr => self.process_bitwise_or(),
            CompileCommand::Ternary => self.process_ternary(),
            CompileCommand::Assignment => self.process_assignment(),
            CompileCommand::ExponentiationAssignment => self.process_exponentiation_assignment(),
            CompileCommand::MultiplicationAssignment => self.process_multiplication_assignment(),
            CompileCommand::DivisionAssignment => self.process_division_assignment(),
            CompileCommand::RemainderAssignment => self.process_remainder_assignment(),
            CompileCommand::AdditionAssignment => self.process_addition_assignment(),
            CompileCommand::SubtractionAssignment => self.process_subtraction_assignment(),
            CompileCommand::LeftShiftAssignment => self.process_left_shift_assignment(),
            CompileCommand::SignedRightShiftAssignment => {
                self.process_signed_right_shift_assignment()
            }
            CompileCommand::UnsignedRightShiftAssignment => {
                self.process_unsigned_right_shift_assignment()
            }
            CompileCommand::BitwiseAndAssignment => self.process_bitwise_and_assignment(),
            CompileCommand::BitwiseXorAssignment => self.process_bitwise_xor_assignment(),
            CompileCommand::BitwiseOrAssignment => self.process_bitwise_or_assignment(),
            CompileCommand::Truthy => self.process_truthy(),
            CompileCommand::FalsyShortCircuit => self.process_falsy_short_circuit(),
            CompileCommand::TruthyShortCircuit => self.process_truthy_short_circuit(),
            CompileCommand::NullishShortCircuit => self.process_nullish_short_circuit(),
            CompileCommand::FalsyShortCircuitAssignment => {
                self.process_falsy_short_circuit_assignment()
            }
            CompileCommand::TruthyShortCircuitAssignment => {
                self.process_truthy_short_circuit_assignment()
            }
            CompileCommand::NullishShortCircuitAssignment => {
                self.process_nullish_short_circuit_assignment()
            }
            CompileCommand::Then => self.process_then(),
            CompileCommand::Else => self.process_else(),
            CompileCommand::IfElseStatement => self.process_if_else_statement(),
            CompileCommand::IfStatement => self.process_if_statement(),
            CompileCommand::DoWhileLoop(id) => self.process_do_while_loop(*id),
            CompileCommand::WhileLoop(id) => self.process_while_loop(*id),
            CompileCommand::ForLoop(id, flags) => self.process_for_loop(*id, *flags),
            CompileCommand::LoopInit => self.process_loop_init(),
            CompileCommand::LoopTest => self.process_loop_test(),
            CompileCommand::LoopNext => self.process_loop_next(),
            CompileCommand::LoopBody => self.process_loop_body(),
            CompileCommand::LoopEnd => self.process_loop_end(),
            CompileCommand::CaseBlock(id, num_cases) => self.process_case_block(*id, *num_cases),
            CompileCommand::CaseClause(has_statement) => self.process_case_clause(*has_statement),
            CompileCommand::DefaultClause(has_statement) => {
                self.process_default_clause(*has_statement)
            }
            CompileCommand::Switch(id, num_cases, default_index) => {
                self.process_switch(*id, *num_cases, *default_index)
            }
            CompileCommand::Try => self.process_try(),
            CompileCommand::Catch(nominal) => self.process_catch(*nominal),
            CompileCommand::Finally(nominal) => self.process_finally(*nominal),
            CompileCommand::TryEnd => self.process_try_end(),
            CompileCommand::LabelStart(symbol, is_iteration_statement) => {
                self.process_label_start(*symbol, *is_iteration_statement)
            }
            CompileCommand::LabelEnd(symbol, is_iteration_statement) => {
                self.process_label_end(*symbol, *is_iteration_statement)
            }
            CompileCommand::Continue(symbol) => self.process_continue(*symbol),
            CompileCommand::Break(symbol) => self.process_break(*symbol),
            CompileCommand::Return(n) => self.process_return(*n),
            CompileCommand::Throw => self.process_throw(),
            CompileCommand::Discard => self.process_discard(),
            CompileCommand::Swap => self.process_swap(),
            CompileCommand::PrepareScopeCleanupChecker(stack_size) => {
                self.process_prepare_scope_cleanup_checker(*stack_size)
            }
        }

        if stack_dump_enabled!() {
            unsafe {
                bridge::compiler_peer_dump_stack(self.peer);
            }
            self.control_flow_stack.print();
        }
    }

    fn process_undefined(&mut self) {
        unsafe {
            bridge::compiler_peer_undefined(self.peer);
        }
    }

    fn process_null(&mut self) {
        unsafe {
            bridge::compiler_peer_null(self.peer);
        }
    }

    fn process_boolean(&mut self, value: bool) {
        unsafe {
            bridge::compiler_peer_boolean(self.peer, value);
        }
    }

    fn process_number(&mut self, value: f64) {
        unsafe {
            bridge::compiler_peer_number(self.peer, value);
        }
    }

    fn process_string(&mut self, _value: &[u16]) {
        unimplemented!("string literal");
    }

    fn process_function(&mut self, func_id: FunctionId) {
        let name = if func_id.is_native() {
            &self.function_registry.get_native(func_id).name
        } else {
            &self.function_registry.get_host(func_id).name
        };
        unsafe {
            bridge::compiler_peer_function(self.peer, func_id.into(), name.as_ptr());
        }
    }

    fn process_closure(&mut self, prologue: bool, num_captures: u16) {
        let block = if prologue {
            self.control_flow_stack.scope_flow().hoisted_block
        } else {
            0
        };
        unsafe {
            // `num_captures` may be 0.
            bridge::compiler_peer_closure(self.peer, block, num_captures);
        }
    }

    fn process_reference(&mut self, symbol: Symbol, locator: Locator) {
        debug_assert_ne!(locator, Locator::NONE);
        unsafe {
            bridge::compiler_peer_reference(self.peer, symbol.id(), locator);
        }
    }

    fn process_exception(&mut self) {
        unsafe {
            bridge::compiler_peer_exception(self.peer);
        }
    }

    fn process_allocate_locals(&mut self, num_locals: u16) {
        debug_assert!(num_locals > 0);
        unsafe {
            bridge::compiler_peer_allocate_locals(self.peer, num_locals);
        }
    }

    fn process_mutable_binding(&mut self) {
        unsafe {
            bridge::compiler_peer_declare_mutable(self.peer);
        }
    }

    fn process_immutable_binding(&mut self) {
        unsafe {
            bridge::compiler_peer_declare_immutable(self.peer);
        }
    }

    fn process_declare_function(&mut self) {
        let block = self.control_flow_stack.scope_flow().hoisted_block;
        unsafe {
            bridge::compiler_peer_declare_function(self.peer, block);
        }
    }

    fn process_declare_closure(&mut self) {
        let block = self.control_flow_stack.scope_flow().hoisted_block;
        unsafe {
            bridge::compiler_peer_declare_closure(self.peer, block);
        }
    }

    fn process_arguments(&mut self, nargs: u16) {
        if nargs > 0 {
            unsafe {
                bridge::compiler_peer_arguments(self.peer, nargs);
            }
        }
    }

    fn process_argument(&mut self, index: u16) {
        unsafe {
            bridge::compiler_peer_argument(self.peer, index);
        }
    }

    fn process_call(&mut self, nargs: u16) {
        let block = self.control_flow_stack.exception_block();
        unsafe {
            bridge::compiler_peer_call(self.peer, nargs, block);
        }
        // The function may throw an exception.
        self.control_flow_stack.set_thrown();
    }

    fn process_push_scope(&mut self, scope_ref: ScopeRef) {
        debug_assert_ne!(scope_ref, ScopeRef::NONE);

        push_bb_name!(self, "scope", scope_ref.id());

        let init_block = self.create_basic_block("init");
        let hoisted_block = self.create_basic_block("hoisted");
        let body_block = self.create_basic_block("body");
        let cleanup_block = self.create_basic_block("cleanup");

        self.control_flow_stack.push_scope_flow(
            init_block,
            hoisted_block,
            body_block,
            cleanup_block,
        );

        unsafe {
            bridge::compiler_peer_create_br(self.peer, init_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, init_block);
            bridge::compiler_peer_set_basic_block(self.peer, body_block);

            bridge::compiler_peer_start_scope_cleanup_checker(self.peer, scope_ref.id());
        }

        let scope = self.scope_tree.scope(scope_ref);
        for binding in scope.bindings.iter() {
            if binding.is_local() {
                unsafe {
                    bridge::compiler_peer_init_local(self.peer, binding.locator(), init_block);
                }
            }
            if binding.captured {
                unsafe {
                    bridge::compiler_peer_create_capture(self.peer, binding.locator(), init_block);
                }
            }
        }
    }

    fn process_pop_scope(&mut self, scope_ref: ScopeRef) {
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
            bridge::compiler_peer_handle_returned_thrown(
                self.peer,
                flow.returned,
                flow.thrown,
                block,
                cleanup_block,
                exception_block,
            );
        }

        unsafe {
            bridge::compiler_peer_move_basic_block_after(self.peer, block);
            bridge::compiler_peer_set_basic_block(self.peer, block);
        }
    }

    fn process_capture_variable(&mut self, declaration: bool) {
        let block = if declaration {
            self.control_flow_stack.scope_flow().hoisted_block
        } else {
            0
        };
        unsafe {
            bridge::compiler_peer_capture_variable(self.peer, block);
        }
    }

    fn process_postfix_increment(&mut self) {
        unsafe {
            bridge::compiler_peer_postfix_increment(self.peer);
        }
    }

    fn process_postfix_decrement(&mut self) {
        unsafe {
            bridge::compiler_peer_postfix_decrement(self.peer);
        }
    }

    fn process_prefix_increment(&mut self) {
        unsafe {
            bridge::compiler_peer_prefix_increment(self.peer);
        }
    }

    fn process_prefix_decrement(&mut self) {
        unsafe {
            bridge::compiler_peer_prefix_decrement(self.peer);
        }
    }

    fn process_delete(&mut self) {
        unimplemented!("delete operator");
    }

    fn process_void(&mut self) {
        unsafe {
            bridge::compiler_peer_void(self.peer);
        }
    }

    fn process_typeof(&mut self) {
        // TODO: implement String before this
        unimplemented!("typeof operator");
    }

    fn process_unary_plus(&mut self) {
        unsafe {
            bridge::compiler_peer_unary_plus(self.peer);
        }
    }

    fn process_unary_minus(&mut self) {
        unsafe {
            bridge::compiler_peer_unary_minus(self.peer);
        }
    }

    fn process_bitwise_not(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_not(self.peer);
        }
    }

    fn process_logical_not(&mut self) {
        unsafe {
            bridge::compiler_peer_logical_not(self.peer);
        }
    }

    fn process_exponentiation(&mut self) {
        unimplemented!("** operator");
    }

    fn process_multiplication(&mut self) {
        unsafe {
            bridge::compiler_peer_multiplication(self.peer);
        }
    }

    fn process_division(&mut self) {
        unsafe {
            bridge::compiler_peer_division(self.peer);
        }
    }

    fn process_remainder(&mut self) {
        unsafe {
            bridge::compiler_peer_remainder(self.peer);
        }
    }

    fn process_addition(&mut self) {
        unsafe {
            bridge::compiler_peer_addition(self.peer);
        }
    }

    fn process_subtraction(&mut self) {
        unsafe {
            bridge::compiler_peer_subtraction(self.peer);
        }
    }

    fn process_left_shift(&mut self) {
        unsafe {
            bridge::compiler_peer_left_shift(self.peer);
        }
    }

    fn process_signed_right_shift(&mut self) {
        unsafe {
            bridge::compiler_peer_signed_right_shift(self.peer);
        }
    }

    fn process_unsigned_right_shift(&mut self) {
        unsafe {
            bridge::compiler_peer_unsigned_right_shift(self.peer);
        }
    }

    fn process_less_than(&mut self) {
        unsafe {
            bridge::compiler_peer_less_than(self.peer);
        }
    }

    fn process_greater_than(&mut self) {
        unsafe {
            bridge::compiler_peer_greater_than(self.peer);
        }
    }

    fn process_less_than_or_equal(&mut self) {
        unsafe {
            bridge::compiler_peer_less_than_or_equal(self.peer);
        }
    }

    fn process_greater_than_or_equal(&mut self) {
        unsafe {
            bridge::compiler_peer_greater_than_or_equal(self.peer);
        }
    }

    fn process_instanceof(&mut self) {
        unimplemented!("instanceof operator");
    }

    fn process_in(&mut self) {
        unimplemented!("in operator");
    }

    fn process_equality(&mut self) {
        unsafe {
            bridge::compiler_peer_equality(self.peer);
        }
    }

    fn process_inequality(&mut self) {
        unsafe {
            bridge::compiler_peer_inequality(self.peer);
        }
    }

    fn process_strict_equality(&mut self) {
        unsafe {
            bridge::compiler_peer_strict_equality(self.peer);
        }
    }

    fn process_strict_inequality(&mut self) {
        unsafe {
            bridge::compiler_peer_strict_inequality(self.peer);
        }
    }

    fn process_bitwise_and(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_and(self.peer);
        }
    }

    fn process_bitwise_xor(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_xor(self.peer);
        }
    }

    fn process_bitwise_or(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_or(self.peer);
        }
    }

    fn process_ternary(&mut self) {
        let else_branch = self.control_flow_stack.pop_branch_flow();
        let then_branch = self.control_flow_stack.pop_branch_flow();
        unsafe {
            bridge::compiler_peer_ternary(
                self.peer,
                then_branch.before_block,
                then_branch.after_block,
                else_branch.before_block,
                else_branch.after_block,
            );
        }
    }

    fn process_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_assignment(self.peer);
        }
    }

    fn process_exponentiation_assignment(&mut self) {
        unimplemented!("**= operator");
    }

    fn process_multiplication_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_multiplication_assignment(self.peer);
        }
    }

    fn process_division_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_division_assignment(self.peer);
        }
    }

    fn process_remainder_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_remainder_assignment(self.peer);
        }
    }

    fn process_addition_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_addition_assignment(self.peer);
        }
    }

    fn process_subtraction_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_subtraction_assignment(self.peer);
        }
    }

    fn process_left_shift_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_left_shift_assignment(self.peer);
        }
    }

    fn process_signed_right_shift_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_signed_right_shift_assignment(self.peer);
        }
    }

    fn process_unsigned_right_shift_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_unsigned_right_shift_assignment(self.peer);
        }
    }

    fn process_bitwise_and_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_and_assignment(self.peer);
        }
    }

    fn process_bitwise_xor_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_xor_assignment(self.peer);
        }
    }

    fn process_bitwise_or_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_bitwise_or_assignment(self.peer);
        }
    }

    fn process_truthy(&mut self) {
        unsafe {
            bridge::compiler_peer_truthy(self.peer);
        }
    }

    fn process_falsy_short_circuit(&mut self) {
        unsafe {
            bridge::compiler_peer_falsy_short_circuit(self.peer);
        }
        self.branch(); // then
        self.branch(); // else
    }

    fn process_truthy_short_circuit(&mut self) {
        unsafe {
            bridge::compiler_peer_truthy_short_circuit(self.peer);
        }
        self.branch(); // then
        self.branch(); // else
    }

    fn process_nullish_short_circuit(&mut self) {
        unsafe {
            bridge::compiler_peer_nullish_short_circuit(self.peer);
        }
        self.branch(); // then
        self.branch(); // else
    }

    fn process_falsy_short_circuit_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_falsy_short_circuit_assignment(self.peer);
        }
        self.branch(); // then
        self.branch(); // else
    }

    fn process_truthy_short_circuit_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_truthy_short_circuit_assignment(self.peer);
        }
        self.branch(); // then
        self.branch(); // else
    }

    fn process_nullish_short_circuit_assignment(&mut self) {
        unsafe {
            bridge::compiler_peer_nullish_short_circuit_assignment(self.peer);
        }
        self.branch(); // then
        self.branch(); // else
    }

    fn process_then(&mut self) {
        self.branch();
    }

    fn process_else(&mut self) {
        self.branch();
    }

    fn process_if_else_statement(&mut self) {
        let else_branch = self.control_flow_stack.pop_branch_flow();
        let then_branch = self.control_flow_stack.pop_branch_flow();
        unsafe {
            bridge::compiler_peer_if_else_statement(
                self.peer,
                then_branch.before_block,
                then_branch.after_block,
                else_branch.before_block,
                else_branch.after_block,
            );
        }
    }

    fn process_if_statement(&mut self) {
        let branch = self.control_flow_stack.pop_branch_flow();
        unsafe {
            bridge::compiler_peer_if_statement(self.peer, branch.before_block, branch.after_block);
        }
    }

    fn process_do_while_loop(&mut self, id: u16) {
        push_bb_name!(self, "do-while", id);

        let loop_body = self.create_basic_block("loop-body");
        let loop_test = self.create_basic_block("loop-test");
        let loop_end = self.create_basic_block("loop-end");

        let loop_start = loop_body;
        let loop_continue = loop_test;
        let loop_break = loop_end;

        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_end, loop_end);
        self.control_flow_stack
            .push_loop_body_flow(loop_test, loop_test);

        self.control_flow_stack.set_continue_target(loop_continue);
        self.control_flow_stack
            .push_break_target(loop_break, Symbol::NONE);
        self.control_flow_stack
            .push_continue_target(loop_continue, Symbol::NONE);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, loop_start);
            bridge::compiler_peer_set_basic_block(self.peer, loop_start);
        }
    }

    fn process_while_loop(&mut self, id: u16) {
        push_bb_name!(self, "while", id);

        let loop_test = self.create_basic_block("loop-test");
        let loop_body = self.create_basic_block("loop-body");
        let loop_end = self.create_basic_block("loop-end");

        let loop_start = loop_test;
        let loop_continue = loop_test;
        let loop_break = loop_end;

        self.control_flow_stack
            .push_loop_body_flow(loop_test, loop_end);
        self.control_flow_stack
            .push_loop_test_flow(loop_body, loop_end, loop_body);

        self.control_flow_stack.set_continue_target(loop_continue);
        self.control_flow_stack
            .push_break_target(loop_break, Symbol::NONE);
        self.control_flow_stack
            .push_continue_target(loop_continue, Symbol::NONE);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, loop_start);
            bridge::compiler_peer_set_basic_block(self.peer, loop_start);
        }
    }

    // TODO: rewrite using if and break
    fn process_for_loop(&mut self, id: u16, flags: LoopFlags) {
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
            self.control_flow_stack
                .push_loop_body_flow(loop_next, loop_end);
        } else if has_test {
            self.control_flow_stack
                .push_loop_body_flow(loop_test, loop_end);
        } else {
            self.control_flow_stack
                .push_loop_body_flow(loop_body, loop_end);
        }

        if has_next {
            if has_test {
                self.control_flow_stack
                    .push_loop_next_flow(loop_test, loop_body);
            } else {
                self.control_flow_stack
                    .push_loop_next_flow(loop_body, loop_body);
            }
            loop_continue = loop_next;
            insert_point = loop_next;
        }

        if has_test {
            if has_next {
                self.control_flow_stack
                    .push_loop_test_flow(loop_body, loop_end, loop_next);
            } else {
                self.control_flow_stack
                    .push_loop_test_flow(loop_body, loop_end, loop_body);
            }
            loop_start = loop_test;
            if !has_next {
                loop_continue = loop_test;
            }
            insert_point = loop_test;
        }

        if has_init {
            if has_test {
                self.control_flow_stack
                    .push_loop_init_flow(loop_test, loop_test);
            } else if has_next {
                self.control_flow_stack
                    .push_loop_init_flow(loop_body, loop_next);
            } else {
                self.control_flow_stack
                    .push_loop_init_flow(loop_body, loop_body);
            }
            loop_start = loop_init;
            insert_point = loop_init;
        }

        self.control_flow_stack.set_continue_target(loop_continue);
        self.control_flow_stack
            .push_break_target(loop_break, Symbol::NONE);
        self.control_flow_stack
            .push_continue_target(loop_continue, Symbol::NONE);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, loop_start);
            bridge::compiler_peer_set_basic_block(self.peer, insert_point);
        }
    }

    fn process_loop_init(&mut self) {
        let loop_init = self.control_flow_stack.pop_loop_init_flow();
        unsafe {
            bridge::compiler_peer_create_br(self.peer, loop_init.branch_block);
            bridge::compiler_peer_set_basic_block(self.peer, loop_init.insert_point);
        }
    }

    fn process_loop_test(&mut self) {
        let loop_test = self.control_flow_stack.pop_loop_test_flow();
        unsafe {
            // TODO: refactoring
            bridge::compiler_peer_loop_test(
                self.peer,
                loop_test.then_block,
                loop_test.else_block,
                loop_test.insert_point,
            );
        }
    }

    fn process_loop_next(&mut self) {
        let loop_next = self.control_flow_stack.pop_loop_next_flow();
        unsafe {
            // Discard the evaluation result.
            bridge::compiler_peer_discard(self.peer);
            bridge::compiler_peer_create_br(self.peer, loop_next.branch_block);
            bridge::compiler_peer_set_basic_block(self.peer, loop_next.insert_point);
        }
    }

    fn process_loop_body(&mut self) {
        let loop_body = self.control_flow_stack.pop_loop_body_flow();
        unsafe {
            bridge::compiler_peer_create_br(self.peer, loop_body.branch_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, loop_body.insert_point);
            bridge::compiler_peer_set_basic_block(self.peer, loop_body.insert_point);
        }
    }

    fn process_loop_end(&mut self) {
        pop_bb_name!(self);
        self.control_flow_stack.pop_break_target();
        self.control_flow_stack.pop_continue_target();
    }

    fn process_case_block(&mut self, id: u16, num_cases: u16) {
        push_bb_name!(self, "switch", id);
        unsafe {
            debug_assert!(num_cases > 0);
            // TODO: refactoring
            bridge::compiler_peer_case_block(self.peer, id, num_cases);
        }
        let end_block = self.create_basic_block("end");
        self.control_flow_stack.push_switch_flow(end_block);
        self.control_flow_stack
            .push_break_target(end_block, Symbol::NONE);
    }

    fn process_case_clause(&mut self, has_statement: bool) {
        let branch = self.control_flow_stack.pop_branch_flow();
        let case_end_block = unsafe { bridge::compiler_peer_get_basic_block(self.peer) };
        unsafe {
            bridge::compiler_peer_case_clause(
                self.peer,
                has_statement,
                branch.before_block,
                branch.after_block,
            );
        }
        self.control_flow_stack
            .push_case_banch_flow(case_end_block, branch.after_block);
    }

    fn process_default_clause(&mut self, has_statement: bool) {
        let branch = self.control_flow_stack.pop_branch_flow();
        let case_end_block = unsafe { bridge::compiler_peer_get_basic_block(self.peer) };
        unsafe {
            bridge::compiler_peer_default_clause(self.peer, has_statement, branch.before_block);
        }
        self.control_flow_stack
            .push_case_banch_flow(case_end_block, branch.after_block);
        self.control_flow_stack
            .set_default_case_block(branch.after_block);
    }

    fn process_switch(&mut self, _id: u16, num_cases: u16, _default_index: Option<u16>) {
        pop_bb_name!(self);

        self.control_flow_stack.pop_break_target();
        let case_block = unsafe { bridge::compiler_peer_get_basic_block(self.peer) };

        // Discard the switch-values
        unsafe {
            bridge::compiler_peer_discard(self.peer);
            bridge::compiler_peer_discard(self.peer);
        }

        // Connect the last basic blocks of each case/default clause to the first basic block of
        // the statement lists of the next case/default clause if it's not terminated.
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

        // Create an unconditional jump to the statement of the default clause if it exists.
        // Otherwise, jump to the end block.
        unsafe {
            bridge::compiler_peer_set_basic_block(self.peer, case_block);
            bridge::compiler_peer_create_br(
                self.peer,
                if switch.default_block != 0 {
                    switch.default_block
                } else {
                    switch.end_block
                },
            );
        }

        unsafe {
            bridge::compiler_peer_move_basic_block_after(self.peer, switch.end_block);
            bridge::compiler_peer_set_basic_block(self.peer, switch.end_block);
        }
    }

    fn branch(&mut self) {
        let before_block = unsafe { bridge::compiler_peer_get_basic_block(self.peer) };

        // Push a newly created block.
        // This will be used in ConditionalExpression() in order to build a branch instruction.
        let after_block = self.create_basic_block("block");

        unsafe {
            bridge::compiler_peer_set_basic_block(self.peer, after_block);
        }

        self.control_flow_stack
            .push_branch_flow(before_block, after_block);
    }

    fn process_try(&mut self) {
        let try_block = self.create_basic_block("try");
        let catch_block = self.create_basic_block("catch");
        let finally_block = self.create_basic_block("finally");
        let end_block = self.create_basic_block("try-end");

        self.control_flow_stack.push_exception_flow(
            try_block,
            catch_block,
            finally_block,
            end_block,
        );

        unsafe {
            // Jump from the end of previous block to the beginning of the try block.
            bridge::compiler_peer_create_br(self.peer, try_block);

            bridge::compiler_peer_set_basic_block(self.peer, try_block);
        }

        push_bb_name!(self, "try");
    }

    fn process_catch(&mut self, nominal: bool) {
        pop_bb_name!(self);

        self.control_flow_stack.set_in_catch(nominal);
        let flow = self.control_flow_stack.exception_flow();

        unsafe {
            // Jump from the end of the try block to the beginning of the finally block.
            bridge::compiler_peer_create_br(self.peer, flow.finally_block);

            bridge::compiler_peer_move_basic_block_after(self.peer, flow.catch_block);
            bridge::compiler_peer_set_basic_block(self.peer, flow.catch_block);

            if !nominal {
                // TODO: Reset the status to Status::Normal.
                bridge::compiler_peer_create_store_normal_status(self.peer);
            }
        }

        push_bb_name!(self, "catch");
    }

    fn process_finally(&mut self, _nominal: bool) {
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

    fn process_try_end(&mut self) {
        pop_bb_name!(self);

        let flow = self.control_flow_stack.pop_exception_flow();
        let exception_block = self.control_flow_stack.exception_block();

        unsafe {
            bridge::compiler_peer_try_end(self.peer, exception_block, flow.end_block);

            bridge::compiler_peer_move_basic_block_after(self.peer, flow.end_block);
            bridge::compiler_peer_set_basic_block(self.peer, flow.end_block);
        }
    }

    fn process_label_start(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        debug_assert_ne!(symbol, Symbol::NONE);

        let start_block = self.create_basic_block("start");
        let end_block = self.create_basic_block("end");

        unsafe {
            bridge::compiler_peer_create_br(self.peer, start_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, end_block);
            bridge::compiler_peer_set_basic_block(self.peer, start_block);
        }

        self.control_flow_stack.push_break_target(end_block, symbol);

        if is_iteration_statement {
            // The `block` member variable will be updated in the method to handle the loop start
            // of the labeled iteration statement.
            self.control_flow_stack.push_continue_target(0, symbol);
        }
    }

    fn process_label_end(&mut self, symbol: Symbol, is_iteration_statement: bool) {
        debug_assert_ne!(symbol, Symbol::NONE);

        if is_iteration_statement {
            self.control_flow_stack.pop_continue_target();
        }

        let break_target = self.control_flow_stack.pop_break_target();
        debug_assert_eq!(break_target.symbol, symbol);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, break_target.block);
            bridge::compiler_peer_move_basic_block_after(self.peer, break_target.block);
            bridge::compiler_peer_set_basic_block(self.peer, break_target.block);
        }
    }

    fn process_continue(&mut self, symbol: Symbol) {
        let target_block = self.control_flow_stack.continue_target(symbol);
        debug_assert_ne!(target_block, 0);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, target_block);
        }

        self.create_basic_block_for_deadcode();
    }

    fn process_break(&mut self, symbol: Symbol) {
        let target_block = self.control_flow_stack.break_target(symbol);
        debug_assert_ne!(target_block, 0);

        unsafe {
            bridge::compiler_peer_create_br(self.peer, target_block);
        }

        self.create_basic_block_for_deadcode();
    }

    fn process_return(&mut self, n: u32) {
        unsafe {
            bridge::compiler_peer_return(self.peer, n as usize);
        }

        self.control_flow_stack.set_returned();
        let next_block = self.control_flow_stack.cleanup_block();

        unsafe {
            bridge::compiler_peer_create_br(self.peer, next_block);
        }

        self.create_basic_block_for_deadcode();
    }

    fn process_throw(&mut self) {
        unsafe {
            bridge::compiler_peer_throw(self.peer);
        }

        self.control_flow_stack.set_thrown();
        let next_block = self.control_flow_stack.exception_block();

        unsafe {
            bridge::compiler_peer_create_br(self.peer, next_block);
            bridge::compiler_peer_move_basic_block_after(self.peer, next_block);
        }

        self.create_basic_block_for_deadcode();
    }

    fn process_discard(&mut self) {
        unsafe {
            // TODO: the stack should be managed in the Rust side.
            bridge::compiler_peer_discard(self.peer);
        }
    }

    fn process_swap(&mut self) {
        unsafe {
            // TODO: the stack should be managed in the Rust side.
            bridge::compiler_peer_swap(self.peer);
        }
    }

    fn process_prepare_scope_cleanup_checker(&mut self, stack_size: u16) {
        debug_assert!(stack_size > 0);
        unsafe {
            bridge::compiler_peer_prepare_scope_cleanup_checker(self.peer, stack_size);
        }
    }

    fn create_basic_block(&mut self, name: &str) -> bridge::BasicBlock {
        push_bb_name!(self, name);
        let (name, name_len) = bb_name!(self);
        let block = unsafe { bridge::compiler_peer_create_basic_block(self.peer, name, name_len) };
        pop_bb_name!(self);
        block
    }

    // FIXME: Handle dead code in the proper way.
    //
    // We insert a **unreachable** basic block for dead code in order to avoid the following
    // validation error: "Terminator found in the middle of a basic block!"
    //
    // IRBuilder accepts inserting instructions after a terminator instruction in a basic block.
    // It's our responsibility to avoid a malformed basic block.  We think that it's not a good
    // direction to check the existence of a terminator instruction in a basic block before
    // insertion in efficiency and maintainability points of view.  Instead, we create an
    // **unreachable** basic block for dead code.  Eventually, this basic block was removed in the
    // optimization passes.
    //
    // At this point, we don't know whether this is a common method or not...
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
        (
            self.buffer.as_ptr() as *const std::ffi::c_char,
            self.buffer.len(),
        )
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
