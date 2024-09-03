#include "compiler.hh"

#include <cassert>
#include <cstdint>
#include <cstdlib>
#include <sstream>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Analysis/CGSCCPassManager.h>
#include <llvm/Analysis/LoopAnalysisManager.h>
#include <llvm/IR/BasicBlock.h>
#include <llvm/IR/DerivedTypes.h>
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

#include "helper.hh"
#include "macros.hh"
#include "module.hh"

namespace {

constexpr uint8_t kValueKindUndefined = static_cast<uint8_t>(ValueKind::Undefined);
constexpr uint8_t kValueKindNull = static_cast<uint8_t>(ValueKind::Null);
constexpr uint8_t kValueKindBoolean = static_cast<uint8_t>(ValueKind::Boolean);
constexpr uint8_t kValueKindNumber = static_cast<uint8_t>(ValueKind::Number);
constexpr uint8_t kValueKindClosure = static_cast<uint8_t>(ValueKind::Closure);

}  // namespace

Compiler::Compiler() {
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

Module* Compiler::TakeModule() {
  if (llvm::verifyModule(*module_, &llvm::errs())) {
    llvm::errs() << "### broken-module\n";
    module_->print(llvm::errs(), nullptr);
    llvm::errs() << '\n';
    std::abort();
  }

  llvm::orc::ThreadSafeModule mod(std::move(module_), std::move(context_));
  return new Module(std::move(mod));
}

void Compiler::SetSourceFileName(const char* input) {
  module_->setSourceFileName(input);
}

void Compiler::SetDataLayout(const char* data_layout) {
  module_->setDataLayout(data_layout);
}

void Compiler::SetTargetTriple(const char* triple) {
  module_->setTargetTriple(triple);
}

llvm::Value* Compiler::GetBoolean(bool value) {
  return llvm::ConstantInt::getBool(*context_, value);
}

llvm::Value* Compiler::GetNumber(double value) {
  return llvm::ConstantFP::get(*context_, llvm::APFloat(value));
}

llvm::Function* Compiler::GetFunction(uint32_t func_id, const char* name) {
  UNUSED(func_id);
  return CreateLambda(name);
}

llvm::Value* Compiler::GetException() {
  // TODO: Should we check status_ at runtime?
  return retv_;
}

// 6.1.6.1.2 Number::bitwiseNOT ( x )
llvm::Value* Compiler::CreateBitwiseNot(llvm::Value* number) {
  auto* int32 = ToInt32(number);
  auto* xored = builder_->CreateXor(int32, -1, REG_NAME("bitwise_not.xor"));
  return builder_->CreateSIToFP(xored, builder_->getDoubleTy(), REG_NAME("bitwise_not"));
}

llvm::Value* Compiler::CreateLogicalNot(llvm::Value* boolean) {
  return builder_->CreateXor(boolean, builder_->getTrue(), REG_NAME("logical_not"));
}

// 6.1.6.1.9 Number::leftShift ( x, y )
llvm::Value* Compiler::CreateLeftShift(llvm::Value* lhs, llvm::Value* rhs) {
  auto* lnum = ToInt32(lhs);
  auto* rnum = ToUint32(rhs);
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
  auto* shifted = builder_->CreateShl(lnum, shift_count, REG_NAME("shl"));
  return builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
}

// 6.1.6.1.10 Number::signedRightShift ( x, y )
llvm::Value* Compiler::CreateSignedRightShift(llvm::Value* lhs, llvm::Value* rhs) {
  auto* lnum = ToInt32(lhs);
  auto* rnum = ToUint32(rhs);
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
  auto* shifted = builder_->CreateAShr(lnum, shift_count, REG_NAME("ashr"));
  return builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
}

// 6.1.6.1.11 Number::unsignedRightShift ( x, y )
llvm::Value* Compiler::CreateUnsignedRightShift(llvm::Value* lhs, llvm::Value* rhs) {
  auto* lnum = ToUint32(lhs);
  auto* rnum = ToUint32(rhs);
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
  auto* shifted = builder_->CreateLShr(lnum, shift_count, REG_NAME("lshr"));
  return builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
}

llvm::Value* Compiler::CreateCallOnClosure(llvm::Value* closure, uint16_t argc, llvm::Value* argv, llvm::Value* retv) {
  auto* prototype = types_->CreateLambdaType();
  auto* lambda = CreateLoadLambdaFromClosure(closure);
  auto* caps = CreateLoadCapturesFromClosure(closure);
  return builder_->CreateCall(prototype, lambda, {exec_context_, caps, types_->GetWord(argc), argv, retv}, REG_NAME("status"));
}

void Compiler::StartFunction(const char* name) {
  function_ = CreateLambda(name);

  exec_context_ = function_->getArg(0);
  caps_ = function_->getArg(1);
  argc_ = function_->getArg(2);
  argv_ = function_->getArg(3);
  retv_ = function_->getArg(4);

  ClearScopeCleanupStack();
}

void Compiler::EndFunction(bool optimize) {
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

void Compiler::StartScopeCleanupChecker(uint16_t scope_id) {
  if (IsScopeCleanupCheckerEnabled()) {
    // We assumed here that the control flow does not enter into a scope which is already entered.
    // However, it may be better to check that explicitly here before pushing the scope ID.
    CreateAssertScopeCleanupStackBounds();
    CreatePushOntoScopeCleanupStack(scope_id);
  }
}

void Compiler::EndScopeCleanupChecker(uint16_t scope_id) {
  if (IsScopeCleanupCheckerEnabled()) {
    CreateAssertScopeCleanupStackHasItem();
    auto* popped = CreatePopFromScopeCleanupStack();
    CreateAssertScopeCleanupStackPoppedValue(popped, scope_id);
  }
}

void Compiler::HandleReturnedThrown(bool returned,
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

llvm::Value* Compiler::CreateLocalVariable(uint16_t index) {
  return CreateAlloc1(types_->CreateVariableType(), REG_NAME("local" + llvm::Twine(index) + ".ptr"));
}

llvm::Value* Compiler::CreateRetv() {
  return CreateAlloc1(types_->CreateVariableType(), REG_NAME("retv.ptr"));
}

void Compiler::CreateEscapeVariable(llvm::Value* capture, llvm::Value* variable) {
  auto* escaped_ptr = CreateGetEscapedPtrOfCapture(capture);
  CreateStoreTargetToCapture(escaped_ptr, capture);
  auto align = llvm::Align(sizeof(double));
  builder_->CreateMemCpy(escaped_ptr, align, variable, align, types_->GetWord(sizeof(Variable)));
}

void Compiler::PrepareScopeCleanupChecker(uint32_t stack_size) {
  scope_cleanup_stack_type_ = llvm::ArrayType::get(builder_->getInt16Ty(), stack_size);
  scope_cleanup_stack_ =
      CreateAllocN(builder_->getInt16Ty(), stack_size, REG_NAME("scope_cleanup_stack"));
  scope_cleanup_stack_top_ =
      CreateAlloc1(builder_->getInt32Ty(), REG_NAME("scope_cleanup_stack_top"));
  builder_->CreateStore(builder_->getInt32(0), scope_cleanup_stack_top_);
  scope_cleanup_stack_size_ = stack_size;
}

// 6.1.6.1.16 NumberBitwiseOp ( op, x, y )
llvm::Value* Compiler::NumberBitwiseOp(char op, llvm::Value* x, llvm::Value* y) {
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

// 7.1.4 ToNumber ( argument )
llvm::Value* Compiler::ToNumeric(llvm::Value* value_ptr) {
  auto* call = types_->CreateRuntimeToNumeric();
  return builder_->CreateCall(call, {exec_context_, value_ptr}, REG_NAME("numeric"));
}

// 7.1.6 ToInt32 ( argument )
llvm::Value* Compiler::ToInt32(llvm::Value* number) {
  // Skip the first step.
  // We assumed that `number` holds a number value.
  // TODO: Create inline instructions if runtime_to_int32() is slow.
  auto* func = types_->CreateRuntimeToInt32();
  return builder_->CreateCall(func, {exec_context_, number}, REG_NAME("int32"));
}

// 7.1.7 ToUint32 ( argument )
llvm::Value* Compiler::ToUint32(llvm::Value* number) {
  // Skip the first step.
  // We assumed that `number` holds a number value.
  // TODO: Create inline instructions if runtime_to_uint32() is slow.
  auto* func = types_->CreateRuntimeToUint32();
  return builder_->CreateCall(func, {exec_context_, number}, REG_NAME("uint32"));
}

llvm::Value* Compiler::CreateUndefinedToAny() {
  auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
  CreateStoreUndefinedToValue(value_ptr);
  return value_ptr;
}

llvm::Value* Compiler::CreateNullToAny() {
  auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
  CreateStoreNullToValue(value_ptr);
  return value_ptr;
}

llvm::Value* Compiler::CreateBooleanToAny(llvm::Value* boolean) {
  auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
  CreateStoreBooleanToValue(boolean, value_ptr);
  return value_ptr;
}

llvm::Value* Compiler::CreateNumberToAny(llvm::Value* number) {
  auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
  CreateStoreNumberToValue(number, value_ptr);
  return value_ptr;
}

llvm::Value* Compiler::CreateClosureToAny(llvm::Value* closure) {
  auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
  CreateStoreClosureToValue(closure, value_ptr);
  return value_ptr;
}

llvm::AllocaInst* Compiler::CreateAlloc1(llvm::Type* ty, const llvm::Twine& name) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(locals_block_);
  auto* alloca = builder_->CreateAlloca(ty, nullptr, name);
  builder_->SetInsertPoint(backup);
  return alloca;
}

llvm::AllocaInst* Compiler::CreateAllocN(llvm::Type* ty, uint32_t n, const llvm::Twine& name) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(locals_block_);
  auto* alloca = builder_->CreateAlloca(ty, builder_->getInt32(n), name);
  builder_->SetInsertPoint(backup);
  return alloca;
}

llvm::Function* Compiler::CreateLambda(const char* name) {
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

// non-nullish

llvm::Value* Compiler::CreateIsNonNullish(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpUGT(
      kind, builder_->getInt8(kValueKindNull), REG_NAME("is_non_nullish"));
}

// 7.1.2 ToBoolean ( argument )

llvm::Value* Compiler::CreateToBoolean(llvm::Value* value_ptr) {
  auto* func = types_->CreateRuntimeToBoolean();
  return builder_->CreateCall(func, {exec_context_, value_ptr}, REG_NAME("boolean"));
}

// 7.2.13 IsLooselyEqual ( x, y )

llvm::Value* Compiler::CreateIsLooselyEqual(llvm::Value* x, llvm::Value* y) {
  // TODO: Create inline instructions if runtime_is_loosely_equal() is slow.
  auto* func = types_->CreateRuntimeIsLooselyEqual();
  return builder_->CreateCall(func, {exec_context_, x, y}, REG_NAME("is_loosely_equal.retval"));
}

// 7.2.14 IsStrictlyEqual ( x, y )

llvm::Value* Compiler::CreateIsStrictlyEqual(llvm::Value* x, llvm::Value* y) {
  // TODO: Create inline instructions if runtime_is_strictly_equal() is slow.
  auto* func = types_->CreateRuntimeIsStrictlyEqual();
  return builder_->CreateCall(func, {exec_context_, x, y}, REG_NAME("is_strictly_equal.retval"));
}

llvm::Value* Compiler::CreateIsUndefined(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(
      kind, builder_->getInt8(kValueKindUndefined), REG_NAME("is_undefined"));
}

llvm::Value* Compiler::CreateIsNull(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindNull), REG_NAME("is_null"));
}

llvm::Value* Compiler::CreateIsBoolean(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindBoolean), REG_NAME("is_boolean"));
}

