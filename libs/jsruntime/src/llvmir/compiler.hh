#pragma once

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <memory>
#include <sstream>
#include <string>
#include <unordered_map>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Analysis/CGSCCPassManager.h>
#include <llvm/Analysis/LoopAnalysisManager.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/DataLayout.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/PassInstrumentation.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Value.h>
#include <llvm/IR/Verifier.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/StandardInstrumentations.h>
#include <llvm/Support/Alignment.h>
#include <llvm/Support/raw_ostream.h>
#include <llvm/Transforms/InstCombine/InstCombine.h>
#include <llvm/Transforms/Scalar/GVN.h>
#include <llvm/Transforms/Scalar/Reassociate.h>
#include <llvm/Transforms/Scalar/SimplifyCFG.h>
#include <llvm/Transforms/Utils/Mem2Reg.h>
#pragma GCC diagnostic pop

#include "bridge.hh"
#include "macros.hh"
#include "module.hh"
#include "type_holder.hh"

class TypeHolder;
struct Module;

#define REG_NAME(expr) (enable_labels_ ? expr : "")

class Compiler {
 public:
  Compiler() {
    context_ = std::make_unique<llvm::LLVMContext>();
    module_ = std::make_unique<llvm::Module>("<main>", *context_);
    builder_ = std::make_unique<llvm::IRBuilder<>>(*context_);
    types_ = std::make_unique<TypeHolder>(*context_, *module_, *builder_);

    // Took from toy.cpp in the Kaleidoscope tutorial.
    fpm_ = std::make_unique<llvm::FunctionPassManager>();
    lam_ = std::make_unique<llvm::LoopAnalysisManager>();
    fam_ = std::make_unique<llvm::FunctionAnalysisManager>();
    cgam_ = std::make_unique<llvm::CGSCCAnalysisManager>();
    mam_ = std::make_unique<llvm::ModuleAnalysisManager>();
    pic_ = std::make_unique<llvm::PassInstrumentationCallbacks>();
    si_ = std::make_unique<llvm::StandardInstrumentations>(*context_, true);  // with debug logs
    si_->registerCallbacks(*pic_, mam_.get());

    fpm_->addPass(llvm::PromotePass());
    fpm_->addPass(llvm::InstCombinePass());
    fpm_->addPass(llvm::ReassociatePass());
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
    fpm_->addPass(llvm::GVNPass());
#pragma GCC diagnostic pop
    fpm_->addPass(llvm::SimplifyCFGPass());

    llvm::PassBuilder pb;
    pb.registerModuleAnalyses(*mam_);
    pb.registerFunctionAnalyses(*fam_);
    pb.crossRegisterProxies(*lam_, *fam_, *cgam_, *mam_);
  }

  ~Compiler() = default;

  void EnableLabels() {
    enable_labels_ = true;
  }

  Module* TakeModule() {
    if (llvm::verifyModule(*module_, &llvm::errs())) {
      llvm::errs() << "### broken-module\n";
      module_->print(llvm::errs(), nullptr);
      llvm::errs() << '\n';
      std::abort();
    }

    llvm::orc::ThreadSafeModule mod(std::move(module_), std::move(context_));
    return new Module(std::move(mod));
  }

  void SetSourceFileName(const char* input) {
    module_->setSourceFileName(input);
  }

  void SetDataLayout(const char* data_layout) {
    module_->setDataLayout(data_layout);
  }

  void SetTargetTriple(const char* triple) {
    module_->setTargetTriple(triple);
  }

  // function

  void StartFunction(const char* name) {
    function_ = CreateLambda(name);

    exec_context_ = function_->getArg(0);
    caps_ = function_->getArg(1);
    argc_ = function_->getArg(2);
    argv_ = function_->getArg(3);
    retv_ = function_->getArg(4);

    ClearScopeCleanupStack();
  }

  void EndFunction(bool optimize) {
    if (IsScopeCleanupCheckerEnabled()) {
      CreateAssertScopeCleanupStackIsEmpty();
    }

    auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_, REG_NAME("status"));
    // Convert STATUS_XXX into Status.
    auto* masked =
        builder_->CreateAnd(status, builder_->getInt32(STATUS_MASK), REG_NAME("status.masked"));
    builder_->CreateRet(masked);

