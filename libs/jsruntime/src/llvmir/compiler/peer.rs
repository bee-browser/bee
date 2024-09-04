use std::ffi::CStr;

use crate::logger;
use crate::Module;

use super::bridge;
use super::FunctionId;
use super::ScopeRef;
use super::Symbol;

pub struct Compiler(*mut bridge::Compiler);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasicBlock(*mut bridge::BasicBlock);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LambdaIr(*mut bridge::LambdaIr);

impl Compiler {
    pub fn new() -> Self {
        Self(unsafe { bridge::compiler_peer_new() })
    }

    pub fn start_compile(&self, enable_labels: bool) {
        unsafe {
            bridge::compiler_peer_start(self.0, enable_labels);
        }
    }

    pub fn end_compile(&self) -> Module {
        Module {
            peer: unsafe {
                bridge::compiler_peer_end(self.0)
            },
        }
    }

    pub fn set_data_layout(&self, data_layout: &CStr) {
        unsafe {
            bridge::compiler_peer_set_data_layout(self.0, data_layout.as_ptr());
        }
    }

    pub fn set_target_triple(&self, triple: &CStr) {
        unsafe {
            bridge::compiler_peer_set_target_triple(self.0, triple.as_ptr());
        }
    }

    // function

    pub fn start_function(&self, name: &CStr) {
        unsafe {
            bridge::compiler_peer_start_function(self.0, name.as_ptr());
        }
    }

    pub fn end_function(&self, optimize: bool) {
        unsafe {
            bridge::compiler_peer_end_function(self.0, optimize);
        }
    }

