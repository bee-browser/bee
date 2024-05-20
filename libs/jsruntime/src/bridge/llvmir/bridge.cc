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

void compiler_peer_add(Compiler* self) {
  self->Add();
}

void compiler_peer_sub(Compiler* self) {
  self->Sub();
}

void compiler_peer_mul(Compiler* self) {
  self->Mul();
}

void compiler_peer_div(Compiler* self) {
  self->Div();
}

void compiler_peer_rem(Compiler* self) {
  self->Rem();
}

void compiler_peer_lt(Compiler* self) {
  self->Lt();
}

void compiler_peer_gt(Compiler* self) {
  self->Gt();
}

void compiler_peer_lte(Compiler* self) {
  self->Lte();
}

void compiler_peer_gte(Compiler* self) {
  self->Gte();
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

void compiler_peer_unary_plus(Compiler* self) {
  self->UnaryPlus();
}

void compiler_peer_unary_minus(Compiler* self) {
  self->UnaryMinus();
}

void compiler_peer_bitwise_not(Compiler *self) {
  self->BitwiseNot();
}

void compiler_peer_eq(Compiler* self) {
  self->Eq();
}

void compiler_peer_ne(Compiler* self) {
  self->Ne();
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

void compiler_peer_set(Compiler* self) {
  self->Set();
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

void compiler_peer_to_boolean(Compiler* self) {
  self->ToBoolean();
}

void compiler_peer_block(Compiler* self) {
  self->Block();
}

void compiler_peer_conditional_expression(Compiler* self) {
  self->ConditionalExpression();
}

void compiler_peer_if_else_statement(Compiler* self) {
  self->IfElseStatement();
}

void compiler_peer_if_statement(Compiler* self) {
  self->IfStatement();
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

void compiler_peer_return(Compiler* self, size_t n) {
  self->Return(n);
}

void compiler_peer_void(Compiler* self) {
  self->Void();
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
