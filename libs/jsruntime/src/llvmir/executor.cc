#include "executor.hh"

#include <memory>
#include <sstream>

#include "module.hh"

namespace {

// TODO: inefficient.  use a fixed size buffer for formatting func_id.
std::string FuncIdToName(uint32_t func_id) {
  std::stringstream ss;
  ss << "fn" << func_id;
  return ss.str();
}

}  // namespace

static llvm::ExitOnError ExitOnErr;

// static
llvm::Expected<Executor*> Executor::Create() {
  auto jit = ExitOnErr(llvm::orc::LLJITBuilder().create());
  return new Executor(std::move(jit));
}

void Executor::RegisterHostFunction(uint32_t func_id, Lambda lambda) {
  llvm::orc::SymbolMap symbols;
  auto name = FuncIdToName(func_id);
  symbols[exec_session().intern(name)] = {
      llvm::orc::ExecutorAddr::fromPtr(lambda),
      llvm::JITSymbolFlags::Exported,
  };
  ExitOnErr(main_jd().define(llvm::orc::absoluteSymbols(std::move(symbols))));
}

void Executor::RegisterModule(Module* mod) {
  ExitOnErr(jit_->addIRModule(std::move(mod->mod)));
}

Lambda Executor::GetNativeFunction(uint32_t func_id) {
  auto name = FuncIdToName(func_id);
  auto addr = ExitOnErr(jit_->lookup(name));
  return addr.toPtr<Lambda>();
}
