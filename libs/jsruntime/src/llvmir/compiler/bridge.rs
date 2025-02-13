use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;

use paste::paste;

use jsparser::Symbol;

use crate::llvmir::module::ModulePeer;
use crate::logger;
use crate::semantics::ScopeRef;
use crate::LambdaId;

pub struct CompilerBridge(CompilerPeer);

// TODO(refactor): Case conversion macros such as `case-macro` does not work inside macro_rules
// properly as we expect...
//
// For example, the following macro definition didn't work:
//
// ```rust
// macro_rules! deifne_ir_types {
//     ($($name:ident,)*) => {
//         $(define_ir_types! {$name, case_macro::snake_case!($name)})*
//     };
// }
// ```
macro_rules! define_ir_types {
    ($($name:ident; $macro:ident,)*) => {
        $(define_ir_types! {$name, $macro})*
    };
    ($name:ident, $macro:ident) => {
        paste! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct $name([<$name Ptr>]);

            macro_rules! $macro {
                ($inner:expr) => {
                    $name(unsafe { $inner })
                }
            }
        }
    };
}

define_ir_types! {
    BasicBlock; basic_block,
    LambdaIr; lambda_ir,
    BooleanIr; boolean_ir,
    NumberIr; number_ir,
    Char16SeqIr; char16_seq_ir,
    ClosureIr; closure_ir,
    CoroutineIr; coroutine_ir,
    PromiseIr; promise_ir,
    ObjectIr; object_ir,
    ValueIr; value_ir,
    ArgvIr; argv_ir,
    StatusIr; status_ir,
    CaptureIr; capture_ir,
    SwitchIr; switch_ir,
}

impl CompilerBridge {
    pub fn new() -> Self {
        Self(unsafe { compiler_peer_new() })
    }

    pub fn start_compile(&self) {
        unsafe {
            compiler_peer_start(self.0);
        }
    }

    pub fn end_compile(&self) -> ModulePeer {
        unsafe { compiler_peer_end(self.0) }
    }

    pub fn set_data_layout(&self, data_layout: &CStr) {
        unsafe {
            compiler_peer_set_data_layout(self.0, data_layout.as_ptr());
        }
    }

    pub fn set_target_triple(&self, triple: &CStr) {
        unsafe {
            compiler_peer_set_target_triple(self.0, triple.as_ptr());
        }
    }

    pub fn enable_labels(&self) {
        unsafe {
            compiler_peer_enable_labels(self.0);
        }
    }

    // function

    pub fn start_function(&self, lambda_id: LambdaId) {
        unsafe {
            compiler_peer_start_function(self.0, lambda_id.into());
        }
    }

    pub fn end_function(&self, optimize: bool) {
        unsafe {
            compiler_peer_end_function(self.0, optimize);
        }
    }

