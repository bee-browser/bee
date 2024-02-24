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

class Evaluator {
 public:
  static llvm::Expected<std::unique_ptr<Evaluator>> Create() {
    auto epc = llvm::orc::SelfExecutorProcessControl::Create();
    if (!epc) {
      return epc.takeError();
    }

    auto exec_session = std::make_unique<llvm::orc::ExecutionSession>(std::move(*epc));

    llvm::orc::JITTargetMachineBuilder jtmb(
        exec_session->getExecutorProcessControl().getTargetTriple());

    auto data_layout = jtmb.getDefaultDataLayoutForTarget();
    if (!data_layout) {
      return data_layout.takeError();
    }

    return std::make_unique<Evaluator>(
        std::move(exec_session), std::move(jtmb), std::move(*data_layout));
  }

  Evaluator(std::unique_ptr<llvm::orc::ExecutionSession> exec_session,
      llvm::orc::JITTargetMachineBuilder jtmb,
      llvm::DataLayout data_layout)
      : exec_session_(std::move(exec_session)),
        data_layout_(std::move(data_layout)),
        mangle_(*exec_session_, data_layout_),
        object_layer_(*exec_session_,
            []() { return std::make_unique<llvm::SectionMemoryManager>(); }),
        compile_layer_(*exec_session_,
            object_layer_,
            std::make_unique<llvm::orc::ConcurrentIRCompiler>(std::move(jtmb))),
        main_jd_(exec_session_->createBareJITDylib("<main>")) {
    // FIXME: jtmb has already been moved...
    if (jtmb.getTargetTriple().isOSBinFormatCOFF()) {
      object_layer_.setOverrideObjectFlagsWithResponsibilityFlags(true);
      object_layer_.setAutoClaimResponsibilityForObjectSymbols(true);
    }
  }

  ~Evaluator() {
    if (auto err = exec_session_->endSession()) {
      exec_session_->reportError(std::move(err));
    }
  }

  llvm::orc::ExecutionSession& exec_session() {
    return *exec_session_;
  }

  const llvm::DataLayout& data_layout() const {
    return data_layout_;
  }

  llvm::orc::JITDylib& main_jd() {
    return main_jd_;
  }

  llvm::Error AddModule(llvm::orc::ThreadSafeModule mod,
      llvm::orc::ResourceTrackerSP tracker = nullptr) {
    if (!tracker) {
      tracker = main_jd_.getDefaultResourceTracker();
    }
    return compile_layer_.add(tracker, std::move(mod));
  }

  llvm::Expected<llvm::orc::ExecutorSymbolDef> Lookup(llvm::StringRef name) {
    return exec_session_->lookup({&main_jd_}, mangle_(name.str()));
  }

 private:
  std::unique_ptr<llvm::orc::ExecutionSession> exec_session_;
  llvm::DataLayout data_layout_;
  llvm::orc::MangleAndInterner mangle_;
  llvm::orc::RTDyldObjectLinkingLayer object_layer_;
  llvm::orc::IRCompileLayer compile_layer_;
  llvm::orc::JITDylib& main_jd_;
};
