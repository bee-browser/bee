#include "compiler.hh"

#include <cstdint>
#include <cstdlib>

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
#include <llvm/Transforms/InstCombine/InstCombine.h>
#include <llvm/Transforms/Scalar/GVN.h>
#include <llvm/Transforms/Scalar/Reassociate.h>
#include <llvm/Transforms/Scalar/SimplifyCFG.h>
#include <llvm/Transforms/Utils/Mem2Reg.h>
#pragma GCC diagnostic pop

#include "macros.hh"
#include "module.hh"

#if defined(BEE_BUILD_DEBUG)
#define NAME(s) MakeDebugName(s)
#define NAME_WITH_ID(name, id) ((name + llvm::Twine(id)).str())
#else
#define NAME(s) ""
#define NAME_WITH_ID(name, id) (UNUSED(id), std::string())
#endif

namespace {

inline uint32_t ComputeKeyFromLocator(Locator locator) {
  return (static_cast<uint32_t>(locator.kind) << 16) | static_cast<uint32_t>(locator.index);
}

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
    llvm::errs() << "<broken-module>\n";
    module_->print(llvm::errs(), nullptr);
    llvm::errs() << "</broken-module>\n";
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

void Compiler::SetRuntime(uintptr_t runtime) {
  UNUSED(runtime);
}

void Compiler::Undefined() {
  PushUndefined();
}

void Compiler::Null() {
  PushNull();
}

void Compiler::Boolean(bool value) {
  PushBoolean(llvm::ConstantInt::getBool(*context_, value));
}

void Compiler::Number(double value) {
  PushNumber(llvm::ConstantFP::get(*context_, llvm::APFloat(value)));
}

void Compiler::Function(uint32_t func_id, const char* name) {
  UNUSED(func_id);
  const auto& found = functions_.find(name);
  if (found != functions_.end()) {
    PushFunction(found->second);
    return;
  }
  auto* prototype = types_->CreateLambdaType();
  auto* func = llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, name, *module_);
  functions_[name] = func;
  PushFunction(func);
}

void Compiler::Closure(bool declaration, uint16_t num_captures) {
  assert(stack_.size() >= 1 + static_cast<size_t>(num_captures));
  assert(!scope_stack_.empty());

  auto& scope = scope_stack_.back();

  llvm::BasicBlock* backup;
  if (declaration) {
    backup = builder_->GetInsertBlock();
    builder_->SetInsertPoint(scope.hoisted_block);
  }

  auto* lambda = PopFunction();
  auto* closure_ptr = CreateCallRuntimeCreateClosure(lambda, num_captures);

  auto* captures = CreateLoadCapturesFromClosure(closure_ptr);
  for (uint16_t i = 0; i < num_captures; ++i) {
    auto* capture_ptr = PopCapture();
    CreateStoreCapturePtrToCaptures(capture_ptr, captures, i);
  }

  PushClosure(closure_ptr);

  if (declaration) {
    builder_->SetInsertPoint(backup);
  }
}

void Compiler::Reference(uint32_t symbol, Locator locator) {
  PushReference(symbol, locator);
}

void Compiler::Exception() {
  // TODO: Should we check status_ at runtime?
  PushAny(ret_);
}

// 13.4.2.1 Runtime Semantics: Evaluation
void Compiler::PostfixIncrement() {
  IncrDecr('$', '+');
}

// 13.4.3.1 Runtime Semantics: Evaluation
void Compiler::PostfixDecrement() {
  IncrDecr('$', '-');
}

// 13.4.4.1 Runtime Semantics: Evaluation
void Compiler::PrefixIncrement() {
  IncrDecr('^', '+');
}

// 13.4.5.1 Runtime Semantics: Evaluation
void Compiler::PrefixDecrement() {
  IncrDecr('^', '-');
}

// 13.5.1.2 Runtime Semantics: Evaluation
void Compiler::UnaryDelete() {
  // TODO
  std::abort();
}

// 13.5.2.1 Runtime Semantics: Evaluation
void Compiler::Void() {
  PopItem();
  PushUndefined();
}

// 13.5.3.1 Runtime Semantics: Evaluation
void Compiler::Typeof() {
  // TODO
  std::abort();
}

// 13.5.4.1 Runtime Semantics: Evaluation
void Compiler::UnaryPlus() {
  auto* v = ToNumeric(Dereference());
  PushNumber(v);
}

// 13.5.5.1 Runtime Semantics: Evaluation
void Compiler::UnaryMinus() {
  auto* num = ToNumeric(Dereference());
  // TODO: BigInt
  // 6.1.6.1.1 Number::unaryMinus ( x )
  auto* v = builder_->CreateFNeg(num);
  PushNumber(v);
}

// 13.5.6.1 Runtime Semantics: Evaluation
void Compiler::BitwiseNot() {
  auto* num = ToNumeric(Dereference());
  // TODO: BigInt
  // 6.1.6.1.2 Number::bitwiseNOT ( x )
  auto* int32 = ToInt32(num);
  auto* xored = builder_->CreateXor(int32, -1);
  auto* v = builder_->CreateSIToFP(xored, builder_->getDoubleTy());
  PushNumber(v);
}

// 13.5.7.1 Runtime Semantics: Evaluation
void Compiler::LogicalNot() {
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  auto* v = builder_->CreateXor(truthy, builder_->getTrue());
  PushBoolean(v);
}

// 13.6.1 Runtime Semantics: Evaluation
void Compiler::Exponentiation() {
  // TODO
  std::abort();
}

// 13.7.1 Runtime Semantics: Evaluation
void Compiler::Multiplication() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFMul(lhs, rhs);
  PushNumber(v);
}

// 13.7.1 Runtime Semantics: Evaluation
void Compiler::Division() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs);
  PushNumber(v);
}

// 13.7.1 Runtime Semantics: Evaluation
void Compiler::Remainder() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs);
  PushNumber(v);
}

// 13.8.1.1 Runtime Semantics: Evaluation
void Compiler::Addition() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs);
  PushNumber(v);
}

// 13.8.2.1 Runtime Semantics: Evaluation
void Compiler::Subtraction() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs);
  PushNumber(v);
}

// 13.9.1.1 Runtime Semantics: Evaluation
void Compiler::LeftShift() {
  // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )

  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
  // TODO: BigInt
  // 6.1.6.1.9 Number::leftShift ( x, y )
  auto* lnum = ToInt32(lhs);
  auto* rnum = ToUint32(rhs);
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32));
  auto* shifted = builder_->CreateShl(lnum, shift_count);
  auto* v = builder_->CreateSIToFP(shifted, builder_->getDoubleTy());
  PushNumber(v);
}

// 13.9.2.1 Runtime Semantics: Evaluation
void Compiler::SignedRightShift() {
  // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
  // TODO: BigInt
  // 6.1.6.1.10 Number::signedRightShift ( x, y )
  auto* lnum = ToInt32(lhs);
  auto* rnum = ToUint32(rhs);
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32));
  auto* shifted = builder_->CreateAShr(lnum, shift_count);
  auto* v = builder_->CreateSIToFP(shifted, builder_->getDoubleTy());
  PushNumber(v);
}

