// Based on KaleidoscopeJIT

#pragma once

#include <memory>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wredundant-move"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/ADT/StringRef.h>
#include <llvm/ExecutionEngine/JITSymbol.h>
#include <llvm/ExecutionEngine/Orc/CompileUtils.h>
#include <llvm/ExecutionEngine/Orc/Core.h>
#include <llvm/ExecutionEngine/Orc/ExecutionUtils.h>
#include <llvm/ExecutionEngine/Orc/ExecutorProcessControl.h>
#include <llvm/ExecutionEngine/Orc/IRCompileLayer.h>
#include <llvm/ExecutionEngine/Orc/JITTargetMachineBuilder.h>
#include <llvm/ExecutionEngine/Orc/RTDyldObjectLinkingLayer.h>
#include <llvm/ExecutionEngine/Orc/Shared/ExecutorSymbolDef.h>
#include <llvm/ExecutionEngine/SectionMemoryManager.h>
#include <llvm/IR/DataLayout.h>
#include <llvm/IR/LLVMContext.h>
#pragma GCC diagnostic pop

#include "bridge.hh"

struct Module;

class Executor {
 public:
  static llvm::Expected<Executor*> Create();

  Executor(std::unique_ptr<llvm::orc::ExecutionSession> exec_session,
      llvm::orc::JITTargetMachineBuilder jtmb,
      llvm::DataLayout data_layout);

  ~Executor();

  void RegisterRuntime(const Runtime* runtime);
  void RegisterHostFunction(const char* name, FuncPtr func);
  void RegisterModule(Module* mod);
  FuncPtr GetNativeFunc(const char* name);

  llvm::orc::ExecutionSession& exec_session() {
    return *exec_session_;
  }

  const llvm::DataLayout& data_layout() const {
    return data_layout_;
  }

  const llvm::Triple& target_triple() const {
    return exec_session_->getTargetTriple();
  }

  llvm::orc::JITDylib& main_jd() {
    return main_jd_;
  }

 private:
  llvm::Expected<llvm::orc::ExecutorSymbolDef> Lookup(llvm::StringRef name);

  std::unique_ptr<llvm::orc::ExecutionSession> exec_session_;
  llvm::DataLayout data_layout_;
  llvm::orc::MangleAndInterner mangle_;
  llvm::orc::RTDyldObjectLinkingLayer object_layer_;
  llvm::orc::IRCompileLayer compile_layer_;
  llvm::orc::JITDylib& main_jd_;
  llvm::orc::ResourceTrackerSP tracker_;
};
