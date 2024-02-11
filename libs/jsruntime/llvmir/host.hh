#pragma once

#include <cstddef>

#include "macros.hh"

BEGIN_C_LINKAGE

typedef void (*PrintBool)(bool value);
typedef void (*PrintF64)(double value);
typedef void (*PrintStr)(const char* value);

struct Host {
  PrintBool print_bool;
  PrintF64 print_f64;
  PrintStr print_str;
};

END_C_LINKAGE
