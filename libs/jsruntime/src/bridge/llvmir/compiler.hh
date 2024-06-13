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

#include "../bridge.hh"
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
  void Null();
  void Boolean(bool value);
  void Number(double value);
  void Function(uint32_t func_id, const char* name);
  void Reference(uint32_t symbol, Locator locator);
  void Exception();
  void PostfixIncrement();
  void PostfixDecrement();
  void PrefixIncrement();
  void PrefixDecrement();
  void UnaryDelete();
  void Void();
  void Typeof();
  void UnaryPlus();
  void UnaryMinus();
  void BitwiseNot();
  void LogicalNot();
  void Exponentiation();
  void Multiplication();
  void Division();
  void Remainder();
  void Addition();
  void Subtraction();
  void LeftShift();
  void SignedRightShift();
  void UnsignedRightShift();
  void LessThan();
  void GreaterThan();
  void LessThanOrEqual();
  void GreaterThanOrEqual();
  void Instanceof();
  void In();
  void Equality();
  void Inequality();
  void StrictEquality();
  void StrictInequality();
  void BitwiseAnd();
  void BitwiseXor();
  void BitwiseOr();
  void ConditionalTernary();
  void Assignment();
  void ExponentiationAssignment();
  void MultiplicationAssignment();
  void DivisionAssignment();
  void RemainderAssignment();
  void AdditionAssignment();
  void SubtractionAssignment();
  void LeftShiftAssignment();
  void SignedRightShiftAssignment();
  void UnsignedRightShiftAssignment();
  void BitwiseAndAssignment();
  void BitwiseXorAssignment();
  void BitwiseOrAssignment();
  void Bindings(uint16_t n);
  void DeclareImmutable();
  void DeclareMutable();
  void DeclareFunction();
  void Arguments(uint16_t argc);
  void Argument(uint16_t index);
  void Call(uint16_t argc);
  void Truthy();
  void FalsyShortCircuit();
  void TruthyShortCircuit();
  void NullishShortCircuit();
  void FalsyShortCircuitAssignment();
  void TruthyShortCircuitAssignment();
  void NullishShortCircuitAssignment();
  void Block();
  void IfElseStatement();
  void IfStatement();
  void DoWhileLoop();
  void WhileLoop();
  void ForLoop(bool has_init, bool has_test, bool has_next);
  void LoopInit();
  void LoopTest();
  void LoopNext();
  void LoopBody();
  void LoopEnd();
  void CaseBlock(uint32_t n);
  void CaseClause(bool has_statement);
  void DefaultClause(bool has_statement);
  void Switch(uint32_t n, uint32_t default_index);
  void Try();
  void Catch(bool nominal);
  void Finally(bool nominal);
  void TryEnd();
  void StartFunction(const char* name);
  void EndFunction(bool optimize = true);
  void AllocateBindings(uint16_t n, bool prologue);
  void ReleaseBindings(uint16_t n);
  void Continue();
  void Break();
  void Return(size_t n);
  void Throw();
  void Discard();

  void DumpStack();

 private:
  struct Reference {
    uint32_t symbol;
    union {
      Locator locator;
      uint32_t opaque;  // used for comparing the locator value.
    };
    Reference() : symbol(0), locator() {}
    Reference(uint32_t symbol, Locator locator) : symbol(symbol), locator(locator) {}
  };

  struct Item {
    enum Type {
      Undefined,
      Null,
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
#ifdef BEE_BUILD_DEBUG
    const char* label = nullptr;
#endif

    explicit Item(Type type) : type(type), value(nullptr) {}
    explicit Item(llvm::Function* func) : type(Item::Function), func(func) {}
    Item(Type type, llvm::Value* value) : type(type), value(value) {}
    Item(uint32_t symbol, Locator locator) : type(Item::Reference), reference(symbol, locator) {}
    explicit Item(llvm::BasicBlock* block) : type(Item::Block), block(block) {}

    inline void SetLabel(const char* label) {
#ifdef BEE_BUILD_DEBUG
      this->label = label;
#endif
    }

    inline bool IsValue() const {
      switch (type) {
        case Item::Undefined:
        case Item::Null:
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

  inline void PushNull() {
    stack_.push_back(Item(Item::Null));
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

  inline void PushBlock(llvm::BasicBlock* block, const char* label) {
    Item item(block);
    item.SetLabel(label);
    stack_.push_back(item);
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

  inline llvm::Value* PopBoolean() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Boolean);
    auto* value = item.value;
    stack_.pop_back();
    return value;
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

  inline llvm::BasicBlock* PeekBlock() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Block);
    auto* block = item.block;
    return block;
  }

  inline void Duplicate() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    stack_.push_back(item);
  }

  Item Dereference(struct Reference* ref = nullptr, llvm::Value** scope = nullptr);
  void IncrDecr(char pos, char op);
  void NumberBitwiseOp(char op, llvm::Value* x, llvm::Value* y);
  llvm::Value* ToNumeric(const Item& item);
  llvm::Value* ToNumeric(llvm::Value* value_ptr);
  llvm::Value* ToInt32(llvm::Value* number);
  llvm::Value* ToUint32(llvm::Value* number);
  llvm::Value* ToAny(const Item& item);
  llvm::AllocaInst* CreateAllocaInEntryBlock(llvm::Type* ty, uint32_t n = 1);

  llvm::Value* CreateIsNonNullish(const Item& item);
  llvm::Value* CreateIsNonNullish(llvm::Value* value_ptr);

  llvm::Value* CreateToBoolean(const Item& item);
  llvm::Value* CreateToBoolean(llvm::Value* value_ptr);

  llvm::Value* CreateIsLooselyEqual(const Item& lhs, const Item& rhs);
  llvm::Value* CreateIsLooselyEqual(llvm::Value* value_ptr, const Item& item);
  llvm::Value* CreateIsLooselyEqual(llvm::Value* x, llvm::Value* y);

  llvm::Value* CreateIsStrictlyEqual(const Item& lhs, const Item& rhs);
  llvm::Value* CreateIsStrictlyEqual(llvm::Value* value_ptr, const Item& item);
  llvm::Value* CreateIsStrictlyEqual(llvm::Value* x, llvm::Value* y);
  llvm::Value* CreateIsUndefined(llvm::Value* value_ptr);
  llvm::Value* CreateIsNull(llvm::Value* value_ptr);
  llvm::Value* CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* value);
  llvm::Value* CreateIsSameNumberValue(llvm::Value* value_ptr, llvm::Value* value);
  llvm::Value* CreateIsSameFunctionValue(llvm::Value* value_ptr, llvm::Value* value);

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

  inline llvm::Value* CreateLoadArgvFromScope(llvm::Value* scope_ptr) {
    auto* ptr = CreateGetArgvPtrOfScope(scope_ptr);
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

  inline llvm::Value* CreateGetReservedPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 2);
  }

  inline llvm::Value* CreateGetSymbolPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 3);
  }

  inline llvm::Value* CreateGetValueHolderPtrOfBinding(llvm::Value* binding_ptr) {
    return builder_->CreateStructGEP(types_->CreateBindingType(), binding_ptr, 4);
  }

  inline void CreateStoreValueKindToBinding(ValueKind value, llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(builder_->getInt8(static_cast<uint8_t>(value)), binding_ptr);
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

  inline void CreateStoreNullToBinding(llvm::Value* binding_ptr) {
    CreateStoreValueKindToBinding(ValueKind::Null, binding_ptr);
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
    CreateStoreValueKindToBinding(ValueKind::Function, binding_ptr);
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

  inline llvm::Value* CreateLoadValueKindFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueKindPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt8Ty(), ptr);
  }

  inline llvm::Value* CreateLoadBooleanFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt1Ty(), ptr);
  }

  inline llvm::Value* CreateLoadNumberFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getDoubleTy(), ptr);
  }

  inline llvm::Value* CreateLoadFunctionFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr);
  }

  inline llvm::Value* CreateLoadValue(llvm::Value* value_ptr) {
    return builder_->CreateLoad(types_->CreateValueType(), value_ptr);
  }

  inline void CreateStoreValueKindToValue(ValueKind value, llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(builder_->getInt8(static_cast<uint8_t>(value)), value_ptr);
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

  inline void CreateStoreNullToValue(llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(ValueKind::Null, value_ptr);
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
    CreateStoreValueKindToValue(ValueKind::Function, value_ptr);
    CreateStoreValueHolderToValue(value, value_ptr);
  }

  void CreateStoreItemToValue(const Item& item, llvm::Value* value_ptr);

  // FIXME: Handle dead code in the proper way.
  //
  // We insert a **unreachable** basic block for dead code in order to avoid the following
  // validation error: "Terminator found in the middle of a basic block!"
  //
  // IRBuilder accepts inserting instructions after a terminator instruction in a basic block.
  // It's our responsibility to avoid a malformed basic block.  We think that it's not a good
  // direction to check the existence of a terminator instruction in a basic block before
  // insertion in efficiency and maintainability points of view.  Instead, we create an
  // **unreachable** basic block for dead code.  Eventually, this basic block was removed in the
  // optimization passes.
  //
  // At this point, we don't know whether this is a common method or not...
  inline void CreateDeadcodeBasicBlock() {
    auto* dummy = llvm::BasicBlock::Create(*context_, "deadcode", function_);
    builder_->SetInsertPoint(dummy);
  }

  // TODO: separate variables that must be reset in EndFunction() from others.

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;
  // function-related data
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* prologue_ = nullptr;
  llvm::BasicBlock* body_ = nullptr;
  llvm::BasicBlock* epilogue_ = nullptr;
  llvm::Value* exec_context_ = nullptr;
  llvm::Value* outer_scope_ = nullptr;
  llvm::Value* argc_ = nullptr;
  llvm::Value* argv_ = nullptr;
  llvm::Value* ret_ = nullptr;
  llvm::Value* status_ = nullptr;
  llvm::Type* bindings_type_ = nullptr;
  llvm::StructType* function_scope_type_ = nullptr;
  llvm::Value* function_scope_ = nullptr;
  llvm::Value* bindings_ = nullptr;
  uint16_t max_bindings_ = 0;
  uint16_t allocated_bindings_ = 0;

  std::vector<Item> stack_;
  std::vector<llvm::BasicBlock*> break_stack_;
  std::vector<llvm::BasicBlock*> continue_stack_;
  std::vector<llvm::BasicBlock*> catch_stack_;

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
