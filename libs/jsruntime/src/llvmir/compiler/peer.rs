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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValueIr(*mut bridge::ValueIr);

macro_rules! value_ir {
    ($inner:expr) => {
        ValueIr(unsafe { $inner })
    };
}

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

    pub fn get_nullptr(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_nullptr(self.0)
        }
    }

    pub fn get_boolean(&self, value: bool) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_boolean(self.0, value)
        }
    }

    pub fn get_zero(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_zero(self.0)
        }
    }

    pub fn get_nan(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_nan(self.0)
        }
    }

    pub fn get_number(&self, value: f64) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_number(self.0, value)
        }
    }

    // conversions

    pub fn create_ui_to_fp(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_ui_to_fp(self.0, value.0)
        }
    }

    pub fn to_numeric(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_to_numeric(self.0, value.0)
        }
    }

    pub fn create_number_to_boolean(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_number_to_boolean(self.0, value.0)
        }
    }

    pub fn create_to_boolean(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_to_boolean(self.0, value.0)
        }
    }

    pub fn create_undefined_to_any(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_undefined_to_any(self.0)
        }
    }

    pub fn create_null_to_any(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_null_to_any(self.0)
        }
    }

    pub fn create_boolean_to_any(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_boolean_to_any(self.0, value.0)
        }
    }

    pub fn create_number_to_any(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_number_to_any(self.0, value.0)
        }
    }

    pub fn create_closure_to_any(&self, value: ValueIr) -> ValueIr {
        debug_assert_ne!(value, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_closure_to_any(self.0, value.0)
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

    pub fn create_fneg(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_fneg(self.0, value.0)
        }
    }

    pub fn create_bitwise_not(&self, number: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_bitwise_not(self.0, number.0)
        }
    }

    pub fn create_logical_not(&self, boolean: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_logical_not(self.0, boolean.0)
        }
    }

    // binary operators

    pub fn create_fmul(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_fmul(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fdiv(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_fdiv(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_frem(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_frem(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fadd(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_fadd(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fsub(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_fsub(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_left_shift(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_left_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_signed_right_shift(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_signed_right_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_unsigned_right_shift(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_unsigned_right_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_less_than(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_less_than(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_greater_than(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_greater_than(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_less_than_or_equal(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_less_than_or_equal(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_greater_than_or_equal(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_greater_than_or_equal(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_and(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_bitwise_and(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_xor(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_bitwise_xor(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_or(&self, lhs: ValueIr, rhs: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_bitwise_or(self.0, lhs.0, rhs.0)
        }
    }

    // incr/decr

    pub fn create_incr(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_incr(self.0, value.0)
        }
    }

    pub fn create_decr(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_decr(self.0, value.0)
        }
    }

    // equality

    pub fn create_is_loosely_equal(&self, a: ValueIr, b: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_loosely_equal(self.0, a.0, b.0)
        }
    }

    pub fn create_is_strictly_equal(&self, a: ValueIr, b: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_strictly_equal(self.0, a.0, b.0)
        }
    }

    pub fn create_is_undefined(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_undefined(self.0, value.0)
        }
    }

    pub fn create_is_null(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_null(self.0, value.0)
        }
    }

    pub fn create_is_boolean(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_boolean(self.0, value.0)
        }
    }

    pub fn create_is_same_boolean(&self, a: ValueIr, b: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_same_boolean(self.0, a.0, b.0)
        }
    }

    pub fn create_is_same_boolean_value(&self, variable: ValueIr, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_same_boolean_value(self.0, variable.0, value.0)
        }
    }

    pub fn create_is_number(&self, value: ValueIr) -> ValueIr {
        logger::debug!(event = "create_is_number", ?value);
        value_ir! {
            bridge::compiler_peer_create_is_number(self.0, value.0)
        }
    }

    pub fn create_is_same_number(&self, a: ValueIr, b: ValueIr) -> ValueIr {
        logger::debug!(event = "create_is_same_number", ?a, ?b);
        value_ir! {
            bridge::compiler_peer_create_is_same_number(self.0, a.0, b.0)
        }
    }

    pub fn create_is_same_number_value(&self, variable: ValueIr, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_same_number_value(self.0, variable.0, value.0)
        }
    }

    pub fn create_is_closure(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_closure(self.0, value.0)
        }
    }

    pub fn create_is_same_closure(&self, a: ValueIr, b: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_same_closure(self.0, a.0, b.0)
        }
    }

    pub fn create_is_same_closure_value(&self, variable: ValueIr, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_same_closure_value(self.0, variable.0, value.0)
        }
    }

    // jump

    pub fn create_cond_br(&self, cond: ValueIr, then_block: BasicBlock, else_block: BasicBlock) {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_cond_br(self.0, cond.0, then_block.0, else_block.0);
        }
    }

    pub fn create_is_non_nullish(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_non_nullish(self.0, value.0)
        }
    }

    // phi

    pub fn create_boolean_ternary(&self, then_value: ValueIr, then_block: BasicBlock, else_value: ValueIr, else_block: BasicBlock) -> ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        value_ir! {
            bridge::compiler_peer_create_boolean_ternary(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    pub fn create_number_ternary(&self, then_value: ValueIr, then_block: BasicBlock, else_value: ValueIr, else_block: BasicBlock) -> ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        logger::debug!(event = "create_number_ternary", ?then_value, ?then_block, ?else_value, ?else_block);
        value_ir! {
            bridge::compiler_peer_create_number_ternary(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    pub fn create_any_ternary(&self, then_value: ValueIr, then_block: BasicBlock, else_value: ValueIr, else_block: BasicBlock) -> ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        value_ir! {
            bridge::compiler_peer_create_any_ternary(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // closure

    pub fn create_closure_ptr(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_closure_ptr(self.0)
        }
    }

    pub fn create_call_on_closure(&self, closure: ValueIr, argc: u16, argv: ValueIr, retv: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_call_on_closure(self.0, closure.0, argc, argv.0, retv.0)
        }
    }

    pub fn create_call_runtime_create_closure(&self, lambda: LambdaIr, num_captures: u16) -> ValueIr {
        debug_assert_ne!(lambda, LambdaIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_call_runtime_create_closure(self.0, lambda.0, num_captures)
        }
    }

    pub fn create_load_captures_from_closure(&self, closure: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_load_captures_from_closure(self.0, closure.0)
        }
    }

    pub fn create_store_capture_ptr_to_captures(&self, capture: ValueIr, captures: ValueIr, index: u16) {
        unsafe {
            bridge::compiler_peer_create_store_capture_ptr_to_captures(self.0, capture.0, captures.0, index);
        }
    }

    pub fn create_load_closure_from_value(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_load_closure_from_value(self.0, value.0)
        }
    }

    pub fn create_load_closure(&self, value: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_load_closure(self.0, value.0)
        }
    }

    // capture

    pub fn create_capture(&self, variable: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_call_runtime_create_capture(self.0, variable.0)
        }
    }

    pub fn create_escape_variable(&self, capture: ValueIr, variable: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_escape_variable(self.0, capture.0, variable.0);
        }
    }

    pub fn create_get_capture_variable_ptr(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_get_capture_variable_ptr(self.0, index)
        }
    }

    pub fn create_load_capture(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_load_capture(self.0, index)
        }
    }

    // argv

    pub fn create_argv(&self, argc: u16) -> ValueIr {
        debug_assert!(argc > 0);
        value_ir! {
            bridge::compiler_peer_create_argv(self.0, argc)
        }
    }

    pub fn create_get_arg_in_argv(&self, argv: ValueIr, index: u16) -> ValueIr {
        debug_assert_ne!(argv, ValueIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_get_arg_in_argv(self.0, argv.0, index)
        }
    }

    pub fn create_get_argument_variable_ptr(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_get_argument_variable_ptr(self.0, index)
        }
    }

    // retv

    pub fn create_retv(&self) -> ValueIr {
        value_ir! {
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

    pub fn create_store_boolean_to_retv(&self, value: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_boolean_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_number_to_retv(&self, value: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_number_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_closure_to_retv(&self, value: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_closure_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_value_to_retv(&self, value: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_value_to_retv(self.0, value.0);
        }
    }

    // exception

    pub fn get_exception(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_exception(self.0)
        }
    }

    pub fn create_has_uncaught_exception(&self) -> ValueIr {
        value_ir! {
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

    pub fn create_is_exception_status(&self, status: ValueIr) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_is_exception_status(self.0, status.0)
        }
    }

    // locals

    pub fn create_local_variable(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_local_variable(self.0, index)
        }
    }

    pub fn create_store_flags_to_variable(&self, flags: u8, variable: ValueIr) {
        debug_assert_ne!(variable, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_flags_to_variable(self.0, flags, variable.0)
        }
    }

    pub fn create_store_symbol_to_variable(&self, symbol: Symbol, variable: ValueIr) {
        debug_assert_ne!(variable, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_symbol_to_variable(self.0, symbol.id(), variable.0);
        }
    }

    pub fn create_store_undefined_to_variable(&self, variable: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_undefined_to_variable(self.0, variable.0);
        }
    }

    pub fn create_store_null_to_variable(&self, variable: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_null_to_variable(self.0, variable.0);
        }
    }

    pub fn create_store_boolean_to_variable(&self, value: ValueIr, variable: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_boolean_to_variable(self.0, value.0, variable.0);
        }
    }

    pub fn create_store_number_to_variable(&self, value: ValueIr, variable: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_number_to_variable(self.0, value.0, variable.0);
        }
    }

    pub fn create_store_closure_to_variable(&self, value: ValueIr, variable: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_closure_to_variable(self.0, value.0, variable.0);
        }
    }

    pub fn create_store_value_to_variable(&self, value: ValueIr, variable: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_value_to_variable(self.0, value.0, variable.0);
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

    pub fn create_store(&self, value: ValueIr, dest: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store(self.0, value.0, dest.0);
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
            bridge::helper_peer_get_value_name_or_as_operand(self.0 as *mut bridge::ValueIr, buf, len);
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

impl ValueIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(self.0, buf, len);
            std::ffi::CStr::from_ptr(buf)
        }
    }
}
