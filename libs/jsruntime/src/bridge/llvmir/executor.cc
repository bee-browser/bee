#include "executor.hh"

#include <memory>

#include "module.hh"

static llvm::ExitOnError ExitOnErr;

// static
llvm::Expected<Executor*> Executor::Create() {
  auto jit = ExitOnErr(llvm::orc::LLJITBuilder().create());
  return new Executor(std::move(jit));
}

void Executor::RegisterHostFunction(const char* name, Lambda lambda) {
  llvm::orc::SymbolMap symbols;
  symbols[exec_session().intern(name)] = {
      llvm::orc::ExecutorAddr::fromPtr(lambda),
      llvm::JITSymbolFlags::Exported,
  };
  ExitOnErr(main_jd().define(llvm::orc::absoluteSymbols(std::move(symbols))));
}

void Executor::RegisterModule(Module* mod) {
  ExitOnErr(jit_->addIRModule(std::move(mod->mod)));
}

Lambda Executor::GetNativeFunction(const char* name) {
  auto addr = ExitOnErr(jit_->lookup(name));
  return addr.toPtr<Lambda>();
}
