#include "bridge.hh"

#include <cstdint>

#include <llvm/Support/TargetSelect.h>

#include "compiler.hh"
#include "executor.hh"
#include "helper.hh"
#include "module.hh"

#define PEER_BB(bb) (reinterpret_cast<BasicBlock*>(bb))
#define LLVM_BB(bb) (reinterpret_cast<llvm::BasicBlock*>(bb))
#define PEER_BOOLEAN(value) (reinterpret_cast<BooleanIr*>(value))
#define PEER_VALUE(value) (reinterpret_cast<ValueIr*>(value))
#define LLVM_VALUE(value) (reinterpret_cast<llvm::Value*>(value))
#define PEER_LAMBDA(lambda) (reinterpret_cast<LambdaIr*>(lambda))
#define LLVM_LAMBDA(lambda) (reinterpret_cast<llvm::Function*>(lambda))

void llvmir_initialize() {
  // Uncomment if you want to enable LLVM_DEBUG().
  // llvm::DebugFlag = true;

  llvm::InitializeNativeTarget();
  llvm::InitializeNativeTargetAsmPrinter();
  llvm::InitializeNativeTargetAsmParser();
}

void module_peer_print(Module* self, bool stderr) {
  self->Print(stderr);
}

void module_peer_delete(Module* self) {
  delete self;
}

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

void compiler_peer_set_locals_block(Compiler* self, BasicBlock* block) {
  self->SetLocalsBlock(LLVM_BB(block));
}

void compiler_peer_create_br(Compiler* self, BasicBlock* block) {
  self->CreateBr(LLVM_BB(block));
}

BooleanIr* compiler_peer_get_boolean(Compiler* self, bool value) {
  return PEER_BOOLEAN(self->GetBoolean(value));
}

ValueIr* compiler_peer_get_number(Compiler* self, double value) {
  return PEER_VALUE(self->GetNumber(value));
}

LambdaIr* compiler_peer_get_function(Compiler* self, uint32_t func_id, const char* name) {
  return PEER_LAMBDA(self->GetFunction(func_id, name));
}

ValueIr* compiler_peer_create_call_runtime_create_closure(Compiler* self, LambdaIr* lambda, uint16_t num_captures) {
  return PEER_VALUE(self->CreateCallRuntimeCreateClosure(LLVM_LAMBDA(lambda), num_captures));
}

ValueIr* compiler_peer_create_load_captures_from_closure(Compiler* self, ValueIr* closure) {
  return PEER_VALUE(self->CreateLoadCapturesFromClosure(LLVM_VALUE(closure)));
}

void compiler_peer_create_store_capture_ptr_to_captures(Compiler* self, ValueIr* capture, ValueIr* captures, uint16_t i) {
  self->CreateStoreCapturePtrToCaptures(LLVM_VALUE(capture), LLVM_VALUE(captures), i);
}

ValueIr* compiler_peer_get_exception(Compiler* self) {
  return PEER_VALUE(self->GetException());
}

