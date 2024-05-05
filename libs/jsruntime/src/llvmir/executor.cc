#include "executor.hh"

#include <memory>

#include "module.hh"

static llvm::ExitOnError ExitOnErr;

// static
llvm::Expected<Executor*> Executor::Create() {
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

  return new Executor(std::move(exec_session), std::move(jtmb), std::move(*data_layout));
}

Executor::Executor(std::unique_ptr<llvm::orc::ExecutionSession> exec_session,
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
      main_jd_(exec_session_->createBareJITDylib("<main>")),
      tracker_(main_jd().createResourceTracker()) {
  // FIXME: jtmb has already been moved...
  if (jtmb.getTargetTriple().isOSBinFormatCOFF()) {
    object_layer_.setOverrideObjectFlagsWithResponsibilityFlags(true);
    object_layer_.setAutoClaimResponsibilityForObjectSymbols(true);
  }
}

Executor::~Executor() {
  ExitOnErr(tracker_->remove());
  if (auto err = exec_session_->endSession()) {
    exec_session_->reportError(std::move(err));
  }
}

void Executor::RegisterModule(Module* mod) {
  ExitOnErr(compile_layer_.add(tracker_, std::move(mod->mod)));
}

FuncFn Executor::GetFunc(const char* name) {
  auto sym = ExitOnErr(Lookup(name));
  return sym.getAddress().toPtr<double (*)(void*)>();
}

llvm::Expected<llvm::orc::ExecutorSymbolDef> Executor::Lookup(llvm::StringRef name) {
  // Uncomment if you want to see the state of the exec_session_.
  // exec_session_->dump(llvm::errs());
  return exec_session_->lookup({&main_jd_}, mangle_(name.str()));
}
