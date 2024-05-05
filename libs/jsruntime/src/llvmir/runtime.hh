// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsruntime/src/llvmir/runtime.hh.njk

#pragma once

#include <cstdint>

#include "macros.hh"

BEGIN_C_LINKAGE

enum ValueKind : uint64_t {
  Undefined = 0,
  Boolean = 1,
  Number = 2,
  Closure = 3,
};

union ValueHolder {
  bool boolean;
  double number;
  uint64_t closure;
  uintptr_t opaque;
};

struct Value {
  ValueKind kind;
  ValueHolder holder;
};

static_assert(sizeof(ValueKind) == sizeof(uint64_t));
static_assert(sizeof(ValueHolder) == sizeof(uint64_t));
static_assert(sizeof(Value) == sizeof(uint64_t) * 2);

struct Runtime {
  void (*declare_immutable)(uintptr_t context,
      uint32_t symbol,
      uint32_t locator,
      const Value* value);
  void (*declare_immutable_undefined)(uintptr_t context, uint32_t symbol, uint32_t locator);
  void (*declare_immutable_boolean)(uintptr_t context,
      uint32_t symbol,
      uint32_t locator,
      bool value);
  void (*declare_immutable_number)(uintptr_t context,
      uint32_t symbol,
      uint32_t locator,
      double value);
  void (
      *declare_mutable)(uintptr_t context, uint32_t symbol, uint32_t locator, const Value* value);
  void (*declare_mutable_undefined)(uintptr_t context, uint32_t symbol, uint32_t locator);
  void (
      *declare_mutable_boolean)(uintptr_t context, uint32_t symbol, uint32_t locator, bool value);
  void (
      *declare_mutable_number)(uintptr_t context, uint32_t symbol, uint32_t locator, double value);
  void (*declare_function)(uintptr_t context, uint32_t symbol, uint32_t locator, uint32_t func_id);
  void (*get_binding)(uintptr_t context, uint32_t symbol, uint32_t locator, Value* value);
  bool (*get_binding_boolean)(uintptr_t context, uint32_t symbol, uint32_t locator);
  double (*get_binding_number)(uintptr_t context, uint32_t symbol, uint32_t locator);
  void (*put_binding)(uintptr_t context, uint32_t symbol, uint32_t locator, const Value* value);
  void (*put_binding_undefined)(uintptr_t context, uint32_t symbol, uint32_t locator);
  void (*put_binding_boolean)(uintptr_t context, uint32_t symbol, uint32_t locator, bool value);
  void (*put_binding_number)(uintptr_t context, uint32_t symbol, uint32_t locator, double value);
  void (*push_argument)(uintptr_t context, const Value* value);
  void (*push_argument_undefined)(uintptr_t context);
  void (*push_argument_boolean)(uintptr_t context, bool value);
  void (*push_argument_number)(uintptr_t context, double value);
  void (*call)(uintptr_t context, const Value* func, Value* result);
  void (*return_value)(uintptr_t context, const Value* value);
  void (*return_boolean)(uintptr_t context, bool value);
  void (*return_number)(uintptr_t context, double value);
  void (*allocate_bindings)(uintptr_t context, uint16_t n);
  void (*release_bindings)(uintptr_t context, uint16_t n);
  void (*inspect)(uintptr_t context, const Value* value);
  void (*inspect_boolean)(uintptr_t context, bool value);
  void (*inspect_number)(uintptr_t context, double value);
};

END_C_LINKAGE
