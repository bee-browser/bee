#pragma once

#include <cstddef>
#include <cstdint>

struct Closure;

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

// opaque types
class Compiler;
struct BasicBlock;
struct LambdaIr;
struct BooleanIr;
struct NumberIr;
struct ClosureIr;
struct ValueIr;
struct ArgvIr;

Compiler* compiler_peer_new();
void compiler_peer_delete(Compiler* self);

void compiler_peer_start(Compiler* self, bool enable_labels);
void compiler_peer_set_data_layout(Compiler* self, const char* data_layout);
void compiler_peer_set_target_triple(Compiler* self, const char* triple);
Module* compiler_peer_end(Compiler* self);

void compiler_peer_start_function(Compiler* self, const char* name);
void compiler_peer_set_locals_block(Compiler* self, BasicBlock* block);
void compiler_peer_end_function(Compiler* self, bool optimize);

// basic block
BasicBlock* compiler_peer_create_basic_block(Compiler* self, const char* name, size_t name_len);
BasicBlock* compiler_peer_get_basic_block(const Compiler* self);
void compiler_peer_set_basic_block(Compiler* self, BasicBlock* block);
void compiler_peer_move_basic_block_after(Compiler* self, BasicBlock* block);
bool compiler_peer_is_basic_block_terminated(Compiler* self, BasicBlock* block);

void compiler_peer_create_br(Compiler* self, BasicBlock* block);
void compiler_peer_create_cond_br(Compiler* self, BooleanIr* cond, BasicBlock* then_block, BasicBlock* else_block);

LambdaIr* compiler_peer_get_function(Compiler* self, uint32_t func_id, const char* name);

ValueIr* compiler_peer_get_exception(Compiler* self);

// boolean
BooleanIr* compiler_peer_get_boolean(Compiler* self, bool value);
BooleanIr* compiler_peer_create_logical_not(Compiler* self, BooleanIr* boolean);
BooleanIr* compiler_peer_create_boolean_phi(Compiler* self, BooleanIr* then_value, BasicBlock* then_block, BooleanIr* else_value, BasicBlock* else_block);
NumberIr* compiler_peer_create_boolean_to_number(Compiler* self, BooleanIr* value);
ValueIr* compiler_peer_create_boolean_to_any(Compiler* self, BooleanIr* boolean);

