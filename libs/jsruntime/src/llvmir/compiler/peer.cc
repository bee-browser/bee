#include <llvm/Support/TargetSelect.h>
#include <sys/types.h>

#include <cstdint>

#include "../bridge.hh"
#include "impl.hh"

#define IMPL(peer) reinterpret_cast<Compiler*>(peer)
#define PEER(impl) reinterpret_cast<CompilerPeer>(impl)

#define LLVM_BB(bb) (reinterpret_cast<llvm::BasicBlock*>(bb))
#define PEER_BB(bb) (reinterpret_cast<BasicBlockPtr>(bb))

#define LLVM_LAMBDA(lambda) (reinterpret_cast<llvm::Function*>(lambda))
#define PEER_LAMBDA(lambda) (reinterpret_cast<LambdaIrPtr>(lambda))

#define LLVM_VALUE(value) (reinterpret_cast<llvm::Value*>(value))
#define PEER_BOOLEAN(value) (reinterpret_cast<BooleanIrPtr>(value))
#define PEER_NUMBER(value) (reinterpret_cast<NumberIrPtr>(value))
#define PEER_CLOSURE(value) (reinterpret_cast<ClosureIrPtr>(value))
#define PEER_COROUTINE(value) (reinterpret_cast<CoroutineIrPtr>(value))
#define PEER_PROMISE(value) (reinterpret_cast<PromiseIrPtr>(value))
#define PEER_OBJECT(value) (reinterpret_cast<ObjectIrPtr>(value))
#define PEER_VALUE(value) (reinterpret_cast<ValueIrPtr>(value))
#define PEER_ARGV(value) (reinterpret_cast<ArgvIrPtr>(value))
#define PEER_STATUS(value) (reinterpret_cast<StatusIrPtr>(value))
#define PEER_CAPTURE(value) (reinterpret_cast<CaptureIrPtr>(value))

#define LLVM_SWITCH(inst) (reinterpret_cast<llvm::SwitchInst*>(inst))
#define PEER_SWITCH(inst) (reinterpret_cast<SwitchIrPtr>(inst))

// compilation

CompilerPeer compiler_peer_new() {
  return PEER(new Compiler());
}

void compiler_peer_delete(CompilerPeer peer) {
  delete IMPL(peer);
}

void compiler_peer_start(CompilerPeer peer, bool enable_labels) {
  if (enable_labels) {
    IMPL(peer)->EnableLabels();
  }
}

ModulePeer compiler_peer_end(CompilerPeer peer) {
  return reinterpret_cast<ModulePeer>(IMPL(peer)->TakeModule());
}

void compiler_peer_set_data_layout(CompilerPeer peer, const char* data_layout) {
  IMPL(peer)->SetDataLayout(data_layout);
}

void compiler_peer_set_target_triple(CompilerPeer peer, const char* triple) {
  IMPL(peer)->SetTargetTriple(triple);
}

void compiler_peer_start_function(CompilerPeer peer, uint32_t func_id) {
  IMPL(peer)->StartFunction(func_id);
}

void compiler_peer_end_function(CompilerPeer peer, bool optimize) {
  IMPL(peer)->EndFunction(optimize);
}

void compiler_peer_set_locals_block(CompilerPeer peer, BasicBlockPtr block) {
  IMPL(peer)->SetLocalsBlock(LLVM_BB(block));
}

LambdaIrPtr compiler_peer_get_function(CompilerPeer peer, uint32_t func_id) {
  return PEER_LAMBDA(IMPL(peer)->GetFunction(func_id));
}

// basic block

BasicBlockPtr compiler_peer_create_basic_block(CompilerPeer peer,
                                               const char* name,
                                               size_t name_len) {
  return PEER_BB(IMPL(peer)->CreateBasicBlock(name, name_len));
}

BasicBlockPtr compiler_peer_get_basic_block(const CompilerPeer peer) {
  return PEER_BB(IMPL(peer)->GetBasicBlock());
}

void compiler_peer_set_basic_block(CompilerPeer peer, BasicBlockPtr block) {
  IMPL(peer)->SetBasicBlock(LLVM_BB(block));
}

