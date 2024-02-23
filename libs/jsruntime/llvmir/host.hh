#pragma once

#include <cstddef>
#include <cstdint>

#include "macros.hh"

BEGIN_C_LINKAGE

typedef void (*PrintBoolFn)(bool value);
typedef void (*PrintF64Fn)(double value);
typedef void (*PrintStrFn)(const char* value);
typedef double (*RuntimeGetFn)(uintptr_t context, uint32_t symbol_id);
typedef void (*RuntimeSetFn)(uintptr_t context, uint32_t symbol_id, double value);
typedef void (*RuntimeSetUndefinedFn)(uintptr_t context, uint32_t symbol_id);
typedef double (*RuntimeCallFn)(uintptr_t context, uint32_t symbol_id);

struct Host {
  PrintBoolFn print_bool;
  PrintF64Fn print_f64;
  PrintStrFn print_str;
  RuntimeGetFn runtime_get;
  RuntimeSetFn runtime_set;
  RuntimeSetUndefinedFn runtime_set_undefined;
  RuntimeCallFn runtime_call;
};

END_C_LINKAGE
