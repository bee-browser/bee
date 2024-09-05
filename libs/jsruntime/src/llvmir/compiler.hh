#pragma once

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>
#include <unordered_map>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Analysis/CGSCCPassManager.h>
#include <llvm/Analysis/LoopAnalysisManager.h>
#include <llvm/IR/BasicBlock.h>
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

  llvm::BasicBlock* CreateBasicBlock(const char* name) {
    return llvm::BasicBlock::Create(*context_, name, function_);
  }

  llvm::BasicBlock* CreateBasicBlock(const char* name, size_t name_len) {
    return llvm::BasicBlock::Create(
        *context_, llvm::Twine(llvm::StringRef(name, name_len)), function_);
  }

  llvm::BasicBlock* GetBasicBlock() const {
    return builder_->GetInsertBlock();
  }

  void SetBasicBlock(llvm::BasicBlock* block) {
    assert(block != nullptr);
    builder_->SetInsertPoint(block);
  }

  void MoveBasicBlockAfter(llvm::BasicBlock* block) const {
    assert(block != nullptr);
    block->moveAfter(builder_->GetInsertBlock());
  }

  bool IsBasicBlockTerminated(llvm::BasicBlock* block) const {
    assert(block != nullptr);
    return block->getTerminator() != nullptr;
  }

  void SetLocalsBlock(llvm::BasicBlock* block) {
    locals_block_ = block;
  }

  void CreateBr(llvm::BasicBlock* block) {
    assert(block != nullptr);
    builder_->CreateBr(block);
  }

  llvm::Value* GetBoolean(bool value);
  llvm::Value* GetNumber(double value);
  llvm::Function* GetFunction(uint32_t func_id, const char* name);
  llvm::Value* GetException();

  llvm::Value* CreateFNeg(llvm::Value* value) {
    return builder_->CreateFNeg(value, REG_NAME("neg"));
  }

  llvm::Value* CreateBitwiseNot(llvm::Value* number);
  llvm::Value* CreateLogicalNot(llvm::Value* boolean);

  llvm::Value* CreateNumberToBoolean(llvm::Value* number) {
    auto* zero = llvm::ConstantFP::getZero(builder_->getDoubleTy());
    // return number != 0.0
    return builder_->CreateFCmpUNE(number, zero, REG_NAME("number_to_boolean"));
  }

  llvm::Value* CreateFMul(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFMul(lhs, rhs, REG_NAME("mul"));
  }

  llvm::Value* CreateFDiv(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFDiv(lhs, rhs, REG_NAME("div"));
  }

  llvm::Value* CreateFRem(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFRem(lhs, rhs, REG_NAME("rem"));
  }

  llvm::Value* CreateFAdd(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFAdd(lhs, rhs, REG_NAME("add"));
  }

  llvm::Value* CreateFSub(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFSub(lhs, rhs, REG_NAME("sub"));
  }

  llvm::Value* CreateLeftShift(llvm::Value* lhs, llvm::Value* rhs);
  llvm::Value* CreateSignedRightShift(llvm::Value* lhs, llvm::Value* rhs);
  llvm::Value* CreateUnsignedRightShift(llvm::Value* lhs, llvm::Value* rhs);

  llvm::Value* CreateLessThan(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFCmpOLT(lhs, rhs, REG_NAME("lt"));
  }

  llvm::Value* CreateGreaterThan(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFCmpOGT(lhs, rhs, REG_NAME("gt"));
  }

  llvm::Value* CreateLessThanOrEqual(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFCmpOLE(lhs, rhs, REG_NAME("le"));
  }

  llvm::Value* CreateGreaterThanOrEqual(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateFCmpOGE(lhs, rhs, REG_NAME("ge"));
  }

  llvm::Value* CreateBitwiseAnd(llvm::Value* lhs, llvm::Value* rhs) {
    // 6.1.6.1.17 Number::bitwiseAND ( x, y )
    return NumberBitwiseOp('&', lhs, rhs);
  }

  llvm::Value* CreateBitwiseXor(llvm::Value* lhs, llvm::Value* rhs) {
    // 6.1.6.1.18 Number::bitwiseXOR ( x, y )
    return NumberBitwiseOp('^', lhs, rhs);
  }

  llvm::Value* CreateBitwiseOr(llvm::Value* lhs, llvm::Value* rhs) {
    // 6.1.6.1.19 Number::bitwiseOR ( x, y )
    return NumberBitwiseOp('|', lhs, rhs);
  }

  llvm::Value* CreateGetValuePtrInValues(llvm::Value* values, uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateValueType(), values, index);
  }

  llvm::Value* CreateCallOnClosure(llvm::Value* closure, uint16_t argc, llvm::Value* argv, llvm::Value* retv);

  llvm::Value* GetNullptr() {
    return llvm::Constant::getNullValue(builder_->getPtrTy());
  }

  llvm::Value* CreateICmpEq(llvm::Value* lhs, llvm::Value* rhs) {
    return builder_->CreateICmpEQ(lhs, rhs);
  }

  void CreateCondBr(llvm::Value* cond, llvm::BasicBlock* then_block, llvm::BasicBlock* else_block) {
    builder_->CreateCondBr(cond, then_block, else_block);
  }

  void CreateStore(llvm::Value* value, llvm::Value* dest) {
    builder_->CreateStore(value, dest);
  }

  void CreateEscapeVariable(llvm::Value* capture, llvm::Value* variable);

  llvm::Value* CreateLoadCapture(uintptr_t index) {
    return CreateLoadCapturePtrFromCaptures(caps_, index);
  }

  llvm::Value* GetNan() {
    return llvm::ConstantFP::getNaN(builder_->getDoubleTy());
  }

  llvm::Value* GetZero() {
    return llvm::ConstantFP::getZero(builder_->getDoubleTy());
  }

  llvm::Value* CreateUIToFP(llvm::Value* value) {
    return builder_->CreateUIToFP(value, builder_->getDoubleTy(), REG_NAME("ui2fp"));
  }

  llvm::Value* CreateBooleanPhi(llvm::Value* then_value, llvm::BasicBlock* then_block, llvm::Value* else_value, llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2, REG_NAME("boolean.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  llvm::Value* CreateNumberPhi(llvm::Value* then_value, llvm::BasicBlock* then_block, llvm::Value* else_value, llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getDoubleTy(), 2, REG_NAME("number.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  llvm::Value* CreateClosurePhi(llvm::Value* then_value, llvm::BasicBlock* then_block, llvm::Value* else_value, llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2, REG_NAME("closure.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  llvm::Value* CreateValuePhi(llvm::Value* then_value, llvm::BasicBlock* then_block, llvm::Value* else_value, llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2, REG_NAME("value.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  llvm::Value* CreateHasUncaughtException() {
    auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_, REG_NAME("status"));
    return builder_->CreateICmpEQ(
        status, builder_->getInt32(STATUS_EXCEPTION), REG_NAME("has_uncaught_exception"));
  }

  void StartFunction(const char* name);
  void EndFunction(bool optimize = true);
  void StartScopeCleanupChecker(uint16_t scope_id);
  void EndScopeCleanupChecker(uint16_t scope_id);
  void HandleReturnedThrown(bool returned,
      bool thrown,
      llvm::BasicBlock* block,
      llvm::BasicBlock* cleanup_block,
      llvm::BasicBlock* exception_block);
  llvm::Value* CreateLocalVariable(uint16_t index);

  void PrepareScopeCleanupChecker(uint32_t stack_size);

  llvm::Value* NumberBitwiseOp(char op, llvm::Value* x, llvm::Value* y);
  llvm::Value* ToNumeric(llvm::Value* value_ptr);
  llvm::Value* ToInt32(llvm::Value* number);
  llvm::Value* ToUint32(llvm::Value* number);
  llvm::Value* CreateUndefinedToAny();
  llvm::Value* CreateNullToAny();
  llvm::Value* CreateBooleanToAny(llvm::Value* boolean);
  llvm::Value* CreateNumberToAny(llvm::Value* number);
  llvm::Value* CreateClosureToAny(llvm::Value* closure);
  llvm::AllocaInst* CreateAlloc1(llvm::Type* ty, const llvm::Twine& name = "");
  llvm::AllocaInst* CreateAllocN(llvm::Type* ty, uint32_t n, const llvm::Twine& name = "");
  llvm::Function* CreateLambda(const char* name);

  llvm::Value* CreateIsNonNullish(llvm::Value* value_ptr);

  llvm::Value* CreateToBoolean(llvm::Value* value_ptr);

  llvm::Value* CreateIsLooselyEqual(llvm::Value* x, llvm::Value* y);

  llvm::Value* CreateIsStrictlyEqual(llvm::Value* x, llvm::Value* y);
  llvm::Value* CreateIsUndefined(llvm::Value* value_ptr);
  llvm::Value* CreateIsNull(llvm::Value* value_ptr);
  llvm::Value* CreateIsBoolean(llvm::Value* value_ptr);
  llvm::Value* CreateIsNumber(llvm::Value* value_ptr);
  llvm::Value* CreateIsClosure(llvm::Value* value_ptr);
  llvm::Value* CreateIsSameBoolean(llvm::Value* a, llvm::Value* b);
  llvm::Value* CreateIsSameNumber(llvm::Value* a, llvm::Value* b);
  llvm::Value* CreateIsSameClosure(llvm::Value* a, llvm::Value* b);
  llvm::Value* CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* boolean);
  llvm::Value* CreateIsSameNumberValue(llvm::Value* value_ptr, llvm::Value* number);
  llvm::Value* CreateIsSameClosureValue(llvm::Value* value_ptr, llvm::Value* closure);

  llvm::Value* CreateCallRuntimeCreateCapture(llvm::Value* variable_ptr);
  llvm::Value* CreateCallRuntimeCreateClosure(llvm::Value* lambda, uint16_t num_captures);
  void CreateCallRuntimeAssert(llvm::Value* assertion, llvm::Value* msg);

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

  inline void CreateStoreCapturePtrToClosure(llvm::Value* capture_ptr, llvm::Value* closure_ptr, uint16_t index) {
    auto* ptr = CreateLoadCapturesFromClosure(closure_ptr);
    CreateStoreCapturePtrToCaptures(capture_ptr, ptr, index);
  }

  // argv

  llvm::Value* CreateArgv(uint16_t argc) {
    assert(argc > 0);
    return CreateAllocN(types_->CreateValueType(), argc, REG_NAME("argv.ptr"));
  }

  llvm::Value* CreateGetArgInArgv(llvm::Value* argv, uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateValueType(), argv, index, REG_NAME("argv." + llvm::Twine(index) + ".ptr"));
  }

  // retv

  llvm::Value* CreateRetv();

  inline void CreateStoreUndefinedToRetv() {
    CreateStoreUndefinedToVariable(retv_);
  }

  inline void CreateStoreNullToRetv() {
    CreateStoreNullToVariable(retv_);
  }

  inline void CreateStoreBooleanToRetv(llvm::Value* value) {
    CreateStoreBooleanToVariable(value, retv_);
  }

  inline void CreateStoreNumberToRetv(llvm::Value* value) {
    CreateStoreNumberToVariable(value, retv_);
  }

  inline void CreateStoreClosureToRetv(llvm::Value* value) {
    CreateStoreClosureToVariable(value, retv_);
  }

  inline void CreateStoreValueToRetv(llvm::Value* value) {
    CreateStoreValueToVariable(value, retv_);
  }

  // status

  void CreateAllocStatus() {
    status_ = CreateAlloc1(builder_->getInt32Ty(), REG_NAME("status.ptr"));
    builder_->CreateStore(builder_->getInt32(STATUS_UNSET), status_);
  }

  void CreateStoreNormalStatus() {
    builder_->CreateStore(builder_->getInt32(STATUS_NORMAL), status_);
  }

  void CreateStoreExceptionStatus() {
    builder_->CreateStore(builder_->getInt32(STATUS_EXCEPTION), status_);
  }

  llvm::Value* CreateIsExceptionStatus(llvm::Value* status) {
    return builder_->CreateICmpEQ(status, builder_->getInt32(STATUS_EXCEPTION), REG_NAME("is_exception"));
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

  // TODO: separate variables that must be reset in EndFunction() from others.

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;

  // The following variables are reset for each function.
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* locals_block_ = nullptr;
  llvm::BasicBlock* body_block_ = nullptr;
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
};
