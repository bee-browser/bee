use std::ffi::CStr;

use crate::logger;
use crate::Module;

use super::bridge;
use super::FunctionId;
use super::ScopeRef;

pub struct Compiler(*mut bridge::Compiler);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BasicBlock(*mut bridge::BasicBlock);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LambdaIr(*mut bridge::LambdaIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BooleanIr(*mut bridge::BooleanIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NumberIr(*mut bridge::NumberIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClosureIr(*mut bridge::ClosureIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValueIr(*mut bridge::ValueIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArgvIr(*mut bridge::ArgvIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusIr(*mut bridge::StatusIr);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaptureIr(*mut bridge::CaptureIr);

macro_rules! boolean_ir {
    ($inner:expr) => {
        BooleanIr(unsafe { $inner })
    };
}

macro_rules! number_ir {
    ($inner:expr) => {
        NumberIr(unsafe { $inner })
    };
}

macro_rules! closure_ir {
    ($inner:expr) => {
        ClosureIr(unsafe { $inner })
    };
}

macro_rules! value_ir {
    ($inner:expr) => {
        ValueIr(unsafe { $inner })
    };
}

macro_rules! argv_ir {
    ($inner:expr) => {
        ArgvIr(unsafe { $inner })
    };
}

macro_rules! status_ir {
    ($inner:expr) => {
        StatusIr(unsafe { $inner })
    };
}

macro_rules! capture_ir {
    ($inner:expr) => {
        CaptureIr(unsafe { $inner })
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
            peer: unsafe { bridge::compiler_peer_end(self.0) },
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
            LambdaIr(bridge::compiler_peer_get_function(
                self.0,
                func_id.into(),
                name.as_ptr(),
            ))
        }
    }

    // basic block

    pub fn create_basic_block(&self, name: *const std::ffi::c_char, name_len: usize) -> BasicBlock {
        unsafe {
            BasicBlock(bridge::compiler_peer_create_basic_block(
                self.0, name, name_len,
            ))
        }
    }

    pub fn get_basic_block(&self) -> BasicBlock {
        unsafe { BasicBlock(bridge::compiler_peer_get_basic_block(self.0)) }
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
        unsafe { bridge::compiler_peer_is_basic_block_terminated(self.0, block.0) }
    }

    // jump

    pub fn create_br(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_br(self.0, block.0);
        }
    }

    pub fn create_cond_br(&self, cond: BooleanIr, then_block: BasicBlock, else_block: BasicBlock) {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        unsafe {
            bridge::compiler_peer_create_cond_br(self.0, cond.0, then_block.0, else_block.0);
        }
    }

    // undefined

    pub fn create_is_undefined(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_undefined(self.0, value.0)
        }
    }

    // null

    pub fn create_is_null(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_null(self.0, value.0)
        }
    }

    pub fn create_is_non_nullish(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_non_nullish(self.0, value.0)
        }
    }

    // boolean

    pub fn create_is_boolean(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_boolean(self.0, value.0)
        }
    }

    pub fn create_is_same_boolean(&self, a: BooleanIr, b: BooleanIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_same_boolean(self.0, a.0, b.0)
        }
    }

    pub fn create_number_to_boolean(&self, value: NumberIr) -> BooleanIr {
        debug_assert_ne!(value, NumberIr::NONE);
        boolean_ir! {
            bridge::compiler_peer_create_number_to_boolean(self.0, value.0)
        }
    }

    pub fn create_to_boolean(&self, value: ValueIr) -> BooleanIr {
        debug_assert_ne!(value, ValueIr::NONE);
        boolean_ir! {
            bridge::compiler_peer_create_to_boolean(self.0, value.0)
        }
    }

    pub fn get_boolean(&self, value: bool) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_get_boolean(self.0, value)
        }
    }

    pub fn create_logical_not(&self, boolean: BooleanIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_logical_not(self.0, boolean.0)
        }
    }

    pub fn create_boolean_phi(
        &self,
        then_value: BooleanIr,
        then_block: BasicBlock,
        else_value: BooleanIr,
        else_block: BasicBlock,
    ) -> BooleanIr {
        debug_assert_ne!(then_value, BooleanIr::NONE);
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_value, BooleanIr::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        boolean_ir! {
            bridge::compiler_peer_create_boolean_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // number

    pub fn create_is_number(&self, value: ValueIr) -> BooleanIr {
        logger::debug!(event = "create_is_number", ?value);
        boolean_ir! {
            bridge::compiler_peer_create_is_number(self.0, value.0)
        }
    }

    pub fn create_is_same_number(&self, a: NumberIr, b: NumberIr) -> BooleanIr {
        logger::debug!(event = "create_is_same_number", ?a, ?b);
        boolean_ir! {
            bridge::compiler_peer_create_is_same_number(self.0, a.0, b.0)
        }
    }

    pub fn create_boolean_to_number(&self, value: BooleanIr) -> NumberIr {
        debug_assert_ne!(value, BooleanIr::NONE);
        number_ir! {
            bridge::compiler_peer_create_boolean_to_number(self.0, value.0)
        }
    }

    pub fn to_numeric(&self, value: ValueIr) -> NumberIr {
        debug_assert_ne!(value, ValueIr::NONE);
        number_ir! {
            bridge::compiler_peer_to_numeric(self.0, value.0)
        }
    }

    pub fn get_nan(&self) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_get_nan(self.0)
        }
    }

    pub fn get_zero(&self) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_get_zero(self.0)
        }
    }

    pub fn get_number(&self, value: f64) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_get_number(self.0, value)
        }
    }

    pub fn create_bitwise_not(&self, value: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_bitwise_not(self.0, value.0)
        }
    }

    pub fn create_fneg(&self, value: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_fneg(self.0, value.0)
        }
    }

    pub fn create_fmul(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_fmul(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fdiv(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_fdiv(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_frem(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_frem(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fadd(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_fadd(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fsub(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_fsub(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_left_shift(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_left_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_signed_right_shift(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_signed_right_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_unsigned_right_shift(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_unsigned_right_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_and(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_bitwise_and(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_xor(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_bitwise_xor(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_or(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            bridge::compiler_peer_create_bitwise_or(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_less_than(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_less_than(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_greater_than(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_greater_than(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_less_than_or_equal(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_less_than_or_equal(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_greater_than_or_equal(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_greater_than_or_equal(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_number_phi(
        &self,
        then_value: NumberIr,
        then_block: BasicBlock,
        else_value: NumberIr,
        else_block: BasicBlock,
    ) -> NumberIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        logger::debug!(
            event = "create_number_phi",
            ?then_value,
            ?then_block,
            ?else_value,
            ?else_block
        );
        number_ir! {
            bridge::compiler_peer_create_number_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // closure

    pub fn create_is_closure(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_closure(self.0, value.0)
        }
    }

    pub fn create_is_same_closure(&self, a: ClosureIr, b: ClosureIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_same_closure(self.0, a.0, b.0)
        }
    }

    pub fn get_closure_nullptr(&self) -> ClosureIr {
        closure_ir! {
            bridge::compiler_peer_get_closure_nullptr(self.0)
        }
    }

    pub fn create_closure(&self, lambda: LambdaIr, num_captures: u16) -> ClosureIr {
        debug_assert_ne!(lambda, LambdaIr::NONE);
        closure_ir! {
            bridge::compiler_peer_create_closure(self.0, lambda.0, num_captures)
        }
    }

    pub fn create_store_capture_to_closure(
        &self,
        capture: CaptureIr,
        closure: ClosureIr,
        index: u16,
    ) {
        unsafe {
            bridge::compiler_peer_create_store_capture_to_closure(
                self.0, capture.0, closure.0, index,
            );
        }
    }

    pub fn create_call_on_closure(
        &self,
        closure: ClosureIr,
        argc: u16,
        argv: ArgvIr,
        retv: ValueIr,
    ) -> StatusIr {
        status_ir! {
            bridge::compiler_peer_create_call_on_closure(self.0, closure.0, argc, argv.0, retv.0)
        }
    }

    pub fn create_closure_phi(
        &self,
        then_value: ClosureIr,
        then_block: BasicBlock,
        else_value: ClosureIr,
        else_block: BasicBlock,
    ) -> ClosureIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        closure_ir! {
            bridge::compiler_peer_create_closure_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // value

    pub fn create_is_loosely_equal(&self, a: ValueIr, b: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_loosely_equal(self.0, a.0, b.0)
        }
    }

    pub fn create_is_strictly_equal(&self, a: ValueIr, b: ValueIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_strictly_equal(self.0, a.0, b.0)
        }
    }

    pub fn create_is_same_boolean_value(&self, any: ValueIr, boolean: BooleanIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_same_boolean_value(self.0, any.0, boolean.0)
        }
    }

    pub fn create_is_same_number_value(&self, any: ValueIr, number: NumberIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_same_number_value(self.0, any.0, number.0)
        }
    }

    pub fn create_is_same_closure_value(&self, any: ValueIr, closure: ClosureIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_same_closure_value(self.0, any.0, closure.0)
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

    pub fn create_boolean_to_any(&self, value: BooleanIr) -> ValueIr {
        debug_assert_ne!(value, BooleanIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_boolean_to_any(self.0, value.0)
        }
    }

    pub fn create_number_to_any(&self, value: NumberIr) -> ValueIr {
        debug_assert_ne!(value, NumberIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_number_to_any(self.0, value.0)
        }
    }

    pub fn create_closure_to_any(&self, value: ClosureIr) -> ValueIr {
        debug_assert_ne!(value, ClosureIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_closure_to_any(self.0, value.0)
        }
    }

    pub fn create_value_phi(
        &self,
        then_value: ValueIr,
        then_block: BasicBlock,
        else_value: ValueIr,
        else_block: BasicBlock,
    ) -> ValueIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        value_ir! {
            bridge::compiler_peer_create_value_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    pub fn create_local_value(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_local_value(self.0, index)
        }
    }

    pub fn create_store_none_to_value(&self, dest: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_none_to_value(self.0, dest.0);
        }
    }

    pub fn create_store_undefined_to_value(&self, dest: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_undefined_to_value(self.0, dest.0);
        }
    }

    pub fn create_store_null_to_value(&self, dest: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_store_null_to_value(self.0, dest.0);
        }
    }

    pub fn create_store_boolean_to_value(&self, value: BooleanIr, dest: ValueIr) {
        debug_assert_ne!(value, BooleanIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_boolean_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_number_to_value(&self, value: NumberIr, dest: ValueIr) {
        debug_assert_ne!(value, NumberIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_number_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_closure_to_value(&self, value: ClosureIr, dest: ValueIr) {
        debug_assert_ne!(value, ClosureIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_closure_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_value_to_value(&self, value: ValueIr, dest: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_value_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_load_closure_from_value(&self, value: ValueIr) -> ClosureIr {
        closure_ir! {
            bridge::compiler_peer_create_load_closure_from_value(self.0, value.0)
        }
    }

    // argv

    pub fn get_argv_nullptr(&self) -> ArgvIr {
        argv_ir! {
            bridge::compiler_peer_get_argv_nullptr(self.0)
        }
    }

    pub fn create_argv(&self, argc: u16) -> ArgvIr {
        debug_assert!(argc > 0);
        argv_ir! {
            bridge::compiler_peer_create_argv(self.0, argc)
        }
    }

    pub fn create_get_arg_in_argv(&self, argv: ArgvIr, index: u16) -> ValueIr {
        logger::debug!(event = "create_get_arg_in_argv", ?argv, index);
        debug_assert_ne!(argv, ArgvIr::NONE);
        value_ir! {
            bridge::compiler_peer_create_get_arg_in_argv(self.0, argv.0, index)
        }
    }

    pub fn create_get_argument_value_ptr(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_get_argument_value_ptr(self.0, index)
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

    pub fn create_store_boolean_to_retv(&self, value: BooleanIr) {
        debug_assert_ne!(value, BooleanIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_boolean_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_number_to_retv(&self, value: NumberIr) {
        debug_assert_ne!(value, NumberIr::NONE);
        unsafe {
            bridge::compiler_peer_create_store_number_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_closure_to_retv(&self, value: ClosureIr) {
        debug_assert_ne!(value, ClosureIr::NONE);
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

    pub fn get_exception(&self) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_get_exception(self.0)
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

    pub fn create_is_exception_status(&self, status: StatusIr) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_exception_status(self.0, status.0)
        }
    }

    // flow selector
    //
    // `break` and `continue` statements are generated unconditional branches in simple situation,
    // but conditional branches must be generated in complex situations.
    //
    // Let think about the following program:
    //
    //   for (;;) {
    //     let x;
    //     if (v == 0)
    //       return;
    //     // compute something using `x`.
    //     break;
    //   }
    //
    // The `x` variable defined in the scope of the for-loop body and it can be collected as
    // garbage once the execution goes out from the scope.  Depending on the algorithm of GC to
    // use, the runtime must do something for GC when the execution goes out from the scope.  In
    // this case, the control flow of the `return` and `break` statements are not determined at
    // compile time.  And a new variable must be needed in order to determine the control flow at
    // runtime.
    //
    // TODO: It might be possible to reuse the status variable instead of introducing the flow
    // selector.  The both are single variables inside a lambda and have common some values
    // partially.  If the status variable is reused, it must not be a single global variable.
    // Because the execution may suspend by `await`.
    //
    // TODO: This design is inefficient in a performance point of view, but it makes it possible to
    // support various GC algorithms.  In addition, we can optimize the runtime cost by removing
    // scope sub-graphs in CFG if those has no lexical variables.  We can detect such lexical scopes
    // in the semantic analysis phase.

    pub fn create_alloc_flow_selector(&self) {
        unsafe {
            bridge::compiler_peer_create_alloc_flow_selector(self.0);
        }
    }

    pub fn create_set_flow_selector_normal(&self) {
        unsafe {
            bridge::compiler_peer_create_set_flow_selector_normal(self.0);
        }
    }

    pub fn create_set_flow_selector_return(&self) {
        unsafe {
            bridge::compiler_peer_create_set_flow_selector_return(self.0);
        }
    }

    pub fn create_set_flow_selector_throw(&self) {
        unsafe {
            bridge::compiler_peer_create_set_flow_selector_throw(self.0);
        }
    }

    pub fn create_set_flow_selector_break(&self, depth: u32) {
        unsafe {
            bridge::compiler_peer_create_set_flow_selector_break(self.0, depth);
        }
    }

    pub fn create_set_flow_selector_continue(&self, depth: u32) {
        unsafe {
            bridge::compiler_peer_create_set_flow_selector_continue(self.0, depth);
        }
    }

    pub fn create_is_flow_selector_normal(&self) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_flow_selector_normal(self.0)
        }
    }

    pub fn create_is_flow_selector_normal_or_continue(&self, depth: u32) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_flow_selector_normal_or_continue(self.0, depth)
        }
    }

    pub fn create_is_flow_selector_break_or_continue(&self, depth: u32) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_flow_selector_break_or_continue(self.0, depth)
        }
    }

    pub fn create_is_flow_selector_break(&self, depth: u32) -> BooleanIr {
        boolean_ir! {
            bridge::compiler_peer_create_is_flow_selector_break(self.0, depth)
        }
    }

    // capture

    pub fn create_capture(&self, value: ValueIr) -> CaptureIr {
        capture_ir! {
            bridge::compiler_peer_create_capture(self.0, value.0)
        }
    }

    pub fn create_escape_value(&self, capture: CaptureIr, value: ValueIr) {
        unsafe {
            bridge::compiler_peer_create_escape_value(self.0, capture.0, value.0);
        }
    }

    pub fn create_get_capture_value_ptr(&self, index: u16) -> ValueIr {
        value_ir! {
            bridge::compiler_peer_create_get_capture_value_ptr(self.0, index)
        }
    }

    pub fn create_load_capture(&self, index: u16) -> CaptureIr {
        capture_ir! {
            bridge::compiler_peer_create_load_capture(self.0, index)
        }
    }

    // scope cleanup checker

    pub fn setup_scope_cleanup_checker(&self, stack_size: u16) {
        debug_assert!(stack_size > 0);
        unsafe {
            bridge::compiler_peer_setup_scope_cleanup_checker(self.0, stack_size);
        }
    }

    pub fn perform_scope_cleanup_precheck(&self, scope_ref: ScopeRef) {
        unsafe {
            bridge::compiler_peer_perform_scope_cleanup_precheck(self.0, scope_ref.id());
        }
    }

    pub fn perform_scope_cleanup_postcheck(&self, scope_ref: ScopeRef) {
        unsafe {
            bridge::compiler_peer_perform_scope_cleanup_postcheck(self.0, scope_ref.id());
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
            bridge::helper_peer_get_value_name_or_as_operand(
                self.0 as *mut bridge::ValueIr,
                buf,
                len,
            );
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

impl BooleanIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(
                self.0 as *mut bridge::ValueIr,
                buf,
                len,
            );
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

impl NumberIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(
                self.0 as *mut bridge::ValueIr,
                buf,
                len,
            );
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

impl ClosureIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(
                self.0 as *mut bridge::ValueIr,
                buf,
                len,
            );
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

impl ArgvIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(
                self.0 as *mut bridge::ValueIr,
                buf,
                len,
            );
            std::ffi::CStr::from_ptr(buf)
        }
    }
}

impl CaptureIr {
    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            bridge::helper_peer_get_value_name_or_as_operand(
                self.0 as *mut bridge::ValueIr,
                buf,
                len,
            );
            std::ffi::CStr::from_ptr(buf)
        }
    }
}
