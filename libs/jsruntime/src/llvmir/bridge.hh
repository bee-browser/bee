#pragma once

#include <cstddef>
#include <cstdint>

#include "macros.hh"
#include "runtime.hh"

BEGIN_C_LINKAGE

void llvmir_initialize();

// Module

struct Module;
void module_peer_print(Module* self, bool stderr);
void module_peer_delete(Module* self);

// Compilation

class Compiler;
Compiler* compiler_peer_new();
void compiler_peer_delete(Compiler* self);
void compiler_peer_start(Compiler* self);
Module* compiler_peer_end(Compiler* self);
void compiler_peer_number(Compiler* self, double value);
void compiler_peer_function(Compiler* self, uint32_t func_id);
void compiler_peer_reference(Compiler* self, uint32_t symbol, uint32_t locator);
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
void compiler_peer_declare_immutable(Compiler* self);
void compiler_peer_declare_mutable(Compiler* self);
void compiler_peer_declare_function(Compiler* self);
void compiler_peer_set(Compiler* self);
void compiler_peer_push_argument(Compiler* self);
void compiler_peer_call(Compiler* self);
void compiler_peer_to_boolean(Compiler* self);
void compiler_peer_block(Compiler* self);
void compiler_peer_conditional_expression(Compiler* self);
void compiler_peer_if_else_statement(Compiler* self);
void compiler_peer_if_statement(Compiler* self);
void compiler_peer_start_function(Compiler* self, const char* name);
void compiler_peer_end_function(Compiler* self);
void compiler_peer_allocate_bindings(Compiler* self, uint16_t n, bool prologue);
void compiler_peer_release_bindings(Compiler* self, uint16_t n);
void compiler_peer_return(Compiler* self, size_t n);
void compiler_peer_void(Compiler* self);
void compiler_peer_dump_stack(Compiler* self);

// Execution

class Executor;
typedef double (*FuncFn)(void*);
Executor* executor_peer_new();
void executor_peer_delete(Executor* self);
void executor_peer_register_runtime(Executor* self, const Runtime* runtime);
void executor_peer_register_module(Executor* self, Module* mod);
FuncFn executor_peer_get_func(Executor* self, const char* name);

END_C_LINKAGE
