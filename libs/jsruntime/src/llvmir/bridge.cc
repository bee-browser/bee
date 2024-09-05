#include "bridge.hh"

#include <cstdint>

#include <llvm/Support/TargetSelect.h>

#include "compiler.hh"
#include "executor.hh"
#include "helper.hh"
#include "module.hh"

#define PEER_BB(bb) (reinterpret_cast<BasicBlock*>(bb))
#define LLVM_BB(bb) (reinterpret_cast<llvm::BasicBlock*>(bb))

#define PEER_LAMBDA(lambda) (reinterpret_cast<LambdaIr*>(lambda))
#define LLVM_LAMBDA(lambda) (reinterpret_cast<llvm::Function*>(lambda))

#define PEER_BOOLEAN(value) (reinterpret_cast<BooleanIr*>(value))
#define PEER_NUMBER(value) (reinterpret_cast<NumberIr*>(value))
#define PEER_CLOSURE(value) (reinterpret_cast<ClosureIr*>(value))
#define PEER_VALUE(value) (reinterpret_cast<ValueIr*>(value))
#define PEER_ARGV(value) (reinterpret_cast<ArgvIr*>(value))
#define PEER_STATUS(value) (reinterpret_cast<StatusIr*>(value))
#define LLVM_VALUE(value) (reinterpret_cast<llvm::Value*>(value))

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

// boolean

BooleanIr* compiler_peer_get_boolean(Compiler* self, bool value) {
  return PEER_BOOLEAN(self->GetBoolean(value));
}

BooleanIr* compiler_peer_create_logical_not(Compiler* self, BooleanIr* boolean) {
  return PEER_BOOLEAN(self->CreateLogicalNot(LLVM_VALUE(boolean)));
}

