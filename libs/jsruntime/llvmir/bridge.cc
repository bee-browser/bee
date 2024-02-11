#include "bridge.hh"

#include "compiler.hh"
#include "runtime.hh"

void runtime_initialize() {
  Runtime::Initialize();
}

Runtime* runtime_new() {
  return new Runtime();
}

void runtime_delete(Runtime* self) {
  delete self;
}

void runtime_register_host(Runtime* self, const Host* host) {
  self->RegisterHost(host);
}

void runtime_dump_module(Runtime *self) {
  self->DumpModule();
}

void runtime_eval(Runtime* self) {
  self->Eval();
}

Compiler* runtime_start_compilation(Runtime* self) {
  return self->StartCompilation();
}

void runtime_populate_module(Runtime* self, Compiler* compiler) {
  self->PopulateModule(compiler);
}

void runtime_end_compilation(Runtime* self, Compiler* compiler) {
  self->EndCompilation(compiler);
}

void compiler_number(Compiler* self, double value) {
  self->Number(value);
}

void compiler_string(Compiler* self, const char* data, size_t size) {
  self->String(data, size);
}

void compiler_add(Compiler* self) {
  self->Add();
}

void compiler_sub(Compiler* self) {
  self->Sub();
}

void compiler_mul(Compiler* self) {
  self->Mul();
}

void compiler_div(Compiler* self) {
  self->Div();
}

void compiler_rem(Compiler* self) {
  self->Rem();
}

void compiler_lt(Compiler* self) {
  self->Lt();
}

void compiler_gt(Compiler* self) {
  self->Gt();
}

void compiler_lte(Compiler* self) {
  self->Lte();
}

void compiler_gte(Compiler* self) {
  self->Gte();
}

void compiler_eq(Compiler* self) {
  self->Eq();
}

void compiler_ne(Compiler* self) {
  self->Ne();
}

void compiler_print(Compiler* self) {
  self->Print();
}
