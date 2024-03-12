#pragma once

#include <cstddef>
#include <cstdint>
#include <memory>
#include <unordered_map>
#include <vector>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include "llvm/IR/DataLayout.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#pragma GCC diagnostic pop

struct Module;

class Compiler {
 public:
  Compiler();
  ~Compiler() = default;

  void SetSourceFileName(const char* input);

  void StartMain();
  void EndMain();
  Module* TakeModule();

  void Number(double value);
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
  void DeclareConst();
  void DeclareVariable();
  void DeclareUndefined();
  void DeclareFunction(uint32_t symbol_id, uint32_t func_id);
  void Get();
  void Set();
  void SetUndefined();
  void PushArgs();
  void PushArg();
  void Call();
  void ToBoolean();
  void Block();
  void ConditionalExpression();
  void IfElseStatement();
  void IfStatement();
  void StartFunction(const char* name);
  void EndFunction();
  void StartScope();
  void EndScope();
  void Return(size_t n);

  void DumpStack();

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
  llvm::Function* CreateRuntimeDeclareConst();
  llvm::Function* CreateRuntimeDeclareVariable();
  llvm::Function* CreateRuntimeDeclareUndefined();
  llvm::Function* CreateRuntimeDeclareFunction();
  llvm::Function* CreateRuntimeGet();
  llvm::Function* CreateRuntimeSet();
  llvm::Function* CreateRuntimeSetUndefined();
  llvm::Function* CreateRuntimePushArgs();
  llvm::Function* CreateRuntimePushArg();
  llvm::Function* CreateRuntimeCall();
  llvm::Function* CreateRuntimeRet();
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

  void Swap() {
    assert(stack_.size() >= 2);
    auto i = stack_.size() - 1;
    Item item = stack_[i];
    stack_[i] = stack_[i - 1];
    stack_[i - 1] = item;
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
  llvm::BasicBlock* prologue_ = nullptr;
  llvm::BasicBlock* body_ = nullptr;
  size_t scope_depth_ = 0;
  size_t base_index_ = 0;

  void PushFunctionData() {
    PushFunction(function_);
    function_ = nullptr;
    PushBlock(prologue_);
    prologue_ = nullptr;
    PushBlock(body_);
    body_ = nullptr;
    PushIndex(scope_depth_);
    scope_depth_ = 0;
    PushIndex(base_index_);
    base_index_ = stack_.size();
  }

  void PopFunctionData() {
    base_index_ = PopIndex();
    scope_depth_ = PopIndex();
    body_ = PopBlock();
    prologue_ = PopBlock();
    function_ = PopFunction();
  }

  std::vector<Item> stack_;

  // caches for host functions.
  llvm::Function* runtime_declare_const_ = nullptr;
  llvm::Function* runtime_declare_variable_ = nullptr;
  llvm::Function* runtime_declare_undefined_ = nullptr;
  llvm::Function* runtime_declare_function_ = nullptr;
  llvm::Function* runtime_get_ = nullptr;
  llvm::Function* runtime_set_ = nullptr;
  llvm::Function* runtime_set_undefined_ = nullptr;
  llvm::Function* runtime_set_push_args_ = nullptr;
  llvm::Function* runtime_set_push_arg_ = nullptr;
  llvm::Function* runtime_call_ = nullptr;
  llvm::Function* runtime_ret_ = nullptr;
  llvm::Function* runtime_push_scope_ = nullptr;
  llvm::Function* runtime_pop_scope_ = nullptr;
};
