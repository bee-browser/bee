#include "bridge.hh"

#include <cstdint>

#include <llvm/Support/TargetSelect.h>
#include <sys/types.h>

#include "compiler.hh"
#include "executor.hh"
#include "helper.hh"
#include "module.hh"

#define LLVM_BB(bb) (reinterpret_cast<llvm::BasicBlock*>(bb))
#define PEER_BB(bb) (reinterpret_cast<BasicBlock*>(bb))

#define LLVM_LAMBDA(lambda) (reinterpret_cast<llvm::Function*>(lambda))
#define PEER_LAMBDA(lambda) (reinterpret_cast<LambdaIr*>(lambda))

#define LLVM_VALUE(value) (reinterpret_cast<llvm::Value*>(value))
#define PEER_BOOLEAN(value) (reinterpret_cast<BooleanIr*>(value))
#define PEER_NUMBER(value) (reinterpret_cast<NumberIr*>(value))
#define PEER_CLOSURE(value) (reinterpret_cast<ClosureIr*>(value))
#define PEER_COROUTINE(value) (reinterpret_cast<CoroutineIr*>(value))
#define PEER_PROMISE(value) (reinterpret_cast<PromiseIr*>(value))
#define PEER_VALUE(value) (reinterpret_cast<ValueIr*>(value))
#define PEER_ARGV(value) (reinterpret_cast<ArgvIr*>(value))
#define PEER_STATUS(value) (reinterpret_cast<StatusIr*>(value))
#define PEER_CAPTURE(value) (reinterpret_cast<CaptureIr*>(value))

#define LLVM_SWITCH(inst) (reinterpret_cast<llvm::SwitchInst*>(inst))
#define PEER_SWITCH(inst) (reinterpret_cast<SwitchIr*>(inst))

void llvmir_initialize() {
  // Uncomment if you want to enable LLVM_DEBUG().
  // llvm::DebugFlag = true;

  llvm::InitializeNativeTarget();
  llvm::InitializeNativeTargetAsmPrinter();
  llvm::InitializeNativeTargetAsmParser();
}

// module

void module_peer_print(Module* self, bool stderr) {
  self->Print(stderr);
}

void module_peer_delete(Module* self) {
  delete self;
}

// compilation

Compiler* compiler_peer_new() {
  return new Compiler();
}

void compiler_peer_delete(Compiler* self) {
  delete self;
}

void compiler_peer_start(Compiler* self, bool enable_labels) {
  if (enable_labels) {
    self->EnableLabels();
  }
}

Module* compiler_peer_end(Compiler* self) {
  return self->TakeModule();
}

void compiler_peer_set_data_layout(Compiler* self, const char* data_layout) {
  self->SetDataLayout(data_layout);
}

void compiler_peer_set_target_triple(Compiler* self, const char* triple) {
  self->SetTargetTriple(triple);
}

void compiler_peer_start_function(Compiler* self, const char* name) {
  self->StartFunction(name);
}

void compiler_peer_end_function(Compiler* self, bool optimize) {
  self->EndFunction(optimize);
}

void compiler_peer_set_locals_block(Compiler* self, BasicBlock* block) {
  self->SetLocalsBlock(LLVM_BB(block));
}

LambdaIr* compiler_peer_get_function(Compiler* self, uint32_t func_id, const char* name) {
  return PEER_LAMBDA(self->GetFunction(func_id, name));
}

// basic block

BasicBlock* compiler_peer_create_basic_block(Compiler* self, const char* name, size_t name_len) {
  return PEER_BB(self->CreateBasicBlock(name, name_len));
}

BasicBlock* compiler_peer_get_basic_block(const Compiler* self) {
  return PEER_BB(self->GetBasicBlock());
}

void compiler_peer_set_basic_block(Compiler* self, BasicBlock* block) {
  self->SetBasicBlock(LLVM_BB(block));
}

void compiler_peer_move_basic_block_after(Compiler* self, BasicBlock* block) {
  self->MoveBasicBlockAfter(LLVM_BB(block));
}

bool compiler_peer_is_basic_block_terminated(Compiler* self, BasicBlock* block) {
  return self->IsBasicBlockTerminated(LLVM_BB(block));
}

// jump

void compiler_peer_create_br(Compiler* self, BasicBlock* block) {
  self->CreateBr(LLVM_BB(block));
}

void compiler_peer_create_cond_br(Compiler* self,
    BooleanIr* cond,
    BasicBlock* then_block,
    BasicBlock* else_block) {
  self->CreateCondBr(LLVM_VALUE(cond), LLVM_BB(then_block), LLVM_BB(else_block));
}

// undefined