    if (llvm::verifyFunction(*function_, &llvm::errs())) {
      llvm::errs() << "### broken-function\n";
      function_->print(llvm::errs());
      llvm::errs() << '\n';
      std::abort();
    }

    if (optimize) {
      fpm_->run(*function_, *fam_);
    }
  }

  void SetLocalsBlock(llvm::BasicBlock* block) {
    locals_block_ = block;
  }

  llvm::Function* GetFunction(uint32_t func_id, const char* name) {
    UNUSED(func_id);
    return CreateLambda(name);
  }

  // basic block

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

  // jump

  void CreateBr(llvm::BasicBlock* block) {
    assert(block != nullptr);
    builder_->CreateBr(block);
  }

  void CreateCondBr(llvm::Value* cond,
      llvm::BasicBlock* then_block,
      llvm::BasicBlock* else_block) {
    builder_->CreateCondBr(cond, then_block, else_block);
  }

  void HandleReturnedThrown(bool returned,
      bool thrown,
      llvm::BasicBlock* block,
      llvm::BasicBlock* cleanup_block,
      llvm::BasicBlock* exception_block) {
    assert(block != nullptr);
    // cleanup_block may be nullptr.
    // exception_block may be nullptr.

    if (!returned && !thrown) {
      builder_->CreateBr(block);
    } else {
      auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_, REG_NAME("status"));
      auto* switch_inst = builder_->CreateSwitch(status, block);
      if (cleanup_block != nullptr) {
        switch_inst->addCase(builder_->getInt32(STATUS_NORMAL), cleanup_block);
      }
      if (exception_block != nullptr) {
        switch_inst->addCase(builder_->getInt32(STATUS_EXCEPTION), exception_block);
      }
    }
  }

  // undefined

  llvm::Value* CreateIsUndefined(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(
        kind, builder_->getInt8(kValueKindUndefined), REG_NAME("is_undefined"));
  }

  // null

  llvm::Value* CreateIsNull(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindNull), REG_NAME("is_null"));
  }

  llvm::Value* CreateIsNonNullish(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpUGT(
        kind, builder_->getInt8(kValueKindNull), REG_NAME("is_non_nullish"));
  }

  // boolean

  llvm::Value* CreateIsBoolean(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(
        kind, builder_->getInt8(kValueKindBoolean), REG_NAME("is_boolean"));
  }

  llvm::Value* CreateIsSameBoolean(llvm::Value* a, llvm::Value* b) {
    return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_boolean"));
  }

  llvm::Value* CreateNumberToBoolean(llvm::Value* number) {
    auto* zero = llvm::ConstantFP::getZero(builder_->getDoubleTy());
    // return number != 0.0
    return builder_->CreateFCmpUNE(number, zero, REG_NAME("number_to_boolean"));
  }

  // 7.1.2 ToBoolean ( argument )
  llvm::Value* CreateToBoolean(llvm::Value* value_ptr) {
    auto* func = types_->CreateRuntimeToBoolean();
    return builder_->CreateCall(func, {exec_context_, value_ptr}, REG_NAME("boolean"));
  }

  llvm::Value* GetBoolean(bool value) {
    return llvm::ConstantInt::getBool(*context_, value);
  }

  llvm::Value* CreateLogicalNot(llvm::Value* boolean) {
    return builder_->CreateXor(boolean, builder_->getTrue(), REG_NAME("logical_not"));
  }

  llvm::Value* CreateBooleanPhi(llvm::Value* then_value,
      llvm::BasicBlock* then_block,
      llvm::Value* else_value,
      llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2, REG_NAME("boolean.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  // number

  llvm::Value* CreateIsNumber(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(
        kind, builder_->getInt8(kValueKindNumber), REG_NAME("is_number"));
  }

  llvm::Value* CreateIsSameNumber(llvm::Value* a, llvm::Value* b) {
    return builder_->CreateFCmpOEQ(a, b, REG_NAME("is_same_number"));
  }

  llvm::Value* CreateUIToFP(llvm::Value* value) {
    return builder_->CreateUIToFP(value, builder_->getDoubleTy(), REG_NAME("ui2fp"));
  }

  // 7.1.4 ToNumber ( argument )
  llvm::Value* ToNumeric(llvm::Value* value_ptr) {
    auto* call = types_->CreateRuntimeToNumeric();
    return builder_->CreateCall(call, {exec_context_, value_ptr}, REG_NAME("numeric"));
  }

  llvm::Value* GetNan() {
    return llvm::ConstantFP::getNaN(builder_->getDoubleTy());
  }

  llvm::Value* GetZero() {
    return llvm::ConstantFP::getZero(builder_->getDoubleTy());
  }

  llvm::Value* GetNumber(double value) {
    return llvm::ConstantFP::get(*context_, llvm::APFloat(value));
  }

  // 6.1.6.1.2 Number::bitwiseNOT ( x )
  llvm::Value* CreateBitwiseNot(llvm::Value* number) {
    auto* int32 = ToInt32(number);
    auto* xored = builder_->CreateXor(int32, -1, REG_NAME("bitwise_not.xor"));
    return builder_->CreateSIToFP(xored, builder_->getDoubleTy(), REG_NAME("bitwise_not"));
  }

  llvm::Value* CreateFNeg(llvm::Value* value) {
    return builder_->CreateFNeg(value, REG_NAME("neg"));
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

  // 6.1.6.1.9 Number::leftShift ( x, y )
  llvm::Value* CreateLeftShift(llvm::Value* lhs, llvm::Value* rhs) {
    auto* lnum = ToInt32(lhs);
    auto* rnum = ToUint32(rhs);
    auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
    auto* shifted = builder_->CreateShl(lnum, shift_count, REG_NAME("shl"));
    return builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
  }

  // 6.1.6.1.10 Number::signedRightShift ( x, y )
  llvm::Value* CreateSignedRightShift(llvm::Value* lhs, llvm::Value* rhs) {
    auto* lnum = ToInt32(lhs);
    auto* rnum = ToUint32(rhs);
    auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
    auto* shifted = builder_->CreateAShr(lnum, shift_count, REG_NAME("ashr"));
    return builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
  }

  // 6.1.6.1.11 Number::unsignedRightShift ( x, y )
  llvm::Value* CreateUnsignedRightShift(llvm::Value* lhs, llvm::Value* rhs) {
    auto* lnum = ToUint32(lhs);
    auto* rnum = ToUint32(rhs);
    auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
    auto* shifted = builder_->CreateLShr(lnum, shift_count, REG_NAME("lshr"));
    return builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
  }

  // 6.1.6.1.17 Number::bitwiseAND ( x, y )
  llvm::Value* CreateBitwiseAnd(llvm::Value* lhs, llvm::Value* rhs) {
    return NumberBitwiseOp('&', lhs, rhs);
  }

  // 6.1.6.1.18 Number::bitwiseXOR ( x, y )
  llvm::Value* CreateBitwiseXor(llvm::Value* lhs, llvm::Value* rhs) {
    return NumberBitwiseOp('^', lhs, rhs);
  }

  // 6.1.6.1.19 Number::bitwiseOR ( x, y )
  llvm::Value* CreateBitwiseOr(llvm::Value* lhs, llvm::Value* rhs) {
    return NumberBitwiseOp('|', lhs, rhs);
  }

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

  llvm::Value* CreateNumberPhi(llvm::Value* then_value,
      llvm::BasicBlock* then_block,
      llvm::Value* else_value,
      llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getDoubleTy(), 2, REG_NAME("number.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  // pointer

  llvm::Value* GetNullptr() {
    return llvm::Constant::getNullValue(builder_->getPtrTy());
  }

  // closure

  llvm::Value* CreateIsClosure(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(
        kind, builder_->getInt8(kValueKindClosure), REG_NAME("is_closure"));
  }

  llvm::Value* CreateIsSameClosure(llvm::Value* a, llvm::Value* b) {
    return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_closure"));
  }

  llvm::Value* CreateClosure(llvm::Value* lambda, uint16_t num_captures) {
    auto* func = types_->CreateRuntimeCreateClosure();
    return builder_->CreateCall(
        func, {exec_context_, lambda, builder_->getInt16(num_captures)}, REG_NAME("closure.ptr"));
  }

  inline void CreateStoreCapturePtrToClosure(llvm::Value* capture_ptr,
      llvm::Value* closure_ptr,
      uint16_t index) {
    auto* ptr = CreateLoadCapturesFromClosure(closure_ptr);
    CreateStoreCapturePtrToCaptures(capture_ptr, ptr, index);
  }

  llvm::Value* CreateCallOnClosure(llvm::Value* closure,
      uint16_t argc,
      llvm::Value* argv,
      llvm::Value* retv) {
    auto* prototype = types_->CreateLambdaType();
    auto* lambda = CreateLoadLambdaFromClosure(closure);
    auto* caps = CreateLoadCapturesFromClosure(closure);
    return builder_->CreateCall(prototype, lambda,
        {exec_context_, caps, types_->GetWord(argc), argv, retv}, REG_NAME("status"));
  }

  llvm::Value* CreateClosurePhi(llvm::Value* then_value,
      llvm::BasicBlock* then_block,
      llvm::Value* else_value,
      llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2, REG_NAME("closure.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  // value

  // 7.2.13 IsLooselyEqual ( x, y )
  llvm::Value* CreateIsLooselyEqual(llvm::Value* x, llvm::Value* y) {
    // TODO: Create inline instructions if runtime_is_loosely_equal() is slow.
    auto* func = types_->CreateRuntimeIsLooselyEqual();
    return builder_->CreateCall(func, {exec_context_, x, y}, REG_NAME("is_loosely_equal.retval"));
  }

  // 7.2.14 IsStrictlyEqual ( x, y )
  llvm::Value* CreateIsStrictlyEqual(llvm::Value* x, llvm::Value* y) {
    // TODO: Create inline instructions if runtime_is_strictly_equal() is slow.
    auto* func = types_->CreateRuntimeIsStrictlyEqual();
    return builder_->CreateCall(func, {exec_context_, x, y}, REG_NAME("is_strictly_equal.retval"));
  }

  llvm::Value* CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* boolean) {
    auto* value = CreateLoadBooleanFromValue(value_ptr);
    return builder_->CreateICmpEQ(value, boolean, REG_NAME("is_same_boolean_value"));
  }

  llvm::Value* CreateIsSameNumberValue(llvm::Value* value_ptr, llvm::Value* number) {
    auto* value = CreateLoadNumberFromValue(value_ptr);
    return builder_->CreateFCmpOEQ(value, number, REG_NAME("is_same_number_value"));
  }

  llvm::Value* CreateIsSameClosureValue(llvm::Value* value_ptr, llvm::Value* closure) {
    auto* value = CreateLoadClosureFromValue(value_ptr);
    return builder_->CreateICmpEQ(value, closure, REG_NAME("is_same_closure_value"));
  }

  llvm::Value* CreateUndefinedToAny() {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreUndefinedToValue(value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateNullToAny() {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreNullToValue(value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateBooleanToAny(llvm::Value* boolean) {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreBooleanToValue(boolean, value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateNumberToAny(llvm::Value* number) {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreNumberToValue(number, value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateClosureToAny(llvm::Value* closure) {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreClosureToValue(closure, value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateValuePhi(llvm::Value* then_value,
      llvm::BasicBlock* then_block,
      llvm::Value* else_value,
      llvm::BasicBlock* else_block) {
    auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2, REG_NAME("value.phi"));
    phi->addIncoming(then_value, then_block);
    phi->addIncoming(else_value, else_block);
    return phi;
  }

  llvm::Value* CreateLocalValue(uint16_t index) {
    return CreateAlloc1(
        types_->CreateValueType(), REG_NAME("local" + llvm::Twine(index) + ".ptr"));
  }

  inline void CreateStoreNoneToValue(llvm::Value* dest) {
    CreateStoreValueKindToValue(ValueKind::None, dest);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), dest);
  }

  inline void CreateStoreUndefinedToValue(llvm::Value* dest) {
    CreateStoreValueKindToValue(ValueKind::Undefined, dest);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), dest);
  }

  inline void CreateStoreNullToValue(llvm::Value* dest) {
    CreateStoreValueKindToValue(ValueKind::Null, dest);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), dest);
  }

  inline void CreateStoreBooleanToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(ValueKind::Boolean, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  inline void CreateStoreNumberToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(ValueKind::Number, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  inline void CreateStoreClosureToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(ValueKind::Closure, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  inline void CreateStoreValueToValue(llvm::Value* value_ptr, llvm::Value* dest) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    CreateStoreValueKindToValue(kind, dest);
    auto* holder = CreateLoadValueHolderFromValue(value_ptr);
    CreateStoreValueHolderToValue(holder, dest);
  }

  inline llvm::Value* CreateLoadClosureFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("value.closure"));
  }

  // argv

  llvm::Value* CreateArgv(uint16_t argc) {
    assert(argc > 0);
    return CreateAllocN(types_->CreateValueType(), argc, REG_NAME("argv.ptr"));
  }

  llvm::Value* CreateGetArgInArgv(llvm::Value* argv, uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(
        types_->CreateValueType(), argv, index, REG_NAME("argv." + llvm::Twine(index) + ".ptr"));
  }

  inline llvm::Value* CreateGetArgumentValuePtr(uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(
        types_->CreateValueType(), argv_, index, REG_NAME("argv." + llvm::Twine(index) + ".ptr"));
  }

  // retv

  llvm::Value* CreateRetv() {
    return CreateAlloc1(types_->CreateValueType(), REG_NAME("retv.ptr"));
  }

  inline void CreateStoreUndefinedToRetv() {
    CreateStoreUndefinedToValue(retv_);
  }

  inline void CreateStoreNullToRetv() {
    CreateStoreNullToValue(retv_);
  }

  inline void CreateStoreBooleanToRetv(llvm::Value* value) {
    CreateStoreBooleanToValue(value, retv_);
  }

  inline void CreateStoreNumberToRetv(llvm::Value* value) {
    CreateStoreNumberToValue(value, retv_);
  }

  inline void CreateStoreClosureToRetv(llvm::Value* value) {
    CreateStoreClosureToValue(value, retv_);
  }

  inline void CreateStoreValueToRetv(llvm::Value* value) {
    CreateStoreValueToValue(value, retv_);
  }

  llvm::Value* GetException() {
    // TODO: Should we check status_ at runtime?
    return retv_;
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
    return builder_->CreateICmpEQ(
        status, builder_->getInt32(STATUS_EXCEPTION), REG_NAME("is_exception"));
  }

  llvm::Value* CreateHasUncaughtException() {
    auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_, REG_NAME("status"));
    return builder_->CreateICmpEQ(
        status, builder_->getInt32(STATUS_EXCEPTION), REG_NAME("has_uncaught_exception"));
  }

  // capture

  llvm::Value* CreateCapture(llvm::Value* value_ptr) {
    auto* func = types_->CreateRuntimeCreateCapture();
    return builder_->CreateCall(func, {exec_context_, value_ptr}, REG_NAME("capture.ptr"));
  }

  void CreateEscapeValue(llvm::Value* capture, llvm::Value* value) {
    auto* escaped_ptr = CreateGetEscapedPtrOfCapture(capture);
    CreateStoreTargetToCapture(escaped_ptr, capture);
    auto align = llvm::Align(sizeof(double));
    builder_->CreateMemCpy(escaped_ptr, align, value, align, types_->GetWord(sizeof(Value)));
  }

  inline llvm::Value* CreateGetCaptureValuePtr(uint16_t index) {
    auto* ptr = CreateLoadCapturePtrFromCaptures(caps_, index);
    return CreateLoadTargetFromCapture(ptr);
  }

  llvm::Value* CreateLoadCapture(uintptr_t index) {
    return CreateLoadCapturePtrFromCaptures(caps_, index);
  }

  // scope cleanup checker

  void SetupScopeCleanupChecker(uint32_t stack_size) {
    scope_cleanup_stack_type_ = llvm::ArrayType::get(builder_->getInt16Ty(), stack_size);
    scope_cleanup_stack_ =
        CreateAllocN(builder_->getInt16Ty(), stack_size, REG_NAME("scope_cleanup_stack"));
    scope_cleanup_stack_top_ =
        CreateAlloc1(builder_->getInt32Ty(), REG_NAME("scope_cleanup_stack_top"));
    builder_->CreateStore(builder_->getInt32(0), scope_cleanup_stack_top_);
    scope_cleanup_stack_size_ = stack_size;
  }

  void PerformScopeCleanupPrecheck(uint16_t scope_id) {
    if (IsScopeCleanupCheckerEnabled()) {
      // We assumed here that the control flow does not enter into a scope which is already
      // entered.  However, it may be better to check that explicitly here before pushing the scope
      // ID.
      CreateAssertScopeCleanupStackBounds();
      CreatePushOntoScopeCleanupStack(scope_id);
    }
  }

  void PerformScopeCleanupPostcheck(uint16_t scope_id) {
    if (IsScopeCleanupCheckerEnabled()) {
      CreateAssertScopeCleanupStackHasItem();
      auto* popped = CreatePopFromScopeCleanupStack();
      CreateAssertScopeCleanupStackPoppedValue(popped, scope_id);
    }
  }

 private:
  static constexpr uint8_t kValueKindUndefined = static_cast<uint8_t>(ValueKind::Undefined);
  static constexpr uint8_t kValueKindNull = static_cast<uint8_t>(ValueKind::Null);
  static constexpr uint8_t kValueKindBoolean = static_cast<uint8_t>(ValueKind::Boolean);
  static constexpr uint8_t kValueKindNumber = static_cast<uint8_t>(ValueKind::Number);
  static constexpr uint8_t kValueKindClosure = static_cast<uint8_t>(ValueKind::Closure);

  void CreateStore(llvm::Value* value, llvm::Value* dest) {
    builder_->CreateStore(value, dest);
  }

  // 6.1.6.1.16 NumberBitwiseOp ( op, x, y )
  llvm::Value* NumberBitwiseOp(char op, llvm::Value* x, llvm::Value* y) {
    auto* lint = ToInt32(x);
    auto* rint = ToInt32(y);
    llvm::Value* oint;
    switch (op) {
      case '&':
        oint = builder_->CreateAnd(lint, rint, REG_NAME("and"));
        break;
      case '^':
        oint = builder_->CreateXor(lint, rint, REG_NAME("xor"));
        break;
      case '|':
        oint = builder_->CreateOr(lint, rint, REG_NAME("or"));
        break;
      default:
        assert(false);
        oint = nullptr;
        break;
    }
    return builder_->CreateSIToFP(oint, builder_->getDoubleTy(), REG_NAME("si2fp"));
  }

  // 7.1.6 ToInt32 ( argument )
  llvm::Value* ToInt32(llvm::Value* number) {
    // Skip the first step.
    // We assumed that `number` holds a number value.
    // TODO: Create inline instructions if runtime_to_int32() is slow.
    auto* func = types_->CreateRuntimeToInt32();
    return builder_->CreateCall(func, {exec_context_, number}, REG_NAME("int32"));
  }

  // 7.1.7 ToUint32 ( argument )
  llvm::Value* ToUint32(llvm::Value* number) {
    // Skip the first step.
    // We assumed that `number` holds a number value.
    // TODO: Create inline instructions if runtime_to_uint32() is slow.
    auto* func = types_->CreateRuntimeToUint32();
    return builder_->CreateCall(func, {exec_context_, number}, REG_NAME("uint32"));
  }

  llvm::AllocaInst* CreateAlloc1(llvm::Type* ty, const llvm::Twine& name) {
    auto* backup = builder_->GetInsertBlock();
    builder_->SetInsertPoint(locals_block_);
    auto* alloca = builder_->CreateAlloca(ty, nullptr, name);
    builder_->SetInsertPoint(backup);
    return alloca;
  }

  llvm::AllocaInst* CreateAllocN(llvm::Type* ty, uint32_t n, const llvm::Twine& name) {
    auto* backup = builder_->GetInsertBlock();
    builder_->SetInsertPoint(locals_block_);
    auto* alloca = builder_->CreateAlloca(ty, builder_->getInt32(n), name);
    builder_->SetInsertPoint(backup);
    return alloca;
  }

  llvm::Function* CreateLambda(const char* name) {
    const auto& found = functions_.find(name);
    if (found != functions_.end()) {
      return found->second;
    }

    auto* prototype = types_->CreateLambdaType();
    auto* lambda =
        llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, name, *module_);
    lambda->getArg(0)->setName(REG_NAME("ctx"));
    lambda->getArg(1)->setName(REG_NAME("caps"));
    lambda->getArg(2)->setName(REG_NAME("argc"));
    lambda->getArg(3)->setName(REG_NAME("argv"));
    lambda->getArg(4)->setName(REG_NAME("retv"));
    functions_[name] = lambda;
    return lambda;
  }

  // Naming convention for field accessors:
  //
  //   CreateGet<field>PtrOf<type>(ptr)
  //     Create instructions to get a pointer of the <field> of <type>.
  //
  //   CreateLoad<type>(ptr)
  //     Create instructions to load the value.
  //
  //   CreateLoad<field>From<type>(ptr)
  //     Create instructions to load the value of the <field> of <type>.
  //
  //   CreateStore<field>To<type>(value, ptr)
  //     Create instructions to store a value to the <field> of <type>.

  // value

  inline llvm::Value* CreateGetValueKindPtrOfValue(llvm::Value* value_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateValueType(), value_ptr, 0, REG_NAME("value.kind.ptr"));
  }

  inline llvm::Value* CreateGetValueHolderPtrOfValue(llvm::Value* value_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateValueType(), value_ptr, 1, REG_NAME("value.holder.ptr"));
  }

  inline llvm::Value* CreateLoadValueKindFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueKindPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt8Ty(), ptr, REG_NAME("value.kind"));
  }

  inline llvm::Value* CreateLoadValueHolderFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt64Ty(), ptr, REG_NAME("value.holder"));
  }

  inline llvm::Value* CreateLoadBooleanFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt1Ty(), ptr, REG_NAME("value.boolean"));
  }

  inline llvm::Value* CreateLoadNumberFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getDoubleTy(), ptr, REG_NAME("value.number"));
  }

  inline void CreateStoreValueKindToValue(ValueKind value, llvm::Value* dest) {
    CreateStoreValueKindToValue(builder_->getInt8(static_cast<uint8_t>(value)), dest);
  }

  inline void CreateStoreValueKindToValue(llvm::Value* value, llvm::Value* dest) {
    auto* ptr = CreateGetValueKindPtrOfValue(dest);
    builder_->CreateStore(value, ptr);
  }

  inline void CreateStoreValueHolderToValue(llvm::Value* holder, llvm::Value* dest) {
    auto* ptr = CreateGetValueHolderPtrOfValue(dest);
    builder_->CreateStore(holder, ptr);
  }

  // closure

  inline llvm::Value* CreateGetLambdaPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateClosureType(), closure_ptr, 0, REG_NAME("closure.lambda.ptr"));
  }

  inline llvm::Value* CreateGetNumCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateClosureType(), closure_ptr, 1, REG_NAME("closure.num_captures.ptr"));
  }

  inline llvm::Value* CreateGetCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateClosureType(), closure_ptr, 2, REG_NAME("closure.captures.ptr"));
  }

  inline llvm::Value* CreateLoadLambdaFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetLambdaPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("closure.lambda"));
  }

  inline llvm::Value* CreateLoadNumCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetNumCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getInt16Ty(), ptr, REG_NAME("closure.num_captures"));
  }

  inline llvm::Value* CreateLoadCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("closure.captures"));
  }

  // capture

  inline llvm::Value* CreateGetTargetPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateCaptureType(), capture_ptr, 0, REG_NAME("capture.target.ptr"));
  }

  inline llvm::Value* CreateGetEscapedPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(
        types_->CreateCaptureType(), capture_ptr, 1, REG_NAME("capture.escaped.ptr"));
  }

  inline llvm::Value* CreateLoadTargetFromCapture(llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("capture.target"));
  }

  inline void CreateStoreTargetToCapture(llvm::Value* value_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    builder_->CreateStore(value_ptr, ptr);
  }

  inline void CreateStoreEscapedToCapture(llvm::Value* value_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetEscapedPtrOfCapture(capture_ptr);
    auto align = llvm::Align(sizeof(double));
    builder_->CreateMemCpy(ptr, align, value_ptr, align, types_->GetWord(sizeof(Value)));
  }

  // captures

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

  // scope cleanup cheker

  void CreatePushOntoScopeCleanupStack(uint16_t scope_id) {
    auto* top = CreateLoadScopeCleanupStackTop();
    // scope_cleanup_stack_[scope_cleanup_stack_top_] = scope_id;
    auto* ptr = builder_->CreateInBoundsGEP(scope_cleanup_stack_type_, scope_cleanup_stack_,
        {builder_->getInt32(0), top}, REG_NAME("scope_cleanup_stack.pushed.ptr"));
    builder_->CreateStore(builder_->getInt16(scope_id), ptr);
    // scope_cleanup_stack_top_++;
    auto* incr =
        builder_->CreateAdd(top, builder_->getInt32(1), REG_NAME("scope_cleanup_stack.top.incr"));
    CreateStoreScopeCleanupStackTop(incr);
  }

  llvm::Value* CreatePopFromScopeCleanupStack() {
    auto* top = CreateLoadScopeCleanupStackTop();
    // scope_cleanup_stack_top_--;
    auto* decr =
        builder_->CreateSub(top, builder_->getInt32(1), REG_NAME("scope_cleanup_stack.top.decr"));
    CreateStoreScopeCleanupStackTop(decr);
    // return scope_cleanup_stack_[scope_cleanup_stack_top_];
    auto* ptr = builder_->CreateInBoundsGEP(scope_cleanup_stack_type_, scope_cleanup_stack_,
        {builder_->getInt32(0), decr}, REG_NAME("scope_cleanup_stack.popped.ptr"));
    return builder_->CreateLoad(
        builder_->getInt16Ty(), ptr, REG_NAME("scope_cleanup_stack.popped"));
  }

  // assert(scope_cleanup_stack_top_ <= scope_cleanup_stack_size_);
  void CreateAssertScopeCleanupStackBounds() {
    auto* top = CreateLoadScopeCleanupStackTop();
    auto* assertion = builder_->CreateICmpULE(top, builder_->getInt32(scope_cleanup_stack_size_),
        REG_NAME("assertion.scope_cleanup_stack.size"));
    auto* msg = builder_->CreateGlobalString(
        "assertion failure: scope_cleanup_stack_top_ <= scoke_cleanup_stack_size_",
        REG_NAME("assertion.msg.scope_cleanup_stack.size"));
    CreateAssert(assertion, msg);
  }

  // assert(popped == scope_id);
  void CreateAssertScopeCleanupStackPoppedValue(llvm::Value* actual, uint16_t expected) {
    auto* assertion = builder_->CreateICmpEQ(
        actual, builder_->getInt16(expected), REG_NAME("assertion.scope_cleanup_stack.popped"));
    std::stringstream ss;
    ss << "assertion failure: popped == " << expected;
    auto* msg = builder_->CreateGlobalString(
        ss.str(), REG_NAME("assertion.msg.scope_cleanup_stack.popped"));
    CreateAssert(assertion, msg);
  }

  // assert(scope_cleanup_stack_top_ == 0);
  void CreateAssertScopeCleanupStackIsEmpty() {
    auto* top = CreateLoadScopeCleanupStackTop();
    auto* assertion = builder_->CreateICmpEQ(
        top, builder_->getInt32(0), REG_NAME("assertion.scope_cleanup_stack.is_empty"));
    auto* msg = builder_->CreateGlobalString("assertion failure: scope_cleanup_stack_top_ == 0",
        REG_NAME("assertion.msg.scope_cleanup_stack.is_empty"));
    CreateAssert(assertion, msg);
  }

  // assert(scope_cleanup_stack_top_ != 0);
  void CreateAssertScopeCleanupStackHasItem() {
    auto* top = CreateLoadScopeCleanupStackTop();
    auto* assertion = builder_->CreateICmpNE(
        top, builder_->getInt32(0), REG_NAME("assertion.scope_cleanup_stack.has_item"));
    auto* msg = builder_->CreateGlobalString("assertion failure: scope_cleanup_stack_top_ != 0",
        REG_NAME("assertion.msg.scope_cleanup_stack.has_item"));
    CreateAssert(assertion, msg);
  }

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

  // helpers

  void CreateAssert(llvm::Value* assertion, llvm::Value* msg) {
    auto* func = types_->CreateRuntimeAssert();
    builder_->CreateCall(func, {exec_context_, assertion, msg});
  }

  // TODO: separate values that must be reset in EndFunction() from others.

  std::unique_ptr<llvm::LLVMContext> context_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;

  // The following values are reset for each function.
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* locals_block_ = nullptr;
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
