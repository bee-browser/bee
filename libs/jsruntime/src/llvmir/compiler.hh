#pragma once

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
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
#include "control_flow.hh"
#include "type_holder.hh"

class TypeHolder;
struct Module;

#define BB_NAME_PUSH(name) (enable_labels_ ? PushBasicBlockName(name) : (void)0)
#define BB_NAME_POP() (enable_labels_ ? PopBasicBlockName() : (void)0)
#define BB_NAME(s) (enable_labels_ ? MakeBasicBlockName(s).c_str() : "")
#define BB_NAME_WITH_ID(name, id) (enable_labels_ ? (name + llvm::Twine(id)).str().c_str() : "")
#define REG_NAME(expr) (enable_labels_ ? expr : "")

class Compiler {
 public:
  Compiler();
  ~Compiler() = default;

  void EnableLabels() {
    enable_labels_ = true;
  }

  Module* TakeModule();

  void SetSourceFileName(const char* input);
  void SetDataLayout(const char* data_layout);
  void SetTargetTriple(const char* triple);

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
  void Ternary();
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
  void Branch();
  void IfElseStatement();
  void IfStatement();
  void DoWhileLoop(uint16_t id);
  void WhileLoop(uint16_t id);
  void ForLoop(uint16_t id, bool has_init, bool has_test, bool has_next);
  void LoopInit();
  void LoopTest();
  void LoopNext();
  void LoopBody();
  void LoopEnd();
  void CaseBlock(uint16_t id, uint16_t num_cases);
  void CaseClause(bool has_statement);
  void DefaultClause(bool has_statement);
  void Switch(uint16_t id, uint16_t num_cases, uint16_t default_index);
  void Try();
  void Catch(bool nominal);
  void Finally(bool nominal);
  void TryEnd();
  void StartFunction(const char* name);
  void EndFunction(bool optimize = true);
  void StartScope(uint16_t scope_id);
  void EndScope(uint16_t scope_id);
  void AllocateLocals(uint16_t num_locals);
  void InitLocal(Locator locator);
  void TidyLocal(Locator locator);
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

