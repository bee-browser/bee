#include "../bridge.hh"

#include <cstdint>

#include <llvm/Support/TargetSelect.h>

#include "compiler.hh"
#include "executor.hh"
#include "macros.hh"
#include "module.hh"

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

void compiler_peer_start(Compiler* self) {
  UNUSED(self);
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

void compiler_peer_set_runtime(Compiler* self, uintptr_t runtime) {
  self->SetRuntime(runtime);
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

void compiler_peer_reference(Compiler* self, uint32_t symbol, Locator locator) {
  self->Reference(symbol, locator);
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

void compiler_peer_conditional_ternary(Compiler* self) {
  self->ConditionalTernary();
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

void compiler_peer_bindings(Compiler* self, uint16_t n) {
  self->Bindings(n);
}

void compiler_peer_declare_immutable(Compiler* self) {
  self->DeclareImmutable();
}

void compiler_peer_declare_mutable(Compiler* self) {
  self->DeclareMutable();
}

void compiler_peer_declare_function(Compiler* self) {
  self->DeclareFunction();
}

void compiler_peer_arguments(Compiler* self, uint16_t argc) {
  self->Arguments(argc);
}

void compiler_peer_argument(Compiler* self, uint16_t index) {
  self->Argument(index);
}

void compiler_peer_call(Compiler* self, uint16_t argc) {
  self->Call(argc);
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

void compiler_peer_block(Compiler* self) {
  self->Block();
}

void compiler_peer_if_else_statement(Compiler* self) {
  self->IfElseStatement();
}

void compiler_peer_if_statement(Compiler* self) {
  self->IfStatement();
}

void compiler_peer_do_while_loop(Compiler* self) {
  self->DoWhileLoop();
}

void compiler_peer_while_loop(Compiler* self) {
  self->WhileLoop();
}

void compiler_peer_for_loop(Compiler* self, bool has_init, bool has_test, bool has_next) {
  self->ForLoop(has_init, has_test, has_next);
}

void compiler_peer_loop_init(Compiler* self) {
  self->LoopInit();
}

void compiler_peer_loop_test(Compiler* self) {
  self->LoopTest();
}

void compiler_peer_loop_next(Compiler* self) {
  self->LoopNext();
}

void compiler_peer_loop_body(Compiler* self) {
  self->LoopBody();
}

void compiler_peer_loop_end(Compiler* self) {
  self->LoopEnd();
}

void compiler_peer_case_block(Compiler* self) {
  self->CaseBlock();
}

void compiler_peer_switch(Compiler* self, uint32_t n) {
  self->Switch(n);
}

void compiler_peer_start_function(Compiler* self, const char* name) {
  self->StartFunction(name);
}

void compiler_peer_end_function(Compiler* self, bool optimize) {
  self->EndFunction(optimize);
}

void compiler_peer_allocate_bindings(Compiler* self, uint16_t n, bool prologue) {
  assert(n > 0);
  self->AllocateBindings(n, prologue);
}

void compiler_peer_release_bindings(Compiler* self, uint16_t n) {
  assert(n > 0);
  self->ReleaseBindings(n);
}

void compiler_peer_continue(Compiler* self) {
  self->Continue();
}

void compiler_peer_break(Compiler* self) {
  self->Break();
}

void compiler_peer_return(Compiler* self, size_t n) {
  self->Return(n);
}

void compiler_peer_discard(Compiler* self) {
  self->Discard();
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

void executor_peer_register_host_function(Executor* self, const char* name, FuncPtr func) {
  self->RegisterHostFunction(name, func);
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

FuncPtr executor_peer_get_native_func(Executor* self, const char* name) {
  return self->GetNativeFunc(name);
}
