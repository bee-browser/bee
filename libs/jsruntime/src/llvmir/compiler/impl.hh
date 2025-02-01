#pragma once

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <memory>
#include <sstream>
#include <string>
#include <unordered_map>

#include "llvm/ADT/ArrayRef.h"

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

#include "../module/impl.hh"
#include "type_holder.hh"

#define UNUSED(var) ((void)(var))
#define REG_NAME(expr) (enable_labels_ ? expr : "")

#define STATUS_UNSET_BIT 0x10
#define STATUS_MASK 0x0F
#define STATUS_NORMAL 0x00
#define STATUS_EXCEPTION 0x01
#define STATUS_SUSPEND 0x02
#define STATUS_UNSET (STATUS_UNSET_BIT | STATUS_NORMAL)

// DO NOT CHANGE THE FOLLOWING VALUES.
// The implementation heavily depends on the values.
#define FLOW_SELECTOR_KIND_RETURN 0x00000000
#define FLOW_SELECTOR_KIND_THROW 0x00000001
#define FLOW_SELECTOR_KIND_BREAK 0x00000002
#define FLOW_SELECTOR_KIND_CONTINUE 0x00000003
#define FLOW_SELECTOR_KIND_NORMAL 0x000000FF

#define FLOW_SELECTOR_WEIGHT_MASK 0x0000FF00

#define DEFINE_FLOW_SELECTOR(extra, depth, kind) ((extra) | (depth) | FLOW_SELECTOR_KIND_##kind)

#define FLOW_SELECTOR_NORMAL DEFINE_FLOW_SELECTOR(0x00010000, 0x0000FF00, NORMAL)
#define FLOW_SELECTOR_RETURN DEFINE_FLOW_SELECTOR(0x00000000, 0x00000000, RETURN)
#define FLOW_SELECTOR_THROW DEFINE_FLOW_SELECTOR(0x00000000, 0x00000000, THROW)
#define FLOW_SELECTOR_BREAK(depth) DEFINE_FLOW_SELECTOR(0x00000000, depth, BREAK)
#define FLOW_SELECTOR_CONTINUE(depth) DEFINE_FLOW_SELECTOR(0x00000000, depth, CONTINUE)

