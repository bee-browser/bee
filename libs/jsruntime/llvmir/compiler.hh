#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <unordered_map>
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
  void Symbol(uint32_t symbol_id);
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
  void Get();
  void Set();
  void Declare();
  void SetUndefined();
  void Call(size_t argc);
  void ToBoolean();
  void Block();
  void ConditionalExpression();
  void IfElseStatement();
  void IfStatement();
  void StartFunction(const char* name, size_t len);
  void EndFunction();
  void StartScope();
  void EndScope();
  void Return(size_t n);
  void Print();

  void DumpStack();
  void DumpModule();
  llvm::orc::ThreadSafeModule TakeModule();

 private:
  struct Item {
    enum Type {
      Value,
      Symbol,
      Block,
      Function,
      Index,
    } type;
    union {
      llvm::Value* value;
      llvm::Value* symbol;
      llvm::BasicBlock* block;
      llvm::Function* function;
      size_t index;
    } data;
  };

  llvm::Function* CreateMainFunction();
  llvm::Function* CreatePrintStrFunction();
  llvm::Function* CreatePrintBoolFunction();
  llvm::Function* CreatePrintF64Function();
  llvm::Function* CreateRuntimeGet();
  llvm::Function* CreateRuntimeSet();
  llvm::Function* CreateRuntimeDeclare();
  llvm::Function* CreateRuntimeSetUndefined();
  llvm::Function* CreateRuntimeCall();
  llvm::Function* CreateRuntimePushScope();
  llvm::Function* CreateRuntimePopScope();

  inline void PushValue(llvm::Value* value) {
    stack_.push_back({Item::Value, value});
  }

  inline void PushSymbol(llvm::Value* symbol) {
    stack_.push_back({Item::Symbol, symbol});
  }

  inline void PushBlock(llvm::BasicBlock* block) {
    stack_.push_back({Item::Block, block});
  }

  inline void PushFunction(llvm::Function* function) {
    stack_.push_back({Item::Function, function});
  }

  inline void PushIndex(size_t index) {
    // TODO: initializer list
    stack_.push_back({Item::Index, nullptr});
    stack_.back().data.index = index;
  }

  inline llvm::Value* exec_context() const {
    assert(!stack_.empty());
    const auto& item = stack_[base_index_];
    assert(item.type == Item::Value);
    return item.data.value;
  }

  inline llvm::Value* PopValue() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Value);
    auto* value = item.data.value;
    stack_.pop_back();
    return value;
  }

  inline llvm::Value* PopSymbol() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Symbol);
    auto* symbol = item.data.symbol;
    stack_.pop_back();
    return symbol;
  }

  inline llvm::BasicBlock* PopBlock() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Block);
    auto* block = item.data.block;
    stack_.pop_back();
    return block;
  }

  inline llvm::Value* Dereference() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    if (item.type == Item::Symbol) {
      Get();
    }
    return PopValue();
  }

  inline llvm::Function* PopFunction() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Function);
    auto* function = item.data.function;
    stack_.pop_back();
    return function;
  }

  size_t PopIndex() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Index);
    auto index = item.data.index;
    stack_.pop_back();
    return index;
  }

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  // function-related data
  llvm::Function* function_ = nullptr;
  size_t scope_depth_ = 0;
  size_t base_index_ = 0;

  void PushFunctionData() {
    PushFunction(function_);
    function_ = nullptr;
    PushIndex(scope_depth_);
    scope_depth_ = 0;
    PushIndex(base_index_);
    base_index_ = stack_.size();
  }

  void PopFunctionData() {
    base_index_ = PopIndex();
    scope_depth_ = PopIndex();
    function_ = PopFunction();
  }

  std::vector<Item> stack_;
};