// 13.9.3.1 Runtime Semantics: Evaluation
void Compiler::UnsignedRightShift() {
  // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
  // TODO: BigInt
  // 6.1.6.1.11 Number::unsignedRightShift ( x, y )
  auto* lnum = ToUint32(lhs);
  auto* rnum = ToUint32(rhs);
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32));
  auto* shifted = builder_->CreateLShr(lnum, shift_count);
  auto* v = builder_->CreateSIToFP(shifted, builder_->getDoubleTy());
  PushNumber(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::LessThan() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs);
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::GreaterThan() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs);
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::LessThanOrEqual() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs);
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::GreaterThanOrEqual() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs);
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::Instanceof() {
  // TODO
  std::abort();
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::In() {
  // TODO
  std::abort();
}

// 13.11.1 Runtime Semantics: Evaluation
void Compiler::Equality() {
  Swap();
  auto lhs = Dereference();
  auto rhs = Dereference();
  // TODO: comparing references improves the performance.
  auto* v = CreateIsLooselyEqual(lhs, rhs);
  PushBoolean(v);
}

// 13.11.1 Runtime Semantics: Evaluation
void Compiler::Inequality() {
  Swap();
  struct Reference lref, rref;
  auto lhs = Dereference();
  auto rhs = Dereference();
  // TODO: comparing references improves the performance.
  auto* eq = CreateIsLooselyEqual(lhs, rhs);
  // TODO: should reuse LogicalNot()?
  auto* v = builder_->CreateXor(eq, builder_->getTrue());
  PushBoolean(v);
}

// 13.11.1 Runtime Semantics: Evaluation
void Compiler::StrictEquality() {
  Swap();
  auto lhs = Dereference();
  auto rhs = Dereference();
  auto* v = CreateIsStrictlyEqual(lhs, rhs);
  PushBoolean(v);
}

// 13.11.1 Runtime Semantics: Evaluation
void Compiler::StrictInequality() {
  Swap();
  auto lhs = Dereference();
  auto rhs = Dereference();
  auto* eq = CreateIsStrictlyEqual(lhs, rhs);
  // TODO: should reuse LogicalNot()?
  auto* v = builder_->CreateXor(eq, builder_->getTrue());
  PushBoolean(v);
}

// 13.12.1 Runtime Semantics: Evaluation
void Compiler::BitwiseAnd() {
  // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
  Swap();
  auto lval = Dereference();
  auto rval = Dereference();

  // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
  auto* lnum = ToNumeric(lval);
  auto* rnum = ToNumeric(rval);
  // TODO: BigInt

  // 6.1.6.1.17 Number::bitwiseAND ( x, y )
  NumberBitwiseOp('&', lnum, rnum);
}

// 13.12.1 Runtime Semantics: Evaluation
void Compiler::BitwiseXor() {
  // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
  Swap();
  auto lval = Dereference();
  auto rval = Dereference();

  // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
  auto* lnum = ToNumeric(lval);
  auto* rnum = ToNumeric(rval);
  // TODO: BigInt

  // 6.1.6.1.17 Number::bitwiseAND ( x, y )
  NumberBitwiseOp('^', lnum, rnum);
}

// 13.12.1 Runtime Semantics: Evaluation
void Compiler::BitwiseOr() {
  // 13.15.4 EvaluateStringOrNumericBinaryExpression ( leftOperand, opText, rightOperand )
  Swap();
  auto lval = Dereference();
  auto rval = Dereference();

  // 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
  auto* lnum = ToNumeric(lval);
  auto* rnum = ToNumeric(rval);
  // TODO: BigInt

  // 6.1.6.1.19 Number::bitwiseOR ( x, y )
  NumberBitwiseOp('|', lnum, rnum);
}

void Compiler::ConditionalTernary() {
  auto* else_tail_block = builder_->GetInsertBlock();

  auto else_item = Dereference();

  auto* else_head_block = PopBlock();
  auto* then_tail_block = PopBlock();

  builder_->SetInsertPoint(then_tail_block);
  auto then_item = Dereference();

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();

  builder_->SetInsertPoint(cond_block);
  auto* cond_value = PopValue();
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  auto* block = CreateBasicBlock(NAME("block"));

  if (then_item.type == else_item.type) {
    builder_->SetInsertPoint(then_tail_block);
    builder_->CreateBr(block);

    builder_->SetInsertPoint(else_tail_block);
    builder_->CreateBr(block);

    builder_->SetInsertPoint(block);

    // In this case, we can use the value of each item as is.
    switch (then_item.type) {
      case Item::Undefined:
        PushUndefined();
        return;
      case Item::Null:
        PushNull();
        return;
      case Item::Boolean: {
        auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2);
        phi->addIncoming(then_item.value, then_tail_block);
        phi->addIncoming(else_item.value, else_tail_block);
        PushBoolean(phi);
        return;
      }
      case Item::Number: {
        auto* phi = builder_->CreatePHI(builder_->getDoubleTy(), 2);
        phi->addIncoming(then_item.value, then_tail_block);
        phi->addIncoming(else_item.value, else_tail_block);
        PushNumber(phi);
        return;
      }
      case Item::Any: {
        auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2);
        phi->addIncoming(then_item.value, then_tail_block);
        phi->addIncoming(else_item.value, else_tail_block);
        PushAny(phi);
        return;
      }
      default:
        // TODO
        assert(false);
        PushUndefined();
        return;
    }
  }

  // We have to convert the value before the branch in each block.

  builder_->SetInsertPoint(then_tail_block);
  auto* then_value = ToAny(then_item);
  builder_->CreateBr(block);

  builder_->SetInsertPoint(else_tail_block);
  auto* else_value = ToAny(else_item);
  builder_->CreateBr(block);

  builder_->SetInsertPoint(block);
  auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2);
  phi->addIncoming(then_value, then_tail_block);
  phi->addIncoming(else_value, else_tail_block);
  PushAny(phi);
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::Assignment() {
  auto item = Dereference();
  auto ref = PopReference();

  auto* variable_ptr = CreateGetVariablePtr(ref.locator);
  // TODO: check the mutable flag
  // auto* flags_ptr = CreateGetFlagsPtr(variable_ptr);

  CreateStoreItemToVariable(item, variable_ptr);

  stack_.push_back(item);
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::ExponentiationAssignment() {
  // TODO
  assert(false);
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::MultiplicationAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  Multiplication();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::DivisionAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  Division();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::RemainderAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  Remainder();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::AdditionAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  Addition();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::SubtractionAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  Subtraction();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::LeftShiftAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  LeftShift();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::SignedRightShiftAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  SignedRightShift();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::UnsignedRightShiftAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  UnsignedRightShift();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::BitwiseAndAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  BitwiseAnd();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::BitwiseXorAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  BitwiseXor();
  Assignment();
}

// 13.15.2 Runtime Semantics: Evaluation
void Compiler::BitwiseOrAssignment() {
  auto item = PopItem();
  assert(!stack_.empty());
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  stack_.push_back(item);
  BitwiseOr();
  Assignment();
}

void Compiler::DeclareImmutable() {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED;

  auto item = PopItem();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = CreateGetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);
}

void Compiler::DeclareMutable() {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED | VARIABLE_MUTABLE;

  auto item = Dereference();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = CreateGetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);
}

void Compiler::DeclareFunction() {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED | VARIABLE_MUTABLE;

  assert(!scope_stack_.empty());
  auto& scope = scope_stack_.back();

  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(scope.hoisted_block);

  auto item = Dereference();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = CreateGetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);

  builder_->SetInsertPoint(backup);
}

void Compiler::DeclareClosure() {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED | VARIABLE_MUTABLE;

  assert(!scope_stack_.empty());
  auto& scope = scope_stack_.back();

  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(scope.hoisted_block);

  auto item = Dereference();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = CreateGetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);

  builder_->SetInsertPoint(backup);
}

void Compiler::Arguments(uint16_t argc) {
  assert(argc > 0);
  auto* argv = CreateAllocaInEntryBlock(types_->CreateValueType(), argc);
  PushArgv(argv);
  Swap();
}

void Compiler::Argument(uint16_t index) {
  auto item = Dereference();
  auto* argv = PopArgv();
  auto* arg_ptr = builder_->CreateConstInBoundsGEP1_32(types_->CreateValueType(), argv, index);
  CreateStoreItemToValue(item, arg_ptr);
  PushArgv(argv);
}