class Compiler {
 public:
  Compiler() {
    llvmctx_ = std::make_unique<llvm::LLVMContext>();
    module_ = std::make_unique<llvm::Module>("<main>", *llvmctx_);
    builder_ = std::make_unique<llvm::IRBuilder<>>(*llvmctx_);
    types_ = std::make_unique<TypeHolder>(*llvmctx_, *module_, *builder_);

    // Took from toy.cpp in the Kaleidoscope tutorial.
    fpm_ = std::make_unique<llvm::FunctionPassManager>();
    lam_ = std::make_unique<llvm::LoopAnalysisManager>();
    fam_ = std::make_unique<llvm::FunctionAnalysisManager>();
    cgam_ = std::make_unique<llvm::CGSCCAnalysisManager>();
    mam_ = std::make_unique<llvm::ModuleAnalysisManager>();
    pic_ = std::make_unique<llvm::PassInstrumentationCallbacks>();
    si_ = std::make_unique<llvm::StandardInstrumentations>(*llvmctx_, true);  // with debug logs
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

    llvm::orc::ThreadSafeModule mod(std::move(module_), std::move(llvmctx_));
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

  void StartFunction(uint32_t func_id) {
    function_ = CreateLambda(func_id);

    runtime_ = function_->getArg(0);
    context_ = function_->getArg(1);
    argc_ = function_->getArg(2);
    argv_ = function_->getArg(3);
    retv_ = function_->getArg(4);

    // captures_ will be overridden if this lambda function is a coroutine.
    captures_ = context_;

    ResetScopeCleanupChecker();
  }

  void EndFunction(bool optimize) {
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

  llvm::Function* GetFunction(uint32_t func_id) {
    return CreateLambda(func_id);
  }

  // basic block

  llvm::BasicBlock* CreateBasicBlock(const char* name) {
    return llvm::BasicBlock::Create(*llvmctx_, name, function_);
  }

  llvm::BasicBlock* CreateBasicBlock(const char* name, size_t name_len) {
    return llvm::BasicBlock::Create(*llvmctx_, llvm::Twine(llvm::StringRef(name, name_len)),
                                    function_);
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

  // undefined

  llvm::Value* CreateIsUndefined(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindUndefined),
                                  REG_NAME("is_undefined"));
  }

  // null

  llvm::Value* CreateIsNull(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindNull), REG_NAME("is_null"));
  }

  llvm::Value* CreateIsNonNullish(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpUGT(kind, builder_->getInt8(kValueKindNull),
                                   REG_NAME("is_non_nullish"));
  }

  // boolean

  llvm::Value* CreateIsBoolean(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindBoolean),
                                  REG_NAME("is_boolean"));
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
    return builder_->CreateCall(func, {runtime_, value_ptr}, REG_NAME("boolean"));
  }

  llvm::Value* GetBoolean(bool value) {
    return llvm::ConstantInt::getBool(*llvmctx_, value);
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
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindNumber),
                                  REG_NAME("is_number"));
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
    return builder_->CreateCall(call, {runtime_, value_ptr}, REG_NAME("numeric"));
  }

  llvm::Value* GetNan() {
    return llvm::ConstantFP::getNaN(builder_->getDoubleTy());
  }

  llvm::Value* GetZero() {
    return llvm::ConstantFP::getZero(builder_->getDoubleTy());
  }

  llvm::Value* GetNumber(double value) {
    return llvm::ConstantFP::get(*llvmctx_, llvm::APFloat(value));
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

  // string - chain of `Char16Seq`s

  llvm::Value* CreateChar16Seq(const uint16_t* ptr, uint32_t len) {
    // The following implementation is based on llvm::IRBuilder::CreateGlobalString().

    // Theoretically, the heap memory pointed by `ptr` can be freed after the IR built by the
    // compiler is freed.
    auto* data = llvm::ConstantDataArray::get(*llvmctx_, llvm::ArrayRef(ptr, len));

    // Define a global variable for `data` w/ private linkage.
    auto* gv = new llvm::GlobalVariable(*module_, data->getType(), true,
                                        llvm::GlobalVariable::PrivateLinkage, data,
                                        REG_NAME("global.char16.ptr"));
    gv->setUnnamedAddr(llvm::GlobalValue::UnnamedAddr::Global);
    gv->setAlignment(llvm::Align(2));

    // Allocate a Char16Seq on the stack.
    auto* seq_ptr = CreateAlloc1(types_->CreateChar16SeqType(), REG_NAME("char16_seq.ptr"));
    CreateStoreNextToChar16Seq(GetNullptr(), seq_ptr);
    CreateStorePtrToChar16Seq(gv, seq_ptr);
    CreateStoreLenToChar16Seq(builder_->getInt32(len), seq_ptr);

    return seq_ptr;
  }

  void CreateStoreNextToChar16Seq(llvm::Value* value, llvm::Value* dest) {
    auto* ptr = CreateGetNextPtrOfChar16Seq(dest);
    builder_->CreateStore(value, ptr);
  }

  void CreateStorePtrToChar16Seq(llvm::Value* value, llvm::Value* dest) {
    auto* ptr = CreateGetPtrPtrOfChar16Seq(dest);
    builder_->CreateStore(value, ptr);
  }

  void CreateStoreLenToChar16Seq(llvm::Value* value, llvm::Value* dest) {
    auto* ptr = CreateGetLenPtrOfChar16Seq(dest);
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateGetNextPtrOfChar16Seq(llvm::Value* seq_ptr) {
    return builder_->CreateStructGEP(types_->CreateChar16SeqType(), seq_ptr, 0,
                                     REG_NAME("char16_seq.next.ptr"));
  }

  llvm::Value* CreateGetPtrPtrOfChar16Seq(llvm::Value* seq_ptr) {
    return builder_->CreateStructGEP(types_->CreateChar16SeqType(), seq_ptr, 1,
                                     REG_NAME("char16_seq.ptr.ptr"));
  }

  llvm::Value* CreateGetLenPtrOfChar16Seq(llvm::Value* seq_ptr) {
    return builder_->CreateStructGEP(types_->CreateChar16SeqType(), seq_ptr, 2,
                                     REG_NAME("char16_seq.len.ptr"));
  }

  // pointer

  llvm::Value* GetNullptr() {
    return llvm::Constant::getNullValue(builder_->getPtrTy());
  }

  // closure

  llvm::Value* CreateIsClosure(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindClosure),
                                  REG_NAME("is_closure"));
  }

  llvm::Value* CreateIsSameClosure(llvm::Value* a, llvm::Value* b) {
    return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_closure"));
  }

  llvm::Value* CreateClosure(llvm::Value* lambda, uint16_t num_captures) {
    auto* func = types_->CreateRuntimeCreateClosure();
    return builder_->CreateCall(func, {runtime_, lambda, builder_->getInt16(num_captures)},
                                REG_NAME("closure.ptr"));
  }

  void CreateStoreCapturePtrToClosure(llvm::Value* capture_ptr,
                                      llvm::Value* closure_ptr,
                                      uint16_t index) {
    auto* ptr = CreateGetCapturesPtrOfClosure(closure_ptr);
    CreateStoreCapturePtrToCaptures(capture_ptr, ptr, index);
  }

  llvm::Value* CreateCallOnClosure(llvm::Value* closure,
                                   uint16_t argc,
                                   llvm::Value* argv,
                                   llvm::Value* retv) {
    auto* prototype = types_->CreateLambdaType();
    auto* lambda = CreateLoadLambdaFromClosure(closure);
    auto* context = CreateGetCapturesPtrOfClosure(closure);
    return builder_->CreateCall(prototype, lambda,
                                {runtime_, context, types_->GetWord(argc), argv, retv},
                                REG_NAME("status"));
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

  // promise

  llvm::Value* CreateIsPromise(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindPromise),
                                  REG_NAME("is_promise"));
  }

  llvm::Value* CreateIsSamePromise(llvm::Value* a, llvm::Value* b) {
    return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_promise"));
  }

  llvm::Value* CreateRegisterPromise(llvm::Value* coroutine) {
    auto* func = types_->CreateRuntimeRegisterPromise();
    return builder_->CreateCall(func, {runtime_, coroutine}, REG_NAME("promise"));
  }

  void CreateAwaitPromise(llvm::Value* promise, llvm::Value* awaiting) {
    auto* func = types_->CreateRuntimeAwaitPromise();
    builder_->CreateCall(func, {runtime_, promise, awaiting});
  }

  void CreateResume(llvm::Value* promise) {
    auto* func = types_->CreateRuntimeResume();
    builder_->CreateCall(func, {runtime_, promise});
  }

  void CreateEmitPromiseResolved(llvm::Value* promise, llvm::Value* result) {
    auto* func = types_->CreateRuntimeEmitPromiseResolved();
    builder_->CreateCall(func, {runtime_, promise, result});
  }

  // object

  llvm::Value* CreateIsObject(llvm::Value* value_ptr) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindObject),
                                  REG_NAME("is_object"));
  }

  llvm::Value* CreateIsSameObject(llvm::Value* a, llvm::Value* b) {
    return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_object"));
  }

  llvm::Value* CreateToObject(llvm::Value* value) {
    auto* func = types_->CreateRuntimeToObject();
    return builder_->CreateCall(func, {runtime_, value}, REG_NAME("to_object"));
  }

  llvm::Value* CreateObject() {
    auto* func = types_->CreateRuntimeCreateObject();
    return builder_->CreateCall(func, {runtime_}, REG_NAME("object.ptr"));
  }

  // value

  llvm::Value* CreateValueIsNullptr(llvm::Value* value) {
    return builder_->CreateICmpEQ(value, GetNullptr(), REG_NAME("value.is_nullptr"));
  }

  llvm::Value* CreateAllocValue() {
    return CreateAlloc1(types_->CreateValueType(), REG_NAME("value.ptr"));
  }

  llvm::Value* CreateHasValue(llvm::Value* value) {
    auto* kind = CreateLoadValueKindFromValue(value);
    return builder_->CreateICmpNE(kind, builder_->getInt8(kValueKindNone),
                                  REG_NAME("value.has_value"));
  }

  // 7.2.13 IsLooselyEqual ( x, y )
  llvm::Value* CreateIsLooselyEqual(llvm::Value* x, llvm::Value* y) {
    // TODO: Create inline instructions if runtime_is_loosely_equal() is slow.
    auto* func = types_->CreateRuntimeIsLooselyEqual();
    return builder_->CreateCall(func, {runtime_, x, y}, REG_NAME("is_loosely_equal.retval"));
  }

  // 7.2.14 IsStrictlyEqual ( x, y )
  llvm::Value* CreateIsStrictlyEqual(llvm::Value* x, llvm::Value* y) {
    // TODO: Create inline instructions if runtime_is_strictly_equal() is slow.
    auto* func = types_->CreateRuntimeIsStrictlyEqual();
    return builder_->CreateCall(func, {runtime_, x, y}, REG_NAME("is_strictly_equal.retval"));
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

  llvm::Value* CreateIsSamePromiseValue(llvm::Value* value_ptr, llvm::Value* promise) {
    auto* value = CreateLoadPromiseFromValue(value_ptr);
    return builder_->CreateICmpEQ(value, promise, REG_NAME("is_same_promise_value"));
  }

  llvm::Value* CreateIsSameObjectValue(llvm::Value* value_ptr, llvm::Value* promise) {
    auto* value = CreateLoadObjectFromValue(value_ptr);
    return builder_->CreateICmpEQ(value, promise, REG_NAME("is_same_object_value"));
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

  llvm::Value* CreateStringToAny(llvm::Value* string) {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    // TODO: this is safe only the value used within the function.
    // returning it as the return value is unsafe.
    CreateStoreStringToValue(string, value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateClosureToAny(llvm::Value* closure) {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreClosureToValue(closure, value_ptr);
    return value_ptr;
  }

  llvm::Value* CreateObjectToAny(llvm::Value* object) {
    auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
    CreateStoreObjectToValue(object, value_ptr);
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
    return CreateAlloc1(types_->CreateValueType(),
                        REG_NAME("local" + llvm::Twine(index) + ".ptr"));
  }

  void CreateStoreNoneToValue(llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindNone, dest);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), dest);
  }

  void CreateStoreUndefinedToValue(llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindUndefined, dest);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), dest);
  }

  void CreateStoreNullToValue(llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindNull, dest);
    // zeroinitializer can be used in optimization by filling the holder with zero.
    CreateStoreValueHolderToValue(builder_->getInt64(0), dest);
  }

  void CreateStoreBooleanToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindBoolean, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  void CreateStoreNumberToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindNumber, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  void CreateStoreStringToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindString, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  void CreateStoreClosureToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindClosure, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  void CreateStorePromiseToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindPromise, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  void CreateStoreObjectToValue(llvm::Value* value, llvm::Value* dest) {
    CreateStoreValueKindToValue(kValueKindObject, dest);
    CreateStoreValueHolderToValue(value, dest);
  }

  void CreateStoreValueToValue(llvm::Value* value_ptr, llvm::Value* dest) {
    auto* kind = CreateLoadValueKindFromValue(value_ptr);
    CreateStoreValueKindToValue(kind, dest);
    auto* holder = CreateLoadValueHolderFromValue(value_ptr);
    CreateStoreValueHolderToValue(holder, dest);
  }

  llvm::Value* CreateLoadBooleanFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt1Ty(), ptr, REG_NAME("value.boolean"));
  }

  llvm::Value* CreateLoadClosureFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("value.closure"));
  }

  llvm::Value* CreateLoadPromiseFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt32Ty(), ptr, REG_NAME("value.promise"));
  }

  llvm::Value* CreateLoadObjectFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt32Ty(), ptr, REG_NAME("value.object"));
  }

  llvm::Value* CreateTypeof(llvm::Value* value_ptr) {
    auto* func = types_->CreateRuntimeGetTypeof();
    return builder_->CreateCall(func, {runtime_, value_ptr});
  }

  // argv

  llvm::Value* CreateArgv(uint16_t argc) {
    assert(argc > 0);
    return CreateAllocN(types_->CreateValueType(), argc, REG_NAME("argv.ptr"));
  }

  llvm::Value* CreateGetArgInArgv(llvm::Value* argv, uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateValueType(), argv, index,
                                                REG_NAME("argv." + llvm::Twine(index) + ".ptr"));
  }

  llvm::Value* CreateGetArgumentValuePtr(uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(types_->CreateValueType(), argv_, index,
                                                REG_NAME("argv." + llvm::Twine(index) + ".ptr"));
  }

  // retv

  llvm::Value* CreateRetv() {
    return CreateAlloc1(types_->CreateValueType(), REG_NAME("retv.ptr"));
  }

  void CreateStoreUndefinedToRetv() {
    CreateStoreUndefinedToValue(retv_);
  }

  void CreateStoreNullToRetv() {
    CreateStoreNullToValue(retv_);
  }

  void CreateStoreBooleanToRetv(llvm::Value* value) {
    CreateStoreBooleanToValue(value, retv_);
  }

  void CreateStoreNumberToRetv(llvm::Value* value) {
    CreateStoreNumberToValue(value, retv_);
  }

  void CreateStoreStringToRetv(llvm::Value* value) {
    // TODO: make sure that the string value is allocated in the heap.
    CreateStoreStringToValue(value, retv_);
  }

  void CreateStoreClosureToRetv(llvm::Value* value) {
    CreateStoreClosureToValue(value, retv_);
  }

  void CreateStorePromiseToRetv(llvm::Value* value) {
    CreateStorePromiseToValue(value, retv_);
  }

  void CreateStoreObjectToRetv(llvm::Value* value) {
    CreateStoreObjectToValue(value, retv_);
  }

  void CreateStoreValueToRetv(llvm::Value* value) {
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

  llvm::Value* CreateIsNormalStatus(llvm::Value* status) {
    return builder_->CreateICmpEQ(status, builder_->getInt32(STATUS_NORMAL),
                                  REG_NAME("status.is_normal"));
  }

  llvm::Value* CreateIsExceptionStatus(llvm::Value* status) {
    return builder_->CreateICmpEQ(status, builder_->getInt32(STATUS_EXCEPTION),
                                  REG_NAME("status.is_exception"));
  }

  // flow selector

  void CreateAllocFlowSelector() {
    flow_selector_ = CreateAlloc1(builder_->getInt32Ty(), REG_NAME("flow_selector.ptr"));
    builder_->CreateStore(builder_->getInt32(FLOW_SELECTOR_NORMAL), flow_selector_);
  }

  void CreateSetFlowSelectorNormal() {
    builder_->CreateStore(builder_->getInt32(FLOW_SELECTOR_NORMAL), flow_selector_);
  }

  void CreateSetFlowSelectorReturn() {
    builder_->CreateStore(builder_->getInt32(FLOW_SELECTOR_RETURN), flow_selector_);
  }

  void CreateSetFlowSelectorThrow() {
    builder_->CreateStore(builder_->getInt32(FLOW_SELECTOR_THROW), flow_selector_);
  }

  void CreateSetFlowSelectorBreak(uint32_t depth) {
    builder_->CreateStore(builder_->getInt32(FLOW_SELECTOR_BREAK(depth)), flow_selector_);
  }

  void CreateSetFlowSelectorContinue(uint32_t depth) {
    builder_->CreateStore(builder_->getInt32(FLOW_SELECTOR_CONTINUE(depth)), flow_selector_);
  }

  llvm::Value* CreateIsFlowSelectorNormal() {
    auto* value =
        builder_->CreateLoad(builder_->getInt32Ty(), flow_selector_, REG_NAME("flow_selector"));
    return builder_->CreateICmpEQ(value, builder_->getInt32(FLOW_SELECTOR_NORMAL),
                                  REG_NAME("flow_selector.is_normal"));
  }

  llvm::Value* CreateIsFlowSelectorNormalOrContinue(uint32_t depth) {
    auto* value =
        builder_->CreateLoad(builder_->getInt32Ty(), flow_selector_, REG_NAME("flow_selector"));
    return builder_->CreateICmpUGT(value, builder_->getInt32(FLOW_SELECTOR_BREAK(depth)),
                                   REG_NAME("flow_selector.is_normal_or_continue"));
  }

  llvm::Value* CreateIsFlowSelectorBreakOrContinue(uint32_t depth) {
    auto* value =
        builder_->CreateLoad(builder_->getInt32Ty(), flow_selector_, REG_NAME("flow_selector"));
    auto* value_depth = builder_->CreateAnd(value, builder_->getInt32(FLOW_SELECTOR_WEIGHT_MASK),
                                            REG_NAME("flow_selector.depth"));
    return builder_->CreateICmpEQ(value_depth, builder_->getInt32(depth),
                                  REG_NAME("flow_selector.is_break_or_continue"));
  }

  llvm::Value* CreateIsFlowSelectorBreak(uint32_t depth) {
    auto* value =
        builder_->CreateLoad(builder_->getInt32Ty(), flow_selector_, REG_NAME("flow_selector"));
    auto* value_depth = builder_->CreateAnd(value, builder_->getInt32(FLOW_SELECTOR_WEIGHT_MASK),
                                            REG_NAME("flow_selector.depth"));
    return builder_->CreateICmpEQ(value_depth, builder_->getInt32(depth),
                                  REG_NAME("flow_selector.is_break"));
  }

  // capture

  llvm::Value* CreateCapture(llvm::Value* value_ptr) {
    auto* func = types_->CreateRuntimeCreateCapture();
    return builder_->CreateCall(func, {runtime_, value_ptr}, REG_NAME("capture.ptr"));
  }

  void CreateEscapeValue(llvm::Value* capture, llvm::Value* value) {
    auto* escaped_ptr = CreateGetEscapedPtrOfCapture(capture);
    CreateStoreTargetToCapture(escaped_ptr, capture);
    CreateMemCpyValue(escaped_ptr, value);
  }

  llvm::Value* CreateGetCaptureValuePtr(uint16_t index) {
    auto* ptr = CreateLoadCapturePtrFromCaptures(captures_, index);
    return CreateLoadTargetFromCapture(ptr);
  }

  llvm::Value* CreateLoadCapture(uintptr_t index) {
    return CreateLoadCapturePtrFromCaptures(captures_, index);
  }

  // coroutine

  llvm::Value* CreateCoroutine(llvm::Value* closure,
                               uint16_t num_locals,
                               uint16_t scratch_buffer_len) {
    auto* func = types_->CreateRuntimeCreateCoroutine();
    return builder_->CreateCall(func,
                                {runtime_, closure, builder_->getInt16(num_locals),
                                 builder_->getInt16(scratch_buffer_len)},
                                REG_NAME("coroutine"));
  }

  llvm::SwitchInst* CreateSwitchForCoroutine(llvm::BasicBlock* block, uint32_t num_states) {
    auto* state = CreateLoadStateFromCoroutine();
    return builder_->CreateSwitch(state, block, num_states);
  }

  void CreateAddStateForCoroutine(llvm::SwitchInst* inst,
                                  uint32_t state,
                                  llvm::BasicBlock* block) {
    inst->addCase(builder_->getInt32(state), block);
  }

  void CreateSuspend() {
    builder_->CreateRet(builder_->getInt32(STATUS_SUSPEND));
  }

  void CreateSetCoroutineState(uint32_t state) {
    auto* ptr = CreateGetStatePtrOfCoroutine();
    builder_->CreateStore(builder_->getInt32(state), ptr);
  }

  void CreateSetCapturesForCoroutine() {
    captures_ = CreateGetCapturesPtrOfCoroutine();
  }

  llvm::Value* CreateGetLocalPtrFromCoroutine(uint16_t index) {
    auto* ptr = CreateGetLocalsPtrOfCoroutine();
    return builder_->CreateConstInBoundsGEP1_32(
        types_->CreateValueType(), ptr, index,
        REG_NAME("co.locals." + llvm::Twine(index) + ".ptr"));
  }

  void CreateWriteBooleanToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.boolean.ptr"));
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateReadBooleanFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.boolean.ptr"));
    return builder_->CreateLoad(builder_->getInt1Ty(), ptr, REG_NAME("scratch.boolean"));
  }

  void CreateWriteNumberToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.number.ptr"));
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateReadNumberFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.number.ptr"));
    return builder_->CreateLoad(builder_->getDoubleTy(), ptr, REG_NAME("scratch.number"));
  }

  void CreateWriteStringToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.string.ptr"));
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateReadStringFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.string.ptr"));
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("scratch.string"));
  }

  void CreateWriteClosureToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.closure.ptr"));
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateReadClosureFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.closure.ptr"));
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("scratch.closure"));
  }

  void CreateWriteObjectToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.object.ptr"));
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateReadObjectFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.object.ptr"));
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("scratch.object"));
  }

  void CreateWritePromiseToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.promise.ptr"));
    builder_->CreateStore(value, ptr);
  }

  llvm::Value* CreateReadPromiseFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.promise.ptr"));
    return builder_->CreateLoad(builder_->getInt32Ty(), ptr, REG_NAME("scratch.promise"));
  }

  void CreateWriteValueToScratchBuffer(uint32_t offset, llvm::Value* value) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    auto* ptr = builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                               REG_NAME("scratch.value.ptr"));
    CreateStoreValueToValue(value, ptr);
  }

  llvm::Value* CreateReadValueFromScratchBuffer(uint32_t offset) {
    auto* scratch_ptr = CreateGetScratchBufferPtrOfCoroutine();
    return builder_->CreateInBoundsPtrAdd(scratch_ptr, builder_->getInt32(offset),
                                          REG_NAME("scratch.value.ptr"));
  }

  // object

  llvm::Value* CreateGetValue(llvm::Value* object, uint32_t key, bool strict) {
    auto* func = types_->CreateRuntimeGetValue();
    return builder_->CreateCall(
        func, {runtime_, object, builder_->getInt32(key), builder_->getInt1(strict)},
        REG_NAME("runtime.get_value.value.ptr"));
  }

  void CreateSetValue(llvm::Value* object, uint32_t key, llvm::Value* value) {
    auto* func = types_->CreateRuntimeSetValue();
    builder_->CreateCall(func, {runtime_, object, builder_->getInt32(key), value});
  }

  // 7.3.5 CreateDataProperty ( O, P, V )
  llvm::Value* CreateCreateDataProperty(llvm::Value* object,
                                        uint32_t key,
                                        llvm::Value* value,
                                        llvm::Value* retv) {
    auto* func = types_->CreateRuntimeCreateDataProperty();
    return builder_->CreateCall(func, {runtime_, object, builder_->getInt32(key), value, retv},
                                REG_NAME("runtime.create_data_property.status.ptr"));
  }

  // 7.3.25 CopyDataProperties ( target, source, excludedItems )
  llvm::Value* CreateCopyDataProperties(llvm::Value* target,
                                        llvm::Value* source,
                                        llvm::Value* retv) {
    auto* func = types_->CreateRuntimeCopyDataProperties();
    return builder_->CreateCall(func, {runtime_, target, source, retv},
                                REG_NAME("runtime.copy_data_properties.status.ptr"));
  }

  // scope cleanup checker

  void EnableScopeCleanupChecker(bool is_coroutine) {
    if (is_coroutine) {
      scope_id_ = CreateGetScopeIdPtrOfCoroutine();
    } else {
      scope_id_ = CreateAlloc1(builder_->getInt16Ty(), REG_NAME("scope_id.ptr"));
      builder_->CreateStore(builder_->getInt16(0), scope_id_);
    }
  }

  void SetScopeIdForChecker(uint16_t scope_id) {
    assert(IsScopeCleanupCheckerEnabled());
    builder_->CreateStore(builder_->getInt16(scope_id), scope_id_);
  }

  // assert(scope_id == expected)
  void AssertScopeId(uint16_t expected) {
    assert(IsScopeCleanupCheckerEnabled());
    auto* scope_id = builder_->CreateLoad(builder_->getInt16Ty(), scope_id_, REG_NAME("scope_id"));
    auto* assertion = builder_->CreateICmpEQ(scope_id, builder_->getInt16(expected),
                                             REG_NAME("assertion.scope_id"));
    std::stringstream ss;
    ss << "scope_id == " << expected;
    CreateAssert(assertion, ss.str().c_str());
  }

  // print

  void CreatePrintString(llvm::Value* value, const char* msg = "") {
    auto* msg_value = builder_->CreateGlobalString(msg, REG_NAME("runtime.print_string.msg"));
    auto* func = types_->CreateRuntimePrintString();
    builder_->CreateCall(func, {runtime_, value, msg_value});
  }

  void CreatePrintValue(llvm::Value* value, const char* msg = "") {
    auto* msg_value = builder_->CreateGlobalString(msg, REG_NAME("runtime.print_value.msg"));
    auto* func = types_->CreateRuntimePrintValue();
    builder_->CreateCall(func, {runtime_, value, msg_value});
  }

  void CreatePrintMessage(const char* msg = "") {
    auto* msg_value = builder_->CreateGlobalString(msg, REG_NAME("runtime.print_message.msg"));
    auto* func = types_->CreateRuntimePrintMessage();
    builder_->CreateCall(func, {runtime_, msg_value});
  }

  // debugger

  void CreateDebugger() {
    auto* func = types_->CreateRuntimeLaunchDebugger();
    builder_->CreateCall(func, {runtime_});
  }

  // assertions

  void CreateAssert(llvm::Value* assertion, const char* msg = "") {
    auto* msg_value = builder_->CreateGlobalString(msg, REG_NAME("runtime.assert.msg"));
    auto* func = types_->CreateRuntimeAssert();
    builder_->CreateCall(func, {runtime_, assertion, msg_value});
  }

  void CreateUnreachable(const char* msg = "") {
    CreateAssert(builder_->getFalse(), msg);
    builder_->CreateUnreachable();
  }

  // helper functions

  // SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
  // based-on: Value::getNameOrAsOperand().
  static std::string GetNameOrAsOperand(llvm::Value* value) {
    assert(value != nullptr);

    auto name = value->getName();
    if (!name.empty()) {
      return std::string(name);
    }

    std::string buffer;
    llvm::raw_string_ostream os(buffer);
    value->printAsOperand(os);
    return buffer;
  }

  static size_t GetNameOrAsOperand(llvm::Value* value, char* buf, size_t len) {
    assert(value != nullptr);
    assert(buf != nullptr);
    assert(len > 1);
    auto s = GetNameOrAsOperand(value);
    auto nwritten = std::min(s.size(), len - 1);
    memcpy(buf, s.data(), nwritten);
    buf[nwritten] = '\0';
    return nwritten;
  }

 private:
  static constexpr uint8_t kValueKindNone = static_cast<uint8_t>(Value::Tag::None);
  static constexpr uint8_t kValueKindUndefined = static_cast<uint8_t>(Value::Tag::Undefined);
  static constexpr uint8_t kValueKindNull = static_cast<uint8_t>(Value::Tag::Null);
  static constexpr uint8_t kValueKindBoolean = static_cast<uint8_t>(Value::Tag::Boolean);
  static constexpr uint8_t kValueKindNumber = static_cast<uint8_t>(Value::Tag::Number);
  static constexpr uint8_t kValueKindString = static_cast<uint8_t>(Value::Tag::String);
  static constexpr uint8_t kValueKindClosure = static_cast<uint8_t>(Value::Tag::Closure);
  static constexpr uint8_t kValueKindPromise = static_cast<uint8_t>(Value::Tag::Promise);
  static constexpr uint8_t kValueKindObject = static_cast<uint8_t>(Value::Tag::Object);

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
    return builder_->CreateCall(func, {runtime_, number}, REG_NAME("int32"));
  }

  // 7.1.7 ToUint32 ( argument )
  llvm::Value* ToUint32(llvm::Value* number) {
    // Skip the first step.
    // We assumed that `number` holds a number value.
    // TODO: Create inline instructions if runtime_to_uint32() is slow.
    auto* func = types_->CreateRuntimeToUint32();
    return builder_->CreateCall(func, {runtime_, number}, REG_NAME("uint32"));
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

  llvm::Function* CreateLambda(uint32_t func_id) {
    const auto& found = functions_.find(func_id);
    if (found != functions_.end()) {
      return found->second;
    }

    auto* prototype = types_->CreateLambdaType();
    auto name = llvm::Twine("fn") + llvm::Twine(func_id);
    auto* lambda =
        llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, name, *module_);
    lambda->getArg(0)->setName(REG_NAME("runtime"));
    lambda->getArg(1)->setName(REG_NAME("context"));
    lambda->getArg(2)->setName(REG_NAME("argc"));
    lambda->getArg(3)->setName(REG_NAME("argv"));
    lambda->getArg(4)->setName(REG_NAME("retv"));
    functions_[func_id] = lambda;
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

  llvm::Value* CreateGetValueKindPtrOfValue(llvm::Value* value_ptr) {
    return builder_->CreateStructGEP(types_->CreateValueType(), value_ptr, 0,
                                     REG_NAME("value.kind.ptr"));
  }

  llvm::Value* CreateGetValueHolderPtrOfValue(llvm::Value* value_ptr) {
    return builder_->CreateStructGEP(types_->CreateValueType(), value_ptr, 1,
                                     REG_NAME("value.holder.ptr"));
  }

  llvm::Value* CreateLoadValueKindFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueKindPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt8Ty(), ptr, REG_NAME("value.kind"));
  }

  llvm::Value* CreateLoadValueHolderFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getInt64Ty(), ptr, REG_NAME("value.holder"));
  }

  llvm::Value* CreateLoadNumberFromValue(llvm::Value* value_ptr) {
    auto* ptr = CreateGetValueHolderPtrOfValue(value_ptr);
    return builder_->CreateLoad(builder_->getDoubleTy(), ptr, REG_NAME("value.number"));
  }

  void CreateStoreValueKindToValue(uint8_t value, llvm::Value* dest) {
    CreateStoreValueKindToValue(builder_->getInt8(value), dest);
  }

  void CreateStoreValueKindToValue(llvm::Value* value, llvm::Value* dest) {
    auto* ptr = CreateGetValueKindPtrOfValue(dest);
    builder_->CreateStore(value, ptr);
  }

  void CreateStoreValueHolderToValue(llvm::Value* holder, llvm::Value* dest) {
    auto* ptr = CreateGetValueHolderPtrOfValue(dest);
    builder_->CreateStore(holder, ptr);
  }

  void CreateMemCpyValue(llvm::Value* dst, llvm::Value* src) {
    auto align = llvm::Align(alignof(Value));
    auto* size = GetSizeofValue();
    builder_->CreateMemCpy(dst, align, src, align, size);
  }

  llvm::Value* GetSizeofValue() {
    return types_->GetWord(sizeof(Value));
  }

  // closure

  llvm::Value* CreateGetLambdaPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(types_->CreateClosureType(), closure_ptr, 0,
                                     REG_NAME("closure.lambda.ptr"));
  }

  llvm::Value* CreateGetNumCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(types_->CreateClosureType(), closure_ptr, 1,
                                     REG_NAME("closure.num_captures.ptr"));
  }

  llvm::Value* CreateGetCapturesPtrOfClosure(llvm::Value* closure_ptr) {
    return builder_->CreateStructGEP(types_->CreateClosureType(), closure_ptr, 2,
                                     REG_NAME("closure.captures.ptr"));
  }

  llvm::Value* CreateLoadLambdaFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetLambdaPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("closure.lambda"));
  }

  llvm::Value* CreateLoadNumCapturesFromClosure(llvm::Value* closure_ptr) {
    auto* ptr = CreateGetNumCapturesPtrOfClosure(closure_ptr);
    return builder_->CreateLoad(builder_->getInt16Ty(), ptr, REG_NAME("closure.num_captures"));
  }

  // capture

  llvm::Value* CreateGetTargetPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(types_->CreateCaptureType(), capture_ptr, 0,
                                     REG_NAME("capture.target.ptr"));
  }

  llvm::Value* CreateGetEscapedPtrOfCapture(llvm::Value* capture_ptr) {
    return builder_->CreateStructGEP(types_->CreateCaptureType(), capture_ptr, 1,
                                     REG_NAME("capture.escaped.ptr"));
  }

  llvm::Value* CreateLoadTargetFromCapture(llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("capture.target"));
  }

  void CreateStoreTargetToCapture(llvm::Value* value_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetTargetPtrOfCapture(capture_ptr);
    builder_->CreateStore(value_ptr, ptr);
  }

  void CreateStoreEscapedToCapture(llvm::Value* value_ptr, llvm::Value* capture_ptr) {
    auto* ptr = CreateGetEscapedPtrOfCapture(capture_ptr);
    CreateMemCpyValue(ptr, value_ptr);
  }

  // captures

  llvm::Value* CreateGetCapturePtrPtrOfCaptures(llvm::Value* captures, uint16_t index) {
    return builder_->CreateConstInBoundsGEP1_32(
        builder_->getPtrTy(), captures, index,
        REG_NAME("captures." + llvm::Twine(index) + ".ptr"));
  }

  llvm::Value* CreateLoadCapturePtrFromCaptures(llvm::Value* captures, uint16_t index) {
    auto* ptr = CreateGetCapturePtrPtrOfCaptures(captures, index);
    return builder_->CreateLoad(builder_->getPtrTy(), ptr,
                                REG_NAME("captures." + llvm::Twine(index)));
  }

  void CreateStoreCapturePtrToCaptures(llvm::Value* capture_ptr,
                                       llvm::Value* captures,
                                       uint16_t index) {
    auto* ptr = CreateGetCapturePtrPtrOfCaptures(captures, index);
    builder_->CreateStore(capture_ptr, ptr);
  }

  // coroutine

  llvm::Value* CreateGetClosurePtrOfCoroutine() {
    return builder_->CreateStructGEP(types_->CreateCoroutineType(), context_, 0,
                                     REG_NAME("co.closure.ptr"));
  }

  llvm::Value* CreateGetStatePtrOfCoroutine() {
    return builder_->CreateStructGEP(types_->CreateCoroutineType(), context_, 1,
                                     REG_NAME("co.state.ptr"));
  }

  llvm::Value* CreateGetNumLocalsPtrOfCoroutine() {
    return builder_->CreateStructGEP(types_->CreateCoroutineType(), context_, 2,
                                     REG_NAME("co.num_locals.ptr"));
  }

  llvm::Value* CreateGetScopeIdPtrOfCoroutine() {
    return builder_->CreateStructGEP(types_->CreateCoroutineType(), context_, 3,
                                     REG_NAME("co.scope_id.ptr"));
  }

  llvm::Value* CreateGetScrachBufferLenPtrOfCoroutine() {
    return builder_->CreateStructGEP(types_->CreateCoroutineType(), context_, 4,
                                     REG_NAME("co.locals.scratch_buffer_len.ptr"));
  }

  llvm::Value* CreateGetLocalsPtrOfCoroutine() {
    return builder_->CreateStructGEP(types_->CreateCoroutineType(), context_, 5,
                                     REG_NAME("co.locals.ptr"));
  }

  llvm::Value* CreateLoadClosureFromCoroutine() {
    auto* ptr = CreateGetClosurePtrOfCoroutine();
    return builder_->CreateLoad(builder_->getPtrTy(), ptr, REG_NAME("co.closure"));
  }

  llvm::Value* CreateLoadStateFromCoroutine() {
    auto* ptr = CreateGetStatePtrOfCoroutine();
    return builder_->CreateLoad(builder_->getInt32Ty(), ptr, REG_NAME("co.state"));
  }

  llvm::Value* CreateLoadNumLocalsFromCoroutine() {
    auto* ptr = CreateGetNumLocalsPtrOfCoroutine();
    return builder_->CreateLoad(builder_->getInt16Ty(), ptr, REG_NAME("co.num_locals"));
  }

  llvm::Value* CreateGetCapturesPtrOfCoroutine() {
    auto* closure = CreateLoadClosureFromCoroutine();
    return CreateGetCapturesPtrOfClosure(closure);
  }

  llvm::Value* CreateGetScratchBufferPtrOfCoroutine() {
    auto* num_locals = CreateLoadNumLocalsFromCoroutine();
    auto* num_locals_usize =
        builder_->CreateSExt(num_locals, types_->GetWordType(), REG_NAME("co.num_locals.usize"));
    auto* sizeof_locals =
        builder_->CreateMul(GetSizeofValue(), num_locals_usize, REG_NAME("co.locals.sizeof"));
    auto* offsetof_locals = types_->GetWord(offsetof(Coroutine, locals));
    auto* offset = builder_->CreateAdd(offsetof_locals, sizeof_locals,
                                       REG_NAME("co.scratch_buffer.offsetof"));
    return builder_->CreateInBoundsPtrAdd(context_, offset, REG_NAME("co.scratch_buffer.ptr"));
  }

  // scope cleanup checker

  bool IsScopeCleanupCheckerEnabled() const {
    return scope_id_ != nullptr;
  }

  void ResetScopeCleanupChecker() {
    scope_id_ = nullptr;
  }

  // helpers

  void CreatePrintU32(llvm::Value* value, const char* msg = "") {
    auto* msg_value = builder_->CreateGlobalString(msg, REG_NAME("runtime.print_u32.msg"));
    auto* func = types_->CreateRuntimePrintU32();
    builder_->CreateCall(func, {runtime_, value, msg_value});
  }

  void CreatePrintF64(llvm::Value* value, const char* msg = "") {
    auto* msg_value = builder_->CreateGlobalString(msg, REG_NAME("runtime.print_f64.msg"));
    auto* func = types_->CreateRuntimePrintF64();
    builder_->CreateCall(func, {runtime_, value, msg_value});
  }

  // TODO: separate values that must be reset in EndFunction() from others.

  std::unique_ptr<llvm::LLVMContext> llvmctx_ = nullptr;
  std::unique_ptr<llvm::Module> module_ = nullptr;
  std::unique_ptr<llvm::IRBuilder<>> builder_ = nullptr;
  std::unique_ptr<TypeHolder> types_ = nullptr;

  // The following values are reset for each function.
  llvm::Function* function_ = nullptr;
  llvm::BasicBlock* locals_block_ = nullptr;
  llvm::Value* runtime_ = nullptr;
  llvm::Value* context_ = nullptr;
  llvm::Value* argc_ = nullptr;
  llvm::Value* argv_ = nullptr;
  llvm::Value* retv_ = nullptr;
  llvm::Value* captures_ = nullptr;
  // Holds one of STATUS_XXX values, not Status::*.
  llvm::Value* status_ = nullptr;
  llvm::Value* flow_selector_ = nullptr;

  // scope cleanup checker
  llvm::Value* scope_id_ = nullptr;

  // A cache of functions does not reset in the end of compilation for each function.
  std::unordered_map<uint32_t, llvm::Function*> functions_;

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
