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
#include "macros.hh"
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
  void Closure(bool prologue, uint16_t num_captures);
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
  void DeclareImmutable();
  void DeclareMutable();
  void DeclareFunction();
  void DeclareClosure();
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
  void StartScope();
  void EndScope();
  void AllocateLocals(uint16_t num_locals);
  void ReleaseLocals(uint16_t num_locals);
  void CreateCapture(Locator locator);
  void CaptureVariable(bool declaration);
  void EscapeVariable(Locator locator);
  void LabelStart(uint32_t symbol, bool is_iteration_statement);
  void LabelEnd(uint32_t symbol, bool is_iteration_statement);
  void Continue(uint32_t symbol);
  void Break(uint32_t symbol);
  void Return(size_t n);
  void Throw();
  void Discard();
  void Swap();

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
      Closure,
      Any,  // undefined, boolean, number or object.
      Reference,
      Argv,
      Block,
      Capture,
    } type;
    union {
      llvm::Value* value;
      llvm::Function* func;
      struct Reference reference;
      llvm::BasicBlock* block;
    };
#if defined(BEE_BUILD_DEBUG)
    const char* label = nullptr;
#endif

    explicit Item(Type type) : type(type), value(nullptr) {}
    explicit Item(llvm::Function* func) : type(Item::Function), func(func) {}
    Item(Type type, llvm::Value* value) : type(type), value(value) {}
    Item(uint32_t symbol, Locator locator) : type(Item::Reference), reference(symbol, locator) {}
    explicit Item(llvm::BasicBlock* block) : type(Item::Block), block(block) {}

    inline void SetLabel(const char* label) {
#if defined(BEE_BUILD_DEBUG)
      this->label = label;
#else
      UNUSED(label);
#endif
    }

    inline bool IsValue() const {
      switch (type) {
        case Item::Undefined:
        case Item::Null:
        case Item::Boolean:
        case Item::Number:
        case Item::Function:
        case Item::Closure:
        case Item::Any:
          return true;
        default:
          return false;
      }
    }
  };

  struct BlockItem {
    llvm::BasicBlock* block;
    uint32_t symbol;
  };

  struct ScopeItem {
    llvm::BasicBlock* init_block;
    llvm::BasicBlock* decl_block;
    llvm::BasicBlock* stmt_block;
    llvm::BasicBlock* tidy_block;
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

  inline void PushClosure(llvm::Value* value) {
    stack_.push_back(Item(Item::Closure, value));
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

  inline void PushCapture(llvm::Value* value) {
    stack_.push_back(Item(Item::Capture, value));
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

  inline llvm::Value* PopCapture() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    assert(item.type == Item::Capture);
    auto* capture_ptr = item.value;
    stack_.pop_back();
    return capture_ptr;
  }

  inline void Duplicate() {
    assert(!stack_.empty());
    const auto& item = stack_.back();
    stack_.push_back(item);
  }

  Item Dereference(struct Reference* ref = nullptr);
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
  llvm::Value* CreateIsSameClosureValue(llvm::Value* value_ptr, llvm::Value* value);

  llvm::Value* CreateCallRuntimeCreateCapture(llvm::Value* variable_ptr);
  llvm::Value* CreateCallRuntimeCreateClosure(llvm::Value* lambda, uint16_t num_captures);

  llvm::Value* CreateGetVariablePtr(Locator locator);
  llvm::Value* CreateGetValuePtr(Locator locator);

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

  // arguments

  inline llvm::Value* CreateGetArgumentVariablePtr(uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateVariableType(), argv_, index);
  }

  inline llvm::Value* CreateGetArgumentValuePtr(uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateValueType(), argv_, index);
  }

  // locals

  inline llvm::Value* CreateGetLocalVariablePtr(uint16_t index) {
    assert(index < locals_.size());
    return locals_[index];
  }

  inline llvm::Value* CreateGetLocalValuePtr(uint16_t index) {
    return CreateGetLocalVariablePtr(index);
  }

  // captures

  inline llvm::Value* CreateGetCaptureVariablePtr(uint16_t index) {
    auto* ptr = CreateLoadCapturePtrFromCaptures(caps_, index);
    return CreateLoadTargetFromCapture(ptr);
  }

  inline llvm::Value* CreateGetCaptureValuePtr(uint16_t index) {
    auto* ptr = CreateLoadCapturePtrFromCaptures(caps_, index);
    return CreateLoadTargetFromCapture(ptr);
  }

  inline llvm::Value* CreateGetCapturePtrPtrOfCaptures(llvm::Value* captures, uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(builder_->getPtrTy(), captures, index);
  }

  inline llvm::Value* CreateLoadCapturePtrFromCaptures(llvm::Value* captures, uint16_t index) {
    auto* ptr = CreateGetCapturePtrPtrOfCaptures(captures, index);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr);
  }

  inline void CreateStoreCapturePtrToCaptures(llvm::Value* capture_ptr,
      llvm::Value* captures,
      uint16_t index) {
    auto* ptr = CreateGetCapturePtrPtrOfCaptures(captures, index);
    builder_->CreateStore(capture_ptr, ptr);
  }

  // variable

  inline llvm::Value* CreateGetValueKindPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(types_->CreateVariableType(), variable_ptr, 0);
  }

  inline llvm::Value* CreateGetFlagsPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(types_->CreateVariableType(), variable_ptr, 1);
  }

  inline llvm::Value* CreateGetReservedPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(types_->CreateVariableType(), variable_ptr, 2);
  }

  inline llvm::Value* CreateGetSymbolPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(types_->CreateVariableType(), variable_ptr, 3);
  }

  inline llvm::Value* CreateGetValueHolderPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(types_->CreateVariableType(), variable_ptr, 4);
  }

  inline llvm::Value* CreateExtractValueKindFromVariable(llvm::Value* variable) {
    return builder_->CreateExtractValue(variable, 0);
  }

  inline llvm::Value* CreateExtractValueHolderFromVariable(llvm::Value* variable) {
    return builder_->CreateExtractValue(variable, 4);
  }

  inline llvm::Value* CreateLoadVariable(llvm::Value* variable_ptr) {
    return builder_->CreateLoad(types_->CreateVariableType(), variable_ptr);
  }

  inline void CreateStoreValueKindToVariable(ValueKind value, llvm::Value* variable_ptr) {
    CreateStoreValueKindToVariable(builder_->getInt8(static_cast<uint8_t>(value)), variable_ptr);
  }

  inline void CreateStoreValueKindToVariable(llvm::Value* value, llvm::Value* variable_ptr) {
    auto* ptr = CreateGetValueKindPtrOfVariable(variable_ptr);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreFlagsToVariable(uint8_t value, llvm::Value* variable_ptr) {
    auto* ptr = CreateGetFlagsPtrOfVariable(variable_ptr);
    builder_->CreateStore(builder_->getInt8(value), ptr);
  }

  inline void CreateStoreSymbolToVariable(uint32_t value, llvm::Value* variable_ptr) {
    auto* ptr = CreateGetSymbolPtrOfVariable(variable_ptr);
    builder_->CreateStore(builder_->getInt32(value), ptr);
  }

  inline void CreateStoreValueHolderToVariable(llvm::Value* holder, llvm::Value* variable_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfVariable(variable_ptr);
    builder_->CreateStore(holder, ptr);
  }

  inline void CreateStoreUndefinedToVariable(llvm::Value* variable_ptr) {
    CreateStoreValueKindToVariable(ValueKind::Undefined, variable_ptr);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToVariable(builder_->getInt64(0), variable_ptr);
  }

  inline void CreateStoreNullToVariable(llvm::Value* variable_ptr) {
    CreateStoreValueKindToVariable(ValueKind::Null, variable_ptr);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToVariable(builder_->getInt64(0), variable_ptr);
  }

  inline void CreateStoreBooleanToVariable(llvm::Value* value, llvm::Value* variable_ptr) {
    CreateStoreValueKindToVariable(ValueKind::Boolean, variable_ptr);
    CreateStoreValueHolderToVariable(value, variable_ptr);
  }

  inline void CreateStoreNumberToVariable(llvm::Value* value, llvm::Value* variable_ptr) {
    CreateStoreValueKindToVariable(ValueKind::Number, variable_ptr);
    CreateStoreValueHolderToVariable(value, variable_ptr);
  }

  inline void CreateStoreClosureToVariable(llvm::Value* value, llvm::Value* variable_ptr) {
    CreateStoreValueKindToVariable(ValueKind::Closure, variable_ptr);
    CreateStoreValueHolderToVariable(value, variable_ptr);
  }

  inline void CreateStoreValueToVariable(llvm::Value* value_ptr, llvm::Value* variable_ptr) {
    auto* value = CreateLoadValue(value_ptr);
    auto* kind = CreateExtractValueKindFromValue(value);
    auto* holder = CreateExtractValueHolderFromValue(value);
    CreateStoreValueKindToVariable(kind, variable_ptr);
    CreateStoreValueHolderToVariable(holder, variable_ptr);
  }

  void CreateStoreItemToVariable(const Item& item, llvm::Value* variable_ptr);

  // value
  //
  // Redirect to the corresponding method for the Variable type.
  // The Variable type has a compatible layout with the Value type.

  inline llvm::Value* CreateGetValueKindPtrOfValue(llvm::Value* value_ptr) {
    return CreateGetValueKindPtrOfVariable(value_ptr);
  }

  inline llvm::Value* CreateGetValueHolderPtrOfValue(llvm::Value* value_ptr) {
    return CreateGetValueHolderPtrOfVariable(value_ptr);
  }

  inline llvm::Value* CreateExtractValueKindFromValue(llvm::Value* value) {
    return CreateExtractValueKindFromVariable(value);
  }

  inline llvm::Value* CreateExtractValueHolderFromValue(llvm::Value* value) {
    return CreateExtractValueHolderFromVariable(value);
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

  inline llvm::Value* CreateLoadClosureFromValue(llvm::Value* value_ptr) {
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

  inline void CreateStoreClosureToValue(llvm::Value* value, llvm::Value* value_ptr) {
    CreateStoreValueKindToValue(ValueKind::Closure, value_ptr);
    CreateStoreValueHolderToValue(value, value_ptr);
  }

  void CreateStoreItemToValue(const Item& item, llvm::Value* value_ptr);

  // capture

  inline llvm::Value* CreateGetTargetPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(types_->CreateCaptureType(), capture_ptr, 0);
  }

  inline llvm::Value* CreateGetEscapedPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(types_->CreateCaptureType(), capture_ptr, 1);
  }

  inline llvm::Value* CreateLoadTargetFromCapture(llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr);
  }

  inline void CreateStoreTargetToCapture(llvm::Value* variable_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    builder_->CreateStore(variable_ptr, ptr);
  }

  inline void CreateStoreEscapedToCapture(llvm::Value* variable, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetEscapedPtrOfCapture(capture_ptr);
    builder_->CreateStore(variable, ptr);
  }

  // closure

  inline llvm::Value* CreateGetLambdaPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(types_->CreateClosureType(), closure_ptr, 0);
  }

  inline llvm::Value* CreateGetNumCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(types_->CreateClosureType(), closure_ptr, 1);
  }

  inline llvm::Value* CreateGetCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(types_->CreateClosureType(), closure_ptr, 2);
  }

  inline llvm::Value* CreateLoadLambdaFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetLambdaPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr);
  }

  inline llvm::Value* CreateLoadNumCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetNumCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getInt16Ty(), ptr);
  }

  inline llvm::Value* CreateLoadCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr);
  }

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
  inline void CreateBasicBlockForDeadcode() {
    auto* dummy = llvm::BasicBlock::Create(*context_, "deadcode", function_);
    builder_->SetInsertPoint(dummy);
  }

  // TODO: separate variables that must be reset in EndFunction() from others.

  llvm::BasicBlock* FindBlockBySymbol(const std::vector<BlockItem>& stack, uint32_t symbol) const;
  void SetBlockForLabelsInContinueStack(llvm::BasicBlock* block);

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;

  // The following variables are reset for each function.
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* prologue_ = nullptr;
  llvm::BasicBlock* body_ = nullptr;
  llvm::BasicBlock* epilogue_ = nullptr;
  llvm::Value* exec_context_ = nullptr;
  llvm::Value* caps_ = nullptr;
  llvm::Value* argc_ = nullptr;
  llvm::Value* argv_ = nullptr;
  llvm::Value* ret_ = nullptr;
  llvm::Value* status_ = nullptr;

  // The following variables must be reset in the end of compilation for each function.
  std::vector<llvm::Value*> locals_;
  std::vector<Item> stack_;
  std::vector<ScopeItem> scope_stack_;
  std::vector<BlockItem> break_stack_;
  std::vector<BlockItem> continue_stack_;
  std::vector<llvm::BasicBlock*> catch_stack_;
  std::unordered_map<uint32_t, llvm::Value*> captures_;

  // A cache of functions does not reset in the end of compilation for each function.
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
