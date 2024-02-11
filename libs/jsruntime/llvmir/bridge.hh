#pragma once

#include <cstddef>

#include "host.hh"
#include "macros.hh"

BEGIN_C_LINKAGE

// Runtime

class Runtime;

void runtime_initialize();
Runtime* runtime_new();
void runtime_delete(Runtime* self);
void runtime_register_host(Runtime* self, const Host* host);
void runtime_dump_module(Runtime* self);
void runtime_eval(Runtime* self);

// Compilation

class Compiler;
Compiler* runtime_start_compilation(Runtime* self);
void runtime_populate_module(Runtime* self, Compiler* compiler);
void runtime_end_compilation(Runtime* self, Compiler* compiler);
void compiler_number(Compiler* self, double value);
void compiler_string(Compiler* self, const char* buf, size_t len);
void compiler_add(Compiler* self);
void compiler_sub(Compiler* self);
void compiler_mul(Compiler* self);
void compiler_div(Compiler* self);
void compiler_rem(Compiler* self);
void compiler_lt(Compiler* self);
void compiler_gt(Compiler* self);
void compiler_lte(Compiler* self);
void compiler_gte(Compiler* self);
void compiler_eq(Compiler* self);
void compiler_ne(Compiler* self);
void compiler_print(Compiler* self);

END_C_LINKAGE
