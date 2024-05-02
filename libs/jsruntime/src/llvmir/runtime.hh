#pragma once

#include <cstdint>

#include "macros.hh"

BEGIN_C_LINKAGE

enum ValueKind : uint64_t {
  Undefined = 0,
  Boolean = 1,
  Number = 2,
  Function = 3,
};

struct Function {
  uint32_t id;
  uint32_t lexical_call_index;
};

union ValueHolder {
  bool boolean;
  double number;
  struct Function function;
  uint64_t opaque;
};

struct Value {
  ValueKind kind;
  ValueHolder holder;
};

static_assert(sizeof(ValueKind) == sizeof(uint64_t));
static_assert(sizeof(ValueHolder) == sizeof(uint64_t));
static_assert(sizeof(Value) == sizeof(uint64_t) * 2);

struct Runtime {
  void (*declare_const)(uintptr_t context, uint32_t symbol_id, uint16_t index, double value);
  void (*declare_variable)(uintptr_t context, uint32_t symbol_id, uint16_t index, double value);
  void (
      *declare_function)(uintptr_t context, uint32_t symbol_id, uint16_t index, uint32_t func_id);
  void (*get_argument)(uintptr_t context, uint32_t symbol_id, uint16_t index, Value* value);
  void (*get_local)(uintptr_t context,
      uint32_t symbol_id,
      uint16_t stack,
      uint16_t index,
      Value* value);
  void (*put_argument)(uintptr_t context, uint32_t symbol_id, uint16_t index, double value);
  void (*put_local)(uintptr_t context,
      uint32_t symbol_id,
      uint16_t stack,
      uint16_t index,
      double value);
  void (*push_arg)(uintptr_t context, double arg);
  double (*call)(uintptr_t context, const Value* value);
  void (*ret)(uintptr_t context, double value);
  void (*allocate_bindings)(uintptr_t context, uint16_t n);
  void (*release_bindings)(uintptr_t context, uint16_t n);
  void (*inspect_number)(uintptr_t context, double value);
  void (*inspect_any)(uintptr_t context, const Value* value);
};

END_C_LINKAGE