void Compiler::Call(uint16_t argc) {
  llvm::Value* argv;
  if (argc > 0) {
    argv = PopArgv();
  } else {
    argv = llvm::Constant::getNullValue(builder_->getPtrTy());
  }

  auto item = Dereference();
  llvm::Value* closure_ptr;
  switch (item.type) {
    case Item::Closure:
      // IIFE
      closure_ptr = item.value;
      break;
    case Item::Any:
      closure_ptr = CreateLoadClosureFromValueOrThrowTypeError(item.value);
      break;
    default:
      // TODO: TypeError
      PushNumber(builder_->getInt32(1));
      Throw();
      return;
  }

  auto* prototype = types_->CreateLambdaType();
  auto* lambda = CreateLoadLambdaFromClosure(closure_ptr);
  auto* caps = CreateLoadCapturesFromClosure(closure_ptr);
  auto* ret = builder_->CreateAlloca(types_->CreateValueType());

  auto* status = builder_->CreateCall(
      prototype, lambda, {exec_context_, caps, types_->GetWord(argc), argv, ret});

  CreateCheckStatusForException(status, ret);

  PushAny(ret);
}

llvm::Value* Compiler::CreateLoadClosureFromValueOrThrowTypeError(llvm::Value* value_ptr) {
  auto* closure = builder_->CreateAlloca(builder_->getPtrTy());

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* then_block = CreateBasicBlock(NAME("then"));
  auto* else_block = CreateBasicBlock(NAME("else"));
  auto* end_block = CreateBasicBlock(NAME("block"));

  // if (value.kind == ValueKind::Closure)
  auto* is_closure =
      builder_->CreateICmpEQ(kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Closure)));
  builder_->CreateCondBr(is_closure, then_block, else_block);
  // {
  builder_->SetInsertPoint(then_block);
  auto* closure_ptr = CreateLoadClosureFromValue(value_ptr);
  builder_->CreateStore(closure_ptr, closure);
  builder_->CreateBr(end_block);
  // } else {
  builder_->SetInsertPoint(else_block);
  // TODO: TypeError
  PushNumber(builder_->getInt32(1));
  Throw();
  builder_->CreateBr(end_block);
  // }

  builder_->SetInsertPoint(end_block);
  return builder_->CreateLoad(builder_->getPtrTy(), closure);
}

// Handle an exception if it's thrown.
void Compiler::CreateCheckStatusForException(llvm::Value* status, llvm::Value* ret) {
  auto* then_block = CreateBasicBlock(NAME("then"));
  auto* end_block = CreateBasicBlock(NAME("block"));

  // if (status == Status::Exception)
  auto* is_exception =
      builder_->CreateICmpEQ(status, builder_->getInt32(static_cast<int32_t>(Status::Exception)));
  builder_->CreateCondBr(is_exception, then_block, end_block);
  // {
  builder_->SetInsertPoint(then_block);
  // Store the exception.
  builder_->CreateStore(status, status_);
  builder_->CreateMemCpy(
      ret_, llvm::MaybeAlign(), ret, llvm::MaybeAlign(), builder_->getInt32(sizeof(Value)));
  assert(!catch_stack_.empty());
  auto* catch_block = catch_stack_.back();
  builder_->CreateBr(catch_block);
  // }

  builder_->SetInsertPoint(end_block);
}

void Compiler::Truthy() {
  const auto item = Dereference();
  auto* v = CreateToBoolean(item);
  PushBoolean(v);
}

void Compiler::FalsyShortCircuit() {
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  LogicalNot();
  Block();  // then
  stack_.push_back(item);
  Block();  // else
}

void Compiler::TruthyShortCircuit() {
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  Block();  // then
  stack_.push_back(item);
  Block();  // else
}

void Compiler::NullishShortCircuit() {
  const auto item = Dereference();
  auto* non_nullish = CreateIsNonNullish(item);
  PushBoolean(non_nullish);
  Block();  // then
  stack_.push_back(item);
  Block();  // else
}

void Compiler::FalsyShortCircuitAssignment() {
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  LogicalNot();
  Block();  // then
  stack_.push_back(item);
  Block();  // else
}

void Compiler::TruthyShortCircuitAssignment() {
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  Block();  // then
  stack_.push_back(item);
  Block();  // else
}

void Compiler::NullishShortCircuitAssignment() {
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  const auto item = Dereference();
  auto* non_nullish = CreateIsNonNullish(item);
  PushBoolean(non_nullish);
  Block();  // then
  stack_.push_back(item);
  Block();  // else
}

void Compiler::Block() {
  // Push the current block.
  auto* current_block = builder_->GetInsertBlock();
  assert(current_block != nullptr);
  PushBlock(current_block, "old-block");

  // Push a newly created block.
  // This will be used in ConditionalExpression() in order to build a branch instruction.
  auto* block = CreateBasicBlock(NAME("block"));
  PushBlock(block, "new-block");

  builder_->SetInsertPoint(block);
}

void Compiler::IfElseStatement() {
  auto* else_tail_block = builder_->GetInsertBlock();

  llvm::BasicBlock* block = nullptr;

  if (else_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    block = CreateBasicBlock(NAME("block"));
    builder_->CreateBr(block);
  }

  auto* else_head_block = PopBlock();
  auto* then_tail_block = PopBlock();

  if (then_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    if (block == nullptr) {
      block = CreateBasicBlock(NAME("block"));
    }
    builder_->SetInsertPoint(then_tail_block);
    builder_->CreateBr(block);
  }

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();
  auto* cond_value = PopBoolean();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  if (block != nullptr) {
    builder_->SetInsertPoint(block);
  }
}

void Compiler::IfStatement() {
  auto* then_tail_block = builder_->GetInsertBlock();

  auto* block = CreateBasicBlock(NAME("block"));

  if (then_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    builder_->CreateBr(block);
  }

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();
  auto* cond_value = PopBoolean();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_head_block, block);

  builder_->SetInsertPoint(block);
}

void Compiler::DoWhileLoop(uint16_t id) {
  PushDebugName(NAME_WITH_ID("do-while", id));

  auto* loop_body = CreateBasicBlock(NAME("loop-body"));
  auto* loop_test = CreateBasicBlock(NAME("loop-test"));
  auto* loop_end = CreateBasicBlock(NAME("loop-end"));

  auto* loop_start = loop_body;
  auto* loop_continue = loop_test;
  auto* loop_break = loop_end;

  // For LoopTest()
  PushBlock(loop_end, "do-while-test-insert-point");
  PushBlock(loop_end, "do-while-test-then");
  PushBlock(loop_body, "do-while-test-else");

  // For LoopBody()
  PushBlock(loop_test, "do-while-body-insert-point");
  PushBlock(loop_test, "do-while-body-br");

  builder_->CreateBr(loop_start);
  builder_->SetInsertPoint(loop_start);

  SetBlockForLabelsInContinueStack(loop_continue);

  break_stack_.push_back({loop_break, 0});
  continue_stack_.push_back({loop_continue, 0});
}

void Compiler::WhileLoop(uint16_t id) {
  PushDebugName(NAME_WITH_ID("while", id));

  auto* loop_test = CreateBasicBlock(NAME("loop-test"));
  auto* loop_body = CreateBasicBlock(NAME("loop-body"));
  auto* loop_end = CreateBasicBlock(NAME("loop-end"));

  auto* loop_start = loop_test;
  auto* loop_continue = loop_test;
  auto* loop_break = loop_end;

  // For LoopBody()
  PushBlock(loop_end, "while-body-insert-point");
  PushBlock(loop_test, "while-body-br");

  // For LoopTest()
  PushBlock(loop_body, "while-test-insert-point");
  PushBlock(loop_end, "while-test-then");
  PushBlock(loop_body, "while-test-else");

  builder_->CreateBr(loop_start);
  builder_->SetInsertPoint(loop_start);

  SetBlockForLabelsInContinueStack(loop_continue);

  break_stack_.push_back({loop_break, 0});
  continue_stack_.push_back({loop_continue, 0});
}