    pub fn set_locals_block(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            compiler_peer_set_locals_block(self.0, block.0);
        }
    }

    pub fn get_function(&self, lambda_id: LambdaId) -> LambdaIr {
        lambda_ir!(compiler_peer_get_function(self.0, lambda_id.into()))
    }

    // basic block

    pub fn create_basic_block(&self, name: *const std::ffi::c_char, name_len: usize) -> BasicBlock {
        basic_block!(compiler_peer_create_basic_block(self.0, name, name_len))
    }

    pub fn get_basic_block(&self) -> BasicBlock {
        basic_block!(compiler_peer_get_basic_block(self.0))
    }

    pub fn set_basic_block(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            compiler_peer_set_basic_block(self.0, block.0);
        }
    }

    pub fn move_basic_block_after(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            compiler_peer_move_basic_block_after(self.0, block.0);
        }
    }

    pub fn is_basic_block_terminated(&self, block: BasicBlock) -> bool {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe { compiler_peer_is_basic_block_terminated(self.0, block.0) }
    }

    // jump

    pub fn create_br(&self, block: BasicBlock) {
        debug_assert_ne!(block, BasicBlock::NONE);
        unsafe {
            compiler_peer_create_br(self.0, block.0);
        }
    }

    pub fn create_cond_br(&self, cond: BooleanIr, then_block: BasicBlock, else_block: BasicBlock) {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        unsafe {
            compiler_peer_create_cond_br(self.0, cond.0, then_block.0, else_block.0);
        }
    }

    // undefined

    pub fn create_is_undefined(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_undefined(self.0, value.0)
        }
    }

    // null

    pub fn create_is_null(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_null(self.0, value.0)
        }
    }

    pub fn create_is_non_nullish(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_non_nullish(self.0, value.0)
        }
    }

    // boolean

    pub fn create_is_boolean(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_boolean(self.0, value.0)
        }
    }

    pub fn create_is_same_boolean(&self, a: BooleanIr, b: BooleanIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_boolean(self.0, a.0, b.0)
        }
    }

    pub fn create_number_to_boolean(&self, value: NumberIr) -> BooleanIr {
        debug_assert_ne!(value, NumberIr::NONE);
        boolean_ir! {
            compiler_peer_create_number_to_boolean(self.0, value.0)
        }
    }

    pub fn create_to_boolean(&self, value: ValueIr) -> BooleanIr {
        debug_assert_ne!(value, ValueIr::NONE);
        boolean_ir! {
            compiler_peer_create_to_boolean(self.0, value.0)
        }
    }

    pub fn get_boolean(&self, value: bool) -> BooleanIr {
        boolean_ir! {
            compiler_peer_get_boolean(self.0, value)
        }
    }

    pub fn create_logical_not(&self, boolean: BooleanIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_logical_not(self.0, boolean.0)
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
            compiler_peer_create_boolean_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // number

    pub fn create_is_number(&self, value: ValueIr) -> BooleanIr {
        logger::debug!(event = "create_is_number", ?value);
        boolean_ir! {
            compiler_peer_create_is_number(self.0, value.0)
        }
    }

    pub fn create_is_same_number(&self, a: NumberIr, b: NumberIr) -> BooleanIr {
        logger::debug!(event = "create_is_same_number", ?a, ?b);
        boolean_ir! {
            compiler_peer_create_is_same_number(self.0, a.0, b.0)
        }
    }

    pub fn create_boolean_to_number(&self, value: BooleanIr) -> NumberIr {
        debug_assert_ne!(value, BooleanIr::NONE);
        number_ir! {
            compiler_peer_create_boolean_to_number(self.0, value.0)
        }
    }

    pub fn to_numeric(&self, value: ValueIr) -> NumberIr {
        debug_assert_ne!(value, ValueIr::NONE);
        number_ir! {
            compiler_peer_to_numeric(self.0, value.0)
        }
    }

    pub fn get_nan(&self) -> NumberIr {
        number_ir! {
            compiler_peer_get_nan(self.0)
        }
    }

    pub fn get_zero(&self) -> NumberIr {
        number_ir! {
            compiler_peer_get_zero(self.0)
        }
    }

    pub fn get_number(&self, value: f64) -> NumberIr {
        number_ir! {
            compiler_peer_get_number(self.0, value)
        }
    }

    pub fn create_bitwise_not(&self, value: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_bitwise_not(self.0, value.0)
        }
    }

    pub fn create_fneg(&self, value: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_fneg(self.0, value.0)
        }
    }

    pub fn create_fmul(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_fmul(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fdiv(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_fdiv(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_frem(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_frem(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fadd(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_fadd(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_fsub(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_fsub(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_left_shift(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_left_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_signed_right_shift(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_signed_right_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_unsigned_right_shift(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_unsigned_right_shift(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_and(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_bitwise_and(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_xor(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_bitwise_xor(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_bitwise_or(&self, lhs: NumberIr, rhs: NumberIr) -> NumberIr {
        number_ir! {
            compiler_peer_create_bitwise_or(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_less_than(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_less_than(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_greater_than(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_greater_than(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_less_than_or_equal(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_less_than_or_equal(self.0, lhs.0, rhs.0)
        }
    }

    pub fn create_greater_than_or_equal(&self, lhs: NumberIr, rhs: NumberIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_greater_than_or_equal(self.0, lhs.0, rhs.0)
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
            compiler_peer_create_number_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // string

    pub fn create_char16_seq(&self, seq: &[u16]) -> Char16SeqIr {
        char16_seq_ir! {
            compiler_peer_create_char16_seq(self.0, seq.as_ptr(), seq.len() as u32)
        }
    }

    pub fn create_string_on_stack(&self, value: Char16SeqIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_string_on_stack(self.0, value.0)
        }
    }

    pub fn create_migrate_string_to_heap(&self, value: Char16SeqIr) -> Char16SeqIr {
        char16_seq_ir! {
            compiler_peer_create_migrate_string_to_heap(self.0, value.0)
        }
    }

    pub fn create_string_phi(
        &self,
        then_value: Char16SeqIr,
        then_block: BasicBlock,
        else_value: Char16SeqIr,
        else_block: BasicBlock,
    ) -> Char16SeqIr {
        debug_assert_ne!(then_block, BasicBlock::NONE);
        debug_assert_ne!(else_block, BasicBlock::NONE);
        logger::debug!(
            event = "create_string_phi",
            ?then_value,
            ?then_block,
            ?else_value,
            ?else_block
        );
        char16_seq_ir! {
            compiler_peer_create_string_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // closure

    pub fn get_closure_nullptr(&self) -> ClosureIr {
        closure_ir! {
            compiler_peer_get_closure_nullptr(self.0)
        }
    }

    pub fn create_is_closure(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_closure(self.0, value.0)
        }
    }

    pub fn create_is_same_closure(&self, a: ClosureIr, b: ClosureIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_closure(self.0, a.0, b.0)
        }
    }

    pub fn create_closure(&self, lambda: LambdaIr, num_captures: u16) -> ClosureIr {
        debug_assert_ne!(lambda, LambdaIr::NONE);
        closure_ir! {
            compiler_peer_create_closure(self.0, lambda.0, num_captures)
        }
    }

    pub fn create_store_capture_to_closure(
        &self,
        capture: CaptureIr,
        closure: ClosureIr,
        index: u16,
    ) {
        unsafe {
            compiler_peer_create_store_capture_to_closure(self.0, capture.0, closure.0, index);
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
            compiler_peer_create_call_on_closure(self.0, closure.0, argc, argv.0, retv.0)
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
            compiler_peer_create_closure_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    // promise

    pub fn create_is_promise(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_promise(self.0, value.0)
        }
    }

    pub fn create_is_same_promise(&self, a: PromiseIr, b: PromiseIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_promise(self.0, a.0, b.0)
        }
    }

    pub fn create_register_promise(&self, coroutine: CoroutineIr) -> PromiseIr {
        promise_ir! {
            compiler_peer_create_register_promise(self.0, coroutine.0)
        }
    }

    pub fn create_await_promise(&self, promise: PromiseIr, awaiting: PromiseIr) {
        unsafe {
            compiler_peer_create_await_promise(self.0, promise.0, awaiting.0);
        }
    }

    pub fn create_resume(&self, promise: PromiseIr) {
        unsafe {
            compiler_peer_create_resume(self.0, promise.0);
        }
    }

    pub fn create_emit_promise_resolved(&self, promise: PromiseIr, result: ValueIr) {
        unsafe {
            compiler_peer_create_emit_promise_resolved(self.0, promise.0, result.0);
        }
    }

    // object

    pub fn get_object_nullptr(&self) -> ObjectIr {
        object_ir! {
            compiler_peer_get_object_nullptr(self.0)
        }
    }

    pub fn create_is_object(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_object(self.0, value.0)
        }
    }

    pub fn create_is_same_object(&self, a: ObjectIr, b: ObjectIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_object(self.0, a.0, b.0)
        }
    }

    pub fn create_to_object(&self, value: ValueIr) -> ObjectIr {
        debug_assert_ne!(value, ValueIr::NONE);
        object_ir! {
            compiler_peer_create_to_object(self.0, value.0)
        }
    }

    pub fn create_object(&self) -> ObjectIr {
        object_ir! {
            compiler_peer_create_object(self.0)
        }
    }

    pub fn create_get_value_by_symbol(
        &self,
        object: ObjectIr,
        key: Symbol,
        strict: bool,
    ) -> ValueIr {
        value_ir! {
            compiler_peer_create_get_value_by_symbol(self.0, object.0, key.id(), strict)
        }
    }

    pub fn create_get_value_by_number(&self, object: ObjectIr, key: f64, strict: bool) -> ValueIr {
        value_ir! {
            compiler_peer_create_get_value_by_number(self.0, object.0, key, strict)
        }
    }

    pub fn create_get_value_by_value(
        &self,
        object: ObjectIr,
        key: ValueIr,
        strict: bool,
    ) -> ValueIr {
        value_ir! {
            compiler_peer_create_get_value_by_value(self.0, object.0, key.0, strict)
        }
    }

    pub fn create_set_value_by_symbol(&self, object: ObjectIr, key: Symbol, value: ValueIr) {
        unsafe {
            compiler_peer_create_set_value_by_symbol(self.0, object.0, key.id(), value.0);
        }
    }

    pub fn create_set_value_by_number(&self, object: ObjectIr, key: f64, value: ValueIr) {
        unsafe {
            compiler_peer_create_set_value_by_number(self.0, object.0, key, value.0);
        }
    }

    pub fn create_set_value_by_value(&self, object: ObjectIr, key: ValueIr, value: ValueIr) {
        unsafe {
            compiler_peer_create_set_value_by_value(self.0, object.0, key.0, value.0);
        }
    }

    // 7.3.5 CreateDataProperty ( O, P, V )

    pub fn create_create_data_property_by_symbol(
        &self,
        object: ObjectIr,
        key: Symbol,
        value: ValueIr,
        retv: ValueIr,
    ) -> StatusIr {
        status_ir! {
            compiler_peer_create_create_data_property_by_symbol(self.0, object.0, key.id(), value.0, retv.0)
        }
    }

    pub fn create_create_data_property_by_number(
        &self,
        object: ObjectIr,
        key: f64,
        value: ValueIr,
        retv: ValueIr,
    ) -> StatusIr {
        status_ir! {
            compiler_peer_create_create_data_property_by_number(self.0, object.0, key, value.0, retv.0)
        }
    }
    pub fn create_create_data_property_by_value(
        &self,
        object: ObjectIr,
        key: ValueIr,
        value: ValueIr,
        retv: ValueIr,
    ) -> StatusIr {
        status_ir! {
            compiler_peer_create_create_data_property_by_value(self.0, object.0, key.0, value.0, retv.0)
        }
    }

    // 7.3.25 CopyDataProperties ( target, source, excludedItems )
    pub fn create_copy_data_properties(
        &self,
        target: ObjectIr,
        source: ValueIr,
        retv: ValueIr,
    ) -> StatusIr {
        status_ir! {
            compiler_peer_create_copy_data_properties(self.0, target.0, source.0, retv.0)
        }
    }

    // value

    pub fn create_value_is_nullptr(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_value_is_nullptr(self.0, value.0)
        }
    }

    pub fn create_has_value(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_has_value(self.0, value.0)
        }
    }

    pub fn create_is_loosely_equal(&self, a: ValueIr, b: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_loosely_equal(self.0, a.0, b.0)
        }
    }

    pub fn create_is_strictly_equal(&self, a: ValueIr, b: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_strictly_equal(self.0, a.0, b.0)
        }
    }

    pub fn create_is_same_boolean_value(&self, any: ValueIr, boolean: BooleanIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_boolean_value(self.0, any.0, boolean.0)
        }
    }

    pub fn create_is_same_number_value(&self, any: ValueIr, number: NumberIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_number_value(self.0, any.0, number.0)
        }
    }

    pub fn create_is_same_closure_value(&self, any: ValueIr, closure: ClosureIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_closure_value(self.0, any.0, closure.0)
        }
    }

    pub fn create_is_same_promise_value(&self, any: ValueIr, promise: PromiseIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_promise_value(self.0, any.0, promise.0)
        }
    }

    pub fn create_is_same_object_value(&self, any: ValueIr, object: ObjectIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_same_object_value(self.0, any.0, object.0)
        }
    }

    pub fn create_undefined_to_any(&self) -> ValueIr {
        value_ir! {
            compiler_peer_create_undefined_to_any(self.0)
        }
    }

    pub fn create_null_to_any(&self) -> ValueIr {
        value_ir! {
            compiler_peer_create_null_to_any(self.0)
        }
    }

    pub fn create_boolean_to_any(&self, value: BooleanIr) -> ValueIr {
        debug_assert_ne!(value, BooleanIr::NONE);
        value_ir! {
            compiler_peer_create_boolean_to_any(self.0, value.0)
        }
    }

    pub fn create_number_to_any(&self, value: NumberIr) -> ValueIr {
        debug_assert_ne!(value, NumberIr::NONE);
        value_ir! {
            compiler_peer_create_number_to_any(self.0, value.0)
        }
    }

    pub fn create_string_to_any(&self, value: Char16SeqIr) -> ValueIr {
        debug_assert_ne!(value, Char16SeqIr::NONE);
        value_ir! {
            compiler_peer_create_string_to_any(self.0, value.0)
        }
    }

    pub fn create_closure_to_any(&self, value: ClosureIr) -> ValueIr {
        debug_assert_ne!(value, ClosureIr::NONE);
        value_ir! {
            compiler_peer_create_closure_to_any(self.0, value.0)
        }
    }

    pub fn create_object_to_any(&self, value: ObjectIr) -> ValueIr {
        debug_assert_ne!(value, ObjectIr::NONE);
        value_ir! {
            compiler_peer_create_object_to_any(self.0, value.0)
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
            compiler_peer_create_value_phi(self.0, then_value.0, then_block.0, else_value.0, else_block.0)
        }
    }

    pub fn create_local_value(&self, index: u16) -> ValueIr {
        value_ir! {
            compiler_peer_create_local_value(self.0, index)
        }
    }

    pub fn create_store_none_to_value(&self, dest: ValueIr) {
        unsafe {
            compiler_peer_create_store_none_to_value(self.0, dest.0);
        }
    }

    pub fn create_store_undefined_to_value(&self, dest: ValueIr) {
        unsafe {
            compiler_peer_create_store_undefined_to_value(self.0, dest.0);
        }
    }

    pub fn create_store_null_to_value(&self, dest: ValueIr) {
        unsafe {
            compiler_peer_create_store_null_to_value(self.0, dest.0);
        }
    }

    pub fn create_store_boolean_to_value(&self, value: BooleanIr, dest: ValueIr) {
        debug_assert_ne!(value, BooleanIr::NONE);
        unsafe {
            compiler_peer_create_store_boolean_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_number_to_value(&self, value: NumberIr, dest: ValueIr) {
        debug_assert_ne!(value, NumberIr::NONE);
        unsafe {
            compiler_peer_create_store_number_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_string_to_value(&self, value: Char16SeqIr, dest: ValueIr) {
        debug_assert_ne!(value, Char16SeqIr::NONE);
        unsafe {
            compiler_peer_create_store_string_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_closure_to_value(&self, value: ClosureIr, dest: ValueIr) {
        debug_assert_ne!(value, ClosureIr::NONE);
        unsafe {
            compiler_peer_create_store_closure_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_promise_to_value(&self, value: PromiseIr, dest: ValueIr) {
        debug_assert_ne!(value, PromiseIr::NONE);
        unsafe {
            compiler_peer_create_store_promise_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_object_to_value(&self, value: ObjectIr, dest: ValueIr) {
        debug_assert_ne!(value, ObjectIr::NONE);
        unsafe {
            compiler_peer_create_store_object_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_store_value_to_value(&self, value: ValueIr, dest: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            compiler_peer_create_store_value_to_value(self.0, value.0, dest.0);
        }
    }

    pub fn create_load_boolean_from_value(&self, value: ValueIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_load_boolean_from_value(self.0, value.0)
        }
    }

    pub fn create_load_closure_from_value(&self, value: ValueIr) -> ClosureIr {
        closure_ir! {
            compiler_peer_create_load_closure_from_value(self.0, value.0)
        }
    }

    pub fn create_load_promise_from_value(&self, value: ValueIr) -> PromiseIr {
        promise_ir! {
            compiler_peer_create_load_promise_from_value(self.0, value.0)
        }
    }

    pub fn create_typeof(&self, value: ValueIr) -> Char16SeqIr {
        char16_seq_ir! {
            compiler_peer_create_typeof(self.0, value.0)
        }
    }

    // argv

    pub fn get_argv_nullptr(&self) -> ArgvIr {
        argv_ir! {
            compiler_peer_get_argv_nullptr(self.0)
        }
    }

    pub fn create_argv(&self, argc: u16) -> ArgvIr {
        debug_assert!(argc > 0);
        argv_ir! {
            compiler_peer_create_argv(self.0, argc)
        }
    }

    pub fn create_get_arg_in_argv(&self, argv: ArgvIr, index: u16) -> ValueIr {
        logger::debug!(event = "create_get_arg_in_argv", ?argv, index);
        debug_assert_ne!(argv, ArgvIr::NONE);
        value_ir! {
            compiler_peer_create_get_arg_in_argv(self.0, argv.0, index)
        }
    }

    pub fn create_get_argument_value_ptr(&self, index: u16) -> ValueIr {
        value_ir! {
            compiler_peer_create_get_argument_value_ptr(self.0, index)
        }
    }

    // retv

    pub fn create_retv(&self) -> ValueIr {
        value_ir! {
            compiler_peer_create_retv(self.0)
        }
    }

    pub fn create_store_undefined_to_retv(&self) {
        unsafe {
            compiler_peer_create_store_undefined_to_retv(self.0);
        }
    }

    pub fn create_store_null_to_retv(&self) {
        unsafe {
            compiler_peer_create_store_null_to_retv(self.0);
        }
    }

    pub fn create_store_boolean_to_retv(&self, value: BooleanIr) {
        debug_assert_ne!(value, BooleanIr::NONE);
        unsafe {
            compiler_peer_create_store_boolean_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_number_to_retv(&self, value: NumberIr) {
        debug_assert_ne!(value, NumberIr::NONE);
        unsafe {
            compiler_peer_create_store_number_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_closure_to_retv(&self, value: ClosureIr) {
        debug_assert_ne!(value, ClosureIr::NONE);
        unsafe {
            compiler_peer_create_store_closure_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_promise_to_retv(&self, value: PromiseIr) {
        debug_assert_ne!(value, PromiseIr::NONE);
        unsafe {
            compiler_peer_create_store_promise_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_object_to_retv(&self, value: ObjectIr) {
        debug_assert_ne!(value, ObjectIr::NONE);
        unsafe {
            compiler_peer_create_store_object_to_retv(self.0, value.0);
        }
    }

    pub fn create_store_value_to_retv(&self, value: ValueIr) {
        debug_assert_ne!(value, ValueIr::NONE);
        unsafe {
            compiler_peer_create_store_value_to_retv(self.0, value.0);
        }
    }

    pub fn get_exception(&self) -> ValueIr {
        value_ir! {
            compiler_peer_get_exception(self.0)
        }
    }

    // status

    pub fn create_alloc_status(&self) {
        unsafe {
            compiler_peer_create_alloc_status(self.0);
        }
    }

    pub fn create_store_normal_status(&self) {
        unsafe {
            compiler_peer_create_store_normal_status(self.0);
        }
    }

    pub fn create_store_exception_status(&self) {
        unsafe {
            compiler_peer_create_store_exception_status(self.0);
        }
    }

    pub fn create_is_exception_status(&self, status: StatusIr) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_exception_status(self.0, status.0)
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
            compiler_peer_create_alloc_flow_selector(self.0);
        }
    }

    pub fn create_set_flow_selector_normal(&self) {
        unsafe {
            compiler_peer_create_set_flow_selector_normal(self.0);
        }
    }

    pub fn create_set_flow_selector_return(&self) {
        unsafe {
            compiler_peer_create_set_flow_selector_return(self.0);
        }
    }

    pub fn create_set_flow_selector_throw(&self) {
        unsafe {
            compiler_peer_create_set_flow_selector_throw(self.0);
        }
    }

    pub fn create_set_flow_selector_break(&self, depth: u32) {
        unsafe {
            compiler_peer_create_set_flow_selector_break(self.0, depth);
        }
    }

    pub fn create_set_flow_selector_continue(&self, depth: u32) {
        unsafe {
            compiler_peer_create_set_flow_selector_continue(self.0, depth);
        }
    }

    pub fn create_is_flow_selector_normal(&self) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_flow_selector_normal(self.0)
        }
    }

    pub fn create_is_flow_selector_normal_or_continue(&self, depth: u32) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_flow_selector_normal_or_continue(self.0, depth)
        }
    }

    pub fn create_is_flow_selector_break_or_continue(&self, depth: u32) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_flow_selector_break_or_continue(self.0, depth)
        }
    }

    pub fn create_is_flow_selector_break(&self, depth: u32) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_is_flow_selector_break(self.0, depth)
        }
    }

    // capture

    pub fn create_capture(&self, value: ValueIr) -> CaptureIr {
        capture_ir! {
            compiler_peer_create_capture(self.0, value.0)
        }
    }

    pub fn create_escape_value(&self, capture: CaptureIr, value: ValueIr) {
        unsafe {
            compiler_peer_create_escape_value(self.0, capture.0, value.0);
        }
    }

    pub fn create_get_capture_value_ptr(&self, index: u16) -> ValueIr {
        value_ir! {
            compiler_peer_create_get_capture_value_ptr(self.0, index)
        }
    }

    pub fn create_load_capture(&self, index: u16) -> CaptureIr {
        capture_ir! {
            compiler_peer_create_load_capture(self.0, index)
        }
    }

    // coroutine

    pub fn create_coroutine(
        &self,
        closure: ClosureIr,
        num_locals: u16,
        scratch_buffer_len: u16,
    ) -> CoroutineIr {
        coroutine_ir! {
            compiler_peer_create_coroutine(self.0, closure.0, num_locals, scratch_buffer_len)
        }
    }

    pub fn create_switch_for_coroutine(&self, block: BasicBlock, num_states: u32) -> SwitchIr {
        switch_ir! {
            compiler_peer_create_switch_for_coroutine(self.0, block.0, num_states)
        }
    }

    pub fn create_add_state_for_coroutine(&self, inst: SwitchIr, state: u32, block: BasicBlock) {
        unsafe {
            compiler_peer_create_add_state_for_coroutine(self.0, inst.0, state, block.0);
        }
    }

    pub fn create_suspend(&self) {
        unsafe {
            compiler_peer_create_suspend(self.0);
        }
    }

    pub fn create_set_coroutine_state(&self, state: u32) {
        unsafe {
            compiler_peer_create_set_coroutine_state(self.0, state);
        }
    }

    pub fn create_set_captures_for_coroutine(&self) {
        unsafe {
            compiler_peer_create_set_captures_for_coroutine(self.0);
        }
    }

    pub fn create_get_local_ptr_from_coroutine(&self, index: u16) -> ValueIr {
        value_ir! {
            compiler_peer_create_get_local_ptr_from_coroutine(self.0, index)
        }
    }

    pub fn create_write_boolean_to_scratch_buffer(&self, offset: u32, value: BooleanIr) {
        unsafe {
            compiler_peer_create_write_boolean_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_boolean_from_scratch_buffer(&self, offset: u32) -> BooleanIr {
        boolean_ir! {
            compiler_peer_create_read_boolean_from_scratch_buffer(self.0, offset)
        }
    }

    pub fn create_write_number_to_scratch_buffer(&self, offset: u32, value: NumberIr) {
        unsafe {
            compiler_peer_create_write_number_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_number_from_scratch_buffer(&self, offset: u32) -> NumberIr {
        number_ir! {
            compiler_peer_create_read_number_from_scratch_buffer(self.0, offset)
        }
    }

    pub fn create_write_string_to_scratch_buffer(&self, offset: u32, value: Char16SeqIr) {
        unsafe {
            compiler_peer_create_write_string_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_string_from_scratch_buffer(&self, offset: u32) -> Char16SeqIr {
        char16_seq_ir! {
            compiler_peer_create_read_string_from_scratch_buffer(self.0, offset)
        }
    }

    pub fn create_write_closure_to_scratch_buffer(&self, offset: u32, value: ClosureIr) {
        unsafe {
            compiler_peer_create_write_closure_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_closure_from_scratch_buffer(&self, offset: u32) -> ClosureIr {
        closure_ir! {
            compiler_peer_create_read_closure_from_scratch_buffer(self.0, offset)
        }
    }

    pub fn create_write_object_to_scratch_buffer(&self, offset: u32, value: ObjectIr) {
        unsafe {
            compiler_peer_create_write_object_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_object_from_scratch_buffer(&self, offset: u32) -> ObjectIr {
        object_ir! {
            compiler_peer_create_read_object_from_scratch_buffer(self.0, offset)
        }
    }

    pub fn create_write_promise_to_scratch_buffer(&self, offset: u32, value: PromiseIr) {
        unsafe {
            compiler_peer_create_write_promise_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_promise_from_scratch_buffer(&self, offset: u32) -> PromiseIr {
        promise_ir! {
            compiler_peer_create_read_promise_from_scratch_buffer(self.0, offset)
        }
    }

    pub fn create_write_value_to_scratch_buffer(&self, offset: u32, value: ValueIr) {
        unsafe {
            compiler_peer_create_write_value_to_scratch_buffer(self.0, offset, value.0);
        }
    }

    pub fn create_read_value_from_scratch_buffer(&self, offset: u32) -> ValueIr {
        value_ir! {
            compiler_peer_create_read_value_from_scratch_buffer(self.0, offset)
        }
    }

    // scope cleanup checker

    pub fn enable_scope_cleanup_checker(&self, is_coroutine: bool) {
        unsafe {
            compiler_peer_enable_scope_cleanup_checker(self.0, is_coroutine);
        }
    }

    pub fn set_scope_id_for_checker(&self, scope_ref: ScopeRef) {
        unsafe {
            compiler_peer_set_scope_id_for_checker(self.0, scope_ref.id());
        }
    }

    pub fn assert_scope_id(&self, expected: ScopeRef) {
        unsafe {
            compiler_peer_assert_scope_id(self.0, expected.id());
        }
    }

    // print

    #[allow(unused)]
    pub fn create_print_boolean(&self, value: BooleanIr, msg: &CStr) {
        unsafe {
            compiler_peer_create_print_boolean(self.0, value.0, msg.as_ptr());
        }
    }

    #[allow(unused)]
    pub fn create_print_string(&self, value: Char16SeqIr, msg: &CStr) {
        unsafe {
            compiler_peer_create_print_string(self.0, value.0, msg.as_ptr());
        }
    }

    #[allow(unused)]
    pub fn create_print_value(&self, value: ValueIr, msg: &CStr) {
        unsafe {
            compiler_peer_create_print_value(self.0, value.0, msg.as_ptr());
        }
    }

    #[allow(unused)]
    pub fn create_print_message(&self, msg: &CStr) {
        unsafe {
            compiler_peer_create_print_message(self.0, msg.as_ptr());
        }
    }

    // debugger

    #[allow(unused)]
    pub fn create_debugger(&self) {
        unsafe {
            compiler_peer_create_debugger(self.0);
        }
    }

    // assertions

    #[allow(unused)]
    pub fn create_assert(&self, assert: BooleanIr, msg: &CStr) {
        unsafe {
            compiler_peer_create_assert(self.0, assert.0, msg.as_ptr());
        }
    }

    #[allow(unused)]
    pub fn create_unreachable(&self, msg: &CStr) {
        unsafe {
            compiler_peer_create_unreachable(self.0, msg.as_ptr());
        }
    }
}

impl Default for CompilerBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for CompilerBridge {
    fn drop(&mut self) {
        unsafe {
            compiler_peer_delete(self.0);
        }
    }
}

impl BasicBlock {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_basic_block_name_or_as_operand(self.0, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl LambdaIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl BooleanIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl NumberIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl Char16SeqIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl ClosureIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl CoroutineIr {
    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl PromiseIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl ObjectIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0 as ValueIrPtr, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl ValueIr {
    pub const NONE: Self = Self(std::ptr::null_mut());

    pub fn get_name_or_as_operand<'a>(&self, buf: *mut std::ffi::c_char, len: usize) -> &'a CStr {
        unsafe {
            compiler_peer_get_value_name_or_as_operand(self.0, buf, len);
            CStr::from_ptr(buf)
        }
    }
}

impl ArgvIr {
    pub const NONE: Self = Self(std::ptr::null_mut());
}

// DO NOT USE MACROS FOR THE FOLLOWING TYPE DEFINITIONS.
// cbindgen does not support macro expansions.
type CompilerPeer = *mut c_void;
type BasicBlockPtr = *mut c_void;
type LambdaIrPtr = *mut c_void;
type BooleanIrPtr = *mut c_void;
type NumberIrPtr = *mut c_void;
type Char16SeqIrPtr = *mut c_void;
type ClosureIrPtr = *mut c_void;
type CoroutineIrPtr = *mut c_void;
type PromiseIrPtr = *mut c_void;
type ObjectIrPtr = *mut c_void;
type ValueIrPtr = *mut c_void;
type ArgvIrPtr = *mut c_void;
type StatusIrPtr = *mut c_void;
type CaptureIrPtr = *mut c_void;
type SwitchIrPtr = *mut c_void;

#[link(name = "llvmir")]
extern "C" {
    fn compiler_peer_new() -> CompilerPeer;
    fn compiler_peer_delete(peer: CompilerPeer);
    fn compiler_peer_start(peer: CompilerPeer);
    fn compiler_peer_end(peer: CompilerPeer) -> ModulePeer;
    fn compiler_peer_set_data_layout(peer: CompilerPeer, data_layout: *const c_char);
    fn compiler_peer_set_target_triple(peer: CompilerPeer, triple: *const c_char);
    fn compiler_peer_enable_labels(peer: CompilerPeer);

    // function

    fn compiler_peer_start_function(peer: CompilerPeer, lambda_id: u32);
    fn compiler_peer_end_function(peer: CompilerPeer, optimize: bool);
    fn compiler_peer_set_locals_block(peer: CompilerPeer, block: BasicBlockPtr);
    fn compiler_peer_get_function(peer: CompilerPeer, lambda_id: u32) -> LambdaIrPtr;

    // basic block

    fn compiler_peer_create_basic_block(
        peer: CompilerPeer,
        name: *const c_char,
        name_len: usize,
    ) -> BasicBlockPtr;
    fn compiler_peer_get_basic_block(peer: CompilerPeer) -> BasicBlockPtr;
    fn compiler_peer_set_basic_block(peer: CompilerPeer, block: BasicBlockPtr);
    fn compiler_peer_move_basic_block_after(peer: CompilerPeer, block: BasicBlockPtr);
    fn compiler_peer_is_basic_block_terminated(peer: CompilerPeer, block: BasicBlockPtr) -> bool;

    // jump

    fn compiler_peer_create_br(peer: CompilerPeer, block: BasicBlockPtr);
    fn compiler_peer_create_cond_br(
        peer: CompilerPeer,
        cond: BooleanIrPtr,
        then_block: BasicBlockPtr,
        else_block: BasicBlockPtr,
    );

    // undefined

    fn compiler_peer_create_is_undefined(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;

    // null

    fn compiler_peer_create_is_null(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_non_nullish(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;

    // boolean

    fn compiler_peer_create_is_boolean(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_boolean(
        peer: CompilerPeer,
        a: BooleanIrPtr,
        b: BooleanIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_number_to_boolean(
        peer: CompilerPeer,
        number: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_to_boolean(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_get_boolean(peer: CompilerPeer, value: bool) -> BooleanIrPtr;
    fn compiler_peer_create_logical_not(peer: CompilerPeer, value: BooleanIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_boolean_phi(
        peer: CompilerPeer,
        then_value: BooleanIrPtr,
        then_block: BasicBlockPtr,
        else_value: BooleanIrPtr,
        else_block: BasicBlockPtr,
    ) -> BooleanIrPtr;

    // number

    fn compiler_peer_create_is_number(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_number(
        peer: CompilerPeer,
        a: NumberIrPtr,
        b: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_boolean_to_number(
        peer: CompilerPeer,
        value: BooleanIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_to_numeric(peer: CompilerPeer, value: ValueIrPtr) -> NumberIrPtr;
    fn compiler_peer_get_nan(peer: CompilerPeer) -> NumberIrPtr;
    fn compiler_peer_get_zero(peer: CompilerPeer) -> NumberIrPtr;
    fn compiler_peer_get_number(peer: CompilerPeer, value: f64) -> NumberIrPtr;
    fn compiler_peer_create_bitwise_not(peer: CompilerPeer, value: NumberIrPtr) -> NumberIrPtr;
    fn compiler_peer_create_fneg(peer: CompilerPeer, value: NumberIrPtr) -> NumberIrPtr;
    fn compiler_peer_create_fmul(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_fdiv(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_frem(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_fadd(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_fsub(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_left_shift(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_signed_right_shift(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_unsigned_right_shift(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_bitwise_and(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_bitwise_xor(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_bitwise_or(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> NumberIrPtr;
    fn compiler_peer_create_less_than(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_greater_than(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_less_than_or_equal(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_greater_than_or_equal(
        peer: CompilerPeer,
        lhs: NumberIrPtr,
        rhs: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_number_phi(
        peer: CompilerPeer,
        then_value: NumberIrPtr,
        then_block: BasicBlockPtr,
        else_value: NumberIrPtr,
        else_block: BasicBlockPtr,
    ) -> NumberIrPtr;

    // string

    fn compiler_peer_create_char16_seq(
        peer: CompilerPeer,
        ptr: *const u16,
        len: u32,
    ) -> Char16SeqIrPtr;
    fn compiler_peer_create_string_on_stack(
        peer: CompilerPeer,
        value: Char16SeqIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_migrate_string_to_heap(
        peer: CompilerPeer,
        value: Char16SeqIrPtr,
    ) -> Char16SeqIrPtr;
    fn compiler_peer_create_string_phi(
        peer: CompilerPeer,
        then_value: Char16SeqIrPtr,
        then_block: BasicBlockPtr,
        else_value: Char16SeqIrPtr,
        else_block: BasicBlockPtr,
    ) -> Char16SeqIrPtr;

    // closure

    fn compiler_peer_get_closure_nullptr(peer: CompilerPeer) -> ClosureIrPtr;
    fn compiler_peer_create_is_closure(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_closure(
        peer: CompilerPeer,
        a: ClosureIrPtr,
        b: ClosureIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_closure(
        peer: CompilerPeer,
        lambda: LambdaIrPtr,
        num_captures: u16,
    ) -> ClosureIrPtr;
    fn compiler_peer_create_store_capture_to_closure(
        peer: CompilerPeer,
        capture: CaptureIrPtr,
        closure: ClosureIrPtr,
        index: u16,
    );
    fn compiler_peer_create_call_on_closure(
        peer: CompilerPeer,
        closure: ClosureIrPtr,
        argc: u16,
        argv: ArgvIrPtr,
        retv: ValueIrPtr,
    ) -> StatusIrPtr;
    fn compiler_peer_create_closure_phi(
        peer: CompilerPeer,
        then_value: ClosureIrPtr,
        then_block: BasicBlockPtr,
        else_value: ClosureIrPtr,
        else_block: BasicBlockPtr,
    ) -> ClosureIrPtr;

    // promise

    fn compiler_peer_create_is_promise(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_promise(
        peer: CompilerPeer,
        a: PromiseIrPtr,
        b: PromiseIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_register_promise(
        peer: CompilerPeer,
        coroutine: CoroutineIrPtr,
    ) -> PromiseIrPtr;
    fn compiler_peer_create_await_promise(
        peer: CompilerPeer,
        promise: PromiseIrPtr,
        awaiting: PromiseIrPtr,
    );
    fn compiler_peer_create_resume(peer: CompilerPeer, promise: PromiseIrPtr);
    fn compiler_peer_create_emit_promise_resolved(
        peer: CompilerPeer,
        promise: PromiseIrPtr,
        result: ValueIrPtr,
    );

    // object

    fn compiler_peer_get_object_nullptr(peer: CompilerPeer) -> ObjectIrPtr;
    fn compiler_peer_create_is_object(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_object(
        peer: CompilerPeer,
        a: ObjectIrPtr,
        b: ObjectIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_to_object(peer: CompilerPeer, value: ValueIrPtr) -> ObjectIrPtr;
    fn compiler_peer_create_object(peer: CompilerPeer) -> ObjectIrPtr;
    fn compiler_peer_create_get_value_by_symbol(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: u32,
        strict: bool,
    ) -> ValueIrPtr;
    fn compiler_peer_create_get_value_by_number(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: f64,
        strict: bool,
    ) -> ValueIrPtr;
    fn compiler_peer_create_get_value_by_value(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: ValueIrPtr,
        strict: bool,
    ) -> ValueIrPtr;
    fn compiler_peer_create_set_value_by_symbol(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: u32,
        value: ValueIrPtr,
    );
    fn compiler_peer_create_set_value_by_number(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: f64,
        value: ValueIrPtr,
    );
    fn compiler_peer_create_set_value_by_value(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: ValueIrPtr,
        value: ValueIrPtr,
    );
    fn compiler_peer_create_create_data_property_by_symbol(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: u32,
        value: ValueIrPtr,
        retv: ValueIrPtr,
    ) -> StatusIrPtr;
    fn compiler_peer_create_create_data_property_by_number(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: f64,
        value: ValueIrPtr,
        retv: ValueIrPtr,
    ) -> StatusIrPtr;
    fn compiler_peer_create_create_data_property_by_value(
        peer: CompilerPeer,
        object: ObjectIrPtr,
        key: ValueIrPtr,
        value: ValueIrPtr,
        retv: ValueIrPtr,
    ) -> StatusIrPtr;
    fn compiler_peer_create_copy_data_properties(
        peer: CompilerPeer,
        target: ObjectIrPtr,
        source: ValueIrPtr,
        retv: ValueIrPtr,
    ) -> StatusIrPtr;

    // value

    fn compiler_peer_create_value_is_nullptr(peer: CompilerPeer, value: ValueIrPtr)
        -> BooleanIrPtr;
    fn compiler_peer_create_has_value(peer: CompilerPeer, value: ValueIrPtr) -> BooleanIrPtr;
    fn compiler_peer_create_is_loosely_equal(
        peer: CompilerPeer,
        lhs: ValueIrPtr,
        rhs: ValueIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_strictly_equal(
        peer: CompilerPeer,
        lhs: ValueIrPtr,
        rhs: ValueIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_boolean_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
        boolean: BooleanIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_number_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
        number: NumberIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_closure_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
        closure: ClosureIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_promise_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
        promise: PromiseIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_same_object_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
        object: ObjectIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_undefined_to_any(peer: CompilerPeer) -> ValueIrPtr;
    fn compiler_peer_create_null_to_any(peer: CompilerPeer) -> ValueIrPtr;
    fn compiler_peer_create_boolean_to_any(peer: CompilerPeer, boolean: BooleanIrPtr)
        -> ValueIrPtr;
    fn compiler_peer_create_number_to_any(peer: CompilerPeer, number: NumberIrPtr) -> ValueIrPtr;
    fn compiler_peer_create_string_to_any(peer: CompilerPeer, string: Char16SeqIrPtr)
        -> ValueIrPtr;
    fn compiler_peer_create_closure_to_any(peer: CompilerPeer, closure: ClosureIrPtr)
        -> ValueIrPtr;
    fn compiler_peer_create_object_to_any(peer: CompilerPeer, object: ObjectIrPtr) -> ValueIrPtr;
    fn compiler_peer_create_value_phi(
        peer: CompilerPeer,
        then_value: ValueIrPtr,
        then_block: BasicBlockPtr,
        else_value: ValueIrPtr,
        else_block: BasicBlockPtr,
    ) -> ValueIrPtr;
    fn compiler_peer_create_local_value(peer: CompilerPeer, index: u16) -> ValueIrPtr;
    fn compiler_peer_create_store_none_to_value(peer: CompilerPeer, dest: ValueIrPtr);
    fn compiler_peer_create_store_undefined_to_value(peer: CompilerPeer, dest: ValueIrPtr);
    fn compiler_peer_create_store_null_to_value(peer: CompilerPeer, dest: ValueIrPtr);
    fn compiler_peer_create_store_boolean_to_value(
        peer: CompilerPeer,
        value: BooleanIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_store_number_to_value(
        peer: CompilerPeer,
        value: NumberIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_store_string_to_value(
        peer: CompilerPeer,
        value: Char16SeqIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_store_closure_to_value(
        peer: CompilerPeer,
        value: ClosureIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_store_promise_to_value(
        peer: CompilerPeer,
        value: PromiseIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_store_object_to_value(
        peer: CompilerPeer,
        value: ObjectIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_store_value_to_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
        dest: ValueIrPtr,
    );
    fn compiler_peer_create_load_boolean_from_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_load_closure_from_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
    ) -> ClosureIrPtr;
    fn compiler_peer_create_load_promise_from_value(
        peer: CompilerPeer,
        value: ValueIrPtr,
    ) -> PromiseIrPtr;
    fn compiler_peer_create_typeof(peer: CompilerPeer, value: ValueIrPtr) -> Char16SeqIrPtr;

    // argv

    fn compiler_peer_get_argv_nullptr(peer: CompilerPeer) -> ArgvIrPtr;
    fn compiler_peer_create_argv(peer: CompilerPeer, argc: u16) -> ArgvIrPtr;
    fn compiler_peer_create_get_arg_in_argv(
        peer: CompilerPeer,
        argv: ArgvIrPtr,
        index: u16,
    ) -> ValueIrPtr;
    fn compiler_peer_create_get_argument_value_ptr(peer: CompilerPeer, index: u16) -> ValueIrPtr;

    // retv
    //
    // The `retv` variable holds either a returned or thrown value.

    fn compiler_peer_create_retv(peer: CompilerPeer) -> ValueIrPtr;
    fn compiler_peer_create_store_undefined_to_retv(peer: CompilerPeer);
    fn compiler_peer_create_store_null_to_retv(peer: CompilerPeer);
    fn compiler_peer_create_store_boolean_to_retv(peer: CompilerPeer, value: BooleanIrPtr);
    fn compiler_peer_create_store_number_to_retv(peer: CompilerPeer, value: NumberIrPtr);
    fn compiler_peer_create_store_closure_to_retv(peer: CompilerPeer, value: ClosureIrPtr);
    fn compiler_peer_create_store_promise_to_retv(peer: CompilerPeer, value: PromiseIrPtr);
    fn compiler_peer_create_store_object_to_retv(peer: CompilerPeer, value: ObjectIrPtr);
    fn compiler_peer_create_store_value_to_retv(peer: CompilerPeer, value: ValueIrPtr);
    fn compiler_peer_get_exception(peer: CompilerPeer) -> ValueIrPtr;

    // status
    //
    // TODO: Currently, each lambda has its own status variable.  However, it might be possible to
    // use a single global variable shared by all lambdas.  Because the execution model of
    // JavaScript is a single threaded model.

    fn compiler_peer_create_alloc_status(peer: CompilerPeer);
    fn compiler_peer_create_store_normal_status(peer: CompilerPeer);
    fn compiler_peer_create_store_exception_status(peer: CompilerPeer);
    fn compiler_peer_create_is_exception_status(
        peer: CompilerPeer,
        status: StatusIrPtr,
    ) -> BooleanIrPtr;

    // flow selector

    fn compiler_peer_create_alloc_flow_selector(peer: CompilerPeer);
    fn compiler_peer_create_set_flow_selector_normal(peer: CompilerPeer);
    fn compiler_peer_create_set_flow_selector_return(peer: CompilerPeer);
    fn compiler_peer_create_set_flow_selector_throw(peer: CompilerPeer);
    fn compiler_peer_create_set_flow_selector_break(peer: CompilerPeer, depth: u32);
    fn compiler_peer_create_set_flow_selector_continue(peer: CompilerPeer, depth: u32);
    fn compiler_peer_create_is_flow_selector_normal(peer: CompilerPeer) -> BooleanIrPtr;
    fn compiler_peer_create_is_flow_selector_normal_or_continue(
        peer: CompilerPeer,
        depth: u32,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_flow_selector_break_or_continue(
        peer: CompilerPeer,
        depth: u32,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_is_flow_selector_break(peer: CompilerPeer, depth: u32) -> BooleanIrPtr;

    // capture

    fn compiler_peer_create_capture(peer: CompilerPeer, value: ValueIrPtr) -> CaptureIrPtr;
    fn compiler_peer_create_escape_value(
        peer: CompilerPeer,
        capture: CaptureIrPtr,
        value: ValueIrPtr,
    );
    fn compiler_peer_create_get_capture_value_ptr(peer: CompilerPeer, index: u16) -> ValueIrPtr;
    fn compiler_peer_create_load_capture(peer: CompilerPeer, index: u16) -> CaptureIrPtr;

    // coroutine

    fn compiler_peer_create_coroutine(
        peer: CompilerPeer,
        closure: ClosureIrPtr,
        num_locals: u16,
        scratch_buffer_len: u16,
    ) -> CoroutineIrPtr;
    fn compiler_peer_create_switch_for_coroutine(
        peer: CompilerPeer,
        block: BasicBlockPtr,
        num_states: u32,
    ) -> SwitchIrPtr;
    fn compiler_peer_create_add_state_for_coroutine(
        peer: CompilerPeer,
        switch_ir: SwitchIrPtr,
        state: u32,
        block: BasicBlockPtr,
    );
    fn compiler_peer_create_suspend(peer: CompilerPeer);
    fn compiler_peer_create_set_coroutine_state(peer: CompilerPeer, state: u32);
    fn compiler_peer_create_set_captures_for_coroutine(peer: CompilerPeer);
    fn compiler_peer_create_get_local_ptr_from_coroutine(
        peer: CompilerPeer,
        index: u16,
    ) -> ValueIrPtr;
    fn compiler_peer_create_write_boolean_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: BooleanIrPtr,
    );
    fn compiler_peer_create_read_boolean_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> BooleanIrPtr;
    fn compiler_peer_create_write_number_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: NumberIrPtr,
    );
    fn compiler_peer_create_read_number_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> NumberIrPtr;
    fn compiler_peer_create_write_string_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: Char16SeqIrPtr,
    );
    fn compiler_peer_create_read_string_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> Char16SeqIrPtr;
    fn compiler_peer_create_write_closure_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: ClosureIrPtr,
    );
    fn compiler_peer_create_read_closure_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> ClosureIrPtr;
    fn compiler_peer_create_write_object_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: ObjectIrPtr,
    );
    fn compiler_peer_create_read_object_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> ObjectIrPtr;
    fn compiler_peer_create_write_promise_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: PromiseIrPtr,
    );
    fn compiler_peer_create_read_promise_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> PromiseIrPtr;
    fn compiler_peer_create_write_value_to_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
        value: ValueIrPtr,
    );
    fn compiler_peer_create_read_value_from_scratch_buffer(
        peer: CompilerPeer,
        offset: u32,
    ) -> ValueIrPtr;

    // scope cleanup checker

    fn compiler_peer_enable_scope_cleanup_checker(peer: CompilerPeer, is_coroutine: bool);
    fn compiler_peer_set_scope_id_for_checker(peer: CompilerPeer, scope_id: u16);
    fn compiler_peer_assert_scope_id(peer: CompilerPeer, expected: u16);

    // print

    fn compiler_peer_create_print_boolean(
        peer: CompilerPeer,
        value: BooleanIrPtr,
        msg: *const c_char,
    );
    fn compiler_peer_create_print_string(
        peer: CompilerPeer,
        value: Char16SeqIrPtr,
        msg: *const c_char,
    );
    fn compiler_peer_create_print_value(peer: CompilerPeer, value: ValueIrPtr, msg: *const c_char);
    fn compiler_peer_create_print_message(peer: CompilerPeer, msg: *const c_char);

    // debugger

    fn compiler_peer_create_debugger(peer: CompilerPeer);

    // assertions

    fn compiler_peer_create_assert(peer: CompilerPeer, assert: BooleanIrPtr, msg: *const c_char);
    fn compiler_peer_create_unreachable(peer: CompilerPeer, msg: *const c_char);

    // helpers

    fn compiler_peer_get_basic_block_name_or_as_operand(
        block: BasicBlockPtr,
        buf: *mut c_char,
        len: usize,
    ) -> usize;
    fn compiler_peer_get_value_name_or_as_operand(
        value: ValueIrPtr,
        buf: *mut c_char,
        len: usize,
    ) -> usize;
}
