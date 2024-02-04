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

void compiler_push_number(Compiler* self, double value) {
  self->PushNumber(value);
}

void compiler_push_string(Compiler* self, const char* data, size_t size) {
  self->PushString(data, size);
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

void compiler_print(Compiler* self) {
  self->Print();
}
