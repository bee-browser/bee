#include "bridge.hh"

#include <cstdint>

#include <llvm/Support/TargetSelect.h>

#include "compiler.hh"
#include "executor.hh"
#include "helper.hh"
#include "module.hh"

#define PEER_BB(bb) (reinterpret_cast<BasicBlock>(bb))
#define LLVM_BB(bb) (reinterpret_cast<llvm::BasicBlock*>(bb))

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

BasicBlock compiler_peer_create_basic_block(Compiler* self, const char* name, size_t name_len) {
  return PEER_BB(self->CreateBasicBlock(name, name_len));
}

BasicBlock compiler_peer_get_basic_block(const Compiler* self) {
  return PEER_BB(self->GetBasicBlock());
}

BasicBlock compiler_peer_get_locals_block(const Compiler* self) {
  return PEER_BB(self->GetLocalsBlock());
}

BasicBlock compiler_peer_get_args_block(const Compiler* self) {
  return PEER_BB(self->GetArgsBlock());
}

BasicBlock compiler_peer_get_body_block(const Compiler* self) {
  return PEER_BB(self->GetBodyBlock());
}

BasicBlock compiler_peer_get_return_block(const Compiler* self) {
  return PEER_BB(self->GetReturnBlock());
}

void compiler_peer_set_basic_block(Compiler* self, BasicBlock block) {
  self->SetBasicBlock(LLVM_BB(block));
}

void compiler_peer_move_basic_block_after(Compiler* self, BasicBlock block) {
  self->MoveBasicBlockAfter(LLVM_BB(block));
}

bool compiler_peer_is_basic_block_terminated(Compiler* self, BasicBlock block) {
  return self->IsBasicBlockTerminated(LLVM_BB(block));
}

void compiler_peer_create_br(Compiler* self, BasicBlock block) {
  self->CreateBr(LLVM_BB(block));
}

void compiler_peer_create_store_normal_status(Compiler* self) {
  self->CreateStoreNormalStatus();
}

void compiler_peer_undefined(Compiler* self) {
  self->Undefined();
}

void compiler_peer_null(Compiler* self) {
  self->Null();
}

void compiler_peer_boolean(Compiler* self, bool value) {
  self->Boolean(value);
}

void compiler_peer_number(Compiler* self, double value) {
  self->Number(value);
}

void compiler_peer_function(Compiler* self, uint32_t func_id, const char* name) {
  self->Function(func_id, name);
}

void compiler_peer_closure(Compiler* self, BasicBlock block, uint16_t num_captures) {
  self->Closure(LLVM_BB(block), num_captures);
}

void compiler_peer_reference(Compiler* self, uint32_t symbol, Locator locator) {
  self->Reference(symbol, locator);
}

void compiler_peer_exception(Compiler* self) {
  self->Exception();
}

void compiler_peer_postfix_increment(Compiler* self) {
  self->PostfixIncrement();
}

void compiler_peer_postfix_decrement(Compiler* self) {
  self->PostfixDecrement();
}

void compiler_peer_prefix_increment(Compiler* self) {
  self->PrefixIncrement();
}

void compiler_peer_prefix_decrement(Compiler* self) {
  self->PrefixDecrement();
}

void compiler_peer_unary_delete(Compiler* self) {
  self->UnaryDelete();
}

void compiler_peer_void(Compiler* self) {
  self->Void();
}

void compiler_peer_typeof(Compiler* self) {
  self->Typeof();
}

void compiler_peer_unary_plus(Compiler* self) {
  self->UnaryPlus();
}

void compiler_peer_unary_minus(Compiler* self) {
  self->UnaryMinus();
}

void compiler_peer_bitwise_not(Compiler* self) {
  self->BitwiseNot();
}

void compiler_peer_logical_not(Compiler* self) {
  self->LogicalNot();
}

void compiler_peer_exponentiation(Compiler* self) {
  self->Exponentiation();
}

void compiler_peer_multiplication(Compiler* self) {
  self->Multiplication();
}

void compiler_peer_division(Compiler* self) {
  self->Division();
}

void compiler_peer_remainder(Compiler* self) {
  self->Remainder();
}

void compiler_peer_addition(Compiler* self) {
  self->Addition();
}

void compiler_peer_subtraction(Compiler* self) {
  self->Subtraction();
}

void compiler_peer_left_shift(Compiler* self) {
  self->LeftShift();
}