BooleanIr* compiler_peer_create_is_undefined(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsUndefined(LLVM_VALUE(value)));
}

// null

BooleanIr* compiler_peer_create_is_null(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsNull(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_non_nullish(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsNonNullish(LLVM_VALUE(value)));
}

// boolean

BooleanIr* compiler_peer_create_is_boolean(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsBoolean(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_same_boolean(Compiler* self, BooleanIr* a, BooleanIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameBoolean(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIr* compiler_peer_create_number_to_boolean(Compiler* self, NumberIr* value) {
  return PEER_BOOLEAN(self->CreateNumberToBoolean(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_to_boolean(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateToBoolean(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_get_boolean(Compiler* self, bool value) {
  return PEER_BOOLEAN(self->GetBoolean(value));
}

BooleanIr* compiler_peer_create_logical_not(Compiler* self, BooleanIr* boolean) {
  return PEER_BOOLEAN(self->CreateLogicalNot(LLVM_VALUE(boolean)));
}

BooleanIr* compiler_peer_create_boolean_phi(Compiler* self,
    BooleanIr* then_value,
    BasicBlock* then_block,
    BooleanIr* else_value,
    BasicBlock* else_block) {
  return PEER_BOOLEAN(self->CreateBooleanPhi(
      LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

// number

BooleanIr* compiler_peer_create_is_number(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsNumber(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_same_number(Compiler* self, NumberIr* a, NumberIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameNumber(LLVM_VALUE(a), LLVM_VALUE(b)));
}

NumberIr* compiler_peer_create_boolean_to_number(Compiler* self, BooleanIr* value) {
  return PEER_NUMBER(self->CreateUIToFP(LLVM_VALUE(value)));
}

NumberIr* compiler_peer_to_numeric(Compiler* self, ValueIr* value) {
  return PEER_NUMBER(self->ToNumeric(LLVM_VALUE(value)));
}

NumberIr* compiler_peer_get_nan(Compiler* self) {
  return PEER_NUMBER(self->GetNan());
}

NumberIr* compiler_peer_get_zero(Compiler* self) {
  return PEER_NUMBER(self->GetZero());
}

NumberIr* compiler_peer_get_number(Compiler* self, double value) {
  return PEER_NUMBER(self->GetNumber(value));
}

NumberIr* compiler_peer_create_bitwise_not(Compiler* self, NumberIr* value) {
  return PEER_NUMBER(self->CreateBitwiseNot(LLVM_VALUE(value)));
}

NumberIr* compiler_peer_create_fneg(Compiler* self, NumberIr* value) {
  return PEER_NUMBER(self->CreateFNeg(LLVM_VALUE(value)));
}

NumberIr* compiler_peer_create_fmul(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateFMul(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_fdiv(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateFDiv(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_frem(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateFRem(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_fadd(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateFAdd(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_fsub(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateFSub(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_left_shift(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateLeftShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_signed_right_shift(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateSignedRightShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_unsigned_right_shift(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateUnsignedRightShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_bitwise_and(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateBitwiseAnd(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_bitwise_xor(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateBitwiseXor(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_bitwise_or(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_NUMBER(self->CreateBitwiseOr(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_less_than(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_BOOLEAN(self->CreateLessThan(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_greater_than(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_BOOLEAN(self->CreateGreaterThan(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_less_than_or_equal(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_BOOLEAN(self->CreateLessThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_greater_than_or_equal(Compiler* self,
    NumberIr* lhs,
    NumberIr* rhs) {
  return PEER_BOOLEAN(self->CreateGreaterThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_number_phi(Compiler* self,
    NumberIr* then_value,
    BasicBlock* then_block,
    NumberIr* else_value,
    BasicBlock* else_block) {
  return PEER_NUMBER(self->CreateNumberPhi(
      LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

// closure

BooleanIr* compiler_peer_create_is_closure(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsClosure(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_same_closure(Compiler* self, ClosureIr* a, ClosureIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameClosure(LLVM_VALUE(a), LLVM_VALUE(b)));
}

ClosureIr* compiler_peer_get_closure_nullptr(Compiler* self) {
  return PEER_CLOSURE(self->GetNullptr());
}

ClosureIr* compiler_peer_create_closure(Compiler* self, LambdaIr* lambda, uint16_t num_captures) {
  return PEER_CLOSURE(self->CreateClosure(LLVM_LAMBDA(lambda), num_captures));
}

void compiler_peer_create_store_capture_to_closure(Compiler* self,
    CaptureIr* capture,
    ClosureIr* closure,
    uint16_t index) {
  self->CreateStoreCapturePtrToClosure(LLVM_VALUE(capture), LLVM_VALUE(closure), index);
}

StatusIr* compiler_peer_create_call_on_closure(Compiler* self,
    ClosureIr* closure,
    uint16_t argc,
    ArgvIr* argv,
    ValueIr* retv) {
  return PEER_STATUS(
      self->CreateCallOnClosure(LLVM_VALUE(closure), argc, LLVM_VALUE(argv), LLVM_VALUE(retv)));
}

ClosureIr* compiler_peer_create_closure_phi(Compiler* self,
    ClosureIr* then_value,
    BasicBlock* then_block,
    ClosureIr* else_value,
    BasicBlock* else_block) {
  return PEER_CLOSURE(self->CreateClosurePhi(
      LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

// promise

BooleanIr* compiler_peer_create_is_promise(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsPromise(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_same_promise(Compiler* self, PromiseIr* a, PromiseIr* b) {
  return PEER_BOOLEAN(self->CreateIsSamePromise(LLVM_VALUE(a), LLVM_VALUE(b)));
}

PromiseIr* compiler_peer_create_register_promise(Compiler* self, CoroutineIr* coroutine) {
  return PEER_PROMISE(self->CreateRegisterPromise(LLVM_VALUE(coroutine)));
}

void compiler_peer_create_await_promise(Compiler* self, PromiseIr* promise, PromiseIr* awaiting) {
  self->CreateAwaitPromise(LLVM_VALUE(promise), LLVM_VALUE(awaiting));
}

void compiler_peer_create_resume(Compiler* self, PromiseIr* promise) {
  self->CreateResume(LLVM_VALUE(promise));
}

void compiler_peer_create_emit_promise_resolved(Compiler* self,
    PromiseIr* promise,
    ValueIr* result) {
  self->CreateEmitPromiseResolved(LLVM_VALUE(promise), LLVM_VALUE(result));
}

// value

BooleanIr* compiler_peer_create_has_value(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateHasValue(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_loosely_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateIsLooselyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_is_strictly_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateIsStrictlyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_is_same_boolean_value(Compiler* self,
    ValueIr* value,
    BooleanIr* boolean) {
  return PEER_BOOLEAN(self->CreateIsSameBooleanValue(LLVM_VALUE(value), LLVM_VALUE(boolean)));
}

BooleanIr* compiler_peer_create_is_same_number_value(Compiler* self,
    ValueIr* value,
    NumberIr* number) {
  return PEER_BOOLEAN(self->CreateIsSameNumberValue(LLVM_VALUE(value), LLVM_VALUE(number)));
}

BooleanIr* compiler_peer_create_is_same_closure_value(Compiler* self,
    ValueIr* value,
    ClosureIr* closure) {
  return PEER_BOOLEAN(self->CreateIsSameClosureValue(LLVM_VALUE(value), LLVM_VALUE(closure)));
}

BooleanIr* compiler_peer_create_is_same_promise_value(Compiler* self,
    ValueIr* value,
    PromiseIr* promise) {
  return PEER_BOOLEAN(self->CreateIsSamePromiseValue(LLVM_VALUE(value), LLVM_VALUE(promise)));
}

ValueIr* compiler_peer_create_undefined_to_any(Compiler* self) {
  return PEER_VALUE(self->CreateUndefinedToAny());
}

ValueIr* compiler_peer_create_null_to_any(Compiler* self) {
  return PEER_VALUE(self->CreateNullToAny());
}

ValueIr* compiler_peer_create_boolean_to_any(Compiler* self, BooleanIr* boolean) {
  return PEER_VALUE(self->CreateBooleanToAny(LLVM_VALUE(boolean)));
}

ValueIr* compiler_peer_create_number_to_any(Compiler* self, NumberIr* value) {
  return PEER_VALUE(self->CreateNumberToAny(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_closure_to_any(Compiler* self, ClosureIr* value) {
  return PEER_VALUE(self->CreateClosureToAny(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_value_phi(Compiler* self,
    ValueIr* then_value,
    BasicBlock* then_block,
    ValueIr* else_value,
    BasicBlock* else_block) {
  return PEER_VALUE(self->CreateValuePhi(
      LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

ValueIr* compiler_peer_create_local_value(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateLocalValue(index));
}

void compiler_peer_create_store_none_to_value(Compiler* self, ValueIr* dest) {
  self->CreateStoreNoneToValue(LLVM_VALUE(dest));
}

void compiler_peer_create_store_undefined_to_value(Compiler* self, ValueIr* dest) {
  self->CreateStoreUndefinedToValue(LLVM_VALUE(dest));
}

void compiler_peer_create_store_null_to_value(Compiler* self, ValueIr* dest) {
  self->CreateStoreNullToValue(LLVM_VALUE(dest));
}

void compiler_peer_create_store_boolean_to_value(Compiler* self, BooleanIr* value, ValueIr* dest) {
  self->CreateStoreBooleanToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_number_to_value(Compiler* self, NumberIr* value, ValueIr* dest) {
  self->CreateStoreNumberToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_closure_to_value(Compiler* self, ClosureIr* value, ValueIr* dest) {
  self->CreateStoreClosureToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_promise_to_value(Compiler* self, PromiseIr* value, ValueIr* dest) {
  self->CreateStorePromiseToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_value_to_value(Compiler* self, ValueIr* value, ValueIr* dest) {
  self->CreateStoreValueToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

ClosureIr* compiler_peer_create_load_closure_from_value(Compiler* self, ValueIr* value) {
  return PEER_CLOSURE(self->CreateLoadClosureFromValue(LLVM_VALUE(value)));
}

PromiseIr* compiler_peer_create_load_promise_from_value(Compiler* self, ValueIr* value) {
  return PEER_PROMISE(self->CreateLoadPromiseFromValue(LLVM_VALUE(value)));
}

// argv

ArgvIr* compiler_peer_get_argv_nullptr(Compiler* self) {
  return PEER_ARGV(self->GetNullptr());
}

ArgvIr* compiler_peer_create_argv(Compiler* self, uint16_t argc) {
  return PEER_ARGV(self->CreateArgv(argc));
}

ValueIr* compiler_peer_create_get_arg_in_argv(Compiler* self, ArgvIr* argv, uint16_t index) {
  return PEER_VALUE(self->CreateGetArgInArgv(LLVM_VALUE(argv), index));
}

ValueIr* compiler_peer_create_get_argument_value_ptr(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateGetArgumentValuePtr(index));
}

// retv

ValueIr* compiler_peer_create_retv(Compiler* self) {
  return PEER_VALUE(self->CreateRetv());
}

void compiler_peer_create_store_undefined_to_retv(Compiler* self) {
  self->CreateStoreUndefinedToRetv();
}

void compiler_peer_create_store_null_to_retv(Compiler* self) {
  self->CreateStoreNullToRetv();
}

void compiler_peer_create_store_boolean_to_retv(Compiler* self, BooleanIr* value) {
  self->CreateStoreBooleanToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_number_to_retv(Compiler* self, NumberIr* value) {
  self->CreateStoreNumberToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_closure_to_retv(Compiler* self, ClosureIr* value) {
  self->CreateStoreClosureToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_promise_to_retv(Compiler* self, PromiseIr* value) {
  self->CreateStorePromiseToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_value_to_retv(Compiler* self, ValueIr* value) {
  self->CreateStoreValueToRetv(LLVM_VALUE(value));
}

ValueIr* compiler_peer_get_exception(Compiler* self) {
  return PEER_VALUE(self->GetException());
}

// status

void compiler_peer_create_alloc_status(Compiler* self) {
  self->CreateAllocStatus();
}

void compiler_peer_create_store_normal_status(Compiler* self) {
  self->CreateStoreNormalStatus();
}

void compiler_peer_create_store_exception_status(Compiler* self) {
  self->CreateStoreExceptionStatus();
}

BooleanIr* compiler_peer_create_is_exception_status(Compiler* self, StatusIr* status) {
  return PEER_BOOLEAN(self->CreateIsExceptionStatus(LLVM_VALUE(status)));
}

// flow selector

void compiler_peer_create_alloc_flow_selector(Compiler* self) {
  self->CreateAllocFlowSelector();
}

void compiler_peer_create_set_flow_selector_normal(Compiler* self) {
  self->CreateSetFlowSelectorNormal();
}

void compiler_peer_create_set_flow_selector_return(Compiler* self) {
  self->CreateSetFlowSelectorReturn();
}

void compiler_peer_create_set_flow_selector_throw(Compiler* self) {
  self->CreateSetFlowSelectorThrow();
}

void compiler_peer_create_set_flow_selector_break(Compiler* self, uint32_t depth) {
  self->CreateSetFlowSelectorBreak(depth);
}

void compiler_peer_create_set_flow_selector_continue(Compiler* self, uint32_t depth) {
  self->CreateSetFlowSelectorContinue(depth);
}

BooleanIr* compiler_peer_create_is_flow_selector_normal(Compiler* self) {
  return PEER_BOOLEAN(self->CreateIsFlowSelectorNormal());
}

BooleanIr* compiler_peer_create_is_flow_selector_normal_or_continue(Compiler* self,
    uint32_t depth) {
  return PEER_BOOLEAN(self->CreateIsFlowSelectorNormalOrContinue(depth));
}

BooleanIr* compiler_peer_create_is_flow_selector_break_or_continue(Compiler* self,
    uint32_t depth) {
  return PEER_BOOLEAN(self->CreateIsFlowSelectorBreakOrContinue(depth));
}

BooleanIr* compiler_peer_create_is_flow_selector_break(Compiler* self, uint32_t depth) {
  return PEER_BOOLEAN(self->CreateIsFlowSelectorBreak(depth));
}

// capture

CaptureIr* compiler_peer_create_capture(Compiler* self, ValueIr* value) {
  return PEER_CAPTURE(self->CreateCapture(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_get_capture_value_ptr(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateGetCaptureValuePtr(index));
}

void compiler_peer_create_escape_value(Compiler* self, CaptureIr* capture, ValueIr* value) {
  self->CreateEscapeValue(LLVM_VALUE(capture), LLVM_VALUE(value));
}

CaptureIr* compiler_peer_create_load_capture(Compiler* self, uint16_t index) {
  return PEER_CAPTURE(self->CreateLoadCapture(index));
}

// coroutine

CoroutineIr* compiler_peer_create_coroutine(Compiler* self,
    ClosureIr* closure,
    uint16_t num_locals) {
  return PEER_COROUTINE(self->CreateCoroutine(LLVM_VALUE(closure), num_locals));
}

SwitchIr* compiler_peer_create_switch_for_coroutine(Compiler* self,
    BasicBlock* block,
    uint32_t num_states) {
  return PEER_SWITCH(self->CreateSwitchForCoroutine(LLVM_BB(block), num_states));
}

void compiler_peer_create_add_state_for_coroutine(Compiler* self,
    SwitchIr* inst,
    uint32_t state,
    BasicBlock* block) {
  self->CreateAddStateForCoroutine(LLVM_SWITCH(inst), state, LLVM_BB(block));
}

void compiler_peer_create_suspend(Compiler* self) {
  self->CreateSuspend();
}

void compiler_peer_create_set_coroutine_state(Compiler* self, uint32_t state) {
  self->CreateSetCoroutineState(state);
}

void compiler_peer_create_set_captures_for_coroutine(Compiler* self) {
  self->CreateSetCapturesForCoroutine();
}

ValueIr* compiler_peer_create_get_local_ptr_from_coroutine(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateGetLocalPtrFromCoroutine(index));
}

// scope cleanup checker

void compiler_peer_enable_scope_cleanup_checker(Compiler* self) {
  self->EnableScopeCleanupChecker();
}

void compiler_peer_set_scope_id_for_checker(Compiler* self, uint16_t scope_id) {
  self->SetScopeIdForChecker(scope_id);
}

void compiler_peer_assert_scope_id(Compiler* self, uint16_t expected) {
  self->AssertScopeId(expected);
}

// print

void compiler_peer_create_print_value(Compiler* self, ValueIr* value, const char* msg) {
  assert(msg != nullptr);
  self->CreatePrintValue(LLVM_VALUE(value), msg);
}

// unreachable

void compiler_peer_create_unreachable(Compiler* self, const char* msg) {
  self->CreateUnreachable(msg);
}

// executor

Executor* executor_peer_new() {
  return llvm::cantFail(Executor::Create());
}

void executor_peer_delete(Executor* self) {
  delete self;
}

void executor_peer_register_runtime(Executor* self, const Runtime* runtime) {
  self->RegisterRuntime(runtime);
}

void executor_peer_register_host_function(Executor* self, const char* name, Lambda lambda) {
  self->RegisterHostFunction(name, lambda);
}

void executor_peer_register_module(Executor* self, Module* mod) {
  self->RegisterModule(mod);
}

const char* executor_peer_get_data_layout(const Executor* self) {
  return self->data_layout().getStringRepresentation().c_str();
}

const char* executor_peer_get_target_triple(const Executor* self) {
  return self->target_triple().getTriple().c_str();
}

Lambda executor_peer_get_native_function(Executor* self, const char* name) {
  return self->GetNativeFunction(name);
}

// helper functions

size_t helper_peer_get_basic_block_name_or_as_operand(BasicBlock* block, char* buf, size_t len) {
  return GetNameOrAsOperand(LLVM_BB(block), buf, len);
}

size_t helper_peer_get_value_name_or_as_operand(ValueIr* value, char* buf, size_t len) {
  return GetNameOrAsOperand(LLVM_VALUE(value), buf, len);
}