// number
NumberIr* compiler_peer_get_nan(Compiler* self);
NumberIr* compiler_peer_get_zero(Compiler* self);
NumberIr* compiler_peer_get_number(Compiler* self, double value);
NumberIr* compiler_peer_create_bitwise_not(Compiler* self, NumberIr* value);
NumberIr* compiler_peer_create_fneg(Compiler* self, NumberIr* value);
NumberIr* compiler_peer_create_fmul(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_fdiv(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_frem(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_fadd(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_fsub(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_left_shift(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_signed_right_shift(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_unsigned_right_shift(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_bitwise_and(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_bitwise_xor(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_bitwise_or(Compiler* self, NumberIr* lhs, NumberIr* rhs);
BooleanIr* compiler_peer_create_less_than(Compiler* self, NumberIr* lhs, NumberIr* rhs);
BooleanIr* compiler_peer_create_greater_than(Compiler* self, NumberIr* lhs, NumberIr* rhs);
BooleanIr* compiler_peer_create_less_than_or_equal(Compiler* self, NumberIr* lhs, NumberIr* rhs);
BooleanIr* compiler_peer_create_greater_than_or_equal(Compiler* self, NumberIr* lhs, NumberIr* rhs);
NumberIr* compiler_peer_create_number_phi(Compiler* self, NumberIr* then_value, BasicBlock* then_block, NumberIr* else_value, BasicBlock* else_block);
BooleanIr* compiler_peer_create_number_to_boolean(Compiler* self, NumberIr* number);
ValueIr* compiler_peer_create_number_to_any(Compiler* self, NumberIr* number);

// closure
ClosureIr* compiler_peer_get_closure_nullptr(Compiler* self);
ClosureIr* compiler_peer_create_closure(Compiler* self, LambdaIr* lambda, uint16_t num_captures);
void compiler_peer_create_store_capture_to_closure(Compiler* self, ValueIr* capture, ClosureIr* closure, uint16_t index);
ValueIr* compiler_peer_create_call_on_closure(Compiler* self, ClosureIr* closure, uint16_t argc, ArgvIr* argv, ValueIr* retv);
ClosureIr* compiler_peer_create_closure_phi(Compiler* self, ClosureIr* then_value, BasicBlock* then_block, ClosureIr* else_value, BasicBlock* else_block);
ValueIr* compiler_peer_create_closure_to_any(Compiler* self, ClosureIr* closure);

// capture
ValueIr* compiler_peer_create_call_runtime_create_capture(Compiler* self, ValueIr* variable);
void compiler_peer_create_escape_variable(Compiler* self, ValueIr* capture, ValueIr* variable);
ValueIr* compiler_peer_create_get_capture_variable_ptr(Compiler* self, uint16_t index);
ValueIr* compiler_peer_create_load_capture(Compiler* self, uint16_t index);

// value
BooleanIr* compiler_peer_create_to_boolean(Compiler* self, ValueIr* value);

// equality/inequality operators
BooleanIr* compiler_peer_create_is_loosely_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs);
BooleanIr* compiler_peer_create_is_strictly_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs);

// type conversions
ValueIr* compiler_peer_create_undefined_to_any(Compiler* self);
ValueIr* compiler_peer_create_null_to_any(Compiler* self);

BooleanIr* compiler_peer_create_is_undefined(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_null(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_boolean(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_number(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_closure(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_non_nullish(Compiler* self, ValueIr* value);

BooleanIr* compiler_peer_create_is_same_boolean(Compiler* self, BooleanIr* a, BooleanIr* b);
BooleanIr* compiler_peer_create_is_same_number(Compiler* self, NumberIr* a, NumberIr* b);
BooleanIr* compiler_peer_create_is_same_closure(Compiler* self, ClosureIr* a, ClosureIr* b);

BooleanIr* compiler_peer_create_is_same_boolean_value(Compiler* self, ValueIr* value, BooleanIr* boolean);
BooleanIr* compiler_peer_create_is_same_number_value(Compiler* self, ValueIr* value, NumberIr* number);
BooleanIr* compiler_peer_create_is_same_closure_value(Compiler* self, ValueIr* value, ClosureIr* closure);

ValueIr* compiler_peer_create_value_phi(Compiler* self, ValueIr* then_value, BasicBlock* then_block, ValueIr* else_value, BasicBlock* else_block);

NumberIr* compiler_peer_to_numeric(Compiler* self, ValueIr* value);

void compiler_peer_create_store_flags_to_variable(Compiler* self, uint8_t flags, ValueIr* variable);
void compiler_peer_create_store_symbol_to_variable(Compiler* self, uint32_t symbol, ValueIr* variable);
void compiler_peer_create_store_undefined_to_variable(Compiler* self, ValueIr* variable);
void compiler_peer_create_store_null_to_variable(Compiler* self, ValueIr* variable);
void compiler_peer_create_store_boolean_to_variable(Compiler* self, BooleanIr* value, ValueIr* variable);
void compiler_peer_create_store_number_to_variable(Compiler* self, NumberIr* value, ValueIr* variable);
void compiler_peer_create_store_closure_to_variable(Compiler* self, ClosureIr* value, ValueIr* variable);
void compiler_peer_create_store_value_to_variable(Compiler* self, ValueIr* value, ValueIr* variable);

ClosureIr* compiler_peer_create_load_closure_from_value(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_has_uncaught_exception(Compiler* self);
void compiler_peer_handle_returned_thrown(Compiler* self,
    bool returned,
    bool thrown,
    BasicBlock* block,
    BasicBlock* cleanup_block,
    BasicBlock* exception_block);
ValueIr* compiler_peer_create_local_variable(Compiler* self, uint16_t index);

// argv
ArgvIr* compiler_peer_get_argv_nullptr(Compiler* self);
ArgvIr* compiler_peer_create_argv(Compiler* self, uint16_t argc);
ValueIr* compiler_peer_create_get_arg_in_argv(Compiler* self, ArgvIr* argv, uint16_t index);
ValueIr* compiler_peer_create_get_argument_variable_ptr(Compiler* self, uint16_t index);

// retv
ValueIr* compiler_peer_create_retv(Compiler* self);
void compiler_peer_create_store_undefined_to_retv(Compiler* self);
void compiler_peer_create_store_null_to_retv(Compiler* self);
void compiler_peer_create_store_boolean_to_retv(Compiler* self, BooleanIr* value);
void compiler_peer_create_store_number_to_retv(Compiler* self, NumberIr* value);
void compiler_peer_create_store_closure_to_retv(Compiler* self, ClosureIr* value);
void compiler_peer_create_store_value_to_retv(Compiler* self, ValueIr* value);

// status
void compiler_peer_create_alloc_status(Compiler* self);
void compiler_peer_create_store_normal_status(Compiler* self);
void compiler_peer_create_store_exception_status(Compiler* self);
BooleanIr* compiler_peer_create_is_exception_status(Compiler* self, ValueIr* status);

// scope cleanup checker
void compiler_peer_prepare_scope_cleanup_checker(Compiler* self, uint16_t stack_size);
void compiler_peer_start_scope_cleanup_checker(Compiler* self, uint16_t scope_id);
void compiler_peer_end_scope_cleanup_checker(Compiler* self, uint16_t scope_id);

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

size_t helper_peer_get_basic_block_name_or_as_operand(BasicBlock* block, char* buf, size_t len);
size_t helper_peer_get_value_name_or_as_operand(ValueIr* value, char* buf, size_t len);
