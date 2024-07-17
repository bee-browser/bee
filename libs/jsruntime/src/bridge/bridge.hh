#pragma once

#include <cstddef>
#include <cstdint>

enum class Status;
struct Value;
struct Closure;
typedef Status (*FuncPtr)(void* ctx, void* caps, size_t argc, Value* argv, Value* ret);

enum class LocatorKind : uint16_t {
  None,
  Argument,
  Local,
  Capture,
};

static_assert(sizeof(LocatorKind) == sizeof(uint16_t), "size mismatched");

// TODO: Changing the order of member variables causes performance regression in fib(41).
// However, we don't know the exact reason at this point.  Deeper investigation is needed.
struct Locator {
  LocatorKind kind = LocatorKind::None;
  uint16_t index = 0;
};

static_assert(sizeof(Locator) == sizeof(uint32_t), "size mismatched");

enum class Status : int32_t {
  Normal = 0,
  Exception,
};

static_assert(sizeof(Status) == sizeof(int32_t), "size mismatched");

enum class ValueKind : uint8_t {
  // DO NOT CHANGE THE ORDER OF THE FOLLOWING ENUM VARIANTS.
  // Some operations heavily rely on the order.
  Undefined = 0,
  Null,
  Boolean,
  Number,
  Function,
  Closure,
};

static_assert(sizeof(ValueKind) == sizeof(uint8_t), "size mismatched");

