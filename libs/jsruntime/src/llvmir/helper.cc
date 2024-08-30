#include "helper.hh"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/IR/Value.h>
#include <llvm/Support/raw_ostream.h>
#pragma GCC diagnostic pop

// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// based-on: Value::getNameOrAsOperand().
std::string GetNameOrAsOperand(llvm::Value* value) {
  assert(value != nullptr);

  auto name = value->getName();
  if (!name.empty()) {
    return std::string(name);
  }

  std::string buffer;
  llvm::raw_string_ostream os(buffer);
  value->printAsOperand(os);
  return buffer;
}

size_t GetNameOrAsOperand(llvm::Value* value, char* buf, size_t len) {
  assert(value != nullptr);
  assert(buf != nullptr);
  assert(len > 1);
  auto s = GetNameOrAsOperand(value);
  auto nwritten = std::min(s.size(), len - 1);
  memcpy(buf, s.data(), nwritten);
  buf[nwritten] = '\0';
  return nwritten;
}