ValueIr* compiler_peer_create_fneg(Compiler* self, ValueIr* value) {
  return PEER_VALUE(self->CreateFNeg(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_bitwise_not(Compiler* self, ValueIr* number) {
  return PEER_VALUE(self->CreateBitwiseNot(LLVM_VALUE(number)));
}

BooleanIr* compiler_peer_create_number_to_boolean(Compiler* self, ValueIr* number) {
  return PEER_BOOLEAN(self->CreateNumberToBoolean(LLVM_VALUE(number)));
}

BooleanIr* compiler_peer_create_to_boolean(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateToBoolean(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_logical_not(Compiler* self, BooleanIr* boolean) {
  return PEER_BOOLEAN(self->CreateLogicalNot(LLVM_VALUE(boolean)));
}

ValueIr* compiler_peer_create_fmul(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateFMul(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_fdiv(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateFDiv(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_frem(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateFRem(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_fadd(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateFAdd(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_fsub(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateFSub(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_left_shift(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateLeftShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_signed_right_shift(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateSignedRightShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_unsigned_right_shift(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateUnsignedRightShift(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_less_than(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateLessThan(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_greater_than(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateGreaterThan(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_less_than_or_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateLessThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_greater_than_or_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateGreaterThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_is_loosely_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateIsLooselyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
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

ValueIr* compiler_peer_create_number_to_any(Compiler* self, ValueIr* number) {
  return PEER_VALUE(self->CreateNumberToAny(LLVM_VALUE(number)));
}

ValueIr* compiler_peer_create_closure_to_any(Compiler* self, ValueIr* closure) {
  return PEER_VALUE(self->CreateClosureToAny(LLVM_VALUE(closure)));
}

BooleanIr* compiler_peer_create_is_undefined(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsUndefined(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_null(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsNull(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_boolean(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsBoolean(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_number(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsNumber(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_closure(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsClosure(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_same_boolean(Compiler* self, BooleanIr* a, BooleanIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameBoolean(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIr* compiler_peer_create_is_same_number(Compiler* self, ValueIr* a, ValueIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameNumber(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIr* compiler_peer_create_is_same_closure(Compiler* self, ValueIr* a, ValueIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameClosure(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIr* compiler_peer_create_is_same_boolean_value(Compiler* self, ValueIr* value, BooleanIr* boolean) {
  return PEER_BOOLEAN(self->CreateIsSameBooleanValue(LLVM_VALUE(value), LLVM_VALUE(boolean)));
}

BooleanIr* compiler_peer_create_is_same_number_value(Compiler* self, ValueIr* value, ValueIr* number) {
  return PEER_BOOLEAN(self->CreateIsSameNumberValue(LLVM_VALUE(value), LLVM_VALUE(number)));
}

BooleanIr* compiler_peer_create_is_same_closure_value(Compiler* self, ValueIr* value, ValueIr* closure) {
  return PEER_BOOLEAN(self->CreateIsSameClosureValue(LLVM_VALUE(value), LLVM_VALUE(closure)));
}

BooleanIr* compiler_peer_create_is_strictly_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateIsStrictlyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_bitwise_and(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateBitwiseAnd(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_bitwise_xor(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateBitwiseXor(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

ValueIr* compiler_peer_create_bitwise_or(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_VALUE(self->CreateBitwiseOr(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_boolean_ternary(Compiler* self, BooleanIr* then_value, BasicBlock* then_block, BooleanIr* else_value, BasicBlock* else_block) {
  return PEER_BOOLEAN(self->CreateBooleanTernary(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

ValueIr* compiler_peer_create_number_ternary(Compiler* self, ValueIr* then_value, BasicBlock* then_block, ValueIr* else_value, BasicBlock* else_block) {
  return PEER_VALUE(self->CreateNumberTernary(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

ValueIr* compiler_peer_create_any_ternary(Compiler* self, ValueIr* then_value, BasicBlock* then_block, ValueIr* else_value, BasicBlock* else_block) {
  return PEER_VALUE(self->CreateAnyTernary(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

void compiler_peer_create_store_flags_to_variable(Compiler* self, uint8_t flags, ValueIr* variable) {
  self->CreateStoreFlagsToVariable(flags, LLVM_VALUE(variable));
}

void compiler_peer_create_store_symbol_to_variable(Compiler* self, uint32_t symbol, ValueIr* variable) {
  self->CreateStoreSymbolToVariable(symbol, LLVM_VALUE(variable));
}

void compiler_peer_create_store_undefined_to_variable(Compiler* self, ValueIr* variable) {
  self->CreateStoreUndefinedToValue(LLVM_VALUE(variable));
}

void compiler_peer_create_store_null_to_variable(Compiler* self, ValueIr* variable) {
  self->CreateStoreNullToValue(LLVM_VALUE(variable));
}

void compiler_peer_create_store_boolean_to_variable(Compiler* self, BooleanIr* value, ValueIr* variable) {
  self->CreateStoreBooleanToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

void compiler_peer_create_store_number_to_variable(Compiler* self, ValueIr* value, ValueIr* variable) {
  self->CreateStoreNumberToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

void compiler_peer_create_store_closure_to_variable(Compiler* self, ValueIr* value, ValueIr* variable) {
  self->CreateStoreClosureToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

void compiler_peer_create_store_value_to_variable(Compiler* self, ValueIr* value, ValueIr* variable) {
  self->CreateStoreValueToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

ValueIr* compiler_peer_create_call_on_closure(Compiler* self, ValueIr* closure, uint16_t argc, ValueIr* argv, ValueIr* retv) {
  return PEER_VALUE(self->CreateCallOnClosure(LLVM_VALUE(closure), argc, LLVM_VALUE(argv), LLVM_VALUE(retv)));
}

ValueIr* compiler_peer_create_closure_ptr(Compiler* self) {
  return PEER_VALUE(self->CreateClosurePtr());
}

ValueIr* compiler_peer_get_nullptr(Compiler* self) {
  return PEER_VALUE(self->GetNullptr());
}

void compiler_peer_create_cond_br(Compiler* self, BooleanIr* cond, BasicBlock* then_block, BasicBlock* else_block) {
  self->CreateCondBr(LLVM_VALUE(cond), LLVM_BB(then_block), LLVM_BB(else_block));
}

ValueIr* compiler_peer_create_load_closure_from_value(Compiler* self, ValueIr* value) {
  return PEER_VALUE(self->CreateLoadClosureFromValue(LLVM_VALUE(value)));
}

void compiler_peer_create_store(Compiler* self, ValueIr* value, ValueIr* dest) {
  self->CreateStore(LLVM_VALUE(value), LLVM_VALUE(dest));
}

ValueIr* compiler_peer_create_load_closure(Compiler* self, ValueIr* closure_ptr) {
  return PEER_VALUE(self->CreateLoadClosure(LLVM_VALUE(closure_ptr)));
}

ValueIr* compiler_peer_create_call_runtime_create_capture(Compiler* self, ValueIr* variable) {
  return PEER_VALUE(self->CreateCallRuntimeCreateCapture(LLVM_VALUE(variable)));
}

ValueIr* compiler_peer_create_get_capture_variable_ptr(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateGetCaptureVariablePtr(index));
}

void compiler_peer_create_escape_variable(Compiler* self, ValueIr* capture, ValueIr* variable) {
  self->CreateEscapeVariable(LLVM_VALUE(capture), LLVM_VALUE(variable));
}

ValueIr* compiler_peer_create_load_capture(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateLoadCapture(index));
}

ValueIr* compiler_peer_get_nan(Compiler* self) {
  return PEER_VALUE(self->GetNan());
}

ValueIr* compiler_peer_get_zero(Compiler* self) {
  return PEER_VALUE(self->GetZero());
}

ValueIr* compiler_peer_create_boolean_to_number(Compiler* self, BooleanIr* value) {
  return PEER_VALUE(self->CreateUIToFP(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_to_numeric(Compiler* self, ValueIr* value) {
  return PEER_VALUE(self->ToNumeric(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_is_non_nullish(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateIsNonNullish(LLVM_VALUE(value)));
}

BooleanIr* compiler_peer_create_has_uncaught_exception(Compiler* self) {
  return PEER_BOOLEAN(self->CreateHasUncaughtException());
}

void compiler_peer_start_function(Compiler* self, const char* name) {
  self->StartFunction(name);
}

void compiler_peer_end_function(Compiler* self, bool optimize) {
  self->EndFunction(optimize);
}

void compiler_peer_handle_returned_thrown(Compiler* self,
    bool returned,
    bool thrown,
    BasicBlock* block,
    BasicBlock* cleanup_block,
    BasicBlock* exception_block) {
  self->HandleReturnedThrown(
      returned, thrown, LLVM_BB(block), LLVM_BB(cleanup_block), LLVM_BB(exception_block));
}

ValueIr* compiler_peer_create_local_variable(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateLocalVariable(index));
}

// incr/decr

ValueIr* compiler_peer_create_incr(Compiler* self, ValueIr* value) {
  return PEER_VALUE(self->CreateIncr(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_decr(Compiler* self, ValueIr* value) {
  return PEER_VALUE(self->CreateDecr(LLVM_VALUE(value)));
}

// argv

ValueIr* compiler_peer_create_get_argument_variable_ptr(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateGetArgumentVariablePtr(index));
}

ValueIr* compiler_peer_create_argv(Compiler* self, uint16_t argc) {
  return PEER_VALUE(self->CreateArgv(argc));
}

ValueIr* compiler_peer_create_get_arg_in_argv(Compiler* self, ValueIr* argv, uint16_t index) {
  return PEER_VALUE(self->CreateGetArgInArgv(LLVM_VALUE(argv), index));
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

void compiler_peer_create_store_number_to_retv(Compiler* self, ValueIr* value) {
  self->CreateStoreNumberToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_closure_to_retv(Compiler* self, ValueIr* value) {
  self->CreateStoreClosureToRetv(LLVM_VALUE(value));
}

void compiler_peer_create_store_value_to_retv(Compiler* self, ValueIr* value) {
  self->CreateStoreValueToRetv(LLVM_VALUE(value));
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

BooleanIr* compiler_peer_create_is_exception_status(Compiler* self, ValueIr* status) {
  return PEER_BOOLEAN(self->CreateIsExceptionStatus(LLVM_VALUE(status)));
}

// scope cleanup checker

void compiler_peer_prepare_scope_cleanup_checker(Compiler* self, uint16_t stack_size) {
  self->PrepareScopeCleanupChecker(stack_size);
}

void compiler_peer_start_scope_cleanup_checker(Compiler* self, uint16_t scope_id) {
  self->StartScopeCleanupChecker(scope_id);
}

void compiler_peer_end_scope_cleanup_checker(Compiler* self, uint16_t scope_id) {
  self->EndScopeCleanupChecker(scope_id);
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