llvm::Value* Compiler::CreateIsNumber(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindNumber), REG_NAME("is_number"));
}

llvm::Value* Compiler::CreateIsClosure(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindClosure), REG_NAME("is_closure"));
}

llvm::Value* Compiler::CreateIsSameBoolean(llvm::Value* a, llvm::Value* b) {
  return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_boolean"));
}

llvm::Value* Compiler::CreateIsSameNumber(llvm::Value* a, llvm::Value* b) {
  return builder_->CreateFCmpOEQ(a, b, REG_NAME("is_same_number"));
}

llvm::Value* Compiler::CreateIsSameClosure(llvm::Value* a, llvm::Value* b) {
  return builder_->CreateICmpEQ(a, b, REG_NAME("is_same_closure"));
}

llvm::Value* Compiler::CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* boolean) {
  auto* value = CreateLoadBooleanFromValue(value_ptr);
  return builder_->CreateICmpEQ(value, boolean, REG_NAME("is_same_boolean_value"));
}

llvm::Value* Compiler::CreateIsSameNumberValue(llvm::Value* value_ptr, llvm::Value* number) {
  auto* value = CreateLoadNumberFromValue(value_ptr);
  return builder_->CreateFCmpOEQ(value, number, REG_NAME("is_same_number_value"));
}

