// Based on KaleidoscopeJIT

#pragma once

#include <memory>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wredundant-move"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/ADT/StringRef.h>
#include "llvm/ExecutionEngine/Orc/LLJIT.h"
#pragma GCC diagnostic pop

#include "../bridge.hh"

struct Module;

class Executor {
 public:
  static llvm::Expected<Executor*> Create();

  explicit Executor(std::unique_ptr<llvm::orc::LLJIT>&& jit) : jit_(std::move(jit)) {}

  ~Executor() = default;

  void RegisterRuntime(const Runtime* runtime);
  void RegisterHostFunction(const char* name, Lambda lambda);
  void RegisterModule(Module* mod);
  Lambda GetNativeFunction(const char* name);

  llvm::orc::ExecutionSession& exec_session() {
    return jit_->getExecutionSession();
  }

  const llvm::DataLayout& data_layout() const {
    return jit_->getDataLayout();
  }

  const llvm::Triple& target_triple() const {
    return jit_->getTargetTriple();
  }

  llvm::orc::JITDylib& main_jd() {
    return jit_->getMainJITDylib();
  }

 private:
  // See examples in:
  // //vendor/src/llvm/llvm-project/examples/HowToUseLLJIT/
  // //vendor/src/llvm/llvm-project/examples/OrcV2Examples/
  std::unique_ptr<llvm::orc::LLJIT> jit_;
};