void Compiler::ForLoop(uint16_t id, bool has_init, bool has_test, bool has_next) {
  PushDebugName(NAME_WITH_ID("for", id));

  auto* loop_init = has_init ? CreateBasicBlock(NAME("loop-init")) : nullptr;
  auto* loop_test = has_test ? CreateBasicBlock(NAME("loop-test")) : nullptr;
  auto* loop_body = CreateBasicBlock(NAME("loop-body"));
  auto* loop_next = has_next ? CreateBasicBlock(NAME("loop-next")) : nullptr;
  auto* loop_end = CreateBasicBlock(NAME("loop-end"));

  auto* loop_start = loop_body;
  auto* loop_continue = loop_body;
  auto* loop_break = loop_end;
  auto* insert_point = loop_body;

  PushBlock(loop_end, "for-body-insert-point");
  if (has_next) {
    PushBlock(loop_next, "for-body-br");
  } else if (has_test) {
    PushBlock(loop_test, "for-body-br");
  } else {
    PushBlock(loop_body, "for-body-br");
  }

  if (has_next) {
    PushBlock(loop_body, "for-next-insert-point");
    if (has_test) {
      PushBlock(loop_test, "for-next-br");
    } else {
      PushBlock(loop_body, "for-next-br");
    }
    loop_continue = loop_next;
    insert_point = loop_next;
  }

  if (has_test) {
    if (has_next) {
      PushBlock(loop_next, "for-test-insert-point");
    } else {
      PushBlock(loop_body, "for-test-insert-point");
    }
    PushBlock(loop_end, "for-test-then");
    PushBlock(loop_body, "for-test-else");
    loop_start = loop_test;
    if (!has_next) {
      loop_continue = loop_test;
    }
    insert_point = loop_test;
  }

  if (has_init) {
    if (has_test) {
      PushBlock(loop_test, "for-init-insert-point");
      PushBlock(loop_test, "for-init-br");
    } else if (has_next) {
      PushBlock(loop_next, "for-init-insert-point");
      PushBlock(loop_body, "for-init-br");
    } else {
      PushBlock(loop_body, "for-init-insert-point");
      PushBlock(loop_body, "for-init-br");
    }
    loop_start = loop_init;
    insert_point = loop_init;
  }

  builder_->CreateBr(loop_start);
  builder_->SetInsertPoint(insert_point);

  SetBlockForLabelsInContinueStack(loop_continue);

  break_stack_.push_back({loop_break, 0});
  continue_stack_.push_back({loop_continue, 0});
}

void Compiler::LoopInit() {
  auto* next_block = PopBlock();
  auto* insert_point = PopBlock();

  builder_->CreateBr(next_block);
  builder_->SetInsertPoint(insert_point);
}

void Compiler::LoopTest() {
  auto cond = Dereference();
  auto* then_block = PopBlock();
  auto* else_block = PopBlock();
  auto* insert_point = PopBlock();

  auto* truthy = CreateToBoolean(cond);
  builder_->CreateCondBr(truthy, then_block, else_block);
  builder_->SetInsertPoint(insert_point);
}

void Compiler::LoopNext() {
  // Discard the evaluation result.
  Discard();
  auto* next_block = PopBlock();
  auto* insert_point = PopBlock();

  builder_->CreateBr(next_block);
  builder_->SetInsertPoint(insert_point);
}

void Compiler::LoopBody() {
  auto* next_block = PopBlock();
  auto* insert_point = PopBlock();

  builder_->CreateBr(next_block);
  break_stack_.back().block->moveAfter(builder_->GetInsertBlock());
  builder_->SetInsertPoint(insert_point);
}

void Compiler::LoopEnd() {
  PopDebugName();
  break_stack_.pop_back();
  continue_stack_.pop_back();
}

void Compiler::CaseBlock(uint16_t id, uint16_t num_cases) {
  UNUSED(num_cases);

  PushDebugName(NAME_WITH_ID("switch", id));

  auto item = Dereference();
  item.SetLabel("switch-value");
  stack_.push_back(item);
  stack_.push_back(item);  // Dup for test on CaseClause

  auto* start_block = CreateBasicBlock(NAME("start"));
  builder_->CreateBr(start_block);
  builder_->SetInsertPoint(start_block);

  auto* end_block = CreateBasicBlock(NAME("end"));
  break_stack_.push_back({end_block, 0});
}

void Compiler::CaseClause(bool has_statement) {
  UNUSED(has_statement);

  auto* case_clause_statement = builder_->GetInsertBlock();

  auto* else_block = CreateBasicBlock(NAME("else"));
  auto* then_block = PopBlock();
  auto* cond_block = PopBlock();
  auto* cond_value = PopBoolean();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_block, else_block);
  builder_->SetInsertPoint(else_block);

  PushBlock(case_clause_statement, "case-clause-statement");
  Swap();
  Duplicate();
}

void Compiler::DefaultClause(bool has_statement) {
  UNUSED(has_statement);

  auto* default_clause_statement = builder_->GetInsertBlock();

  auto* default_clause_br = PopBlock();
  auto* case_block = PopBlock();

  builder_->SetInsertPoint(case_block);

  PushBlock(default_clause_br, "default-clause-br");
  Swap();
  PushBlock(default_clause_statement, "default-clause-statement");
  Swap();
  Duplicate();
}

void Compiler::Switch(uint16_t id, uint16_t num_cases, uint16_t default_index) {
  UNUSED(id);

  PopDebugName();

  auto* end_block = break_stack_.back().block;
  break_stack_.pop_back();

  // Discard the switch-values
  Discard();
  Discard();

  auto* case_block = builder_->GetInsertBlock();

  // Connect statement blocks of case/default clauses.
  // The blocks has been stored in the stack in reverse order.
  auto* fallback_block = end_block;
  llvm::BasicBlock* default_block = nullptr;
  for (auto i = num_cases - 1;; --i) {
    auto* block = PopBlock();
    if (block->getTerminator() == nullptr) {
      builder_->SetInsertPoint(block);
      builder_->CreateBr(fallback_block);
      fallback_block->moveAfter(builder_->GetInsertBlock());
    }
    fallback_block = block;
    if (i == default_index) {
      default_block = PopBlock();
    }
    if (i == 0) {
      break;
    }
  }

  // Create an unconditional jump to the statement of the default clause if it exists.
  // Otherwise, jump to the end block.
  builder_->SetInsertPoint(case_block);
  if (default_block != nullptr) {
    builder_->CreateBr(default_block);
  } else {
    builder_->CreateBr(end_block);
    end_block->moveAfter(builder_->GetInsertBlock());
  }

  builder_->SetInsertPoint(end_block);
}

void Compiler::Try() {
  auto* try_block = CreateBasicBlock(NAME("try"));
  auto* catch_block = CreateBasicBlock(NAME("catch"));
  auto* finally_block = CreateBasicBlock(NAME("finally"));
  auto* end_block = CreateBasicBlock(NAME("end"));

  // Jump from the end of previous block to the beginning of the try block.
  builder_->CreateBr(try_block);

  builder_->SetInsertPoint(try_block);

  PushBlock(end_block, "try-end");
  PushBlock(finally_block, "finally");
  PushBlock(catch_block, "catch");

  catch_stack_.push_back(catch_block);
}

