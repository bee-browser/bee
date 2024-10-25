#pragma once

#include <cstddef>
#include <cstdint>

struct Closure;

#define STATUS_UNSET_BIT 0x10
#define STATUS_MASK 0x0F
#define STATUS_NORMAL 0x00
#define STATUS_EXCEPTION 0x01
#define STATUS_SUSPEND 0x02
#define STATUS_UNSET (STATUS_UNSET_BIT | STATUS_NORMAL)

enum class Status : uint32_t {
  Normal = STATUS_NORMAL,
  Exception = STATUS_EXCEPTION,
  Suspend = STATUS_SUSPEND,
};

static_assert(sizeof(Status) == sizeof(uint32_t), "size mismatched");

enum class ValueKind : uint8_t {
  // DO NOT CHANGE THE ORDER OF THE FOLLOWING ENUM VARIANTS.
  // Some operations heavily rely on the order.
  None = 0,
  Undefined,
  Null,
  Boolean,
  Number,
  Closure,
  Promise,
};

static_assert(sizeof(ValueKind) == sizeof(uint8_t), "size mismatched");

union ValueHolder {
  uintptr_t opaque;
  bool boolean;
  double number;
  // TODO(issue#237): GcCellRef
  Closure* closure;
  uint32_t promise;
};

static_assert(sizeof(ValueHolder) == sizeof(uint64_t), "size mismatched");

// Can be copied as Value.
struct Value {
  ValueKind kind;
  ValueHolder holder;
};

static_assert(sizeof(Value) == sizeof(uint64_t) * 2, "size mismatched");

// The actual type of `lctx` varies depending on usage of the lambda function:
//
//   Regular functions: Capture**
//   Coroutine functions: Coroutine*
//
typedef Status (*Lambda)(void* gctx, void* lctx, size_t argc, Value* argv, Value* ret);

// TODO(issue#237): GcCell
struct Capture {
  // NOTE: The `target` may point to the `escaped`.  In this case, the `target` must be updated if
  // the capture is moved during GC, so that the `target` points to the `escaped` correctly.
  Value* target;
  Value escaped;
};

static_assert(sizeof(Capture) == sizeof(uint64_t) * 3, "size mismatched");

// TODO(issue#237): GcCell
struct Closure {
  // A pointer to a function compiled from a JavaScript function.
  Lambda lambda;

  // The number of captures.
  //
  // Usually, this field does not used in the compiled function, but we add this field here for
  // debugging purposes.  If we need to reduce the heap memory usage and `Closure`s dominant, we
  // can remove this field.
  uint16_t num_captures;

  // A variable-length list of captures used in the lambda function.
  // TODO(issue#237): GcCellRef
  Capture* captures[32];
};

// TODO(issue#237): GcCell
struct Coroutine {
  // The closure of the coroutine.
  // TODO(issue#237): GcCellRef
  Closure* closure;

  // The state of the coroutine.
  uint32_t state;

  // The number of local variables.
  uint16_t num_locals;

  // The current scope id used by the scope cleanup checker.
  uint16_t scope_id;

  // A variable-length list of local variables used in the coroutine.
  Value locals[32];
};

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
struct CoroutineIr;
struct PromiseIr;
struct ValueIr;
struct ArgvIr;
struct StatusIr;
struct CaptureIr;
struct SwitchIr;

Compiler* compiler_peer_new();
void compiler_peer_delete(Compiler* self);

void compiler_peer_start(Compiler* self, bool enable_labels);
Module* compiler_peer_end(Compiler* self);
void compiler_peer_set_data_layout(Compiler* self, const char* data_layout);
void compiler_peer_set_target_triple(Compiler* self, const char* triple);

// function
void compiler_peer_start_function(Compiler* self, uint32_t func_id);
void compiler_peer_end_function(Compiler* self, bool optimize);
void compiler_peer_set_locals_block(Compiler* self, BasicBlock* block);
LambdaIr* compiler_peer_get_function(Compiler* self, uint32_t func_id);

// basic block
BasicBlock* compiler_peer_create_basic_block(Compiler* self, const char* name, size_t name_len);
BasicBlock* compiler_peer_get_basic_block(const Compiler* self);
void compiler_peer_set_basic_block(Compiler* self, BasicBlock* block);
void compiler_peer_move_basic_block_after(Compiler* self, BasicBlock* block);
bool compiler_peer_is_basic_block_terminated(Compiler* self, BasicBlock* block);

// jump
void compiler_peer_create_br(Compiler* self, BasicBlock* block);
void compiler_peer_create_cond_br(Compiler* self,
    BooleanIr* cond,
    BasicBlock* then_block,
    BasicBlock* else_block);

// undefined
BooleanIr* compiler_peer_create_is_undefined(Compiler* self, ValueIr* value);