NumberIr* compiler_peer_create_boolean_to_number(Compiler* self, BooleanIr* value) {
  return PEER_NUMBER(self->CreateUIToFP(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_boolean_to_any(Compiler* self, BooleanIr* boolean) {
  return PEER_VALUE(self->CreateBooleanToAny(LLVM_VALUE(boolean)));
}

// number

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

BooleanIr* compiler_peer_create_greater_than_or_equal(Compiler* self, NumberIr* lhs, NumberIr* rhs) {
  return PEER_BOOLEAN(self->CreateGreaterThanOrEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

NumberIr* compiler_peer_create_number_phi(Compiler* self, NumberIr* then_value, BasicBlock* then_block, NumberIr* else_value, BasicBlock* else_block) {
  return PEER_NUMBER(self->CreateNumberPhi(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

BooleanIr* compiler_peer_create_number_to_boolean(Compiler* self, NumberIr* value) {
  return PEER_BOOLEAN(self->CreateNumberToBoolean(LLVM_VALUE(value)));
}

ValueIr* compiler_peer_create_number_to_any(Compiler* self, NumberIr* value) {
  return PEER_VALUE(self->CreateNumberToAny(LLVM_VALUE(value)));
}

// closure

ClosureIr* compiler_peer_get_closure_nullptr(Compiler* self) {
  return PEER_CLOSURE(self->GetNullptr());
}

ClosureIr* compiler_peer_create_closure(Compiler* self, LambdaIr* lambda, uint16_t num_captures) {
  return PEER_CLOSURE(self->CreateCallRuntimeCreateClosure(LLVM_LAMBDA(lambda), num_captures));
}

void compiler_peer_create_store_capture_to_closure(Compiler* self, ValueIr* capture, ClosureIr* closure, uint16_t index) {
  self->CreateStoreCapturePtrToClosure(LLVM_VALUE(capture), LLVM_VALUE(closure), index);
}

StatusIr* compiler_peer_create_call_on_closure(Compiler* self, ClosureIr* closure, uint16_t argc, ArgvIr* argv, ValueIr* retv) {
  return PEER_STATUS(self->CreateCallOnClosure(LLVM_VALUE(closure), argc, LLVM_VALUE(argv), LLVM_VALUE(retv)));
}

ClosureIr* compiler_peer_create_closure_phi(Compiler* self, ClosureIr* then_value, BasicBlock* then_block, ClosureIr* else_value, BasicBlock* else_block) {
  return PEER_CLOSURE(self->CreateClosurePhi(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

ValueIr* compiler_peer_create_closure_to_any(Compiler* self, ClosureIr* value) {
  return PEER_VALUE(self->CreateClosureToAny(LLVM_VALUE(value)));
}

// value

LambdaIr* compiler_peer_get_function(Compiler* self, uint32_t func_id, const char* name) {
  return PEER_LAMBDA(self->GetFunction(func_id, name));
}

ValueIr* compiler_peer_get_exception(Compiler* self) {
  return PEER_VALUE(self->GetException());
}

BooleanIr* compiler_peer_create_to_boolean(Compiler* self, ValueIr* value) {
  return PEER_BOOLEAN(self->CreateToBoolean(LLVM_VALUE(value)));
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

BooleanIr* compiler_peer_create_is_same_number(Compiler* self, NumberIr* a, NumberIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameNumber(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIr* compiler_peer_create_is_same_closure(Compiler* self, ClosureIr* a, ClosureIr* b) {
  return PEER_BOOLEAN(self->CreateIsSameClosure(LLVM_VALUE(a), LLVM_VALUE(b)));
}

BooleanIr* compiler_peer_create_is_same_boolean_value(Compiler* self, ValueIr* value, BooleanIr* boolean) {
  return PEER_BOOLEAN(self->CreateIsSameBooleanValue(LLVM_VALUE(value), LLVM_VALUE(boolean)));
}

BooleanIr* compiler_peer_create_is_same_number_value(Compiler* self, ValueIr* value, NumberIr* number) {
  return PEER_BOOLEAN(self->CreateIsSameNumberValue(LLVM_VALUE(value), LLVM_VALUE(number)));
}

BooleanIr* compiler_peer_create_is_same_closure_value(Compiler* self, ValueIr* value, ClosureIr* closure) {
  return PEER_BOOLEAN(self->CreateIsSameClosureValue(LLVM_VALUE(value), LLVM_VALUE(closure)));
}

BooleanIr* compiler_peer_create_is_strictly_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs) {
  return PEER_BOOLEAN(self->CreateIsStrictlyEqual(LLVM_VALUE(lhs), LLVM_VALUE(rhs)));
}

BooleanIr* compiler_peer_create_boolean_phi(Compiler* self, BooleanIr* then_value, BasicBlock* then_block, BooleanIr* else_value, BasicBlock* else_block) {
  return PEER_BOOLEAN(self->CreateBooleanPhi(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
}

ValueIr* compiler_peer_create_value_phi(Compiler* self, ValueIr* then_value, BasicBlock* then_block, ValueIr* else_value, BasicBlock* else_block) {
  return PEER_VALUE(self->CreateValuePhi(LLVM_VALUE(then_value), LLVM_BB(then_block), LLVM_VALUE(else_value), LLVM_BB(else_block)));
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

void compiler_peer_create_store_number_to_variable(Compiler* self, NumberIr* value, ValueIr* variable) {
  self->CreateStoreNumberToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

void compiler_peer_create_store_closure_to_variable(Compiler* self, ClosureIr* value, ValueIr* variable) {
  self->CreateStoreClosureToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

void compiler_peer_create_store_value_to_variable(Compiler* self, ValueIr* value, ValueIr* variable) {
  self->CreateStoreValueToVariable(LLVM_VALUE(value), LLVM_VALUE(variable));
}

void compiler_peer_create_cond_br(Compiler* self, BooleanIr* cond, BasicBlock* then_block, BasicBlock* else_block) {
  self->CreateCondBr(LLVM_VALUE(cond), LLVM_BB(then_block), LLVM_BB(else_block));
}

ClosureIr* compiler_peer_create_load_closure_from_value(Compiler* self, ValueIr* value) {
  return PEER_CLOSURE(self->CreateLoadClosureFromValue(LLVM_VALUE(value)));
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

NumberIr* compiler_peer_to_numeric(Compiler* self, ValueIr* value) {
  return PEER_NUMBER(self->ToNumeric(LLVM_VALUE(value)));
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

ValueIr* compiler_peer_create_get_argument_variable_ptr(Compiler* self, uint16_t index) {
  return PEER_VALUE(self->CreateGetArgumentVariablePtr(index));
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

BooleanIr* compiler_peer_create_is_exception_status(Compiler* self, StatusIr* status) {
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