void Compiler::Catch(bool nominal) {
  auto* catch_block = PopBlock();
  auto* finally_block = PeekBlock();

  // Jump from the end of the try block to the beginning of the finally block.
  builder_->CreateBr(finally_block);

  builder_->SetInsertPoint(catch_block);

  if (!nominal) {
    // TODO: Reset the status to Status::Normal.
    builder_->CreateStore(builder_->getInt32(static_cast<int32_t>(Status::Normal)), status_);
  }

  catch_stack_.pop_back();
  catch_stack_.push_back(finally_block);
}

void Compiler::Finally(bool nominal) {
  UNUSED(nominal);

  auto* finally_block = PopBlock();

  // Jump from the end of the catch block to the beginning of the finally block.
  builder_->CreateBr(finally_block);

  builder_->SetInsertPoint(finally_block);

  catch_stack_.pop_back();
}

void Compiler::TryEnd() {
  auto* end_block = PopBlock();

  // Jump from the end of the finally block to the beginning of the outer catch block if there is
  // an uncaught exception.  Otherwise, jump to the beginning of the try-end block.
  auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_);
  auto* has_uncaught_exception =
      builder_->CreateICmpEQ(status, builder_->getInt32(static_cast<int32_t>(Status::Exception)));
  assert(!catch_stack_.empty());
  auto* catch_block = catch_stack_.back();
  builder_->CreateCondBr(has_uncaught_exception, catch_block, end_block);

  builder_->SetInsertPoint(end_block);
}

void Compiler::StartFunction(const char* name) {
  const auto& found = functions_.find(name);
  if (found != functions_.end()) {
    function_ = found->second;
  } else {
    // Create a function.
    auto* prototype = types_->CreateLambdaType();
    function_ = llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, name, *module_);
    functions_[name] = function_;
  }

  prologue_ = CreateBasicBlock("prologue");
  body_ = CreateBasicBlock("body");
  epilogue_ = CreateBasicBlock("epilogue");

  exec_context_ = function_->getArg(0);
  caps_ = function_->getArg(1);
  argc_ = function_->getArg(2);
  argv_ = function_->getArg(3);
  ret_ = function_->getArg(4);
  catch_stack_.push_back(epilogue_);

  builder_->SetInsertPoint(prologue_);
  status_ = builder_->CreateAlloca(builder_->getInt32Ty());
  builder_->CreateStore(builder_->getInt32(static_cast<int32_t>(Status::Normal)), status_);
  CreateStoreUndefinedToValue(ret_);

  // Switch the insertion point.
  builder_->SetInsertPoint(body_);
}

void Compiler::EndFunction(bool optimize) {
  builder_->CreateBr(epilogue_);
  epilogue_->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(prologue_);
  builder_->CreateBr(body_);

  builder_->SetInsertPoint(epilogue_);

  auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_);
  builder_->CreateRet(status);

  // DumpStack();

  assert(locals_.empty());
  locals_.clear();

  assert(stack_.empty());
  stack_.clear();

  assert(break_stack_.empty());
  break_stack_.clear();

  assert(continue_stack_.empty());
  continue_stack_.clear();

  assert(catch_stack_.size() == 1);
  catch_stack_.clear();

  if (llvm::verifyFunction(*function_, &llvm::errs())) {
    llvm::errs() << "<broken-function>\n";
    function_->print(llvm::errs());
    llvm::errs() << "</broken-function>\n";
    std::abort();
  }

  if (optimize) {
    fpm_->run(*function_, *fam_);
  }
}

void Compiler::StartScope(size_t scope_id) {
  PushDebugName(NAME_WITH_ID("scope", scope_id));

  auto* locals = CreateBasicBlock(NAME("locals"));
  auto* hoisted = CreateBasicBlock(NAME("hoisted"));
  auto* block = CreateBasicBlock(NAME("block"));
  auto* cleanup = CreateBasicBlock(NAME("cleanup"));
  builder_->CreateBr(locals);
  locals->moveAfter(builder_->GetInsertBlock());
  builder_->SetInsertPoint(block);
  scope_stack_.push_back({locals, hoisted, block, cleanup});
}

void Compiler::EndScope(size_t scope_id) {
  UNUSED(scope_id);

  PopDebugName();

  assert(!scope_stack_.empty());
  auto& scope = scope_stack_.back();

  builder_->CreateBr(scope.cleanup_block);
  scope.cleanup_block->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(scope.locals_block);
  builder_->CreateBr(scope.hoisted_block);
  scope.hoisted_block->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(scope.hoisted_block);
  builder_->CreateBr(scope.block);
  scope.block->moveAfter(builder_->GetInsertBlock());

  auto* block = CreateBasicBlock(NAME("block"));
  builder_->SetInsertPoint(scope.cleanup_block);
  builder_->CreateBr(block);
  block->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(block);

  scope_stack_.pop_back();
}

void Compiler::AllocateLocals(uint16_t num_locals) {
  assert(!scope_stack_.empty());

  auto& scope = scope_stack_.back();

  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(scope.locals_block);

  for (auto i = 0; i < num_locals; ++i) {
    auto* local = builder_->CreateAlloca(types_->CreateVariableType());
    CreateStoreFlagsToVariable(0, local);
    locals_.push_back(local);
  }

  builder_->SetInsertPoint(backup);
}

void Compiler::ReleaseLocals(uint16_t num_locals) {
  for (auto i = 0; i < num_locals; ++i) {
    // TODO: GC
    locals_.pop_back();
  }
}

void Compiler::CreateCapture(Locator locator) {
  assert(locator.kind != LocatorKind::Capture);
  assert(!scope_stack_.empty());

  auto& scope = scope_stack_.back();

  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(scope.locals_block);

  llvm::Value* variable_ptr;
  switch (locator.kind) {
    case LocatorKind::Argument:
      variable_ptr = CreateGetArgumentVariablePtr(locator.index);
      break;
    case LocatorKind::Local:
      variable_ptr = CreateGetLocalVariablePtr(locator.index);
      break;
    default:
      assert(false);
      return;
  }

  auto* capture_ptr = CreateCallRuntimeCreateCapture(variable_ptr);

  auto key = ComputeKeyFromLocator(locator);
  assert(captures_.find(key) == captures_.end());
  captures_[key] = capture_ptr;

  builder_->SetInsertPoint(backup);
}

void Compiler::CaptureVariable(bool declaration) {
  assert(!scope_stack_.empty());

  auto& scope = scope_stack_.back();

  llvm::BasicBlock* backup;
  if (declaration) {
    backup = builder_->GetInsertBlock();
    builder_->SetInsertPoint(scope.hoisted_block);
  }

  llvm::Value* capture_ptr;
  auto ref = PopReference();
  switch (ref.locator.kind) {
    case LocatorKind::Argument:
    case LocatorKind::Local: {
      auto key = ComputeKeyFromLocator(ref.locator);
      assert(captures_.find(key) != captures_.end());
      capture_ptr = captures_[key];
      break;
    }
    case LocatorKind::Capture:
      capture_ptr = CreateLoadCapturePtrFromCaptures(caps_, ref.locator.index);
      break;
    default:
      assert(false);
      return;
  }

  PushCapture(capture_ptr);

  if (declaration) {
    builder_->SetInsertPoint(backup);
  }
}

void Compiler::EscapeVariable(Locator locator) {
  assert(locator.kind != LocatorKind::Capture);
  assert(!scope_stack_.empty());

  auto& scope = scope_stack_.back();

  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(scope.cleanup_block);

  auto key = ComputeKeyFromLocator(locator);
  assert(captures_.find(key) != captures_.end());

  auto* capture_ptr = captures_[key];
  auto* escaped_ptr = CreateGetEscapedPtrOfCapture(capture_ptr);
  CreateStoreTargetToCapture(escaped_ptr, capture_ptr);

  auto* variable_ptr = CreateGetVariablePtr(locator);
  auto* variable = CreateLoadVariable(variable_ptr);
  CreateStoreEscapedToCapture(variable, capture_ptr);

  // The value of `locator.index` may be reused for another local variable.
  // The element identified by `locator.index` is removed from `captures_` here.
  captures_.erase(key);

  builder_->SetInsertPoint(backup);
}

