#include "runtime.hh"

#include <cstdint>
#include <iostream>

#include "llvm/Support/TargetSelect.h"

#include "host.hh"

static llvm::ExitOnError ExitOnErr;

// static
void Runtime::Initialize() {
  llvm::InitializeNativeTarget();
  llvm::InitializeNativeTargetAsmPrinter();
  llvm::InitializeNativeTargetAsmParser();
}

Runtime::Runtime() {
  evaluator_ = llvm::cantFail(Evaluator::Create());
  compiler_ = std::make_unique<Compiler>(evaluator_->data_layout());
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
  symbols[exec_session.intern("runtime_set_undefined")] = {
    llvm::orc::ExecutorAddr::fromPtr(host->runtime_set_undefined),
    llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session.intern("runtime_call")] = {
    llvm::orc::ExecutorAddr::fromPtr(host->runtime_call),
    llvm::JITSymbolFlags::Exported,
  };
  ExitOnErr(evaluator_->main_jd().define(llvm::orc::absoluteSymbols(std::move(symbols))));
}

void Runtime::SetSourceFileName(const char* input) {
  compiler_->SetSourceFileName(input);
}

void Runtime::DumpModule() {
  compiler_->DumpModule();
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

void Runtime::Call(const char* name, size_t name_len, double* return_value) {
  auto sym = ExitOnErr(evaluator_->Lookup({name, name_len}));
  double (*func)() = sym.getAddress().toPtr<double (*)()>();
  *return_value = func();
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
