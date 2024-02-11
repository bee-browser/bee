#pragma once

#include <cstddef>
#include <memory>
#include <vector>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include "llvm/ExecutionEngine/Orc/ThreadSafeModule.h"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#pragma GCC diagnostic pop

class Compiler {
 public:
  explicit Compiler(const llvm::DataLayout& data_layout);
  ~Compiler() = default;

  void SetSourceFileName(const char* input);

  void StartMain();
  void EndMain();
  void Number(double value);
  void String(const char* data, size_t size);
  void Add();
  void Sub();
  void Mul();
  void Div();
  void Rem();
  void Lt();
  void Gt();
  void Lte();
  void Gte();
  void Eq();
  void Ne();
  void Print();

  void DumpModule();
  llvm::orc::ThreadSafeModule TakeModule();

 private:
  void CompileHelloWorld();

  llvm::Function* CreateMainFunction();
  llvm::Function* CreatePrintStrFunction();
  llvm::Function* CreatePrintBoolFunction();
  llvm::Function* CreatePrintF64Function();

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::vector<llvm::Value*> stack_;
};
