#include "bridge.hh"

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

Compiler* compiler_peer_new(const char* data_layout) {
  return new Compiler(data_layout);
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

void compiler_peer_number(Compiler* self, double value) {
  self->Number(value);
}

void compiler_peer_function(Compiler* self, uint32_t func_id) {
  self->Function(func_id);
}

void compiler_peer_reference(Compiler* self, uint32_t symbol, uint32_t locator) {
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

void compiler_peer_eq(Compiler* self) {
  self->Eq();
}

void compiler_peer_ne(Compiler* self) {
  self->Ne();
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

void compiler_peer_push_argument(Compiler* self) {
  self->PushArgument();
}

void compiler_peer_call(Compiler* self) {
  self->Call();
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

void compiler_peer_end_function(Compiler* self) {
  self->EndFunction();
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

void executor_peer_register_module(Executor* self, Module* mod) {
  self->RegisterModule(mod);
}

const char* executor_peer_get_data_layout(const Executor* self) {
  return self->data_layout().getStringRepresentation().c_str();
}

FuncPtr executor_peer_get_func(Executor* self, const char* name) {
  return self->GetFunc(name);
}
