#pragma once

#include <cstddef>
#include <cstdint>

#include "host.hh"
#include "macros.hh"

BEGIN_C_LINKAGE

// Runtime

class Runtime;

void runtime_peer_initialize();
Runtime* runtime_peer_new();
void runtime_peer_delete(Runtime* self);
void runtime_peer_register_host(Runtime* self, const Host* host);
void runtime_peer_dump_module(Runtime* self);
void runtime_peer_eval(Runtime* self, uintptr_t context);
void runtime_peer_call(Runtime* self,
    uintptr_t context,
    const char* name,
    size_t name_len,
    double* return_value);

// Compilation

class Compiler;

Compiler* runtime_peer_start_compilation(Runtime* self);
void runtime_peer_populate_module(Runtime* self, Compiler* compiler);
void runtime_peer_end_compilation(Runtime* self, Compiler* compiler);
void compiler_peer_number(Compiler* self, double value);
void compiler_peer_string(Compiler* self, const char* buf, size_t len);
void compiler_peer_symbol(Compiler* self, uint32_t symbol_id);
void compiler_peer_add(Compiler* self);
void compiler_peer_sub(Compiler* self);
void compiler_peer_mul(Compiler* self);
void compiler_peer_div(Compiler* self);
void compiler_peer_rem(Compiler* self);
void compiler_peer_lt(Compiler* self);
void compiler_peer_gt(Compiler* self);
void compiler_peer_lte(Compiler* self);
void compiler_peer_gte(Compiler* self);
void compiler_peer_eq(Compiler* self);
void compiler_peer_ne(Compiler* self);
void compiler_peer_get(Compiler* self);
void compiler_peer_set(Compiler* self);
void compiler_peer_declare(Compiler* self);
void compiler_peer_set_undefined(Compiler* self);
void compiler_peer_call(Compiler* self, size_t argc);
void compiler_peer_to_boolean(Compiler* self);
void compiler_peer_block(Compiler* self);
void compiler_peer_conditional_expression(Compiler* self);
void compiler_peer_if_else_statement(Compiler* self);
void compiler_peer_if_statement(Compiler* self);
void compiler_peer_start_function(Compiler* self, const char* name, size_t len);
void compiler_peer_end_function(Compiler* self);
void compiler_peer_start_scope(Compiler* self);
void compiler_peer_end_scope(Compiler* self);
void compiler_peer_return(Compiler* self, size_t n);
void compiler_peer_print(Compiler* self);

END_C_LINKAGE