    pub fn set_locals_block(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_set_locals_block(self.0, block.0);
        }
    }

    pub fn get_function(&self, func_id: FunctionId, name: &CStr) -> LambdaIr {
        unsafe {
            LambdaIr(bridge::compiler_peer_get_function(self.0, func_id.into(), name.as_ptr()))
        }
    }

    // basic block

    pub fn create_basic_block(&self, name: *const std::ffi::c_char, name_len: usize) -> BasicBlock {
        unsafe {
            BasicBlock(bridge::compiler_peer_create_basic_block(self.0, name, name_len))
        }
    }

    pub fn get_basic_block(&self) -> BasicBlock {
        unsafe {
            BasicBlock(bridge::compiler_peer_get_basic_block(self.0))
        }
    }

    pub fn set_basic_block(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_set_basic_block(self.0, block.0);
        }
    }

    pub fn move_basic_block_after(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_move_basic_block_after(self.0, block.0);
        }
    }

    pub fn is_basic_block_terminated(&self, block: BasicBlock) -> bool {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_is_basic_block_terminated(self.0, block.0)
        }
    }

    // const values

    pub fn get_nullptr(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_get_nullptr(self.0)
        }
    }

    pub fn get_boolean(&self, value: bool) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_get_boolean(self.0, value)
        }
    }

    pub fn get_zero(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_get_zero(self.0)
        }
    }

    pub fn get_nan(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_get_nan(self.0)
        }
    }

    pub fn get_number(&self, value: f64) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_get_number(self.0, value)
        }
    }

    // conversions

    pub fn create_ui_to_fp(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_ui_to_fp(self.0, value)
        }
    }

    pub fn to_numeric(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_to_numeric(self.0, value)
        }
    }

    pub fn create_number_to_boolean(&self, number: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_number_to_boolean(self.0, number)
        }
    }

    pub fn create_to_boolean(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_to_boolean(self.0, value)
        }
    }

    pub fn create_undefined_to_any(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_undefined_to_any(self.0)
        }
    }

    pub fn create_null_to_any(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_null_to_any(self.0)
        }
    }

    pub fn create_boolean_to_any(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_boolean_to_any(self.0, value)
        }
    }

    pub fn create_number_to_any(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_number_to_any(self.0, value)
        }
    }

    pub fn create_closure_to_any(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_closure_to_any(self.0, value)
        }
    }

    // jump

    pub fn create_br(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_br(self.0, block.0);
        }
    }

    // unary operators

    pub fn create_fneg(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_fneg(self.0, value)
        }
    }

    pub fn create_bitwise_not(&self, number: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_bitwise_not(self.0, number)
        }
    }

    pub fn create_logical_not(&self, boolean: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_logical_not(self.0, boolean)
        }
    }

    // binary operators

    pub fn create_fmul(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_fmul(self.0, lhs, rhs)
        }
    }

    pub fn create_fdiv(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_fdiv(self.0, lhs, rhs)
        }
    }

    pub fn create_frem(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_frem(self.0, lhs, rhs)
        }
    }

    pub fn create_fadd(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_fadd(self.0, lhs, rhs)
        }
    }

    pub fn create_fsub(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_fsub(self.0, lhs, rhs)
        }
    }

    pub fn create_left_shift(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_left_shift(self.0, lhs, rhs)
        }
    }

    pub fn create_signed_right_shift(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_signed_right_shift(self.0, lhs, rhs)
        }
    }

    pub fn create_unsigned_right_shift(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_unsigned_right_shift(self.0, lhs, rhs)
        }
    }

    pub fn create_less_than(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_less_than(self.0, lhs, rhs)
        }
    }

    pub fn create_greater_than(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_greater_than(self.0, lhs, rhs)
        }
    }

    pub fn create_less_than_or_equal(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_less_than_or_equal(self.0, lhs, rhs)
        }
    }

    pub fn create_greater_than_or_equal(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_greater_than_or_equal(self.0, lhs, rhs)
        }
    }

    pub fn create_bitwise_and(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_bitwise_and(self.0, lhs, rhs)
        }
    }

    pub fn create_bitwise_xor(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_bitwise_xor(self.0, lhs, rhs)
        }
    }

    pub fn create_bitwise_or(&self, lhs: bridge::ValueIr, rhs: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_bitwise_or(self.0, lhs, rhs)
        }
    }

    // incr/decr

    pub fn create_incr(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_incr(self.0, value)
        }
    }

    pub fn create_decr(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_decr(self.0, value)
        }
    }

    // equality

    pub fn create_is_loosely_equal(&self, a: bridge::ValueIr, b: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            return bridge::compiler_peer_create_is_loosely_equal(self.0, a, b);
        }
    }

    pub fn create_is_strictly_equal(&self, a: bridge::ValueIr, b: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_strictly_equal(self.0, a, b)
        }
    }

    pub fn create_is_undefined(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_undefined(self.0, value)
        }
    }

    pub fn create_is_null(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_null(self.0, value)
        }
    }

    pub fn create_is_boolean(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_boolean(self.0, value)
        }
    }

    pub fn create_is_same_boolean(&self, a: bridge::ValueIr, b: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_same_boolean(self.0, a, b)
        }
    }

    pub fn create_is_same_boolean_value(&self, variable: bridge::ValueIr, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_same_boolean_value(self.0, variable, value)
        }
    }

    pub fn create_is_number(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        logger::debug!(event = "create_is_number", ?value);
        unsafe {
            bridge::compiler_peer_create_is_number(self.0, value)
        }
    }

    pub fn create_is_same_number(&self, a: bridge::ValueIr, b: bridge::ValueIr) -> bridge::ValueIr {
        logger::debug!(event = "create_is_same_number", ?a, ?b);
        unsafe {
            bridge::compiler_peer_create_is_same_number(self.0, a, b)
        }
    }

    pub fn create_is_same_number_value(&self, variable: bridge::ValueIr, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_same_number_value(self.0, variable, value)
        }
    }

    pub fn create_is_closure(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_closure(self.0, value)
        }
    }

    pub fn create_is_same_closure(&self, a: bridge::ValueIr, b: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_same_closure(self.0, a, b)
        }
    }

    pub fn create_is_same_closure_value(&self, variable: bridge::ValueIr, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_same_closure_value(self.0, variable, value)
        }
    }

    // jump

    pub fn create_cond_br(&self, cond: bridge::ValueIr, then_block: BasicBlock, else_block: BasicBlock) {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_cond_br(self.0, cond, then_block.0, else_block.0);
        }
    }

    pub fn create_is_non_nullish(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_non_nullish(self.0, value)
        }
    }

    // phi

    pub fn create_boolean_ternary(&self, then_value: bridge::ValueIr, then_block: BasicBlock, else_value: bridge::ValueIr, else_block: BasicBlock) -> bridge::ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_boolean_ternary(self.0, then_value, then_block.0, else_value, else_block.0)
        }
    }

    pub fn create_number_ternary(&self, then_value: bridge::ValueIr, then_block: BasicBlock, else_value: bridge::ValueIr, else_block: BasicBlock) -> bridge::ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        logger::debug!(event = "create_number_ternary", ?then_value, ?then_block, ?else_value, ?else_block);
        unsafe {
            bridge::compiler_peer_create_number_ternary(self.0, then_value, then_block.0, else_value, else_block.0)
        }
    }

    pub fn create_any_ternary(&self, then_value: bridge::ValueIr, then_block: BasicBlock, else_value: bridge::ValueIr, else_block: BasicBlock) -> bridge::ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_any_ternary(self.0, then_value, then_block.0, else_value, else_block.0)
        }
    }

    // closure

    pub fn create_closure_ptr(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_closure_ptr(self.0)
        }
    }

    pub fn create_call_on_closure(&self, closure: bridge::ValueIr, argc: u16, argv: bridge::ValueIr, retv: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_call_on_closure(self.0, closure, argc, argv, retv)
        }
    }

    pub fn create_call_runtime_create_closure(&self, lambda: LambdaIr, num_captures: u16) -> bridge::ValueIr {
        debug_assert_ne!(lambda, LambdaIr::NONE);
        unsafe {
            bridge::compiler_peer_create_call_runtime_create_closure(self.0, lambda.0, num_captures)
        }
    }

    pub fn create_load_captures_from_closure(&self, closure: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_load_captures_from_closure(self.0, closure)
        }
    }

    pub fn create_store_capture_ptr_to_captures(&self, capture: bridge::ValueIr, captures: bridge::ValueIr, index: u16) {
        unsafe {
            bridge::compiler_peer_create_store_capture_ptr_to_captures(self.0, capture, captures, index);
        }
    }

    pub fn create_load_closure_from_value(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_load_closure_from_value(self.0, value)
        }
    }

    pub fn create_load_closure(&self, value: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_load_closure(self.0, value)
        }
    }

    // capture

    pub fn create_capture(&self, variable: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_call_runtime_create_capture(self.0, variable)
        }
    }

    pub fn create_escape_variable(&self, capture: bridge::ValueIr, variable: bridge::ValueIr) {
        unsafe {
            bridge::compiler_peer_create_escape_variable(self.0, capture, variable);
        }
    }

    pub fn create_get_capture_variable_ptr(&self, index: u16) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_get_capture_variable_ptr(self.0, index)
        }
    }

    pub fn create_load_capture(&self, index: u16) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_load_capture(self.0, index)
        }
    }

    // argv

    pub fn create_argv(&self, argc: u16) -> bridge::ValueIr {
        debug_assert!(argc > 0);
        unsafe {
            bridge::compiler_peer_create_argv(self.0, argc)
        }
    }

    pub fn create_get_arg_in_argv(&self, argv: bridge::ValueIr, index: u16) -> bridge::ValueIr {
        debug_assert_ne!(argv, 0);
        unsafe {
            bridge::compiler_peer_create_get_arg_in_argv(self.0, argv, index)
        }
    }

    pub fn create_get_argument_variable_ptr(&self, index: u16) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_get_argument_variable_ptr(self.0, index)
        }
    }

    // retv

    pub fn create_retv(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_retv(self.0)
        }
    }

    pub fn create_store_undefined_to_retv(&self) {
        unsafe {
            bridge::compiler_peer_create_store_undefined_to_retv(self.0);
        }
    }

    pub fn create_store_null_to_retv(&self) {
        unsafe {
            bridge::compiler_peer_create_store_null_to_retv(self.0);
        }
    }

    pub fn create_store_boolean_to_retv(&self, value: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_boolean_to_retv(self.0, value);
        }
    }

    pub fn create_store_number_to_retv(&self, value: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_number_to_retv(self.0, value);
        }
    }

    pub fn create_store_closure_to_retv(&self, value: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_closure_to_retv(self.0, value);
        }
    }

    pub fn create_store_value_to_retv(&self, value: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_value_to_retv(self.0, value);
        }
    }

    // exception

    pub fn get_exception(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_get_exception(self.0)
        }
    }

    pub fn create_has_uncaught_exception(&self) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_has_uncaught_exception(self.0)
        }
    }

    // status

    pub fn create_alloc_status(&self) {
        unsafe {
            bridge::compiler_peer_create_alloc_status(self.0);
        }
    }

    pub fn create_store_normal_status(&self) {
        unsafe {
            bridge::compiler_peer_create_store_normal_status(self.0);
        }
    }

    pub fn create_store_exception_status(&self) {
        unsafe {
            bridge::compiler_peer_create_store_exception_status(self.0);
        }
    }

    pub fn create_is_exception_status(&self, status: bridge::ValueIr) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_is_exception_status(self.0, status)
        }
    }

    // locals

    pub fn create_local_variable(&self, index: u16) -> bridge::ValueIr {
        unsafe {
            bridge::compiler_peer_create_local_variable(self.0, index)
        }
    }

    pub fn create_store_flags_to_variable(&self, flags: u8, variable: bridge::ValueIr) {
        debug_assert_ne!(variable, 0);
        unsafe {
            bridge::compiler_peer_create_store_flags_to_variable(self.0, flags, variable)
        }
    }

    pub fn create_store_symbol_to_variable(&self, symbol: Symbol, variable: bridge::ValueIr) {
        debug_assert_ne!(variable, 0);
        unsafe {
            bridge::compiler_peer_create_store_symbol_to_variable(self.0, symbol.id(), variable);
        }
    }

    pub fn create_store_undefined_to_variable(&self, variable: bridge::ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_undefined_to_variable(self.0, variable);
        }
    }

    pub fn create_store_null_to_variable(&self, variable: bridge::ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_null_to_variable(self.0, variable);
        }
    }

    pub fn create_store_boolean_to_variable(&self, value: bridge::ValueIr, variable: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_boolean_to_variable(self.0, value, variable);
        }
    }

    pub fn create_store_number_to_variable(&self, value: bridge::ValueIr, variable: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_number_to_variable(self.0, value, variable);
        }
    }

    pub fn create_store_closure_to_variable(&self, value: bridge::ValueIr, variable: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_closure_to_variable(self.0, value, variable);
        }
    }

    pub fn create_store_value_to_variable(&self, value: bridge::ValueIr, variable: bridge::ValueIr) {
        debug_assert_ne!(value, 0);
        unsafe {
            bridge::compiler_peer_create_store_value_to_variable(self.0, value, variable);
        }
    }

    // scope cleanup checker

    pub fn process_prepare_scope_cleanup_checker(&self, stack_size: u16) {
        debug_assert!(stack_size > 0);
        unsafe {
            bridge::compiler_peer_prepare_scope_cleanup_checker(self.0, stack_size);
        }
    }

    pub fn start_scope_cleanup_checker(&self, scope_ref: ScopeRef) {
        unsafe {
            bridge::compiler_peer_start_scope_cleanup_checker(self.0, scope_ref.id());
        }
    }

    pub fn end_scope_cleanup_checker(&self, scope_ref: ScopeRef) {
        unsafe {
            bridge::compiler_peer_end_scope_cleanup_checker(self.0, scope_ref.id());
        }
    }

    pub fn create_store(&self, value: bridge::ValueIr, dest: bridge::ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store(self.0, value, dest);
        }
    }

    pub fn handle_returned_thrown(&self, returned: bool, thrown: bool, block: BasicBlock, cleanup_block: BasicBlock, exception_block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        // cleanup_block may NONE.
        // exception_block may NONE.
        unsafe {
            bridge::compiler_peer_handle_returned_thrown(
                self.0,
                returned,
                thrown,
                block.0,
                cleanup_block.0,
                exception_block.0,
            );
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new()
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            bridge::compiler_peer_delete(self.0);
        }
    }
}

impl BasicBlock {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_basic_block_name_or_as_operand(self.0, buf, len);
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

impl LambdaIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(self.0 as usize, buf, len); // TODO
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

pub fn get_value_name_or_as_operand<'a>(value: bridge::ValueIr, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
    unsafe {
        bridge::helper_peer_get_value_name_or_as_operand(value, buf, len);
        std::ffi::CStr::from_ptr(buf)
    }
}
