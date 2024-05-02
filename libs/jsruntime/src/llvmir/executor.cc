#include "executor.hh"

#include <memory>

#include "module.hh"
#include "runtime.hh"

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

void Executor::RegisterRuntime(const Runtime* runtime) {
  // register runtime functions.
  llvm::orc::SymbolMap symbols;
  symbols[exec_session().intern("runtime_declare_const")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->declare_const),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_declare_variable")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->declare_variable),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_declare_function")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->declare_function),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_get_argument")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->get_argument),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_get_local")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->get_local),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_put_argument")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->put_argument),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_put_local")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->put_local),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_push_arg")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->push_arg),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_call")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->call),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_ret")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->ret),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_push_scope")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->push_scope),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_pop_scope")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->pop_scope),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_inspect_number")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->inspect_number),
      llvm::JITSymbolFlags::Exported,
  };
  symbols[exec_session().intern("runtime_inspect_any")] = {
      llvm::orc::ExecutorAddr::fromPtr(runtime->inspect_any),
      llvm::JITSymbolFlags::Exported,
  };
  ExitOnErr(main_jd().define(llvm::orc::absoluteSymbols(std::move(symbols))));
}

void Executor::RegisterModule(Module* mod) {
  ExitOnErr(compile_layer_.add(tracker_, std::move(mod->mod)));
}

FuncFn Executor::GetFunc(const char* name) {
  auto sym = ExitOnErr(Lookup(name));
  return sym.getAddress().toPtr<double (*)(void*)>();
}

llvm::Expected<llvm::orc::ExecutorSymbolDef> Executor::Lookup(llvm::StringRef name) {
  // Uncomment if you want to show the state of the exec_session_.
  // exec_session_->dump(llvm::errs());
  return exec_session_->lookup({&main_jd_}, mangle_(name.str()));
}
