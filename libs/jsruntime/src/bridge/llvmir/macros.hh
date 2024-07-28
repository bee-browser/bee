#pragma once

#define BEGIN_C_LINKAGE extern "C" {
#define END_C_LINKAGE }

#define UNUSED(var) ((void)(var))

#if defined(BEE_BUILD_DEBUG)
#define REG_NAME(expr) expr
#else
#define REG_NAME(expr) ""
#endif
