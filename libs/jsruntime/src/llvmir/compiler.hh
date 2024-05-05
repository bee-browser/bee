#pragma once

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <unordered_map>
#include <vector>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Analysis/CGSCCPassManager.h>
#include <llvm/Analysis/LoopAnalysisManager.h>
#include <llvm/IR/DataLayout.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/PassInstrumentation.h>
#include <llvm/IR/PassManager.h>
#include <llvm/Passes/StandardInstrumentations.h>
#pragma GCC diagnostic pop

#include "type_holder.hh"

class TypeHolder;
struct Module;

class Compiler {
 public:
  Compiler();
  ~Compiler() = default;

  void SetDataLayout(const char* data_layout);
  void SetTargetTriple(const char* triple);
  void SetSourceFileName(const char* input);
  Module* TakeModule();

  void Number(double value);
  void Function(uint32_t func_id);
  void Reference(uint32_t symbol, uint32_t locator);
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
  void DeclareImmutable();
  void DeclareMutable();
  void DeclareFunction();
  void GetReference();
  void Set();
  void PushArgument();
  void Call();
  void ToBoolean();
  void Block();
  void ConditionalExpression();
  void IfElseStatement();
  void IfStatement();
  void StartFunction(const char* name);
  void EndFunction(bool optimize = true);
  void AllocateBindings(uint16_t n, bool prologue);
  void ReleaseBindings(uint16_t n);
  void Return(size_t n);
  void Void();

  void DumpStack();

 private:
  struct Reference {
    uint32_t symbol;
    uint32_t locator;
    Reference(uint32_t symbol, uint32_t locator) : symbol(symbol), locator(locator) {}
  };

  struct Item {
    enum Type {
      Undefined,
      Boolean,
      Number,
      Function,
      Any,  // undefined, boolean, number or object.
      Reference,
      Block,
      ExecContext,
    } type;
    union {
      llvm::Value* value;
      struct Reference reference;
      llvm::BasicBlock* block;
    };

    explicit Item(Type type) : type(type), value(nullptr) {}
    Item(Type type, llvm::Value* value) : type(type), value(value) {}
    Item(uint32_t symbol, uint32_t locator) : type(Item::Reference), reference(symbol, locator) {}
    explicit Item(llvm::BasicBlock* block) : type(Item::Block), block(block) {}

    inline bool IsValue() const {
      switch (type) {
        case Item::Boolean:
        case Item::Number:
        case Item::Function:
        case Item::Any:
          return true;
        default:
          return false;
      }
    }
  };

  void CreateCallRuntimeDeclareImmutable(const struct LocalRef& ref, llvm::Value* value);
  void CreateCallRuntimeDeclareMutable(const struct LocalRef& ref, llvm::Value* value);
  void CreateCallRuntimeDeclareFunction(const struct LocalRef& ref, llvm::Value* value);
  void CreateCallRuntimeReturnValue(llvm::Value* value);
  void CreateCallRuntimeAllocateBindings(uint16_t n);
  void CreateCallRuntimeReleaseBindings(uint16_t n);
  void CreateCallRuntimeInspectNumber(llvm::Value* value);
  void CreateCallRuntimeInspect(llvm::Value* value);

  inline void PushUndefined() {
    stack_.push_back(Item(Item::Undefined));
  }

  inline void PushBoolean(llvm::Value* value) {
    stack_.push_back(Item(Item::Boolean, value));
  }

  inline void PushNumber(llvm::Value* value) {
    stack_.push_back(Item(Item::Number, value));
  }

  inline void PushFunction(llvm::Value* value) {
    stack_.push_back(Item(Item::Function, value));
  }

  inline void PushAny(llvm::Value* value) {
    stack_.push_back(Item(Item::Any, value));
  }

  inline void PushReference(uint32_t symbol, uint32_t locator) {
    stack_.push_back(Item(symbol, locator));
  }

  inline void PushBlock(llvm::BasicBlock* block) {
    stack_.push_back(Item(block));
  }

  inline void PushExecContext(llvm::Value* exec_context) {
    stack_.push_back(Item(Item::ExecContext, exec_context));
  }

  inline llvm::Value* exec_context() const {
    assert(!stack_.empty());
    const auto& item = stack_[base_index_];
    assert(item.type == Item::ExecContext);
    return item.value;
  }

  void Swap() {
    assert(stack_.size() >= 2);
    auto i = stack_.size() - 1;
    Item item = stack_[i];
    stack_[i] = stack_[i - 1];
    stack_[i - 1] = item;
  }

  inline Item PopItem() {
    assert(!stack_.empty());
    Item item = stack_.back();
    stack_.pop_back();
    return item;
  }

  inline llvm::Value* PopValue() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.IsValue());
    auto* value = item.value;
    stack_.pop_back();
    return value;
  }

  inline struct Reference PopReference() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Reference);
    auto reference = item.reference;
    stack_.pop_back();
    return reference;
  }

  inline llvm::BasicBlock* PopBlock() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Block);
    auto* block = item.block;
    stack_.pop_back();
    return block;
  }

  Item Dereference();
  llvm::Value* ToNumber(const Item& item);
  llvm::Value* ToAny(const Item& item);

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;
  // function-related data
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* prologue_ = nullptr;
  llvm::BasicBlock* body_ = nullptr;
  size_t scope_depth_ = 0;
  size_t base_index_ = 0;

  std::vector<Item> stack_;

  // for optimization
  std::unique_ptr<llvm::FunctionPassManager> fpm_;
  std::unique_ptr<llvm::LoopAnalysisManager> lam_;
  std::unique_ptr<llvm::FunctionAnalysisManager> fam_;
  std::unique_ptr<llvm::CGSCCAnalysisManager> cgam_;
  std::unique_ptr<llvm::ModuleAnalysisManager> mam_;
  std::unique_ptr<llvm::PassInstrumentationCallbacks> pic_;
  std::unique_ptr<llvm::StandardInstrumentations> si_;

  // TODO: At this point, we use a cache in order to remove redundant runtime function calls to
  // retrieve values of lexical bindings.  But it should be replaced with a LLVM-IR pass to do
  // that.
  std::unordered_map<uint32_t, llvm::Value*> reference_cache_;
};
