#include "runtime.hh"

#include <cstdint>
#include <iostream>

#include "host.hh"
#include "llvm/Support/TargetSelect.h"

static llvm::ExitOnError ExitOnErr;

// static
void Runtime::Initialize() {
  // Uncomment if you want to enable LLVM_DEBUG().
  // llvm::DebugFlag = true;

  llvm::InitializeNativeTarget();
  llvm::InitializeNativeTargetAsmPrinter();
  llvm::InitializeNativeTargetAsmParser();
}

Runtime::Runtime() {
  evaluator_ = llvm::cantFail(Evaluator::Create());
  compiler_ = std::make_unique<Compiler>();
}

void Runtime::RegisterHost(const Host* host) {
  // register built-in functions.
  auto& exec_session = evaluator_->exec_session();
  llvm::orc::SymbolMap symbols;
  symbols[exec_session.intern("print_bool")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->print_bool),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("print_f64")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->print_f64),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("print_str")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->print_str),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_get")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_get),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_set")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_set),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_declare")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_declare),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_set_undefined")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_set_undefined),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_call")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_call),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_push_scope")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_push_scope),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_pop_scope")] = {
      llvm::orc::ExecutorAddr::fromPtr(host->runtime_pop_scope),
      llvm::JITSymbolFlags::Exported,
  };
  ExitOnErr(evaluator_->main_jd().define(llvm::orc::absoluteSymbols(std::move(symbols))));
}

void Runtime::SetSourceFileName(const char* input) {
  compiler_->SetSourceFileName(input);
}

void Runtime::Eval(uintptr_t context) {
  auto mod = compiler_->TakeModule();
  // Create a ResourceTracker to track JIT'd memory allocated to our
  // anonymous expression -- that way we can free it after executing.
  auto tracker = evaluator_->main_jd().createResourceTracker();
  ExitOnErr(evaluator_->AddModule(std::move(mod), tracker));
  auto sym = ExitOnErr(evaluator_->Lookup("main"));
  int32_t (*func)(uintptr_t) = sym.getAddress().toPtr<int32_t (*)(uintptr_t)>();
  func(context);
  ExitOnErr(tracker->remove());
}

void Runtime::Call(uintptr_t context, const char* name, size_t name_len, double* return_value) {
  auto sym = ExitOnErr(evaluator_->Lookup({name, name_len}));
  double (*func)(uintptr_t) = sym.getAddress().toPtr<double (*)(uintptr_t)>();
  *return_value = func(context);
}

Compiler* Runtime::StartCompilation() {
  compiler_->StartMain();
  return compiler_.get();
}

void Runtime::PopulateModule(Compiler* compiler) {
  (void)compiler;
  compiler_->EndMain();
}

void Runtime::EndCompilation(Compiler* compiler) {
  // TODO
  (void)compiler;
}
