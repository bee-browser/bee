#pragma once

#include <string>

namespace llvm {
class Value;
}

std::string GetNameOrAsOperand(llvm::Value* value);
size_t GetNameOrAsOperand(llvm::Value* value, char* buf, size_t len);

#define V2S(v) (GetNameOrAsOperand(v))
