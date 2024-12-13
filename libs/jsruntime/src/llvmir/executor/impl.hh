// Based on KaleidoscopeJIT

#pragma once

#include <cmath>
#include <cstring>
#include <limits>
#include <memory>
#include <sstream>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wredundant-move"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/ADT/StringRef.h>
#include <llvm/ExecutionEngine/Orc/LLJIT.h>
#pragma GCC diagnostic pop

#include "../bridge.hh"
#include "../module/impl.hh"

namespace {

// TODO(perf): Inefficient.  Use a fixed size buffer for formatting `id`.
std::string IdToName(uint32_t id) {
  std::stringstream ss;
  ss << "fn" << id;
  return ss.str();
}

}  // namespace

static llvm::ExitOnError ExitOnErr;

class Executor {
 public:
  static llvm::Expected<Executor*> Create() {
    auto jit = ExitOnErr(llvm::orc::LLJITBuilder().create());
    return new Executor(std::move(jit));
  }

  explicit Executor(std::unique_ptr<llvm::orc::LLJIT>&& jit) : jit_(std::move(jit)) {}

  ~Executor() = default;

  void RegisterRuntimeFunctions(const RuntimeFunctions* functions);

  void RegisterModule(Module* mod) {
    ExitOnErr(jit_->addIRModule(std::move(mod->mod)));
  }

  Lambda GetLambda(uint32_t id) {
    auto name = IdToName(id);
    auto addr = ExitOnErr(jit_->lookup(name));
    return addr.toPtr<Lambda>();
  }

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

#include "impl.codegen.hh"
