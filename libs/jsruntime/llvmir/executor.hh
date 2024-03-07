// Based on KaleidoscopeJIT

#pragma once

#include <memory>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wredundant-move"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include "llvm/ADT/StringRef.h"
#include "llvm/ExecutionEngine/JITSymbol.h"
#include "llvm/ExecutionEngine/Orc/CompileUtils.h"
#include "llvm/ExecutionEngine/Orc/Core.h"
#include "llvm/ExecutionEngine/Orc/ExecutionUtils.h"
#include "llvm/ExecutionEngine/Orc/ExecutorProcessControl.h"
#include "llvm/ExecutionEngine/Orc/IRCompileLayer.h"
#include "llvm/ExecutionEngine/Orc/JITTargetMachineBuilder.h"
#include "llvm/ExecutionEngine/Orc/RTDyldObjectLinkingLayer.h"
#include "llvm/ExecutionEngine/Orc/Shared/ExecutorSymbolDef.h"
#include "llvm/ExecutionEngine/SectionMemoryManager.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/LLVMContext.h"
#pragma GCC diagnostic pop

class Host;
struct Module;
typedef void (*MainFn)(void*);
typedef double (*FuncFn)(void*);

class Executor {
 public:
  static llvm::Expected<Executor*> Create();

  Executor(std::unique_ptr<llvm::orc::ExecutionSession> exec_session,
      llvm::orc::JITTargetMachineBuilder jtmb,
      llvm::DataLayout data_layout);

  ~Executor();

  void RegisterHost(const Host* host);
  void RegisterModule(Module* mod);
  MainFn GetMain();
  FuncFn GetFunc(const char* name);

  llvm::orc::ExecutionSession& exec_session() {
    return *exec_session_;
  }

  const llvm::DataLayout& data_layout() const {
    return data_layout_;
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
