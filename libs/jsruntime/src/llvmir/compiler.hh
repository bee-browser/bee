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

#include "bridge.hh"
#include "type_holder.hh"

class TypeHolder;
struct Module;

class Compiler {
 public:
  Compiler();
  ~Compiler() = default;

  Module* TakeModule();

  void SetSourceFileName(const char* input);
  void SetDataLayout(const char* data_layout);
  void SetTargetTriple(const char* triple);
  void SetRuntime(uintptr_t runtime);

  void Undefined();
  void Boolean(bool value);
  void Number(double value);
  void Function(uint32_t func_id, const char* name);
  void Reference(uint32_t symbol, Locator locator);
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
  void Bindings(uint16_t n);
  void DeclareImmutable();
  void DeclareMutable();
  void DeclareFunction();
  void Set();
  void Arguments(uint16_t argc);
  void Argument(uint16_t index);
  void Call(uint16_t argc);
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
    Locator locator;
    Reference(uint32_t symbol, Locator locator) : symbol(symbol), locator(locator) {}
  };

  struct Item {
    enum Type {
      Undefined,
      Boolean,
      Number,
      Function,
      Any,  // undefined, boolean, number or object.
      Reference,
      Argv,
      Block,
    } type;
    union {
      llvm::Value* value;
      llvm::Function* func;
      struct Reference reference;
      llvm::BasicBlock* block;
    };

    explicit Item(Type type) : type(type), value(nullptr) {}
    explicit Item(llvm::Function* func) : type(Item::Function), func(func) {}
    Item(Type type, llvm::Value* value) : type(type), value(value) {}
    Item(uint32_t symbol, Locator locator) : type(Item::Reference), reference(symbol, locator) {}
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

  inline void PushUndefined() {
    stack_.push_back(Item(Item::Undefined));
  }

  inline void PushBoolean(llvm::Value* value) {
    stack_.push_back(Item(Item::Boolean, value));
  }

  inline void PushNumber(llvm::Value* value) {
    stack_.push_back(Item(Item::Number, value));
  }

  inline void PushFunction(llvm::Function* func) {
    stack_.push_back(Item(Item::Function, func));
  }

  inline void PushAny(llvm::Value* value) {
    stack_.push_back(Item(Item::Any, value));
  }

  inline void PushReference(uint32_t symbol, Locator locator) {
    stack_.push_back(Item(symbol, locator));
  }

  inline void PushArgv(llvm::Value* value) {
    stack_.push_back(Item(Item::Argv, value));
  }

  inline void PushBlock(llvm::BasicBlock* block) {
    stack_.push_back(Item(block));
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

  inline llvm::Function* PopFunction() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Function);
    auto* func = item.func;
    stack_.pop_back();
    return func;
  }

  inline struct Reference PopReference() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Reference);
    auto reference = item.reference;
    stack_.pop_back();
    return reference;
  }

  inline llvm::Value* PopArgv() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Argv);
    auto* argv = item.value;
    stack_.pop_back();
    return argv;
  }

  inline llvm::BasicBlock* PopBlock() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Block);
    auto* block = item.block;
    stack_.pop_back();
    return block;
  }

  Item Dereference(llvm::Value** scope = nullptr);
  llvm::Value* ToNumeric(const Item& item);
  llvm::Value* ToAny(const Item& item);

  // Naming convention for field accessors:
  //
  //   CreateGet<field>PtrOf<type>(ptr)
  //     Create instructions to get a pointer of the <field> of <type>.
  //
  //   CreateExtract<field>From<type>(value)
  //     Create instructions to extract the value of the <field> from a value of <type>.
  //
  //   CreateLoad<type>(ptr)
  //     Create instructions to load the value.
  //
  //   CreateLoad<field>From<type>(ptr)
  //     Create instructions to load the value of the <field> of <type>.
  //
  //   CreateStore<field>To<type>(value, ptr)
  //     Create instructions to store a value to the <field> of <type>.

  // function scope

  llvm::Value* CreateGetScope(const Locator& locator);

  inline llvm::Value* CreateGetOuterScopePtrOfScope(llvm::Value* scope_ptr) {
    return builder_->CreateStructGEP(function_scope_type_, scope_ptr, 0);
  }

  inline llvm::Value* CreateGetArgcPtrOfScope(llvm::Value* scope_ptr) {
    return builder_->CreateStructGEP(function_scope_type_, scope_ptr, 1);
  }

  inline llvm::Value* CreateGetArgvPtrOfScope(llvm::Value* scope_ptr) {
    return builder_->CreateStructGEP(function_scope_type_, scope_ptr, 2);
  }

  inline llvm::Value* CreateGetBindingsPtrOfScope(llvm::Value* scope_ptr) {
    return builder_->CreateStructGEP(function_scope_type_, scope_ptr, 3);
  }

  inline llvm::Value* CreateGetBindingPtrOfScope(llvm::Value* scope_ptr, uint16_t index) {
    auto* ptr = CreateGetBindingsPtrOfScope(scope_ptr);
    return builder_->CreateConstInBoundsGEP2_32(bindings_type_, ptr, 0, index);
  }

  inline llvm::Value* CreateLoadOuterScopeFromScope(llvm::Value* scope_ptr) {
    auto* ptr = CreateGetOuterScopePtrOfScope(scope_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr);
  }

  inline void CreateStoreOuterScopeToScope(llvm::Value* value, llvm::Value* scope_ptr) {
    auto* ptr = CreateGetOuterScopePtrOfScope(scope_ptr);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreArgcToScope(llvm::Value* value, llvm::Value* scope_ptr) {
    auto* ptr = CreateGetArgcPtrOfScope(scope_ptr);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreArgvToScope(llvm::Value* value, llvm::Value* scope_ptr) {
    auto* ptr = CreateGetArgvPtrOfScope(scope_ptr);
    builder_->CreateStore(value, ptr);
  }

  // bindings

  inline llvm::Value* CreateGetBindingPtr(const Locator& locator) {
    if (locator.offset == 0) {
      return builder_->CreateConstInBoundsGEP2_32(bindings_type_, bindings_, 0, locator.index);
    }
    auto* scope_ptr = CreateGetScope(locator);
    return CreateGetBindingPtrOfScope(scope_ptr, locator.index);
  }

  inline llvm::Value* CreateGetValueKindPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 0);
  }

  inline llvm::Value* CreateGetFlagsPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 1);
  }

  inline llvm::Value* CreateGetSymbolPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 2);
  }

  inline llvm::Value* CreateGetValueHolderPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 3);
  }

  inline void CreateStoreValueKindToBinding(ValueKind value, llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(builder_->getInt8(value), binding_ptr);
  }

  inline void CreateStoreValueKindToBinding(llvm::Value* value, llvm::Value* binding_ptr) {
    auto* ptr = CreateGetValueKindPtrOfBinding(binding_ptr);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreFlagsToBinding(uint8_t value, llvm::Value* binding_ptr) {
    auto* ptr = CreateGetFlagsPtrOfBinding(binding_ptr);
    builder_->CreateStore(builder_->getInt8(value), ptr);
  }

  inline void CreateStoreSymbolToBinding(uint32_t value, llvm::Value* binding_ptr) {
    auto* ptr = CreateGetSymbolPtrOfBinding(binding_ptr);
    builder_->CreateStore(builder_->getInt32(value), ptr);
  }

  inline void CreateStoreValueHolderToBinding(llvm::Value* holder, llvm::Value* binding_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfBinding(binding_ptr);
    builder_->CreateStore(holder, ptr);
  }

  inline void CreateStoreUndefinedToBinding(llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(ValueKind::Undefined, binding_ptr);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToBinding(builder_->getInt64(0), binding_ptr);
  }

  inline void CreateStoreBooleanToBinding(llvm::Value* value, llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(ValueKind::Boolean, binding_ptr);
    CreateStoreValueHolderToBinding(value, binding_ptr);
  }

  inline void CreateStoreNumberToBinding(llvm::Value* value, llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(ValueKind::Number, binding_ptr);
    CreateStoreValueHolderToBinding(value, binding_ptr);
  }

  inline void CreateStoreFunctionToBinding(llvm::Value* value, llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(ValueKind::Closure, binding_ptr);
    CreateStoreValueHolderToBinding(value, binding_ptr);
  }

  inline void CreateStoreValueToBinding(llvm::Value* value_ptr, llvm::Value* binding_ptr) {
    auto* value = CreateLoadValue(value_ptr);
    auto* kind = CreateExtractValueKindFromValue(value);
    auto* holder = CreateExtractValueHolderFromValue(value);
    CreateStoreValueKindToBinding(kind, binding_ptr);
    CreateStoreValueHolderToBinding(holder, binding_ptr);
  }

  void CreateStoreItemToBinding(const Item& item, llvm::Value* binding_ptr);

  // value

  inline llvm::Value* CreateGetValueKindPtrOfValue(llvm::Value* value_ptr) {
    return builder_->CreateStructGEP(types_->CreateValueType(), value_ptr, 0);
  }

  inline llvm::Value* CreateGetValueHolderPtrOfValue(llvm::Value* value_ptr) {
    return builder_->CreateStructGEP(types_->CreateValueType(), value_ptr, 1);
  }

  inline llvm::Value* CreateExtractValueKindFromValue(llvm::Value* value) {
    return builder_->CreateExtractValue(value, 0);
  }

  inline llvm::Value* CreateExtractValueHolderFromValue(llvm::Value* value) {
    return builder_->CreateExtractValue(value, 1);
  }

  inline llvm::Value* CreateLoadValue(llvm::Value* value_ptr) {
    return builder_->CreateLoad(types_->CreateValueType(), value_ptr);
  }

  inline void CreateStoreValueKindToValue(ValueKind value, llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(builder_->getInt8(value), value_ptr);
  }

  inline void CreateStoreValueKindToValue(llvm::Value* value, llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueKindPtrOfValue(value_ptr);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreValueHolderToValue(llvm::Value* value, llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreUndefinedToValue(llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(ValueKind::Undefined, value_ptr);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), value_ptr);
  }

  inline void CreateStoreBooleanToValue(llvm::Value* value, llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(ValueKind::Boolean, value_ptr);
    CreateStoreValueHolderToValue(value, value_ptr);
  }

  inline void CreateStoreNumberToValue(llvm::Value* value, llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(ValueKind::Number, value_ptr);
    CreateStoreValueHolderToValue(value, value_ptr);
  }

  inline void CreateStoreFunctionToValue(llvm::Value* value, llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(ValueKind::Closure, value_ptr);
    CreateStoreValueHolderToValue(value, value_ptr);
  }

  void CreateStoreItemToValue(const Item& item, llvm::Value* value_ptr);

  llvm::AllocaInst* CreateAllocaInEntryBlock(llvm::Type* ty, uint32_t n = 1);

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;
  // function-related data
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* prologue_ = nullptr;
  llvm::BasicBlock* body_ = nullptr;
  llvm::Value* exec_context_ = nullptr;
  llvm::Value* outer_scope_ = nullptr;
  llvm::Value* argc_ = nullptr;
  llvm::Value* argv_ = nullptr;
  llvm::Type* bindings_type_ = nullptr;
  llvm::StructType* function_scope_type_ = nullptr;
  llvm::Value* function_scope_ = nullptr;
  llvm::Value* bindings_ = nullptr;
  size_t scope_depth_ = 0;
  size_t base_index_ = 0;

  std::vector<Item> stack_;
  std::unordered_map<std::string, llvm::Function*> functions_;

  // for optimization
  std::unique_ptr<llvm::FunctionPassManager> fpm_;
  std::unique_ptr<llvm::LoopAnalysisManager> lam_;
  std::unique_ptr<llvm::FunctionAnalysisManager> fam_;
  std::unique_ptr<llvm::CGSCCAnalysisManager> cgam_;
  std::unique_ptr<llvm::ModuleAnalysisManager> mam_;
  std::unique_ptr<llvm::PassInstrumentationCallbacks> pic_;
  std::unique_ptr<llvm::StandardInstrumentations> si_;
};
