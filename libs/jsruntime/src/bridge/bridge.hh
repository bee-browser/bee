#pragma once

#include <cstddef>
#include <cstdint>

struct Value;
typedef Value (*FuncPtr)(void* exec_context, void* outer_scope, size_t argc, Value* argv);

enum LocatorKind : uint8_t {
  None,
  Argument,
  Local,
};

static_assert(sizeof(LocatorKind) == sizeof(uint8_t), "size mismatched");

// TODO: Changing the order of member variables causes performance regression in fib(41).
// However, we don't know the exact reason at this point.  Deeper investigation is needed.
struct Locator {
  uint8_t offset;
  LocatorKind kind;
  uint16_t index;
};

static_assert(sizeof(Locator) == sizeof(uint32_t), "size mismatched");

enum ValueKind : uint8_t {
  Undefined,
  Null,
  Boolean,
  Number,
  Function,
};

static_assert(sizeof(ValueKind) == sizeof(uint8_t), "size mismatched");

union ValueHolder {
  uintptr_t opaque;
  bool boolean;
  double number;
  FuncPtr function;
};

static_assert(sizeof(ValueHolder) == sizeof(uint64_t), "size mismatched");

struct Value {
  ValueKind kind;
  // uint8_t padding[7];
  ValueHolder holder;
};

static_assert(sizeof(Value) == sizeof(uint64_t) * 2, "size mismatched");

// Can be copied as Value.
struct Binding {
  ValueKind kind;
  uint8_t flags;
  uint16_t reserved;
  uint32_t symbol;
  ValueHolder holder;
};

#define BINDING_INITIALIZED 0x01
#define BINDING_DELETABLE 0x02
#define BINDING_MUTABLE 0x04
#define BINDING_STRICT 0x08

static_assert(sizeof(Binding) == sizeof(uint64_t) * 2, "size mismatched");

#include "runtime.hh"

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
void compiler_peer_set_data_layout(Compiler* self, const char* data_layout);
void compiler_peer_set_target_triple(Compiler* self, const char* triple);
void compiler_peer_set_runtime(Compiler* self, uintptr_t runtime);
void compiler_peer_undefined(Compiler* self);
void compiler_peer_null(Compiler* self);
void compiler_peer_boolean(Compiler* self, bool value);
void compiler_peer_number(Compiler* self, double value);
void compiler_peer_function(Compiler* self, uint32_t func_id, const char* name);
void compiler_peer_reference(Compiler* self, uint32_t symbol, Locator locator);
void compiler_peer_add(Compiler* self);
void compiler_peer_sub(Compiler* self);
void compiler_peer_mul(Compiler* self);
void compiler_peer_div(Compiler* self);
void compiler_peer_rem(Compiler* self);
void compiler_peer_lt(Compiler* self);
void compiler_peer_gt(Compiler* self);
void compiler_peer_lte(Compiler* self);
void compiler_peer_gte(Compiler* self);
void compiler_peer_left_shift(Compiler* self);
void compiler_peer_signed_right_shift(Compiler* self);
void compiler_peer_unsigned_right_shift(Compiler* self);
void compiler_peer_unary_plus(Compiler* self);
void compiler_peer_unary_minus(Compiler* self);
void compiler_peer_eq(Compiler* self);
void compiler_peer_ne(Compiler* self);
void compiler_peer_bindings(Compiler* self, uint16_t n);
void compiler_peer_declare_immutable(Compiler* self);
void compiler_peer_declare_mutable(Compiler* self);
void compiler_peer_declare_function(Compiler* self);
void compiler_peer_set(Compiler* self);
void compiler_peer_arguments(Compiler* self, uint16_t argc);
void compiler_peer_argument(Compiler* self, uint16_t index);
void compiler_peer_call(Compiler* self, uint16_t argc);
void compiler_peer_to_boolean(Compiler* self);
void compiler_peer_block(Compiler* self);
void compiler_peer_conditional_expression(Compiler* self);
void compiler_peer_if_else_statement(Compiler* self);
void compiler_peer_if_statement(Compiler* self);
void compiler_peer_start_function(Compiler* self, const char* name);
void compiler_peer_end_function(Compiler* self, bool optimize);
void compiler_peer_allocate_bindings(Compiler* self, uint16_t n, bool prologue);
void compiler_peer_release_bindings(Compiler* self, uint16_t n);
void compiler_peer_return(Compiler* self, size_t n);
void compiler_peer_void(Compiler* self);
void compiler_peer_dump_stack(Compiler* self);

// Execution

class Executor;
Executor* executor_peer_new();
void executor_peer_delete(Executor* self);
void executor_peer_register_runtime(Executor* self, const Runtime* runtime);
void executor_peer_register_host_function(Executor* self, const char* name, FuncPtr func);
void executor_peer_register_module(Executor* self, Module* mod);
const char* executor_peer_get_data_layout(const Executor* self);
const char* executor_peer_get_target_triple(const Executor* self);
FuncPtr executor_peer_get_native_func(Executor* self, const char* name);
