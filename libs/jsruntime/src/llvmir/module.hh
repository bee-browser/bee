#pragma once

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/ExecutionEngine/Orc/ThreadSafeModule.h>
#pragma GCC diagnostic pop

struct Module {
  Module(llvm::orc::ThreadSafeModule&& mod) : mod(std::move(mod)) {}
  ~Module() = default;

  void Print(bool stderr) const {
    if (stderr) {
      mod.getModuleUnlocked()->print(llvm::errs(), nullptr);
    } else {
      mod.getModuleUnlocked()->print(llvm::outs(), nullptr);
    }
  }

  llvm::orc::ThreadSafeModule mod;
};