llvm::Value* Compiler::CreateIsSameClosureValue(llvm::Value* value_ptr, llvm::Value* closure) {
  auto* value = CreateLoadClosureFromValue(value_ptr);
  return builder_->CreateICmpEQ(value, closure, REG_NAME("is_same_closure_value"));
}

llvm::Value* Compiler::CreateCallRuntimeCreateCapture(llvm::Value* variable_ptr) {
  auto* func = types_->CreateRuntimeCreateCapture();
  return builder_->CreateCall(func, {exec_context_, variable_ptr}, REG_NAME("capture.ptr"));
}

llvm::Value* Compiler::CreateCallRuntimeCreateClosure(llvm::Value* lambda, uint16_t num_captures) {
  auto* func = types_->CreateRuntimeCreateClosure();
  return builder_->CreateCall(
      func, {exec_context_, lambda, builder_->getInt16(num_captures)}, REG_NAME("closure.ptr"));
}

void Compiler::CreateCallRuntimeAssert(llvm::Value* assertion, llvm::Value* msg) {
  auto* func = types_->CreateRuntimeAssert();
  builder_->CreateCall(func, {exec_context_, assertion, msg});
}

void Compiler::CreatePushOntoScopeCleanupStack(uint16_t scope_id) {
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

llvm::Value* Compiler::CreatePopFromScopeCleanupStack() {
  auto* top = CreateLoadScopeCleanupStackTop();
  // scope_cleanup_stack_top_--;
  auto* decr =
      builder_->CreateSub(top, builder_->getInt32(1), REG_NAME("scope_cleanup_stack.top.decr"));
  CreateStoreScopeCleanupStackTop(decr);
  // return scope_cleanup_stack_[scope_cleanup_stack_top_];
  auto* ptr = builder_->CreateInBoundsGEP(scope_cleanup_stack_type_, scope_cleanup_stack_,
      {builder_->getInt32(0), decr}, REG_NAME("scope_cleanup_stack.popped.ptr"));
  return builder_->CreateLoad(builder_->getInt16Ty(), ptr, REG_NAME("scope_cleanup_stack.popped"));
}

// assert(scope_cleanup_stack_top_ <= scope_cleanup_stack_size_);
void Compiler::CreateAssertScopeCleanupStackBounds() {
  auto* top = CreateLoadScopeCleanupStackTop();
  auto* assertion = builder_->CreateICmpULE(top, builder_->getInt32(scope_cleanup_stack_size_),
      REG_NAME("assertion.scope_cleanup_stack.size"));
  auto* msg = builder_->CreateGlobalString(
      "assertion failure: scope_cleanup_stack_top_ <= scoke_cleanup_stack_size_",
      REG_NAME("assertion.msg.scope_cleanup_stack.size"));
  CreateCallRuntimeAssert(assertion, msg);
}

// assert(popped == scope_id);
void Compiler::CreateAssertScopeCleanupStackPoppedValue(llvm::Value* actual, uint16_t expected) {
  auto* assertion = builder_->CreateICmpEQ(
      actual, builder_->getInt16(expected), REG_NAME("assertion.scope_cleanup_stack.popped"));
  std::stringstream ss;
  ss << "assertion failure: popped == " << expected;
  auto* msg =
      builder_->CreateGlobalString(ss.str(), REG_NAME("assertion.msg.scope_cleanup_stack.popped"));
  CreateCallRuntimeAssert(assertion, msg);
}

// assert(scope_cleanup_stack_top_ == 0);
void Compiler::CreateAssertScopeCleanupStackIsEmpty() {
  auto* top = CreateLoadScopeCleanupStackTop();
  auto* assertion = builder_->CreateICmpEQ(
      top, builder_->getInt32(0), REG_NAME("assertion.scope_cleanup_stack.is_empty"));
  auto* msg = builder_->CreateGlobalString("assertion failure: scope_cleanup_stack_top_ == 0",
      REG_NAME("assertion.msg.scope_cleanup_stack.is_empty"));
  CreateCallRuntimeAssert(assertion, msg);
}

// assert(scope_cleanup_stack_top_ != 0);
void Compiler::CreateAssertScopeCleanupStackHasItem() {
  auto* top = CreateLoadScopeCleanupStackTop();
  auto* assertion = builder_->CreateICmpNE(
      top, builder_->getInt32(0), REG_NAME("assertion.scope_cleanup_stack.has_item"));
  auto* msg = builder_->CreateGlobalString("assertion failure: scope_cleanup_stack_top_ != 0",
      REG_NAME("assertion.msg.scope_cleanup_stack.has_item"));
  CreateCallRuntimeAssert(assertion, msg);
}