  void PrepareScopeCleanupChecker(uint32_t stack_size);

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
      Capture,
    } type;
    union {
      llvm::Value* value;
      llvm::Function* func;
      struct Reference reference;
    };
    const char* label = nullptr;

    explicit Item(Type type) : type(type), value(nullptr) {}
    explicit Item(llvm::Function* func) : type(Item::Function), func(func) {}
    Item(Type type, llvm::Value* value) : type(type), value(value) {}
    Item(uint32_t symbol, Locator locator) : type(Item::Reference), reference(symbol, locator) {}

    inline void SetLabel(const char* label) {
      this->label = label;
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
  llvm::AllocaInst* CreateAlloc1(llvm::Type* ty, const llvm::Twine& name = "");
  llvm::AllocaInst* CreateAllocN(llvm::Type* ty, uint32_t n, const llvm::Twine& name = "");
  llvm::Function* CreateLambda(const char* name);

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
  void CreateCallRuntimeAssert(llvm::Value* assertion, llvm::Value* msg);

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
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateVariableType(), argv_, index,
        REG_NAME("argv." + llvm::Twine(index) + ".ptr"));
  }

  inline llvm::Value* CreateGetArgumentValuePtr(uint16_t index) {
    return CreateGetArgumentVariablePtr(index);
  }

  // locals

  // NOTE: No instruction is emitted.
  inline llvm::Value* GetLocalVariablePtr(uint16_t index) {
    assert(index < locals_.size());
    return locals_[index];
  }

  // NOTE: No instruction is emitted.
  inline llvm::Value* GetLocalValuePtr(uint16_t index) {
    return GetLocalVariablePtr(index);
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
    return builder_->CreateConstInBoundsGEP1_32(
        builder_->getPtrTy(), captures, index, REG_NAME("caps." + llvm::Twine(index) + ".ptr"));
  }

  inline llvm::Value* CreateLoadCapturePtrFromCaptures(llvm::Value* captures, uint16_t index) {
    auto* ptr = CreateGetCapturePtrPtrOfCaptures(captures, index);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("caps." + llvm::Twine(index)));
  }

  inline void CreateStoreCapturePtrToCaptures(llvm::Value* capture_ptr,
      llvm::Value* captures,
      uint16_t index) {
    auto* ptr = CreateGetCapturePtrPtrOfCaptures(captures, index);
    builder_->CreateStore(capture_ptr, ptr);
  }

  // variable

  inline llvm::Value* CreateGetValueKindPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateVariableType(), variable_ptr, 0, REG_NAME("kind.ptr"));
  }

  inline llvm::Value* CreateGetFlagsPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateVariableType(), variable_ptr, 1, REG_NAME("flags.ptr"));
  }

  inline llvm::Value* CreateGetReservedPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(types_->CreateVariableType(), variable_ptr, 2);
  }

  inline llvm::Value* CreateGetSymbolPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateVariableType(), variable_ptr, 3, REG_NAME("symbol.ptr"));
  }

  inline llvm::Value* CreateGetValueHolderPtrOfVariable(llvm::Value* variable_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateVariableType(), variable_ptr, 4, REG_NAME("holder.ptr"));
  }

  inline llvm::Value* CreateExtractValueKindFromVariable(llvm::Value* variable) {
    return builder_->CreateExtractValue(variable, 0, REG_NAME("kind"));
  }

  inline llvm::Value* CreateExtractValueHolderFromVariable(llvm::Value* variable) {
    return builder_->CreateExtractValue(variable, 4, REG_NAME("holder"));
  }

  inline llvm::Value* CreateLoadVariable(llvm::Value* variable_ptr) {
    return builder_->CreateLoad(types_->CreateVariableType(), variable_ptr, REG_NAME("variable"));
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
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    CreateStoreValueKindToVariable(kind, variable_ptr);
    auto* holder = CreateLoadValueHolderFromValue(value_ptr);
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
    return builder_->CreateLoad(builder_->getInt8Ty(), ptr, REG_NAME("kind"));
  }

  inline llvm::Value* CreateLoadValueHolderFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt64Ty(), ptr, REG_NAME("holder"));
  }

  inline llvm::Value* CreateLoadBooleanFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt1Ty(), ptr, REG_NAME("boolean"));
  }

  inline llvm::Value* CreateLoadNumberFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getDoubleTy(), ptr, REG_NAME("number"));
  }

  inline llvm::Value* CreateLoadFunctionFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("lambda"));
  }

  inline llvm::Value* CreateLoadClosureFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("closure"));
  }

  inline llvm::Value* CreateLoadValue(llvm::Value* value_ptr) {
    return CreateLoadVariable(value_ptr);
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
    return builder_->CreateStructGEP(
        types_->CreateCaptureType(), capture_ptr, 0, REG_NAME("target.ptr"));
  }

  inline llvm::Value* CreateGetEscapedPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateCaptureType(), capture_ptr, 1, REG_NAME("escaped.ptr"));
  }

  inline llvm::Value* CreateLoadTargetFromCapture(llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("target"));
  }

  inline void CreateStoreTargetToCapture(llvm::Value* variable_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    builder_->CreateStore(variable_ptr, ptr);
  }

  inline void CreateStoreEscapedToCapture(llvm::Value* variable_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetEscapedPtrOfCapture(capture_ptr);
    auto align = llvm::Align(sizeof(double));
    builder_->CreateMemCpy(ptr, align, variable_ptr, align, types_->GetWord(sizeof(Variable)));
  }

  // closure

  inline llvm::Value* CreateGetLambdaPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateClosureType(), closure_ptr, 0, REG_NAME("lambda.ptr"));
  }

  inline llvm::Value* CreateGetNumCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateClosureType(), closure_ptr, 1, REG_NAME("num_captures.ptr"));
  }

  inline llvm::Value* CreateGetCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateClosureType(), closure_ptr, 2, REG_NAME("captures.ptr"));
  }

  inline llvm::Value* CreateLoadLambdaFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetLambdaPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("lambda"));
  }

  inline llvm::Value* CreateLoadNumCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetNumCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getInt16Ty(), ptr, REG_NAME("num_captures"));
  }

  inline llvm::Value* CreateLoadCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("captures"));
  }

  // scope cleanup cheker

  void CreatePushOntoScopeCleanupStack(uint16_t scope_id);
  llvm::Value* CreatePopFromScopeCleanupStack();
  void CreateAssertScopeCleanupStackBounds();
  void CreateAssertScopeCleanupStackPoppedValue(llvm::Value* actual, uint16_t expected);
  void CreateAssertScopeCleanupStackIsEmpty();
  void CreateAssertScopeCleanupStackHasItem();

  bool IsScopeCleanupCheckerEnabled() const {
    return scope_cleanup_stack_ != nullptr;
  }

  llvm::Value* CreateLoadScopeCleanupStackTop() {
    return builder_->CreateLoad(
        builder_->getInt32Ty(), scope_cleanup_stack_top_, REG_NAME("scope_cleanup_stack.top"));
  }

  void CreateStoreScopeCleanupStackTop(llvm::Value* value) {
    builder_->CreateStore(value, scope_cleanup_stack_top_);
  }

  void ClearScopeCleanupStack() {
    scope_cleanup_stack_type_ = nullptr;
    scope_cleanup_stack_ = nullptr;
    scope_cleanup_stack_top_ = nullptr;
    scope_cleanup_stack_size_ = 0;
  }

  // helper methods for basic blocks

  llvm::BasicBlock* CreateBasicBlock(const char* name) {
    return llvm::BasicBlock::Create(*context_, name, function_);
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
  void CreateBasicBlockForDeadcode();

  // TODO: separate variables that must be reset in EndFunction() from others.

  // Helper methods for Call().
  llvm::Value* CreateLoadClosureFromValueOrThrowTypeError(llvm::Value* value_ptr);
  void CreateCheckStatusForException(llvm::Value* status, llvm::Value* ret);

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;

  // The following variables are reset for each function.
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* locals_block_ = nullptr;
  llvm::BasicBlock* args_block_ = nullptr;
  llvm::BasicBlock* body_block_ = nullptr;
  llvm::BasicBlock* return_block_ = nullptr;
  llvm::Value* exec_context_ = nullptr;
  llvm::Value* caps_ = nullptr;
  llvm::Value* argc_ = nullptr;
  llvm::Value* argv_ = nullptr;
  llvm::Value* retv_ = nullptr;
  // Holds one of STATUS_XXX values, not Status::*.
  llvm::Value* status_ = nullptr;

  // scope cleanup checker
  llvm::Type* scope_cleanup_stack_type_ = nullptr;
  llvm::Value* scope_cleanup_stack_ = nullptr;
  llvm::Value* scope_cleanup_stack_top_ = nullptr;
  uint16_t scope_cleanup_stack_size_ = 0;

  // The following variables must be reset in the end of compilation for each function.
  std::vector<llvm::Value*> locals_;
  std::vector<Item> stack_;
  ControlFlowStack control_flow_stack_;
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

  bool enable_labels_ = false;

  inline void PushBasicBlockName(std::string&& name) {
    assert(enable_labels_);
    basic_block_name_stack_.push_back(std::move(name));
  }

  // TODO: detect an ill-nested block name
  inline void PopBasicBlockName() {
    assert(enable_labels_);
    basic_block_name_stack_.pop_back();
  }

  std::string MakeBasicBlockName(const char* name) const;

  std::vector<std::string> basic_block_name_stack_;
};
