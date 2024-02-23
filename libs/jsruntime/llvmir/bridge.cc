#include "bridge.hh"

#include <cstdint>

#include "compiler.hh"
#include "runtime.hh"

void runtime_peer_initialize() {
  Runtime::Initialize();
}

Runtime* runtime_peer_new() {
  return new Runtime();
}

void runtime_peer_delete(Runtime* self) {
  delete self;
}

void runtime_peer_register_host(Runtime* self, const Host* host) {
  self->RegisterHost(host);
}

void runtime_peer_dump_module(Runtime *self) {
  self->DumpModule();
}

void runtime_peer_eval(Runtime* self, uintptr_t context) {
  self->Eval(context);
}

void runtime_peer_call(Runtime* self, const char *name, size_t name_len, double *return_value) {
  self->Call(name, name_len, return_value);
}

Compiler* runtime_peer_start_compilation(Runtime* self) {
  return self->StartCompilation();
}

void runtime_peer_populate_module(Runtime* self, Compiler* compiler) {
  self->PopulateModule(compiler);
}

void runtime_peer_end_compilation(Runtime* self, Compiler* compiler) {
  self->EndCompilation(compiler);
}

void compiler_peer_number(Compiler* self, double value) {
  self->Number(value);
}

void compiler_peer_string(Compiler* self, const char* data, size_t size) {
  self->String(data, size);
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

void compiler_peer_get(Compiler* self) {
  self->Get();
}

void compiler_peer_set(Compiler* self) {
  self->Set();
}

void compiler_peer_set_undefined(Compiler* self) {
  self->SetUndefined();
}

void compiler_peer_call(Compiler* self, size_t argc) {
  self->Call(argc);
}

void compiler_peer_to_boolean(Compiler* self) {
  self->ToBoolean();
}

void compiler_peer_block(Compiler *self) {
  self->Block();
}

void compiler_peer_conditional_expression(Compiler *self) {
  self->ConditionalExpression();
}

void compiler_peer_start_function(Compiler* self, const char* name, size_t len) {
  self->StartFunction(name, len);
}

void compiler_peer_end_function(Compiler* self) {
  self->EndFunction();
}

void compiler_peer_return(Compiler* self, size_t n) {
  self->Return(n);
}

void compiler_peer_print(Compiler* self) {
  self->Print();
}
