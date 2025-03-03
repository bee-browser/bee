#include <llvm/Support/TargetSelect.h>

#include "bridge.hh"

void llvmir_initialize() {
  // Uncomment if you want to enable LLVM_DEBUG().
  // llvm::DebugFlag = true;

  llvm::InitializeNativeTarget();
  llvm::InitializeNativeTargetAsmPrinter();
  llvm::InitializeNativeTargetAsmParser();
}