void Compiler::LabelStart(uint32_t symbol, bool is_iteration_statement) {
  assert(symbol != 0);
  auto* start_block = CreateBasicBlock(NAME("start"));
  auto* end_block = CreateBasicBlock(NAME("end"));
  builder_->CreateBr(start_block);
  end_block->moveAfter(builder_->GetInsertBlock());
  builder_->SetInsertPoint(start_block);
  break_stack_.push_back({end_block, symbol});
  if (is_iteration_statement) {
    // The `block` member variable will be updated in the method to handle the loop start of the
    // labeled iteration statement.
    continue_stack_.push_back({nullptr, symbol});
  }
}

void Compiler::LabelEnd(uint32_t symbol, bool is_iteration_statement) {
  assert(symbol != 0);
  assert(break_stack_.back().symbol == symbol);
  auto* end_block = break_stack_.back().block;
  builder_->CreateBr(end_block);
  end_block->moveAfter(builder_->GetInsertBlock());
  builder_->SetInsertPoint(end_block);
  break_stack_.pop_back();
  if (is_iteration_statement) {
    continue_stack_.pop_back();
  }
}

void Compiler::Continue(uint32_t symbol) {
  llvm::BasicBlock* target_block = nullptr;
  if (symbol == 0) {
    target_block = continue_stack_.back().block;
  } else {
    target_block = FindBlockBySymbol(continue_stack_, symbol);
  }
  assert(target_block != nullptr);
  builder_->CreateBr(target_block);
  // TODO(issue#234)
  CreateBasicBlockForDeadcode();
}

void Compiler::Break(uint32_t symbol) {
  llvm::BasicBlock* target_block = nullptr;
  if (symbol == 0) {
    target_block = break_stack_.back().block;
  } else {
    target_block = FindBlockBySymbol(break_stack_, symbol);
  }
  assert(target_block != nullptr);
  builder_->CreateBr(target_block);
  // TODO(issue#234)
  CreateBasicBlockForDeadcode();
}

void Compiler::Return(size_t n) {
  if (n > 0) {
    assert(n == 1);
    auto item = Dereference();
    CreateStoreItemToValue(item, ret_);
  }
  builder_->CreateBr(epilogue_);
  // TODO(issue#234)
  CreateBasicBlockForDeadcode();
}

void Compiler::Throw() {
  auto item = Dereference();
  CreateStoreItemToValue(item, ret_);
  builder_->CreateStore(builder_->getInt32(static_cast<int32_t>(Status::Exception)), status_);
  auto* catch_block = catch_stack_.back();
  builder_->CreateBr(catch_block);
  // TODO(issue#234)
  CreateBasicBlockForDeadcode();
}

void Compiler::Discard() {
  assert(!stack_.empty());
  PopItem();
}

void Compiler::Swap() {
  assert(stack_.size() >= 2);
  auto i = stack_.size() - 1;
  Item item = stack_[i];
  stack_[i] = stack_[i - 1];
  stack_[i - 1] = item;
}

void Compiler::DumpStack() {
  llvm::errs() << "<llvm-ir:compiler-stack>\n";
  for (auto it = stack_.rbegin(); it != stack_.rend(); ++it) {
    const auto& item = *it;
    switch (item.type) {
      case Item::Undefined:
        llvm::errs() << "value: undefined";
        break;
      case Item::Null:
        llvm::errs() << "value: null";
        break;
      case Item::Boolean:
        llvm::errs() << "boolean: " << item.value;
        break;
      case Item::Number:
        llvm::errs() << "number: " << item.value;
        break;
      case Item::Function:
        llvm::errs() << "function: " << item.func;
        break;
      case Item::Closure:
        llvm::errs() << "closure: " << item.value;
        break;
      case Item::Any:
        llvm::errs() << "any: " << item.value;
        break;
      case Item::Reference:
        llvm::errs() << "reference: symbol=" << item.reference.symbol;
        switch (item.reference.locator.kind) {
          case LocatorKind::None:
            llvm::errs() << " locator=none";
            return;
          case LocatorKind::Argument:
            llvm::errs() << " locator=argument(";
            break;
          case LocatorKind::Local:
            llvm::errs() << " locator=local(";
            break;
          case LocatorKind::Capture:
            llvm::errs() << " locator=capture(";
            break;
        }
        llvm::errs() << item.reference.locator.index;
        llvm::errs() << ")";
        break;
      case Item::Argv:
        llvm::errs() << "argv: " << item.value;
        break;
      case Item::Block:
        llvm::errs() << "block: " << item.block;
        break;
      case Item::Capture:
        llvm::errs() << "capture: " << item.value;
        break;
    }
#if defined(BEE_BUILD_DEBUG)
    if (item.label != nullptr) {
      llvm::errs() << " [" << item.label << "]";
    }
#endif
    llvm::errs() << "\n";
  }
  llvm::errs() << "</llvm-ir:compiler-stack>\n";
}

Compiler::Item Compiler::Dereference(struct Reference* ref) {
  const auto item = PopItem();
  switch (item.type) {
    case Item::Undefined:
    case Item::Null:
    case Item::Boolean:
    case Item::Number:
    case Item::Function:
    case Item::Closure:
    case Item::Any:
      return item;
    case Item::Reference: {
      if (ref != nullptr) {
        *ref = item.reference;
      }
      auto* value_ptr = CreateGetValuePtr(item.reference.locator);
      return Item(Item::Any, value_ptr);
    }
    default:
      // never reach here
      assert(false);
      return Item(Item::Undefined);
  }
}

// 13.4.2.1 Runtime Semantics: Evaluation
// 13.4.3.1 Runtime Semantics: Evaluation
// 13.4.4.1 Runtime Semantics: Evaluation
// 13.4.5.1 Runtime Semantics: Evaluation
void Compiler::IncrDecr(char pos, char op) {
  struct Reference ref;
  auto* old_value = ToNumeric(Dereference(&ref));
  // TODO: BigInt
  auto* one = llvm::ConstantFP::get(builder_->getDoubleTy(), 1.0);
  auto* new_value =
      op == '+' ? builder_->CreateFAdd(old_value, one) : builder_->CreateFSub(old_value, one);
  if (ref.symbol != 0) {
    assert(ref.locator.kind != LocatorKind::None);
    PushReference(ref.symbol, ref.locator);
    PushNumber(new_value);
    Assignment();
    Discard();
  } else {
    // TODO: throw a ReferenceError at runtime
  }
  pos == '^' ? PushNumber(new_value) : PushNumber(old_value);
}

// 6.1.6.1.16 NumberBitwiseOp ( op, x, y )
void Compiler::NumberBitwiseOp(char op, llvm::Value* x, llvm::Value* y) {
  auto* lint = ToInt32(x);
  auto* rint = ToInt32(y);
  llvm::Value* oint;
  switch (op) {
    case '&':
      oint = builder_->CreateAnd(lint, rint);
      break;
    case '^':
      oint = builder_->CreateXor(lint, rint);
      break;
    case '|':
      oint = builder_->CreateOr(lint, rint);
      break;
    default:
      assert(false);
      oint = nullptr;
      break;
  }
  auto* onum = builder_->CreateSIToFP(oint, builder_->getDoubleTy());
  PushNumber(onum);
}

