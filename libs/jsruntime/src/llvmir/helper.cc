#include "helper.hh"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/IR/Value.h>
#include <llvm/Support/raw_ostream.h>
#pragma GCC diagnostic pop

// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// based-on: Value::getNameOrAsOperand().
std::string GetNameOrAsOperand(llvm::Value* value) {
  auto name = value->getName();
  if (!name.empty()) {
    return std::string(name);
  }

  std::string buffer;
  llvm::raw_string_ostream os(buffer);
  value->printAsOperand(os);
  return buffer;
}
