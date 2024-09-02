#pragma once

#include <cstddef>
#include <cstdint>

using BasicBlock = uintptr_t;
using ValueIr = uintptr_t;
using LambdaIr = uintptr_t;

struct Closure;

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

#define STATUS_UNSET_BIT 0x10
#define STATUS_MASK 0x0F
#define STATUS_NORMAL 0x00
#define STATUS_EXCEPTION 0x01
#define STATUS_UNSET (STATUS_UNSET_BIT | STATUS_NORMAL)

enum class Status : uint32_t {
  Normal = STATUS_NORMAL,
  Exception = STATUS_EXCEPTION,
};

static_assert(sizeof(Status) == sizeof(uint32_t), "size mismatched");

enum class ValueKind : uint8_t {
  // DO NOT CHANGE THE ORDER OF THE FOLLOWING ENUM VARIANTS.
  // Some operations heavily rely on the order.
  Undefined = 0,
  Null,
  Boolean,
  Number,
  Closure,
};

static_assert(sizeof(ValueKind) == sizeof(uint8_t), "size mismatched");

union ValueHolder {
  uintptr_t opaque;
  bool boolean;
  double number;
  // TODO(issue#237): GcCellRef
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
struct Variable {
  ValueKind kind;
  uint8_t flags;
  uint16_t reserved;
  uint32_t symbol;
  ValueHolder holder;
};

#define VARIABLE_INITIALIZED 0x01
#define VARIABLE_DELETABLE 0x02
#define VARIABLE_MUTABLE 0x04
#define VARIABLE_STRICT 0x08

static_assert(sizeof(Variable) == sizeof(uint64_t) * 2, "size mismatched");

typedef Status (*Lambda)(void* ctx, void* caps, size_t argc, Value* argv, Value* ret);

// TODO(issue#237): GcCell
struct Capture {
  Variable* target;
  Variable escaped;
};

static_assert(sizeof(Capture) == sizeof(uint64_t) * 3, "size mismatched");

// TODO(issue#237): GcCell
struct Closure {
  // A pointer to a function compiled from a JavaScript function.
  Lambda lambda;

  // The number of elements in `storage[]`.
  //
  // Usually, this field does not used in the compiled function, but we add this field here for
  // debugging purposes.  If we need to reduce the heap memory usage and `Closure`s dominant, we
  // can remove this field.
  uint16_t num_captures;
  // uint8_t padding[6];

  // Using the following definition instead of `Capture* captures[]`, we can avoid accessing the
  // `num_captures` field and comparison and conditional branch instructions that are needed for
  // checking whether `captures` is empty or not.
  Capture** captures;

