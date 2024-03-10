#pragma once

#include <cstddef>
#include <cstdint>

#include "macros.hh"

BEGIN_C_LINKAGE

typedef void (*PrintBoolFn)(bool value);
typedef void (*PrintF64Fn)(double value);
typedef void (*PrintStrFn)(const char* value);
typedef void (*RuntimeDeclareConstFn)(uintptr_t context, uint32_t symbol_id, double value);
typedef void (*RuntimeDeclareVariableFn)(uintptr_t context, uint32_t symbol_id, double value);
typedef void (*RuntimeDeclareUndefinedFn)(uintptr_t context, uint32_t symbol_id);
typedef void (*RuntimeDeclareFunctionFn)(uintptr_t context, uint32_t symbol_id, uint32_t func_id);
typedef double (*RuntimeGetFn)(uintptr_t context, uint32_t symbol_id);
typedef void (*RuntimeSetFn)(uintptr_t context, uint32_t symbol_id, double value);
typedef void (*RuntimeSetUndefinedFn)(uintptr_t context, uint32_t symbol_id);
typedef void (*RuntimePushArgsFn)(uintptr_t context);
typedef void (*RuntimePushArgFn)(uintptr_t context, double arg);
typedef double (*RuntimeCallFn)(uintptr_t context, uint32_t symbol_id);
typedef void (*RuntimeRetFn)(uintptr_t context, double value);
typedef void (*RuntimePushScopeFn)(uintptr_t context);
typedef void (*RuntimePopScopeFn)(uintptr_t context);

struct Host {
  PrintBoolFn print_bool;
  PrintF64Fn print_f64;
  PrintStrFn print_str;
  RuntimeDeclareConstFn runtime_declare_const;
  RuntimeDeclareVariableFn runtime_declare_variable;
  RuntimeDeclareUndefinedFn runtime_declare_undefined;
  RuntimeDeclareFunctionFn runtime_declare_function;
  RuntimeGetFn runtime_get;
  RuntimeSetFn runtime_set;
  RuntimeSetUndefinedFn runtime_set_undefined;
  RuntimePushArgsFn runtime_push_args;
  RuntimePushArgFn runtime_push_arg;
  RuntimeCallFn runtime_call;
  RuntimeRetFn runtime_ret;
  RuntimePushScopeFn runtime_push_scope;
  RuntimePopScopeFn runtime_pop_scope;
};

END_C_LINKAGE