union ValueHolder {
  uintptr_t opaque;
  bool boolean;
  double number;
  FuncPtr function;
  // TODO: GcCellRef
  Closure* closure;
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

// TODO: GcCell
struct Capture {
  Binding* target;
  Binding escaped;
};

// TODO: GcCell
struct Closure {
  FuncPtr lambda;
  uint16_t num_captures;
  Capture** captures;
  // `Capture* storage[num_captures]` is placed here if it's not empty.
};

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
void compiler_peer_closure(Compiler* self, bool prologue, uint16_t num_captures);
void compiler_peer_reference(Compiler* self, uint32_t symbol, Locator locator);
void compiler_peer_exception(Compiler* self);
void compiler_peer_postfix_increment(Compiler* self);
void compiler_peer_postfix_decrement(Compiler* self);
void compiler_peer_prefix_increment(Compiler* self);
void compiler_peer_prefix_decrement(Compiler* self);
void compiler_peer_unary_delete(Compiler* self);
void compiler_peer_void(Compiler* self);
void compiler_peer_typeof(Compiler* self);
void compiler_peer_unary_plus(Compiler* self);
void compiler_peer_unary_minus(Compiler* self);
void compiler_peer_bitwise_not(Compiler* self);
void compiler_peer_logical_not(Compiler* self);
void compiler_peer_exponentiation(Compiler* self);
void compiler_peer_multiplication(Compiler* self);
void compiler_peer_division(Compiler* self);
void compiler_peer_remainder(Compiler* self);
void compiler_peer_addition(Compiler* self);
void compiler_peer_subtraction(Compiler* self);
void compiler_peer_left_shift(Compiler* self);
void compiler_peer_signed_right_shift(Compiler* self);
void compiler_peer_unsigned_right_shift(Compiler* self);
void compiler_peer_less_than(Compiler* self);
void compiler_peer_greater_than(Compiler* self);
void compiler_peer_less_than_or_equal(Compiler* self);
void compiler_peer_greater_than_or_equal(Compiler* self);
void compiler_peer_instanceof(Compiler* self);
void compiler_peer_in(Compiler* self);
void compiler_peer_equality(Compiler* self);
void compiler_peer_inequality(Compiler* self);
void compiler_peer_strict_equality(Compiler* self);
void compiler_peer_strict_inequality(Compiler* self);
void compiler_peer_bitwise_and(Compiler* self);
void compiler_peer_bitwise_xor(Compiler* self);
void compiler_peer_bitwise_or(Compiler* self);
void compiler_peer_conditional_ternary(Compiler* self);
void compiler_peer_assignment(Compiler* self);
void compiler_peer_exponentiation_assignment(Compiler* self);
void compiler_peer_multiplication_assignment(Compiler* self);
void compiler_peer_division_assignment(Compiler* self);
void compiler_peer_remainder_assignment(Compiler* self);
void compiler_peer_addition_assignment(Compiler* self);
void compiler_peer_subtraction_assignment(Compiler* self);
void compiler_peer_left_shift_assignment(Compiler* self);
void compiler_peer_signed_right_shift_assignment(Compiler* self);
void compiler_peer_unsigned_right_shift_assignment(Compiler* self);
void compiler_peer_bitwise_and_assignment(Compiler* self);
void compiler_peer_bitwise_xor_assignment(Compiler* self);
void compiler_peer_bitwise_or_assignment(Compiler* self);
void compiler_peer_bindings(Compiler* self, uint16_t n);
void compiler_peer_declare_immutable(Compiler* self);
void compiler_peer_declare_mutable(Compiler* self);
void compiler_peer_declare_function(Compiler* self);
void compiler_peer_declare_closure(Compiler* self);
void compiler_peer_arguments(Compiler* self, uint16_t argc);
void compiler_peer_argument(Compiler* self, uint16_t index);
void compiler_peer_call(Compiler* self, uint16_t argc);
void compiler_peer_truthy(Compiler* self);
void compiler_peer_falsy_short_circuit(Compiler* self);
void compiler_peer_truthy_short_circuit(Compiler* self);
void compiler_peer_nullish_short_circuit(Compiler* self);
void compiler_peer_falsy_short_circuit_assignment(Compiler* self);
void compiler_peer_truthy_short_circuit_assignment(Compiler* self);
void compiler_peer_nullish_short_circuit_assignment(Compiler* self);
void compiler_peer_block(Compiler* self);
void compiler_peer_if_else_statement(Compiler* self);
void compiler_peer_if_statement(Compiler* self);
void compiler_peer_do_while_loop(Compiler* self);
void compiler_peer_while_loop(Compiler* self);
void compiler_peer_for_loop(Compiler* self, bool has_init, bool has_test, bool has_next);
void compiler_peer_loop_init(Compiler* self);
void compiler_peer_loop_test(Compiler* self);
void compiler_peer_loop_next(Compiler* self);
void compiler_peer_loop_body(Compiler* self);
void compiler_peer_loop_end(Compiler* self);
void compiler_peer_case_block(Compiler* self, uint32_t n);
void compiler_peer_case_clause(Compiler* self, bool has_statement);
void compiler_peer_default_clause(Compiler* self, bool has_statement);
void compiler_peer_switch(Compiler* self, uint32_t n, uint32_t default_index);
void compiler_peer_try(Compiler* self);
void compiler_peer_catch(Compiler* self, bool nominal);
void compiler_peer_finally(Compiler* self, bool nominal);
void compiler_peer_try_end(Compiler* self);
void compiler_peer_start_function(Compiler* self, const char* name);
void compiler_peer_end_function(Compiler* self, bool optimize);
void compiler_peer_allocate_bindings(Compiler* self, uint16_t n, bool prologue);
void compiler_peer_release_bindings(Compiler* self, uint16_t n);
void compiler_peer_create_capture(Compiler* self, Locator locator, bool prologue);
void compiler_peer_capture_binding(Compiler* self, bool prologue);
void compiler_peer_escape_binding(Compiler* self, Locator locator);
void compiler_peer_label_start(Compiler* self, uint32_t symbol, bool is_iteration_statement);
void compiler_peer_label_end(Compiler* self, uint32_t symbol, bool is_iteration_statement);
void compiler_peer_continue(Compiler* self, uint32_t symbol);
void compiler_peer_break(Compiler* self, uint32_t symbol);
void compiler_peer_return(Compiler* self, size_t n);
void compiler_peer_throw(Compiler* self);
void compiler_peer_discard(Compiler* self);
void compiler_peer_swap(Compiler* self);
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