  // `Capture* storage[num_captures]` is placed here if it's not empty.
};

static_assert(sizeof(Closure) == sizeof(uint64_t) * 3, "size mismatched");

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
void compiler_peer_start(Compiler* self, bool enable_labels);
Module* compiler_peer_end(Compiler* self);
void compiler_peer_set_data_layout(Compiler* self, const char* data_layout);
void compiler_peer_set_target_triple(Compiler* self, const char* triple);
BasicBlock compiler_peer_create_basic_block(Compiler* self, const char* name, size_t name_len);
BasicBlock compiler_peer_get_basic_block(const Compiler* self);
void compiler_peer_set_basic_block(Compiler* self, BasicBlock block);
void compiler_peer_move_basic_block_after(Compiler* self, BasicBlock block);
bool compiler_peer_is_basic_block_terminated(Compiler* self, BasicBlock block);
void compiler_peer_set_locals_block(Compiler* self, BasicBlock block);
void compiler_peer_create_br(Compiler* self, BasicBlock block);
void compiler_peer_create_store_normal_status(Compiler* self);
void compiler_peer_create_store_exception_status(Compiler* self);
ValueIr compiler_peer_get_boolean(Compiler* self, bool value);
ValueIr compiler_peer_get_number(Compiler* self, double value);
LambdaIr compiler_peer_get_function(Compiler* self, uint32_t func_id, const char* name);
ValueIr compiler_peer_create_call_runtime_create_closure(Compiler* self, LambdaIr lambda, uint16_t num_captures);
ValueIr compiler_peer_create_load_captures_from_closure(Compiler* self, ValueIr closure);
void compiler_peer_create_store_capture_ptr_to_captures(Compiler* self, ValueIr capture, ValueIr captures, uint16_t i);
ValueIr compiler_peer_get_exception(Compiler* self);
ValueIr compiler_peer_create_fneg(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_bitwise_not(Compiler* self, ValueIr number);
ValueIr compiler_peer_create_number_to_boolean(Compiler* self, ValueIr number);
ValueIr compiler_peer_create_to_boolean(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_logical_not(Compiler* self, ValueIr boolean);
ValueIr compiler_peer_create_fmul(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_fdiv(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_frem(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_fadd(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_fsub(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_left_shift(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_signed_right_shift(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_unsigned_right_shift(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_less_than(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_greater_than(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_less_than_or_equal(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_greater_than_or_equal(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_is_loosely_equal(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_undefined_to_any(Compiler* self);
ValueIr compiler_peer_create_null_to_any(Compiler* self);
ValueIr compiler_peer_create_boolean_to_any(Compiler* self, ValueIr boolean);
ValueIr compiler_peer_create_number_to_any(Compiler* self, ValueIr number);
ValueIr compiler_peer_create_closure_to_any(Compiler* self, ValueIr closure);
ValueIr compiler_peer_create_is_undefined(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_is_null(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_is_boolean(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_is_number(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_is_closure(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_is_same_boolean(Compiler* self, ValueIr a, ValueIr b);
ValueIr compiler_peer_create_is_same_number(Compiler* self, ValueIr a, ValueIr b);
ValueIr compiler_peer_create_is_same_closure(Compiler* self, ValueIr a, ValueIr b);
ValueIr compiler_peer_create_is_same_boolean_value(Compiler* self, ValueIr value, ValueIr boolean);
ValueIr compiler_peer_create_is_same_number_value(Compiler* self, ValueIr value, ValueIr number);
ValueIr compiler_peer_create_is_same_closure_value(Compiler* self, ValueIr value, ValueIr closure);
ValueIr compiler_peer_create_is_strictly_equal(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_bitwise_and(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_bitwise_xor(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_bitwise_or(Compiler* self, ValueIr lhs, ValueIr rhs);
ValueIr compiler_peer_create_boolean_ternary(Compiler* self, ValueIr then_value, BasicBlock then_block, ValueIr else_value, BasicBlock else_block);
ValueIr compiler_peer_create_number_ternary(Compiler* self, ValueIr then_value, BasicBlock then_block, ValueIr else_value, BasicBlock else_block);
ValueIr compiler_peer_create_any_ternary(Compiler* self, ValueIr then_value, BasicBlock then_block, ValueIr else_value, BasicBlock else_block);
void compiler_peer_create_store_flags_to_variable(Compiler* self, uint8_t flags, ValueIr variable);
void compiler_peer_create_store_symbol_to_variable(Compiler* self, uint32_t symbol, ValueIr variable);
void compiler_peer_create_store_undefined_to_variable(Compiler* self, ValueIr variable);
void compiler_peer_create_store_null_to_variable(Compiler* self, ValueIr variable);
void compiler_peer_create_store_boolean_to_variable(Compiler* self, ValueIr value, ValueIr variable);
void compiler_peer_create_store_number_to_variable(Compiler* self, ValueIr value, ValueIr variable);
void compiler_peer_create_store_closure_to_variable(Compiler* self, ValueIr value, ValueIr variable);
void compiler_peer_create_store_value_to_variable(Compiler* self, ValueIr value, ValueIr variable);
void compiler_peer_create_store_undefined_to_retv(Compiler* self);
void compiler_peer_create_store_null_to_retv(Compiler* self);
void compiler_peer_create_store_boolean_to_retv(Compiler* self, ValueIr value);
void compiler_peer_create_store_number_to_retv(Compiler* self, ValueIr value);
void compiler_peer_create_store_closure_to_retv(Compiler* self, ValueIr value);
void compiler_peer_create_store_value_to_retv(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_variables(Compiler* self, uint16_t n);
ValueIr compiler_peer_create_get_value_ptr_in_values(Compiler* self, ValueIr values, uint16_t index);
ValueIr compiler_peer_create_call_on_closure(Compiler* self, ValueIr closure, uint16_t argc, ValueIr argv, ValueIr retv);
ValueIr compiler_peer_create_ptr(Compiler* self);
ValueIr compiler_peer_create_load_value_kind_from_value(Compiler* self, ValueIr value);
ValueIr compiler_peer_get_nullptr(Compiler* self);
ValueIr compiler_peer_get_u8(Compiler* self, uint8_t value);
ValueIr compiler_peer_get_u32(Compiler* self, uint32_t value);
ValueIr compiler_peer_create_icmp_eq(Compiler* self, ValueIr lhs, ValueIr rhs);
void compiler_peer_create_cond_br(Compiler* self, ValueIr cond, BasicBlock then_block, BasicBlock else_block);
ValueIr compiler_peer_create_load_closure_from_value(Compiler* self, ValueIr value);
void compiler_peer_create_store(Compiler* self, ValueIr value, ValueIr dest);
ValueIr compiler_peer_create_load_ptr(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_call_runtime_create_capture(Compiler* self, ValueIr variable);
ValueIr compiler_peer_create_get_capture_variable_ptr(Compiler* self, uint16_t index);
void compiler_peer_create_escape_variable(Compiler* self, ValueIr capture, ValueIr variable);
ValueIr compiler_peer_create_get_argument_variable_ptr(Compiler* self, uint16_t index);
void compiler_peer_create_alloc_status(Compiler* self);
void compiler_peer_create_store_status(Compiler* self, Status status);
ValueIr compiler_peer_create_load_capture(Compiler* self, uint16_t index);
ValueIr compiler_peer_get_nan(Compiler* self);
ValueIr compiler_peer_get_zero(Compiler* self);
ValueIr compiler_peer_create_ui_to_fp(Compiler* self, ValueIr value);
ValueIr compiler_peer_to_numeric(Compiler* self, ValueIr value);
void compiler_peer_create_store_retv(Compiler* self, ValueIr retv);
ValueIr compiler_peer_create_is_non_nullish(Compiler* self, ValueIr value);
ValueIr compiler_peer_create_has_uncaught_exception(Compiler* self);
void compiler_peer_start_function(Compiler* self, const char* name);
void compiler_peer_end_function(Compiler* self, bool optimize);
void compiler_peer_start_scope_cleanup_checker(Compiler* self, uint16_t scope_id);
void compiler_peer_end_scope_cleanup_checker(Compiler* self, uint16_t scope_id);
void compiler_peer_handle_returned_thrown(Compiler* self,
    bool returned,
    bool thrown,
    BasicBlock block,
    BasicBlock cleanup_block,
    BasicBlock exception_block);
ValueIr compiler_peer_create_local_variable(Compiler* self, uint16_t index);
ValueIr compiler_peer_create_retv(Compiler* self);
void compiler_peer_prepare_scope_cleanup_checker(Compiler* self, uint16_t stack_size);

// Execution

class Executor;
Executor* executor_peer_new();
void executor_peer_delete(Executor* self);
void executor_peer_register_runtime(Executor* self, const Runtime* runtime);
void executor_peer_register_host_function(Executor* self, const char* name, Lambda func);
void executor_peer_register_module(Executor* self, Module* mod);
const char* executor_peer_get_data_layout(const Executor* self);
const char* executor_peer_get_target_triple(const Executor* self);
Lambda executor_peer_get_native_function(Executor* self, const char* name);

// Hepler Functions

size_t helper_peer_get_basic_block_name_or_as_operand(BasicBlock block, char* buf, size_t len);
size_t helper_peer_get_value_name_or_as_operand(ValueIr value, char* buf, size_t len);
