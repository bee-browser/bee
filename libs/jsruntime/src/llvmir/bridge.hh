#pragma once

#include <cstddef>
#include <cstdint>

#include "host.hh"
#include "macros.hh"

BEGIN_C_LINKAGE

void llvmir_initialize();

// Module

struct Module;
void module_peer_dump(Module* self);
void module_peer_delete(Module* self);

// Compilation

class Compiler;
Compiler* compiler_peer_new();
void compiler_peer_delete(Compiler* self);
void compiler_peer_start(Compiler* self);
Module* compiler_peer_end(Compiler* self);
void compiler_peer_number(Compiler* self, double value);
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
void compiler_peer_declare_const(Compiler* self);
void compiler_peer_declare_variable(Compiler* self);
void compiler_peer_declare_undefined(Compiler* self);
void compiler_peer_declare_function(Compiler* self, uint32_t symbol_id, uint32_t func_id);
void compiler_peer_get(Compiler* self);
void compiler_peer_set(Compiler* self);
void compiler_peer_set_undefined(Compiler* self);
void compiler_peer_push_args(Compiler* self);
void compiler_peer_push_arg(Compiler* self);
void compiler_peer_call(Compiler* self);
void compiler_peer_to_boolean(Compiler* self);
void compiler_peer_block(Compiler* self);
void compiler_peer_conditional_expression(Compiler* self);
void compiler_peer_if_else_statement(Compiler* self);
void compiler_peer_if_statement(Compiler* self);
void compiler_peer_start_function(Compiler* self, const char* name);
void compiler_peer_end_function(Compiler* self);
void compiler_peer_start_scope(Compiler* self);
void compiler_peer_end_scope(Compiler* self);
void compiler_peer_return(Compiler* self, size_t n);
void compiler_peer_print(Compiler* self);

// Execution

class Executor;
typedef void (*MainFn)(void*);
typedef double (*FuncFn)(void*);
Executor* executor_peer_new();
void executor_peer_delete(Executor* self);
void executor_peer_register_host(Executor* self, const Host* host);
void executor_peer_register_module(Executor* self, Module* mod);
MainFn executor_peer_get_main(Executor* self);
FuncFn executor_peer_get_func(Executor* self, const char* name);

END_C_LINKAGE