void Compiler::CreateStoreItemToVariable(const Item& item, llvm::Value* variable_ptr) {
  switch (item.type) {
    case Item::Undefined:
      CreateStoreUndefinedToVariable(variable_ptr);
      break;
    case Item::Null:
      CreateStoreNullToVariable(variable_ptr);
      break;
    case Item::Boolean:
      CreateStoreBooleanToVariable(item.value, variable_ptr);
      break;
    case Item::Number:
      CreateStoreNumberToVariable(item.value, variable_ptr);
      break;
    case Item::Closure:
      CreateStoreClosureToVariable(item.value, variable_ptr);
      break;
    case Item::Any:
      CreateStoreValueToVariable(item.value, variable_ptr);
      break;
    default:
      assert(false);
      break;
  }
}

void Compiler::CreateStoreItemToValue(const Item& item, llvm::Value* value_ptr) {
  switch (item.type) {
    case Item::Undefined:
      CreateStoreUndefinedToValue(value_ptr);
      break;
    case Item::Null:
      CreateStoreNullToValue(value_ptr);
      break;
    case Item::Boolean:
      CreateStoreBooleanToValue(item.value, value_ptr);
      break;
    case Item::Number:
      CreateStoreNumberToValue(item.value, value_ptr);
      break;
    case Item::Closure:
      CreateStoreClosureToValue(item.value, value_ptr);
      break;
    case Item::Any:
      builder_->CreateMemCpy(value_ptr, llvm::MaybeAlign(), item.value, llvm::MaybeAlign(),
          builder_->getInt32(sizeof(Value)));
      break;
    default:
      assert(false);
      break;
  }
}

// 7.1.4 ToNumber ( argument )
llvm::Value* Compiler::ToNumeric(const Item& item) {
  switch (item.type) {
    case Item::Undefined:
      return llvm::ConstantFP::getNaN(builder_->getDoubleTy());
    case Item::Null:
      return llvm::ConstantFP::getZero(builder_->getDoubleTy());
    case Item::Boolean:
      return builder_->CreateUIToFP(item.value, builder_->getDoubleTy());
    case Item::Number:
      return item.value;
    case Item::Function:
    case Item::Closure:
      return llvm::ConstantFP::getNaN(builder_->getDoubleTy());
    case Item::Any:
      return ToNumeric(item.value);
    default:
      assert(false);
      return nullptr;
  }
}

llvm::Value* Compiler::ToNumeric(llvm::Value* value_ptr) {
  auto* call = types_->CreateRuntimeToNumeric();
  return builder_->CreateCall(call, {exec_context_, value_ptr});
}

// 7.1.6 ToInt32 ( argument )
llvm::Value* Compiler::ToInt32(llvm::Value* number) {
  // Skip the first step.
  // We assumed that `number` holds a number value.
  // TODO: Create inline instructions if runtime_to_int32() is slow.
  auto* func = types_->CreateRuntimeToInt32();
  return builder_->CreateCall(func, {exec_context_, number});
}

// 7.1.7 ToUint32 ( argument )
llvm::Value* Compiler::ToUint32(llvm::Value* number) {
  // Skip the first step.
  // We assumed that `number` holds a number value.
  // TODO: Create inline instructions if runtime_to_uint32() is slow.
  auto* func = types_->CreateRuntimeToUint32();
  return builder_->CreateCall(func, {exec_context_, number});
}

llvm::Value* Compiler::ToAny(const Item& item) {
  if (item.type == Item::Any) {
    return item.value;
  }
  auto* value_ptr = CreateAllocaInEntryBlock(types_->CreateValueType());
  CreateStoreItemToValue(item, value_ptr);
  return value_ptr;
}

llvm::AllocaInst* Compiler::CreateAllocaInEntryBlock(llvm::Type* ty, uint32_t n) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  auto* alloca = builder_->CreateAlloca(ty, builder_->getInt32(n));
  builder_->SetInsertPoint(backup);
  return alloca;
}

// non-nullish

llvm::Value* Compiler::CreateIsNonNullish(const Item& item) {
  switch (item.type) {
    case Item::Undefined:
    case Item::Null:
      return builder_->getFalse();
    case Item::Boolean:
    case Item::Number:
    case Item::Function:
    case Item::Closure:
      return builder_->getTrue();
    case Item::Any:
      return CreateIsNonNullish(item.value);
    default:
      // never reach here.
      assert(false);
      return nullptr;
  }
}

llvm::Value* Compiler::CreateIsNonNullish(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpUGT(kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Null)));
}

// 7.1.2 ToBoolean ( argument )

llvm::Value* Compiler::CreateToBoolean(const Item& item) {
  switch (item.type) {
    case Item::Undefined:
      return builder_->getFalse();
    case Item::Null:
      return builder_->getFalse();
    case Item::Boolean:
      return item.value;
    case Item::Number:
      return builder_->CreateFCmpUNE(
          item.value, llvm::ConstantFP::getZero(builder_->getDoubleTy()));
    case Item::Function:
    case Item::Closure:
      return builder_->getTrue();
    case Item::Any:
      return CreateToBoolean(item.value);
    default:
      // never reach here.
      assert(false);
      return nullptr;
  }
}

llvm::Value* Compiler::CreateToBoolean(llvm::Value* value_ptr) {
  auto* func = types_->CreateRuntimeToBoolean();
  return builder_->CreateCall(func, {exec_context_, value_ptr});
}

// 7.2.13 IsLooselyEqual ( x, y )

llvm::Value* Compiler::CreateIsLooselyEqual(const Item& lhs, const Item& rhs) {
  if (lhs.type == Item::Any) {
    return CreateIsLooselyEqual(lhs.value, rhs);
  }
  if (rhs.type == Item::Any) {
    return CreateIsLooselyEqual(rhs.value, lhs);
  }
  // 1. If Type(x) is Type(y), then Return IsStrictlyEqual(x, y).
  if (lhs.type == rhs.type) {
    return CreateIsStrictlyEqual(lhs, rhs);
  }
  // 2. If x is null and y is undefined, return true.
  if (lhs.type == Item::Undefined && rhs.type == Item::Null) {
    return builder_->getTrue();
  }
  // 3. If x is undefined and y is null, return true.
  if (lhs.type == Item::Null && rhs.type == Item::Undefined) {
    return builder_->getTrue();
  }
  // TODO: 5. If x is a Number and y is a String, return ! IsLooselyEqual(x, ! ToNumber(y)).
  // TODO: 6. If x is a String and y is a Number, return ! IsLooselyEqual(! ToNumber(x), y).
  // TODO: 7. If x is a BigInt and y is a String, then
  // TODO: 8. If x is a String and y is a BigInt, return ! IsLooselyEqual(y, x).
  // TODO
  // TODO: 9. If x is a Boolean, return ! IsLooselyEqual(! ToNumber(x), y).
  // TODO: 10. If y is a Boolean, return ! IsLooselyEqual(x, ! ToNumber(y)).
  // TODO: ...
  auto* lval = ToAny(lhs);
  auto* rval = ToAny(rhs);
  return CreateIsLooselyEqual(lval, rval);
}

llvm::Value* Compiler::CreateIsLooselyEqual(llvm::Value* value_ptr, const Item& item) {
  // TODO: compile-time evaluation
  auto* any = ToAny(item);
  return CreateIsLooselyEqual(value_ptr, any);
}

llvm::Value* Compiler::CreateIsLooselyEqual(llvm::Value* x, llvm::Value* y) {
  // TODO: Create inline instructions if runtime_is_loosely_equal() is slow.
  auto* func = types_->CreateRuntimeIsLooselyEqual();
  return builder_->CreateCall(func, {exec_context_, x, y});
}

// 7.2.14 IsStrictlyEqual ( x, y )