// null
BooleanIr* compiler_peer_create_is_null(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_non_nullish(Compiler* self, ValueIr* value);

// boolean
BooleanIr* compiler_peer_create_is_boolean(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_same_boolean(Compiler* self, BooleanIr* a, BooleanIr* b);
BooleanIr* compiler_peer_create_number_to_boolean(Compiler* self, NumberIr* number);
BooleanIr* compiler_peer_create_to_boolean(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_get_boolean(Compiler* self, bool value);
BooleanIr* compiler_peer_create_logical_not(Compiler* self, BooleanIr* boolean);
BooleanIr* compiler_peer_create_boolean_phi(Compiler* self,
    BooleanIr* then_value,
    BasicBlock* then_block,
    BooleanIr* else_value,
    BasicBlock* else_block);

// number
BooleanIr* compiler_peer_create_is_number(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_same_number(Compiler* self, NumberIr* a, NumberIr* b);
NumberIr* compiler_peer_create_boolean_to_number(Compiler* self, BooleanIr* value);
NumberIr* compiler_peer_to_numeric(Compiler* self, ValueIr* value);
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
BooleanIr* compiler_peer_create_greater_than_or_equal(Compiler* self,
    NumberIr* lhs,
    NumberIr* rhs);
NumberIr* compiler_peer_create_number_phi(Compiler* self,
    NumberIr* then_value,
    BasicBlock* then_block,
    NumberIr* else_value,
    BasicBlock* else_block);

// closure
BooleanIr* compiler_peer_create_is_closure(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_same_closure(Compiler* self, ClosureIr* a, ClosureIr* b);
ClosureIr* compiler_peer_get_closure_nullptr(Compiler* self);
ClosureIr* compiler_peer_create_closure(Compiler* self, LambdaIr* lambda, uint16_t num_captures);
void compiler_peer_create_store_capture_to_closure(Compiler* self,
    CaptureIr* capture,
    ClosureIr* closure,
    uint16_t index);
StatusIr* compiler_peer_create_call_on_closure(Compiler* self,
    ClosureIr* closure,
    uint16_t argc,
    ArgvIr* argv,
    ValueIr* retv);
ClosureIr* compiler_peer_create_closure_phi(Compiler* self,
    ClosureIr* then_value,
    BasicBlock* then_block,
    ClosureIr* else_value,
    BasicBlock* else_block);

// promise
BooleanIr* compiler_peer_create_is_promise(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_same_promise(Compiler* self, PromiseIr* a, PromiseIr* b);
PromiseIr* compiler_peer_create_register_promise(Compiler* self, CoroutineIr* coroutine);
void compiler_peer_create_await_promise(Compiler* self, PromiseIr* promise, PromiseIr* awaiting);
void compiler_peer_create_resume(Compiler* self, PromiseIr* promise);
void compiler_peer_create_emit_promise_resolved(Compiler* self,
    PromiseIr* promise,
    ValueIr* result);

// value
BooleanIr* compiler_peer_create_has_value(Compiler* self, ValueIr* value);
BooleanIr* compiler_peer_create_is_loosely_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs);
BooleanIr* compiler_peer_create_is_strictly_equal(Compiler* self, ValueIr* lhs, ValueIr* rhs);
BooleanIr* compiler_peer_create_is_same_boolean_value(Compiler* self,
    ValueIr* value,
    BooleanIr* boolean);
BooleanIr* compiler_peer_create_is_same_number_value(Compiler* self,
    ValueIr* value,
    NumberIr* number);
BooleanIr* compiler_peer_create_is_same_closure_value(Compiler* self,
    ValueIr* value,
    ClosureIr* closure);
BooleanIr* compiler_peer_create_is_same_promise_value(Compiler* self,
    ValueIr* value,
    PromiseIr* promise);
ValueIr* compiler_peer_create_undefined_to_any(Compiler* self);
ValueIr* compiler_peer_create_null_to_any(Compiler* self);
ValueIr* compiler_peer_create_boolean_to_any(Compiler* self, BooleanIr* boolean);
ValueIr* compiler_peer_create_number_to_any(Compiler* self, NumberIr* number);
ValueIr* compiler_peer_create_closure_to_any(Compiler* self, ClosureIr* closure);
ValueIr* compiler_peer_create_value_phi(Compiler* self,
    ValueIr* then_value,
    BasicBlock* then_block,
    ValueIr* else_value,
    BasicBlock* else_block);
ValueIr* compiler_peer_create_local_value(Compiler* self, uint16_t index);
void compiler_peer_create_store_none_to_value(Compiler* self, ValueIr* dest);
void compiler_peer_create_store_undefined_to_value(Compiler* self, ValueIr* dest);
void compiler_peer_create_store_null_to_value(Compiler* self, ValueIr* dest);
void compiler_peer_create_store_boolean_to_value(Compiler* self, BooleanIr* value, ValueIr* dest);
void compiler_peer_create_store_number_to_value(Compiler* self, NumberIr* value, ValueIr* dest);
void compiler_peer_create_store_closure_to_value(Compiler* self, ClosureIr* value, ValueIr* dest);
void compiler_peer_create_store_promise_to_value(Compiler* self, PromiseIr* value, ValueIr* dest);
void compiler_peer_create_store_value_to_value(Compiler* self, ValueIr* value, ValueIr* dest);
ClosureIr* compiler_peer_create_load_closure_from_value(Compiler* self, ValueIr* value);
PromiseIr* compiler_peer_create_load_promise_from_value(Compiler* self, ValueIr* value);

// argv
ArgvIr* compiler_peer_get_argv_nullptr(Compiler* self);
ArgvIr* compiler_peer_create_argv(Compiler* self, uint16_t argc);
ValueIr* compiler_peer_create_get_arg_in_argv(Compiler* self, ArgvIr* argv, uint16_t index);
ValueIr* compiler_peer_create_get_argument_value_ptr(Compiler* self, uint16_t index);

// retv
//
// The `retv` variable holds either a returned or thrown value.
ValueIr* compiler_peer_create_retv(Compiler* self);
void compiler_peer_create_store_undefined_to_retv(Compiler* self);
void compiler_peer_create_store_null_to_retv(Compiler* self);
void compiler_peer_create_store_boolean_to_retv(Compiler* self, BooleanIr* value);
void compiler_peer_create_store_number_to_retv(Compiler* self, NumberIr* value);
void compiler_peer_create_store_closure_to_retv(Compiler* self, ClosureIr* value);
void compiler_peer_create_store_promise_to_retv(Compiler* self, PromiseIr* value);
void compiler_peer_create_store_value_to_retv(Compiler* self, ValueIr* value);
ValueIr* compiler_peer_get_exception(Compiler* self);

// status
//
// TODO: Currently, each lambda has its own status variable.  However, it might be possible to use
// a single global variable shared by all lambdas.  Because the execution model of JavaScript is a
// single threaded model.
void compiler_peer_create_alloc_status(Compiler* self);
void compiler_peer_create_store_normal_status(Compiler* self);
void compiler_peer_create_store_exception_status(Compiler* self);
BooleanIr* compiler_peer_create_is_exception_status(Compiler* self, StatusIr* status);

// flow selector
void compiler_peer_create_alloc_flow_selector(Compiler* self);
void compiler_peer_create_set_flow_selector_normal(Compiler* self);
void compiler_peer_create_set_flow_selector_return(Compiler* self);
void compiler_peer_create_set_flow_selector_throw(Compiler* self);
void compiler_peer_create_set_flow_selector_break(Compiler* self, uint32_t depth);
void compiler_peer_create_set_flow_selector_continue(Compiler* self, uint32_t depth);
BooleanIr* compiler_peer_create_is_flow_selector_normal(Compiler* self);
BooleanIr* compiler_peer_create_is_flow_selector_normal_or_continue(Compiler* self,
    uint32_t depth);
BooleanIr* compiler_peer_create_is_flow_selector_break_or_continue(Compiler* self, uint32_t depth);
BooleanIr* compiler_peer_create_is_flow_selector_break(Compiler* self, uint32_t depth);

// capture
CaptureIr* compiler_peer_create_capture(Compiler* self, ValueIr* value);
void compiler_peer_create_escape_value(Compiler* self, CaptureIr* capture, ValueIr* value);
ValueIr* compiler_peer_create_get_capture_value_ptr(Compiler* self, uint16_t index);
CaptureIr* compiler_peer_create_load_capture(Compiler* self, uint16_t index);

// coroutine
CoroutineIr* compiler_peer_create_coroutine(Compiler* self,
    ClosureIr* closure,
    uint16_t num_locals);
SwitchIr* compiler_peer_create_switch_for_coroutine(Compiler* self,
    BasicBlock* block,
    uint32_t num_states);
void compiler_peer_create_add_state_for_coroutine(Compiler* self,
    SwitchIr* switch_ir,
    uint32_t state,
    BasicBlock* block);
void compiler_peer_create_suspend(Compiler* self);
void compiler_peer_create_set_coroutine_state(Compiler* self, uint32_t state);
void compiler_peer_create_set_captures_for_coroutine(Compiler* self);
ValueIr* compiler_peer_create_get_local_ptr_from_coroutine(Compiler* self, uint16_t index);

// scope cleanup checker
void compiler_peer_enable_scope_cleanup_checker(Compiler* self, bool is_coroutine);
void compiler_peer_set_scope_id_for_checker(Compiler* self, uint16_t scope_id);
void compiler_peer_assert_scope_id(Compiler* self, uint16_t expected);

// print
void compiler_peer_create_print_value(Compiler* self, ValueIr* value, const char* msg);

// unreachable
void compiler_peer_create_unreachable(Compiler* self, const char* msg);

// Execution

class Executor;
Executor* executor_peer_new();
void executor_peer_delete(Executor* self);
void executor_peer_register_runtime(Executor* self, const Runtime* runtime);
void executor_peer_register_host_function(Executor* self, uint32_t func_id, Lambda func);
void executor_peer_register_module(Executor* self, Module* mod);
const char* executor_peer_get_data_layout(const Executor* self);
const char* executor_peer_get_target_triple(const Executor* self);
Lambda executor_peer_get_native_function(Executor* self, uint32_t func_id);

// Hepler Functions

size_t helper_peer_get_basic_block_name_or_as_operand(BasicBlock* block, char* buf, size_t len);
size_t helper_peer_get_value_name_or_as_operand(ValueIr* value, char* buf, size_t len);