void compiler_peer_signed_right_shift(Compiler* self) {
  self->SignedRightShift();
}

void compiler_peer_unsigned_right_shift(Compiler* self) {
  self->UnsignedRightShift();
}

void compiler_peer_less_than(Compiler* self) {
  self->LessThan();
}

void compiler_peer_greater_than(Compiler* self) {
  self->GreaterThan();
}

void compiler_peer_less_than_or_equal(Compiler* self) {
  self->LessThanOrEqual();
}

void compiler_peer_greater_than_or_equal(Compiler* self) {
  self->GreaterThanOrEqual();
}

void compiler_peer_instanceof(Compiler* self) {
  self->Instanceof();
}

void compiler_peer_in(Compiler* self) {
  self->In();
}

void compiler_peer_equality(Compiler* self) {
  self->Equality();
}

void compiler_peer_inequality(Compiler* self) {
  self->Inequality();
}

void compiler_peer_strict_equality(Compiler* self) {
  self->StrictEquality();
}

void compiler_peer_strict_inequality(Compiler* self) {
  self->StrictInequality();
}

void compiler_peer_bitwise_and(Compiler* self) {
  self->BitwiseAnd();
}

void compiler_peer_bitwise_xor(Compiler* self) {
  self->BitwiseXor();
}

void compiler_peer_bitwise_or(Compiler* self) {
  self->BitwiseOr();
}

void compiler_peer_ternary(Compiler* self,
    BasicBlock test_block,
    BasicBlock then_head_block,
    BasicBlock then_tail_block,
    BasicBlock else_head_block) {
  self->Ternary(LLVM_BB(test_block), LLVM_BB(then_head_block), LLVM_BB(then_tail_block),
      LLVM_BB(else_head_block));
}

void compiler_peer_assignment(Compiler* self) {
  self->Assignment();
}

void compiler_peer_exponentiation_assignment(Compiler* self) {
  self->ExponentiationAssignment();
}

void compiler_peer_multiplication_assignment(Compiler* self) {
  self->MultiplicationAssignment();
}

void compiler_peer_division_assignment(Compiler* self) {
  self->DivisionAssignment();
}

void compiler_peer_remainder_assignment(Compiler* self) {
  self->RemainderAssignment();
}

void compiler_peer_addition_assignment(Compiler* self) {
  self->AdditionAssignment();
}

void compiler_peer_subtraction_assignment(Compiler* self) {
  self->SubtractionAssignment();
}

void compiler_peer_left_shift_assignment(Compiler* self) {
  self->LeftShiftAssignment();
}

void compiler_peer_signed_right_shift_assignment(Compiler* self) {
  self->SignedRightShiftAssignment();
}

void compiler_peer_unsigned_right_shift_assignment(Compiler* self) {
  self->UnsignedRightShiftAssignment();
}

void compiler_peer_bitwise_and_assignment(Compiler* self) {
  self->BitwiseAndAssignment();
}

void compiler_peer_bitwise_xor_assignment(Compiler* self) {
  self->BitwiseXorAssignment();
}

void compiler_peer_bitwise_or_assignment(Compiler* self) {
  self->BitwiseOrAssignment();
}

void compiler_peer_declare_immutable(Compiler* self) {
  self->DeclareImmutable();
}

void compiler_peer_declare_mutable(Compiler* self) {
  self->DeclareMutable();
}

void compiler_peer_declare_function(Compiler* self, BasicBlock block) {
  self->DeclareFunction(LLVM_BB(block));
}

void compiler_peer_declare_closure(Compiler* self, BasicBlock block) {
  self->DeclareClosure(LLVM_BB(block));
}

void compiler_peer_arguments(Compiler* self, uint16_t argc) {
  self->Arguments(argc);
}

void compiler_peer_argument(Compiler* self, uint16_t index) {
  self->Argument(index);
}

void compiler_peer_call(Compiler* self, uint16_t argc, BasicBlock block) {
  self->Call(argc, LLVM_BB(block));
}

void compiler_peer_truthy(Compiler* self) {
  self->Truthy();
}

void compiler_peer_falsy_short_circuit(Compiler* self) {
  self->FalsyShortCircuit();
}

void compiler_peer_truthy_short_circuit(Compiler* self) {
  self->TruthyShortCircuit();
}

void compiler_peer_nullish_short_circuit(Compiler* self) {
  self->NullishShortCircuit();
}

void compiler_peer_falsy_short_circuit_assignment(Compiler* self) {
  self->FalsyShortCircuitAssignment();
}

