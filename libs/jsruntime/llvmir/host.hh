#pragma once

#include <cstddef>
#include <cstdint>

#include "macros.hh"

BEGIN_C_LINKAGE

typedef void (*PrintBoolFn)(bool value);
typedef void (*PrintF64Fn)(double value);
typedef void (*PrintStrFn)(const char* value);
typedef double (*RuntimeCallFn)(uintptr_t userdata, uint32_t symbol_id);

struct Host {
  PrintBoolFn print_bool;
  PrintF64Fn print_f64;
  PrintStrFn print_str;
  RuntimeCallFn runtime_call;
};

END_C_LINKAGE
