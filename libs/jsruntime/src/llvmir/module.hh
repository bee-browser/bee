#pragma once

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include "llvm/ExecutionEngine/Orc/ThreadSafeModule.h"
#pragma GCC diagnostic pop

struct Module {
  Module(llvm::orc::ThreadSafeModule&& mod) : mod(std::move(mod)) {}
  ~Module() = default;

  void Dump() const {
    llvm::errs() << "<llvm-ir:module>\n";
    mod.getModuleUnlocked()->print(llvm::errs(), nullptr);
    llvm::errs() << "</llvm-ir:module>\n";
  }

  llvm::orc::ThreadSafeModule mod;
};