void compiler_peer_move_basic_block_after(CompilerPeer peer, BasicBlockPtr block) {
  IMPL(peer)->MoveBasicBlockAfter(LLVM_BB(block));
}

bool compiler_peer_is_basic_block_terminated(CompilerPeer peer, BasicBlockPtr block) {
  return IMPL(peer)->IsBasicBlockTerminated(LLVM_BB(block));
}

// jump

void compiler_peer_create_br(CompilerPeer peer, BasicBlockPtr block) {
  IMPL(peer)->CreateBr(LLVM_BB(block));
}

void compiler_peer_create_cond_br(CompilerPeer peer,
                                  BooleanIrPtr cond,
                                  BasicBlockPtr then_block,
                                  BasicBlockPtr else_block) {
  IMPL(peer)->CreateCondBr(LLVM_VALUE(cond), LLVM_BB(then_block), LLVM_BB(else_block));
}

// undefined

BooleanIrPtr compiler_peer_create_is_undefined(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsUndefined(LLVM_VALUE(value)));
}

// null

BooleanIrPtr compiler_peer_create_is_null(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsNull(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_non_nullish(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsNonNullish(LLVM_VALUE(value)));
}

// boolean

BooleanIrPtr compiler_peer_create_is_boolean(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsBoolean(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_same_boolean(CompilerPeer peer,
                                                  BooleanIrPtr a,
                                                  BooleanIrPtr b) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSameBoolean(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIrPtr compiler_peer_create_number_to_boolean(CompilerPeer peer, NumberIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateNumberToBoolean(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_to_boolean(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateToBoolean(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_get_boolean(CompilerPeer peer, bool value) {
  return PEER_BOOLEAN(IMPL(peer)->GetBoolean(value));
}

BooleanIrPtr compiler_peer_create_logical_not(CompilerPeer peer, BooleanIrPtr boolean) {
  return PEER_BOOLEAN(IMPL(peer)->CreateLogicalNot(LLVM_VALUE(boolean)));
}

BooleanIrPtr compiler_peer_create_boolean_phi(CompilerPeer peer,
                                              BooleanIrPtr then_value,
                                              BasicBlockPtr then_block,
                                              BooleanIrPtr else_value,
                                              BasicBlockPtr else_block) {
  return PEER_BOOLEAN(IMPL(peer)->CreateBooleanPhi(LLVM_VALUE(then_value), LLVM_BB(then_block),
                                                   LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

// number

BooleanIrPtr compiler_peer_create_is_number(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsNumber(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_same_number(CompilerPeer peer, NumberIrPtr a, NumberIrPtr b) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSameNumber(LLVM_VALUE(a), LLVM_VALUE(b)));
}

NumberIrPtr compiler_peer_create_boolean_to_number(CompilerPeer peer, BooleanIrPtr value) {
  return PEER_NUMBER(IMPL(peer)->CreateUIToFP(LLVM_VALUE(value)));
}

NumberIrPtr compiler_peer_to_numeric(CompilerPeer peer, ValueIrPtr value) {
  return PEER_NUMBER(IMPL(peer)->ToNumeric(LLVM_VALUE(value)));
}

NumberIrPtr compiler_peer_get_nan(CompilerPeer peer) {
  return PEER_NUMBER(IMPL(peer)->GetNan());
}

NumberIrPtr compiler_peer_get_zero(CompilerPeer peer) {
  return PEER_NUMBER(IMPL(peer)->GetZero());
}

NumberIrPtr compiler_peer_get_number(CompilerPeer peer, double value) {
  return PEER_NUMBER(IMPL(peer)->GetNumber(value));
}

NumberIrPtr compiler_peer_create_bitwise_not(CompilerPeer peer, NumberIrPtr value) {
  return PEER_NUMBER(IMPL(peer)->CreateBitwiseNot(LLVM_VALUE(value)));
}

NumberIrPtr compiler_peer_create_fneg(CompilerPeer peer, NumberIrPtr value) {
  return PEER_NUMBER(IMPL(peer)->CreateFNeg(LLVM_VALUE(value)));
}

NumberIrPtr compiler_peer_create_fmul(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateFMul(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_fdiv(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateFDiv(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_frem(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateFRem(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_fadd(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateFAdd(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_fsub(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateFSub(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_left_shift(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateLeftShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_signed_right_shift(CompilerPeer peer,
                                                    NumberIrPtr lhs,
                                                    NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateSignedRightShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_unsigned_right_shift(CompilerPeer peer,
                                                      NumberIrPtr lhs,
                                                      NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateUnsignedRightShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_bitwise_and(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateBitwiseAnd(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_bitwise_xor(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateBitwiseXor(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_bitwise_or(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_NUMBER(IMPL(peer)->CreateBitwiseOr(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIrPtr compiler_peer_create_less_than(CompilerPeer peer, NumberIrPtr lhs, NumberIrPtr rhs) {
  return PEER_BOOLEAN(IMPL(peer)->CreateLessThan(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIrPtr compiler_peer_create_greater_than(CompilerPeer peer,
                                               NumberIrPtr lhs,
                                               NumberIrPtr rhs) {
  return PEER_BOOLEAN(IMPL(peer)->CreateGreaterThan(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIrPtr compiler_peer_create_less_than_or_equal(CompilerPeer peer,
                                                     NumberIrPtr lhs,
                                                     NumberIrPtr rhs) {
  return PEER_BOOLEAN(IMPL(peer)->CreateLessThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIrPtr compiler_peer_create_greater_than_or_equal(CompilerPeer peer,
                                                        NumberIrPtr lhs,
                                                        NumberIrPtr rhs) {
  return PEER_BOOLEAN(IMPL(peer)->CreateGreaterThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIrPtr compiler_peer_create_number_phi(CompilerPeer peer,
                                            NumberIrPtr then_value,
                                            BasicBlockPtr then_block,
                                            NumberIrPtr else_value,
                                            BasicBlockPtr else_block) {
  return PEER_NUMBER(IMPL(peer)->CreateNumberPhi(LLVM_VALUE(then_value), LLVM_BB(then_block),
                                                 LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

// closure

BooleanIrPtr compiler_peer_create_is_closure(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsClosure(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_same_closure(CompilerPeer peer,
                                                  ClosureIrPtr a,
                                                  ClosureIrPtr b) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSameClosure(LLVM_VALUE(a), LLVM_VALUE(b)));
}

ClosureIrPtr compiler_peer_get_closure_nullptr(CompilerPeer peer) {
  return PEER_CLOSURE(IMPL(peer)->GetNullptr());
}

ClosureIrPtr compiler_peer_create_closure(CompilerPeer peer,
                                          LambdaIrPtr lambda,
                                          uint16_t num_captures) {
  return PEER_CLOSURE(IMPL(peer)->CreateClosure(LLVM_LAMBDA(lambda), num_captures));
}

void compiler_peer_create_store_capture_to_closure(CompilerPeer peer,
                                                   CaptureIrPtr capture,
                                                   ClosureIrPtr closure,
                                                   uint16_t index) {
  IMPL(peer)->CreateStoreCapturePtrToClosure(LLVM_VALUE(capture), LLVM_VALUE(closure), index);
}

StatusIrPtr compiler_peer_create_call_on_closure(CompilerPeer peer,
                                                 ClosureIrPtr closure,
                                                 uint16_t argc,
                                                 ArgvIrPtr argv,
                                                 ValueIrPtr retv) {
  return PEER_STATUS(IMPL(peer)->CreateCallOnClosure(LLVM_VALUE(closure), argc, LLVM_VALUE(argv),
                                                     LLVM_VALUE(retv)));
}

ClosureIrPtr compiler_peer_create_closure_phi(CompilerPeer peer,
                                              ClosureIrPtr then_value,
                                              BasicBlockPtr then_block,
                                              ClosureIrPtr else_value,
                                              BasicBlockPtr else_block) {
  return PEER_CLOSURE(IMPL(peer)->CreateClosurePhi(LLVM_VALUE(then_value), LLVM_BB(then_block),
                                                   LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

// promise

BooleanIrPtr compiler_peer_create_is_promise(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsPromise(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_same_promise(CompilerPeer peer,
                                                  PromiseIrPtr a,
                                                  PromiseIrPtr b) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSamePromise(LLVM_VALUE(a), LLVM_VALUE(b)));
}

PromiseIrPtr compiler_peer_create_register_promise(CompilerPeer peer, CoroutineIrPtr coroutine) {
  return PEER_PROMISE(IMPL(peer)->CreateRegisterPromise(LLVM_VALUE(coroutine)));
}

void compiler_peer_create_await_promise(CompilerPeer peer,
                                        PromiseIrPtr promise,
                                        PromiseIrPtr awaiting) {
  IMPL(peer)->CreateAwaitPromise(LLVM_VALUE(promise), LLVM_VALUE(awaiting));
}

void compiler_peer_create_resume(CompilerPeer peer, PromiseIrPtr promise) {
  IMPL(peer)->CreateResume(LLVM_VALUE(promise));
}

void compiler_peer_create_emit_promise_resolved(CompilerPeer peer,
                                                PromiseIrPtr promise,
                                                ValueIrPtr result) {
  IMPL(peer)->CreateEmitPromiseResolved(LLVM_VALUE(promise), LLVM_VALUE(result));
}

// object

BooleanIrPtr compiler_peer_create_is_object(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsObject(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_same_object(CompilerPeer peer, ObjectIrPtr a, ObjectIrPtr b) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSameObject(LLVM_VALUE(a), LLVM_VALUE(b)));
}

ObjectIrPtr compiler_peer_create_object(CompilerPeer peer) {
  return PEER_OBJECT(IMPL(peer)->CreateObject());
}

// value

BooleanIrPtr compiler_peer_create_is_nullptr(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsNullptr(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_has_value(CompilerPeer peer, ValueIrPtr value) {
  return PEER_BOOLEAN(IMPL(peer)->CreateHasValue(LLVM_VALUE(value)));
}

BooleanIrPtr compiler_peer_create_is_loosely_equal(CompilerPeer peer,
                                                   ValueIrPtr lhs,
                                                   ValueIrPtr rhs) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsLooselyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIrPtr compiler_peer_create_is_strictly_equal(CompilerPeer peer,
                                                    ValueIrPtr lhs,
                                                    ValueIrPtr rhs) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsStrictlyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIrPtr compiler_peer_create_is_same_boolean_value(CompilerPeer peer,
                                                        ValueIrPtr value,
                                                        BooleanIrPtr boolean) {
  return PEER_BOOLEAN(
      IMPL(peer)->CreateIsSameBooleanValue(LLVM_VALUE(value), LLVM_VALUE(boolean)));
}

BooleanIrPtr compiler_peer_create_is_same_number_value(CompilerPeer peer,
                                                       ValueIrPtr value,
                                                       NumberIrPtr number) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSameNumberValue(LLVM_VALUE(value), LLVM_VALUE(number)));
}

BooleanIrPtr compiler_peer_create_is_same_closure_value(CompilerPeer peer,
                                                        ValueIrPtr value,
                                                        ClosureIrPtr closure) {
  return PEER_BOOLEAN(
      IMPL(peer)->CreateIsSameClosureValue(LLVM_VALUE(value), LLVM_VALUE(closure)));
}

BooleanIrPtr compiler_peer_create_is_same_promise_value(CompilerPeer peer,
                                                        ValueIrPtr value,
                                                        PromiseIrPtr promise) {
  return PEER_BOOLEAN(
      IMPL(peer)->CreateIsSamePromiseValue(LLVM_VALUE(value), LLVM_VALUE(promise)));
}

BooleanIrPtr compiler_peer_create_is_same_object_value(CompilerPeer peer,
                                                       ValueIrPtr value,
                                                       ObjectIrPtr object) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsSameObjectValue(LLVM_VALUE(value), LLVM_VALUE(object)));
}

ValueIrPtr compiler_peer_create_undefined_to_any(CompilerPeer peer) {
  return PEER_VALUE(IMPL(peer)->CreateUndefinedToAny());
}

ValueIrPtr compiler_peer_create_null_to_any(CompilerPeer peer) {
  return PEER_VALUE(IMPL(peer)->CreateNullToAny());
}

ValueIrPtr compiler_peer_create_boolean_to_any(CompilerPeer peer, BooleanIrPtr boolean) {
  return PEER_VALUE(IMPL(peer)->CreateBooleanToAny(LLVM_VALUE(boolean)));
}

ValueIrPtr compiler_peer_create_number_to_any(CompilerPeer peer, NumberIrPtr value) {
  return PEER_VALUE(IMPL(peer)->CreateNumberToAny(LLVM_VALUE(value)));
}

ValueIrPtr compiler_peer_create_closure_to_any(CompilerPeer peer, ClosureIrPtr value) {
  return PEER_VALUE(IMPL(peer)->CreateClosureToAny(LLVM_VALUE(value)));
}

ValueIrPtr compiler_peer_create_object_to_any(CompilerPeer peer, ObjectIrPtr value) {
  return PEER_VALUE(IMPL(peer)->CreateObjectToAny(LLVM_VALUE(value)));
}

ValueIrPtr compiler_peer_create_value_phi(CompilerPeer peer,
                                          ValueIrPtr then_value,
                                          BasicBlockPtr then_block,
                                          ValueIrPtr else_value,
                                          BasicBlockPtr else_block) {
  return PEER_VALUE(IMPL(peer)->CreateValuePhi(LLVM_VALUE(then_value), LLVM_BB(then_block),
                                               LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

ValueIrPtr compiler_peer_create_local_value(CompilerPeer peer, uint16_t index) {
  return PEER_VALUE(IMPL(peer)->CreateLocalValue(index));
}

void compiler_peer_create_store_none_to_value(CompilerPeer peer, ValueIrPtr dest) {
  IMPL(peer)->CreateStoreNoneToValue(LLVM_VALUE(dest));
}

void compiler_peer_create_store_undefined_to_value(CompilerPeer peer, ValueIrPtr dest) {
  IMPL(peer)->CreateStoreUndefinedToValue(LLVM_VALUE(dest));
}

void compiler_peer_create_store_null_to_value(CompilerPeer peer, ValueIrPtr dest) {
  IMPL(peer)->CreateStoreNullToValue(LLVM_VALUE(dest));
}

void compiler_peer_create_store_boolean_to_value(CompilerPeer peer,
                                                 BooleanIrPtr value,
                                                 ValueIrPtr dest) {
  IMPL(peer)->CreateStoreBooleanToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_number_to_value(CompilerPeer peer,
                                                NumberIrPtr value,
                                                ValueIrPtr dest) {
  IMPL(peer)->CreateStoreNumberToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_closure_to_value(CompilerPeer peer,
                                                 ClosureIrPtr value,
                                                 ValueIrPtr dest) {
  IMPL(peer)->CreateStoreClosureToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_promise_to_value(CompilerPeer peer,
                                                 PromiseIrPtr value,
                                                 ValueIrPtr dest) {
  IMPL(peer)->CreateStorePromiseToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_object_to_value(CompilerPeer peer,
                                                ObjectIrPtr value,
                                                ValueIrPtr dest) {
  IMPL(peer)->CreateStoreObjectToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

void compiler_peer_create_store_value_to_value(CompilerPeer peer,
                                               ValueIrPtr value,
                                               ValueIrPtr dest) {
  IMPL(peer)->CreateStoreValueToValue(LLVM_VALUE(value), LLVM_VALUE(dest));
}

ClosureIrPtr compiler_peer_create_load_closure_from_value(CompilerPeer peer, ValueIrPtr value) {
  return PEER_CLOSURE(IMPL(peer)->CreateLoadClosureFromValue(LLVM_VALUE(value)));
}

PromiseIrPtr compiler_peer_create_load_promise_from_value(CompilerPeer peer, ValueIrPtr value) {
  return PEER_PROMISE(IMPL(peer)->CreateLoadPromiseFromValue(LLVM_VALUE(value)));
}

// argv

ArgvIrPtr compiler_peer_get_argv_nullptr(CompilerPeer peer) {
  return PEER_ARGV(IMPL(peer)->GetNullptr());
}

ArgvIrPtr compiler_peer_create_argv(CompilerPeer peer, uint16_t argc) {
  return PEER_ARGV(IMPL(peer)->CreateArgv(argc));
}

ValueIrPtr compiler_peer_create_get_arg_in_argv(CompilerPeer peer,
                                                ArgvIrPtr argv,
                                                uint16_t index) {
  return PEER_VALUE(IMPL(peer)->CreateGetArgInArgv(LLVM_VALUE(argv), index));
}

ValueIrPtr compiler_peer_create_get_argument_value_ptr(CompilerPeer peer, uint16_t index) {
  return PEER_VALUE(IMPL(peer)->CreateGetArgumentValuePtr(index));
}

// retv

ValueIrPtr compiler_peer_create_retv(CompilerPeer peer) {
  return PEER_VALUE(IMPL(peer)->CreateRetv());
}

void compiler_peer_create_store_undefined_to_retv(CompilerPeer peer) {
  IMPL(peer)->CreateStoreUndefinedToRetv();
}

void compiler_peer_create_store_null_to_retv(CompilerPeer peer) {
  IMPL(peer)->CreateStoreNullToRetv();
}

void compiler_peer_create_store_boolean_to_retv(CompilerPeer peer, BooleanIrPtr value) {
  IMPL(peer)->CreateStoreBooleanToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_number_to_retv(CompilerPeer peer, NumberIrPtr value) {
  IMPL(peer)->CreateStoreNumberToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_closure_to_retv(CompilerPeer peer, ClosureIrPtr value) {
  IMPL(peer)->CreateStoreClosureToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_promise_to_retv(CompilerPeer peer, PromiseIrPtr value) {
  IMPL(peer)->CreateStorePromiseToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_object_to_retv(CompilerPeer peer, ObjectIrPtr value) {
  IMPL(peer)->CreateStoreObjectToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_value_to_retv(CompilerPeer peer, ValueIrPtr value) {
  IMPL(peer)->CreateStoreValueToRetv(LLVM_VALUE(value));
}

ValueIrPtr compiler_peer_get_exception(CompilerPeer peer) {
  return PEER_VALUE(IMPL(peer)->GetException());
}

// status

void compiler_peer_create_alloc_status(CompilerPeer peer) {
  IMPL(peer)->CreateAllocStatus();
}

void compiler_peer_create_store_normal_status(CompilerPeer peer) {
  IMPL(peer)->CreateStoreNormalStatus();
}

void compiler_peer_create_store_exception_status(CompilerPeer peer) {
  IMPL(peer)->CreateStoreExceptionStatus();
}

BooleanIrPtr compiler_peer_create_is_exception_status(CompilerPeer peer, StatusIrPtr status) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsExceptionStatus(LLVM_VALUE(status)));
}

// flow selector

void compiler_peer_create_alloc_flow_selector(CompilerPeer peer) {
  IMPL(peer)->CreateAllocFlowSelector();
}

void compiler_peer_create_set_flow_selector_normal(CompilerPeer peer) {
  IMPL(peer)->CreateSetFlowSelectorNormal();
}

void compiler_peer_create_set_flow_selector_return(CompilerPeer peer) {
  IMPL(peer)->CreateSetFlowSelectorReturn();
}

void compiler_peer_create_set_flow_selector_throw(CompilerPeer peer) {
  IMPL(peer)->CreateSetFlowSelectorThrow();
}

void compiler_peer_create_set_flow_selector_break(CompilerPeer peer, uint32_t depth) {
  IMPL(peer)->CreateSetFlowSelectorBreak(depth);
}

void compiler_peer_create_set_flow_selector_continue(CompilerPeer peer, uint32_t depth) {
  IMPL(peer)->CreateSetFlowSelectorContinue(depth);
}

BooleanIrPtr compiler_peer_create_is_flow_selector_normal(CompilerPeer peer) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsFlowSelectorNormal());
}

BooleanIrPtr compiler_peer_create_is_flow_selector_normal_or_continue(CompilerPeer peer,
                                                                      uint32_t depth) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsFlowSelectorNormalOrContinue(depth));
}

BooleanIrPtr compiler_peer_create_is_flow_selector_break_or_continue(CompilerPeer peer,
                                                                     uint32_t depth) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsFlowSelectorBreakOrContinue(depth));
}

BooleanIrPtr compiler_peer_create_is_flow_selector_break(CompilerPeer peer, uint32_t depth) {
  return PEER_BOOLEAN(IMPL(peer)->CreateIsFlowSelectorBreak(depth));
}

// capture

CaptureIrPtr compiler_peer_create_capture(CompilerPeer peer, ValueIrPtr value) {
  return PEER_CAPTURE(IMPL(peer)->CreateCapture(LLVM_VALUE(value)));
}

ValueIrPtr compiler_peer_create_get_capture_value_ptr(CompilerPeer peer, uint16_t index) {
  return PEER_VALUE(IMPL(peer)->CreateGetCaptureValuePtr(index));
}

void compiler_peer_create_escape_value(CompilerPeer peer, CaptureIrPtr capture, ValueIrPtr value) {
  IMPL(peer)->CreateEscapeValue(LLVM_VALUE(capture), LLVM_VALUE(value));
}

CaptureIrPtr compiler_peer_create_load_capture(CompilerPeer peer, uint16_t index) {
  return PEER_CAPTURE(IMPL(peer)->CreateLoadCapture(index));
}

// coroutine

CoroutineIrPtr compiler_peer_create_coroutine(CompilerPeer peer,
                                              ClosureIrPtr closure,
                                              uint16_t num_locals,
                                              uint16_t scratch_buffer_len) {
  return PEER_COROUTINE(
      IMPL(peer)->CreateCoroutine(LLVM_VALUE(closure), num_locals, scratch_buffer_len));
}

SwitchIrPtr compiler_peer_create_switch_for_coroutine(CompilerPeer peer,
                                                      BasicBlockPtr block,
                                                      uint32_t num_states) {
  return PEER_SWITCH(IMPL(peer)->CreateSwitchForCoroutine(LLVM_BB(block), num_states));
}

void compiler_peer_create_add_state_for_coroutine(CompilerPeer peer,
                                                  SwitchIrPtr inst,
                                                  uint32_t state,
                                                  BasicBlockPtr block) {
  IMPL(peer)->CreateAddStateForCoroutine(LLVM_SWITCH(inst), state, LLVM_BB(block));
}

void compiler_peer_create_suspend(CompilerPeer peer) {
  IMPL(peer)->CreateSuspend();
}

void compiler_peer_create_set_coroutine_state(CompilerPeer peer, uint32_t state) {
  IMPL(peer)->CreateSetCoroutineState(state);
}

void compiler_peer_create_set_captures_for_coroutine(CompilerPeer peer) {
  IMPL(peer)->CreateSetCapturesForCoroutine();
}

ValueIrPtr compiler_peer_create_get_local_ptr_from_coroutine(CompilerPeer peer, uint16_t index) {
  return PEER_VALUE(IMPL(peer)->CreateGetLocalPtrFromCoroutine(index));
}

void compiler_peer_create_write_boolean_to_scratch_buffer(CompilerPeer peer,
                                                          uint32_t offset,
                                                          BooleanIrPtr value) {
  IMPL(peer)->CreateWriteBooleanToScratchBuffer(offset, LLVM_VALUE(value));
}

BooleanIrPtr compiler_peer_create_read_boolean_from_scratch_buffer(CompilerPeer peer,
                                                                   uint32_t offset) {
  return PEER_BOOLEAN(IMPL(peer)->CreateReadBooleanFromScratchBuffer(offset));
}

void compiler_peer_create_write_number_to_scratch_buffer(CompilerPeer peer,
                                                         uint32_t offset,
                                                         NumberIrPtr value) {
  IMPL(peer)->CreateWriteNumberToScratchBuffer(offset, LLVM_VALUE(value));
}

NumberIrPtr compiler_peer_create_read_number_from_scratch_buffer(CompilerPeer peer,
                                                                 uint32_t offset) {
  return PEER_NUMBER(IMPL(peer)->CreateReadNumberFromScratchBuffer(offset));
}

void compiler_peer_create_write_closure_to_scratch_buffer(CompilerPeer peer,
                                                          uint32_t offset,
                                                          ClosureIrPtr value) {
  IMPL(peer)->CreateWriteClosureToScratchBuffer(offset, LLVM_VALUE(value));
}

ClosureIrPtr compiler_peer_create_read_closure_from_scratch_buffer(CompilerPeer peer,
                                                                   uint32_t offset) {
  return PEER_CLOSURE(IMPL(peer)->CreateReadClosureFromScratchBuffer(offset));
}

void compiler_peer_create_write_object_to_scratch_buffer(CompilerPeer peer,
                                                         uint32_t offset,
                                                         ObjectIrPtr value) {
  IMPL(peer)->CreateWriteObjectToScratchBuffer(offset, LLVM_VALUE(value));
}

ClosureIrPtr compiler_peer_create_read_object_from_scratch_buffer(CompilerPeer peer,
                                                                  uint32_t offset) {
  return PEER_OBJECT(IMPL(peer)->CreateReadObjectFromScratchBuffer(offset));
}

void compiler_peer_create_write_promise_to_scratch_buffer(CompilerPeer peer,
                                                          uint32_t offset,
                                                          PromiseIrPtr value) {
  IMPL(peer)->CreateWritePromiseToScratchBuffer(offset, LLVM_VALUE(value));
}

PromiseIrPtr compiler_peer_create_read_promise_from_scratch_buffer(CompilerPeer peer,
                                                                   uint32_t offset) {
  return PEER_PROMISE(IMPL(peer)->CreateReadPromiseFromScratchBuffer(offset));
}

void compiler_peer_create_write_value_to_scratch_buffer(CompilerPeer peer,
                                                        uint32_t offset,
                                                        ValueIrPtr value) {
  IMPL(peer)->CreateWriteValueToScratchBuffer(offset, LLVM_VALUE(value));
}

ValueIrPtr compiler_peer_create_read_value_from_scratch_buffer(CompilerPeer peer,
                                                               uint32_t offset) {
  return PEER_VALUE(IMPL(peer)->CreateReadValueFromScratchBuffer(offset));
}

// object

ValueIrPtr compiler_peer_create_get(CompilerPeer peer, uint32_t symbol) {
  return PEER_VALUE(IMPL(peer)->CreateGet(symbol));
}

void compiler_peer_create_set(CompilerPeer peer, uint32_t symbol, ValueIrPtr value) {
  assert(value != nullptr);
  IMPL(peer)->CreateSet(symbol, LLVM_VALUE(value));
}

// scope cleanup checker

void compiler_peer_enable_scope_cleanup_checker(CompilerPeer peer, bool is_coroutine) {
  IMPL(peer)->EnableScopeCleanupChecker(is_coroutine);
}

void compiler_peer_set_scope_id_for_checker(CompilerPeer peer, uint16_t scope_id) {
  IMPL(peer)->SetScopeIdForChecker(scope_id);
}

void compiler_peer_assert_scope_id(CompilerPeer peer, uint16_t expected) {
  IMPL(peer)->AssertScopeId(expected);
}

// print

void compiler_peer_create_print_value(CompilerPeer peer, ValueIrPtr value, const char* msg) {
  IMPL(peer)->CreatePrintValue(LLVM_VALUE(value), msg);
}

void compiler_peer_create_print_message(CompilerPeer peer, const char* msg) {
  IMPL(peer)->CreatePrintMessage(msg);
}

// debugger

void compiler_peer_create_debugger(CompilerPeer peer) {
  IMPL(peer)->CreateDebugger();
}

// unreachable

void compiler_peer_create_unreachable(CompilerPeer peer, const char* msg) {
  IMPL(peer)->CreateUnreachable(msg);
}

// helper functions

size_t compiler_peer_get_basic_block_name_or_as_operand(BasicBlockPtr block,
                                                        char* buf,
                                                        size_t len) {
  return Compiler::GetNameOrAsOperand(LLVM_BB(block), buf, len);
}

size_t compiler_peer_get_value_name_or_as_operand(ValueIrPtr value, char* buf, size_t len) {
  return Compiler::GetNameOrAsOperand(LLVM_VALUE(value), buf, len);
}
