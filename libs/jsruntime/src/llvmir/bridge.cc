#include "bridge.hh"

#include <cstdint>

#include "llvm/Support/TargetSelect.h"

#include "compiler.hh"
#include "executor.hh"
#include "module.hh"

void llvmir_initialize() {
  // Uncomment if you want to enable LLVM_DEBUG().
  // llvm::DebugFlag = true;

  llvm::InitializeNativeTarget();
  llvm::InitializeNativeTargetAsmPrinter();
  llvm::InitializeNativeTargetAsmParser();
}

void module_peer_dump(Module* self) {
  self->Dump();
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
  self->StartMain();
}

Module* compiler_peer_end(Compiler* self) {
  self->EndMain();
  return self->TakeModule();
}

void compiler_peer_number(Compiler* self, double value) {
  self->Number(value);
}

void compiler_peer_symbol(Compiler* self, uint32_t symbol_id) {
  self->Symbol(symbol_id);
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

void compiler_peer_declare_const(Compiler* self) {
  self->DeclareConst();
}

void compiler_peer_declare_variable(Compiler* self) {
  self->DeclareVariable();
}

void compiler_peer_declare_function(Compiler* self, uint32_t symbol_id, uint32_t func_id) {
  self->DeclareFunction(symbol_id, func_id);
}

void compiler_peer_get(Compiler* self) {
  self->Get();
}

void compiler_peer_set(Compiler* self) {
  self->Set();
}

void compiler_peer_push_args(Compiler* self) {
  self->PushArgs();
}

void compiler_peer_push_arg(Compiler* self) {
  self->PushArg();
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

void compiler_peer_start_scope(Compiler* self) {
  self->StartScope();
}

void compiler_peer_end_scope(Compiler* self) {
  self->EndScope();
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

void executor_peer_register_host(Executor* self, const Host* host) {
  self->RegisterHost(host);
}

void executor_peer_register_module(Executor* self, Module* mod) {
  self->RegisterModule(mod);
}

MainFn executor_peer_get_main(Executor* self) {
  return self->GetMain();
}

FuncFn executor_peer_get_func(Executor* self, const char* name) {
  return self->GetFunc(name);
}
