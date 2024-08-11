#pragma once

#include <string>

namespace llvm {
class Value;
}

std::string GetNameOrAsOperand(llvm::Value* value);

#define V2S(v) (GetNameOrAsOperand(v))