void compiler_peer_truthy_short_circuit_assignment(Compiler* self) {
  self->TruthyShortCircuitAssignment();
}

void compiler_peer_nullish_short_circuit_assignment(Compiler* self) {
  self->NullishShortCircuitAssignment();
}

void compiler_peer_if_else_statement(Compiler* self,
    BasicBlock test_block,
    BasicBlock then_head_block,
    BasicBlock then_tail_block,
    BasicBlock else_head_block) {
  self->IfElseStatement(LLVM_BB(test_block), LLVM_BB(then_head_block), LLVM_BB(then_tail_block),
      LLVM_BB(else_head_block));
}

void compiler_peer_if_statement(Compiler* self, BasicBlock test_block, BasicBlock then_block) {
  self->IfStatement(LLVM_BB(test_block), LLVM_BB(then_block));
}

void compiler_peer_loop_test(Compiler* self,
    BasicBlock then_block,
    BasicBlock else_block,
    BasicBlock insert_point) {
  self->LoopTest(LLVM_BB(then_block), LLVM_BB(else_block), LLVM_BB(insert_point));
}

void compiler_peer_case_block(Compiler* self, uint16_t id, uint16_t num_cases) {
  self->CaseBlock(id, num_cases);
}

void compiler_peer_case_clause(Compiler* self,
    bool has_statement,
    BasicBlock before_block,
    BasicBlock after_block) {
  self->CaseClause(has_statement, LLVM_BB(before_block), LLVM_BB(after_block));
}

void compiler_peer_default_clause(Compiler* self, bool has_statement, BasicBlock before_block) {
  self->DefaultClause(has_statement, LLVM_BB(before_block));
}

void compiler_peer_try_end(Compiler* self, BasicBlock exception_block, BasicBlock end_block) {
  self->TryEnd(LLVM_BB(exception_block), LLVM_BB(end_block));
}

void compiler_peer_start_function(Compiler* self, const char* name) {
  self->StartFunction(name);
}

void compiler_peer_end_function(Compiler* self, bool optimize) {
  self->EndFunction(optimize);
}

void compiler_peer_start_scope_cleanup_checker(Compiler* self, uint16_t scope_id) {
  self->StartScopeCleanupChecker(scope_id);
}

void compiler_peer_end_scope_cleanup_checker(Compiler* self, uint16_t scope_id) {
  self->EndScopeCleanupChecker(scope_id);
}

void compiler_peer_handle_returned_thrown(Compiler* self,
    bool returned,
    bool thrown,
    BasicBlock block,
    BasicBlock cleanup_block,
    BasicBlock exception_block) {
  self->HandleReturnedThrown(
      returned, thrown, LLVM_BB(block), LLVM_BB(cleanup_block), LLVM_BB(exception_block));
}
void compiler_peer_allocate_locals(Compiler* self, uint16_t num_locals) {
  self->AllocateLocals(num_locals);
}

void compiler_peer_init_local(Compiler* self, Locator locator, BasicBlock block) {
  self->InitLocal(locator, LLVM_BB(block));
}

void compiler_peer_tidy_local(Compiler* self, Locator locator) {
  self->TidyLocal(locator);
}

void compiler_peer_create_capture(Compiler* self, Locator locator, BasicBlock block) {
  self->CreateCapture(locator, LLVM_BB(block));
}

void compiler_peer_capture_variable(Compiler* self, BasicBlock block) {
  self->CaptureVariable(LLVM_BB(block));
}

void compiler_peer_escape_variable(Compiler* self, Locator locator, BasicBlock block) {
  self->EscapeVariable(locator, LLVM_BB(block));
}

void compiler_peer_return(Compiler* self, size_t n) {
  self->Return(n);
}

void compiler_peer_throw(Compiler* self) {
  self->Throw();
}

void compiler_peer_discard(Compiler* self) {
  self->Discard();
}

void compiler_peer_swap(Compiler* self) {
  self->Swap();
}

void compiler_peer_prepare_scope_cleanup_checker(Compiler* self, uint16_t stack_size) {
  self->PrepareScopeCleanupChecker(stack_size);
}

void compiler_peer_dump_stack(Compiler* self) {
  self->DumpStack();
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

size_t helper_peer_get_basic_block_name_or_as_operand(BasicBlock block, char* buf, size_t len) {
  return GetNameOrAsOperand(LLVM_BB(block), buf, len);
}