llvm::Value* Compiler::CreateIsStrictlyEqual(const Item& lhs, const Item& rhs) {
  if (lhs.type == Item::Any) {
    return CreateIsStrictlyEqual(lhs.value, rhs);
  }
  if (rhs.type == Item::Any) {
    return CreateIsStrictlyEqual(rhs.value, lhs);
  }
  if (lhs.type != rhs.type) {
    return builder_->getFalse();
  }
  // TODO: BigInt
  switch (lhs.type) {
    case Item::Undefined:
    case Item::Null:
      return builder_->getTrue();
    case Item::Boolean:
      return builder_->CreateICmpEQ(lhs.value, rhs.value);
    case Item::Number:
      return builder_->CreateFCmpOEQ(lhs.value, rhs.value);
    case Item::Function:
    case Item::Closure:
      return builder_->CreateICmpEQ(lhs.value, rhs.value);
    default:
      // never reach here.
      assert(false);
      return nullptr;
  }
}

llvm::Value* Compiler::CreateIsStrictlyEqual(llvm::Value* value_ptr, const Item& item) {
  switch (item.type) {
    case Item::Undefined:
      return CreateIsUndefined(value_ptr);
    case Item::Null:
      return CreateIsNull(value_ptr);
    case Item::Boolean:
      return CreateIsSameBooleanValue(value_ptr, item.value);
    case Item::Number:
      return CreateIsSameNumberValue(value_ptr, item.value);
    case Item::Closure:
      return CreateIsSameClosureValue(value_ptr, item.value);
    case Item::Any:
      return CreateIsStrictlyEqual(value_ptr, item.value);
    default:
      // never reach here.
      assert(false);
      return nullptr;
  }
}

llvm::Value* Compiler::CreateIsStrictlyEqual(llvm::Value* x, llvm::Value* y) {
  // TODO: Create inline instructions if runtime_is_strictly_equal() is slow.
  auto* func = types_->CreateRuntimeIsStrictlyEqual();
  return builder_->CreateCall(func, {exec_context_, x, y});
}

llvm::Value* Compiler::CreateIsUndefined(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(
      kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Undefined)));
}

llvm::Value* Compiler::CreateIsNull(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Null)));
}

llvm::Value* Compiler::CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = CreateBasicBlock(NAME("then"));
  auto* else_block = CreateBasicBlock(NAME("else"));
  auto* merge_block = CreateBasicBlock(NAME("end"));

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond =
      builder_->CreateICmpEQ(kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Boolean)));
  builder_->CreateCondBr(cond, then_block, else_block);

  builder_->SetInsertPoint(then_block);
  auto* boolean = CreateLoadBooleanFromValue(value_ptr);
  auto* then_value = builder_->CreateICmpEQ(boolean, value);
  builder_->CreateBr(merge_block);

  builder_->SetInsertPoint(else_block);
  auto* else_value = builder_->getFalse();
  builder_->CreateBr(merge_block);

  builder_->SetInsertPoint(merge_block);
  auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2);
  phi->addIncoming(then_value, then_block);
  phi->addIncoming(else_value, else_block);

  return phi;
}

llvm::Value* Compiler::CreateIsSameNumberValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = CreateBasicBlock(NAME("then"));
  auto* else_block = CreateBasicBlock(NAME("else"));
  auto* merge_block = CreateBasicBlock(NAME("end"));

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond =
      builder_->CreateICmpEQ(kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Number)));
  builder_->CreateCondBr(cond, then_block, else_block);

  builder_->SetInsertPoint(then_block);
  auto* number = CreateLoadNumberFromValue(value_ptr);
  auto* then_value = builder_->CreateFCmpOEQ(number, value);
  builder_->CreateBr(merge_block);

  builder_->SetInsertPoint(else_block);
  auto* else_value = builder_->getFalse();
  builder_->CreateBr(merge_block);

  builder_->SetInsertPoint(merge_block);
  auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2);
  phi->addIncoming(then_value, then_block);
  phi->addIncoming(else_value, else_block);

  return phi;
}

llvm::Value* Compiler::CreateIsSameClosureValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = CreateBasicBlock(NAME("then"));
  auto* else_block = CreateBasicBlock(NAME("else"));
  auto* merge_block = CreateBasicBlock(NAME("end"));

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond =
      builder_->CreateICmpEQ(kind, builder_->getInt8(static_cast<uint8_t>(ValueKind::Closure)));
  builder_->CreateCondBr(cond, then_block, else_block);

  builder_->SetInsertPoint(then_block);
  auto* func_ptr = CreateLoadFunctionFromValue(value_ptr);
  auto* then_value = builder_->CreateICmpEQ(func_ptr, value);
  builder_->CreateBr(merge_block);

  builder_->SetInsertPoint(else_block);
  auto* else_value = builder_->getFalse();
  builder_->CreateBr(merge_block);

  builder_->SetInsertPoint(merge_block);
  auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2);
  phi->addIncoming(then_value, then_block);
  phi->addIncoming(else_value, else_block);

  return phi;
}

llvm::Value* Compiler::CreateCallRuntimeCreateCapture(llvm::Value* variable_ptr) {
  auto* func = types_->CreateRuntimeCreateCapture();
  return builder_->CreateCall(func, {exec_context_, variable_ptr});
}

llvm::Value* Compiler::CreateCallRuntimeCreateClosure(llvm::Value* lambda, uint16_t num_captures) {
  auto* func = types_->CreateRuntimeCreateClosure();
  return builder_->CreateCall(func, {exec_context_, lambda, builder_->getInt16(num_captures)});
}

llvm::Value* Compiler::CreateGetVariablePtr(Locator locator) {
  switch (locator.kind) {
    case LocatorKind::Argument:
      return CreateGetArgumentVariablePtr(locator.index);
    case LocatorKind::Local:
      return CreateGetLocalVariablePtr(locator.index);
    case LocatorKind::Capture:
      return CreateGetCaptureVariablePtr(locator.index);
    default:
      // never reach here
      assert(false);
      return nullptr;
  }
}

llvm::Value* Compiler::CreateGetValuePtr(Locator locator) {
  switch (locator.kind) {
    case LocatorKind::Argument:
      return CreateGetArgumentValuePtr(locator.index);
    case LocatorKind::Local:
      return CreateGetLocalValuePtr(locator.index);
    case LocatorKind::Capture:
      return CreateGetCaptureValuePtr(locator.index);
    default:
      // never reach here
      assert(false);
      return nullptr;
  }
}

void Compiler::CreateBasicBlockForDeadcode() {
  auto* block = CreateBasicBlock(NAME("deadcode"));
  builder_->SetInsertPoint(block);
}

llvm::BasicBlock* Compiler::FindBlockBySymbol(const std::vector<BlockItem>& stack,
    uint32_t symbol) const {
  assert(!break_stack_.empty());
  for (auto it = stack.rbegin(); it != stack.rend(); ++it) {
    if (it->symbol == symbol) {
      return it->block;
    }
  }
  assert(false);  // never reach here
  return nullptr;
}

void Compiler::SetBlockForLabelsInContinueStack(llvm::BasicBlock* block) {
  assert(block != nullptr);
  for (auto it = continue_stack_.rbegin(); it != continue_stack_.rend(); ++it) {
    if (it->symbol == 0) {
      assert(it->block != nullptr);
      return;
    }
    assert(it->block == nullptr);
    it->block = block;
  }
}

#if defined(BEE_BUILD_DEBUG)
#include <sstream>

std::string Compiler::MakeDebugName(const char* name) const {
  if (debug_name_stack_.empty()) {
    return name;
  }
  std::stringstream ss;
  ss << debug_name_stack_[0];
  for (size_t i = 1; i < debug_name_stack_.size(); ++i) {
    ss << '.' << debug_name_stack_[i];
  }
  ss << '.' << name;
  return ss.str();
}
#endif
