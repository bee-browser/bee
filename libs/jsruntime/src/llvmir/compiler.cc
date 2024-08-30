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
  auto* lambda = CreateLambda(name);
  PushFunction(lambda);
}

void Compiler::Closure(llvm::BasicBlock* block, uint16_t num_captures) {
  assert(stack_.size() >= 1 + static_cast<size_t>(num_captures));

  llvm::BasicBlock* backup;
  if (block != nullptr) {
    backup = builder_->GetInsertBlock();
    builder_->SetInsertPoint(block);
  }

  auto* lambda = PopFunction();
  auto* closure_ptr = CreateCallRuntimeCreateClosure(lambda, num_captures);

  auto* captures = CreateLoadCapturesFromClosure(closure_ptr);
  for (uint16_t i = 0; i < num_captures; ++i) {
    auto* capture_ptr = PopCapture();
    CreateStoreCapturePtrToCaptures(capture_ptr, captures, i);
  }

  PushClosure(closure_ptr);

  if (block != nullptr) {
    builder_->SetInsertPoint(backup);
  }
}

void Compiler::Reference(uint32_t symbol, Locator locator) {
  PushReference(symbol, locator);
}

void Compiler::Exception() {
  // TODO: Should we check status_ at runtime?
  PushAny(retv_);
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
  auto* v = builder_->CreateFNeg(num, REG_NAME("neg"));
  PushNumber(v);
}

// 13.5.6.1 Runtime Semantics: Evaluation
void Compiler::BitwiseNot() {
  auto* num = ToNumeric(Dereference());
  // TODO: BigInt
  // 6.1.6.1.2 Number::bitwiseNOT ( x )
  auto* int32 = ToInt32(num);
  auto* xored = builder_->CreateXor(int32, -1, REG_NAME("xor"));
  auto* v = builder_->CreateSIToFP(xored, builder_->getDoubleTy(), REG_NAME("si2fp"));
  PushNumber(v);
}

// 13.5.7.1 Runtime Semantics: Evaluation
void Compiler::LogicalNot() {
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  auto* v = builder_->CreateXor(truthy, builder_->getTrue(), REG_NAME("xor"));
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
  auto* v = builder_->CreateFMul(lhs, rhs, REG_NAME("mul"));
  PushNumber(v);
}

// 13.7.1 Runtime Semantics: Evaluation
void Compiler::Division() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs, REG_NAME("div"));
  PushNumber(v);
}

// 13.7.1 Runtime Semantics: Evaluation
void Compiler::Remainder() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs, REG_NAME("rem"));
  PushNumber(v);
}

// 13.8.1.1 Runtime Semantics: Evaluation
void Compiler::Addition() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs, REG_NAME("add"));
  PushNumber(v);
}

// 13.8.2.1 Runtime Semantics: Evaluation
void Compiler::Subtraction() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs, REG_NAME("sub"));
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
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
  auto* shifted = builder_->CreateShl(lnum, shift_count, REG_NAME("shl"));
  auto* v = builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
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
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
  auto* shifted = builder_->CreateAShr(lnum, shift_count, REG_NAME("ashr"));
  auto* v = builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
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
  auto* shift_count = builder_->CreateURem(rnum, builder_->getInt32(32), REG_NAME("rem"));
  auto* shifted = builder_->CreateLShr(lnum, shift_count, REG_NAME("lshr"));
  auto* v = builder_->CreateSIToFP(shifted, builder_->getDoubleTy(), REG_NAME("si2fp"));
  PushNumber(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::LessThan() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs, REG_NAME("lt"));
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::GreaterThan() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs, REG_NAME("gt"));
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::LessThanOrEqual() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs, REG_NAME("le"));
  PushBoolean(v);
}

// 13.10.1 Runtime Semantics: Evaluation
void Compiler::GreaterThanOrEqual() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs, REG_NAME("ge"));
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
  auto* v = builder_->CreateXor(eq, builder_->getTrue(), REG_NAME("not"));
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
  auto* v = builder_->CreateXor(eq, builder_->getTrue(), REG_NAME("not"));
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

void Compiler::Ternary(llvm::BasicBlock* test_block, llvm::BasicBlock* then_head_block, llvm::BasicBlock* then_tail_block, llvm::BasicBlock* else_head_block) {
  assert(test_block != nullptr);
  assert(then_head_block != nullptr);
  assert(then_tail_block != nullptr);
  assert(else_head_block != nullptr);

  auto* else_tail_block = builder_->GetInsertBlock();

  auto else_item = Dereference();

  builder_->SetInsertPoint(then_tail_block);
  auto then_item = Dereference();

  builder_->SetInsertPoint(test_block);
  auto* cond_value = PopBoolean();
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  auto* block = CreateBasicBlock(BB_NAME("ternary"));

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
        auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2, REG_NAME("ternary"));
        phi->addIncoming(then_item.value, then_tail_block);
        phi->addIncoming(else_item.value, else_tail_block);
        PushBoolean(phi);
        return;
      }
      case Item::Number: {
        auto* phi = builder_->CreatePHI(builder_->getDoubleTy(), 2, REG_NAME("ternary"));
        phi->addIncoming(then_item.value, then_tail_block);
        phi->addIncoming(else_item.value, else_tail_block);
        PushNumber(phi);
        return;
      }
      case Item::Any: {
        auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2, REG_NAME("ternary"));
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
  auto* phi = builder_->CreatePHI(builder_->getPtrTy(), 2, REG_NAME("ternary"));
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

  auto* variable_ptr = GetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);
}

void Compiler::DeclareMutable() {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED | VARIABLE_MUTABLE;

  auto item = Dereference();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = GetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);
}

void Compiler::DeclareFunction(llvm::BasicBlock* block) {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED | VARIABLE_MUTABLE;

  assert(block != nullptr);

  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(block);

  auto item = Dereference();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = GetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);

  builder_->SetInsertPoint(backup);
}

void Compiler::DeclareClosure(llvm::BasicBlock* block) {
  static constexpr uint8_t FLAGS = VARIABLE_INITIALIZED | VARIABLE_MUTABLE;

  assert(block != nullptr);

  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(block);

  auto item = Dereference();
  auto ref = PopReference();
  assert(ref.locator.kind == LocatorKind::Local);

  auto* variable_ptr = GetLocalVariablePtr(ref.locator.index);
  CreateStoreFlagsToVariable(FLAGS, variable_ptr);
  CreateStoreSymbolToVariable(ref.symbol, variable_ptr);
  CreateStoreItemToVariable(item, variable_ptr);

  builder_->SetInsertPoint(backup);
}

void Compiler::Arguments(uint16_t argc) {
  assert(argc > 0);
  auto* argv = CreateAllocN(types_->CreateValueType(), argc, REG_NAME("args.ptr"));
  PushArgv(argv);
  Swap();
}

void Compiler::Argument(uint16_t index) {
  auto item = Dereference();
  auto* argv = PopArgv();
  auto* arg_ptr = builder_->CreateConstInBoundsGEP1_32(
      types_->CreateValueType(), argv, index, REG_NAME("args." + llvm::Twine(index) + ".ptr"));
  CreateStoreItemToValue(item, arg_ptr);
  PushArgv(argv);
}

void Compiler::Call(uint16_t argc, llvm::BasicBlock* block) {
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
  auto* retv = CreateAlloc1(types_->CreateValueType(), REG_NAME("retv.ptr"));

  auto* status = builder_->CreateCall(prototype, lambda,
      {exec_context_, caps, types_->GetWord(argc), argv, retv}, REG_NAME("status"));

  CreateCheckStatusForException(status, retv, block);

  PushAny(retv);
}

llvm::Value* Compiler::CreateLoadClosureFromValueOrThrowTypeError(llvm::Value* value_ptr) {
  auto* closure = CreateAlloc1(builder_->getPtrTy(), REG_NAME("closure.ptr"));

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* then_block = CreateBasicBlock(BB_NAME("is_closure.then"));
  auto* else_block = CreateBasicBlock(BB_NAME("is_closure.else"));
  auto* end_block = CreateBasicBlock(BB_NAME("closure"));

  // if (value.kind == ValueKind::Closure)
  auto* is_closure =
      builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindClosure), REG_NAME("is_closure"));
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
  return builder_->CreateLoad(builder_->getPtrTy(), closure, REG_NAME("closure"));
}

// Handle an exception if it's thrown.
void Compiler::CreateCheckStatusForException(llvm::Value* status, llvm::Value* retv, llvm::BasicBlock* block) {
  assert(block != nullptr);

  auto* status_exception = builder_->getInt32(STATUS_EXCEPTION);
  auto* exception_block = CreateBasicBlock(BB_NAME("status.exception"));
  auto* normal_block = CreateBasicBlock(BB_NAME("status.normal"));

  // if (status == Status::Exception)
  auto* is_exception = builder_->CreateICmpEQ(status, status_exception, REG_NAME("is_exception"));
  builder_->CreateCondBr(is_exception, exception_block, normal_block);
  // {
  builder_->SetInsertPoint(exception_block);
  // Store the exception.
  builder_->CreateStore(status_exception, status_);
  CreateStoreValueToVariable(retv, retv_);
  builder_->CreateBr(block);
  // }

  builder_->SetInsertPoint(normal_block);
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
  stack_.push_back(item);
}

void Compiler::TruthyShortCircuit() {
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  stack_.push_back(item);
}

void Compiler::NullishShortCircuit() {
  const auto item = Dereference();
  auto* non_nullish = CreateIsNonNullish(item);
  PushBoolean(non_nullish);
  stack_.push_back(item);
}

void Compiler::FalsyShortCircuitAssignment() {
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  LogicalNot();
  stack_.push_back(item);
}

void Compiler::TruthyShortCircuitAssignment() {
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  const auto item = Dereference();
  auto* truthy = CreateToBoolean(item);
  PushBoolean(truthy);
  stack_.push_back(item);
}

void Compiler::NullishShortCircuitAssignment() {
  assert(stack_.back().type == Item::Reference);
  Duplicate();
  const auto item = Dereference();
  auto* non_nullish = CreateIsNonNullish(item);
  PushBoolean(non_nullish);
  stack_.push_back(item);
}

void Compiler::IfElseStatement(llvm::BasicBlock* test_block, llvm::BasicBlock* then_head_block, llvm::BasicBlock* then_tail_block, llvm::BasicBlock* else_head_block) {
  assert(test_block != nullptr);
  assert(then_head_block != nullptr);
  assert(then_tail_block != nullptr);
  assert(else_head_block != nullptr);

  auto* else_tail_block = builder_->GetInsertBlock();

  llvm::BasicBlock* block = nullptr;

  if (else_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    block = CreateBasicBlock(BB_NAME("block"));
    builder_->CreateBr(block);
  }

  if (then_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    if (block == nullptr) {
      block = CreateBasicBlock(BB_NAME("block"));
    }
    builder_->SetInsertPoint(then_tail_block);
    builder_->CreateBr(block);
  }

  auto* cond_value = PopBoolean();

  builder_->SetInsertPoint(test_block);
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  if (block != nullptr) {
    builder_->SetInsertPoint(block);
  }
}

void Compiler::IfStatement(llvm::BasicBlock* test_block, llvm::BasicBlock* then_block) {
  assert(test_block != nullptr);
  assert(then_block != nullptr);

  auto* then_tail_block = builder_->GetInsertBlock();

  auto* block = CreateBasicBlock(BB_NAME("block"));

  if (then_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    builder_->CreateBr(block);
  }

  auto* cond_value = PopBoolean();

  builder_->SetInsertPoint(test_block);
  builder_->CreateCondBr(cond_value, then_block, block);

  builder_->SetInsertPoint(block);
}

// TODO: refactoring
void Compiler::LoopTest(llvm::BasicBlock* then_block, llvm::BasicBlock* else_block, llvm::BasicBlock* insert_point) {
  assert(then_block != nullptr);
  assert(else_block != nullptr);
  assert(insert_point != nullptr);
  auto cond = Dereference();
  auto* truthy = CreateToBoolean(cond);
  builder_->CreateCondBr(truthy, then_block, else_block);
  builder_->SetInsertPoint(insert_point);
}

void Compiler::CaseBlock(uint16_t id, uint16_t num_cases) {
  UNUSED(id);
  UNUSED(num_cases);

  auto item = Dereference();
  item.SetLabel("switch-value");
  stack_.push_back(item);
  stack_.push_back(item);  // Dup for test on CaseClause

  auto* start_block = CreateBasicBlock(BB_NAME("start"));
  builder_->CreateBr(start_block);
  builder_->SetInsertPoint(start_block);
}

void Compiler::CaseClause(bool has_statement, llvm::BasicBlock* before_block, llvm::BasicBlock* after_block) {
  UNUSED(has_statement);
  assert(before_block != nullptr);
  assert(after_block != nullptr);

  auto* else_block = CreateBasicBlock(BB_NAME("else"));
  auto* cond_value = PopBoolean();

  builder_->SetInsertPoint(before_block);
  builder_->CreateCondBr(cond_value, after_block, else_block);
  builder_->SetInsertPoint(else_block);

  Duplicate();
}

void Compiler::DefaultClause(bool has_statement, llvm::BasicBlock* before_block) {
  UNUSED(has_statement);
  assert(before_block != nullptr);

  builder_->SetInsertPoint(before_block);

  Duplicate();
}

void Compiler::TryEnd(llvm::BasicBlock* exception_block, llvm::BasicBlock* end_block) {
  assert(exception_block != nullptr);
  assert(end_block != nullptr);

  // Jump from the end of the finally block to the beginning of the outer catch block if there is
  // an uncaught exception.  Otherwise, jump to the beginning of the try-end block.
  auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_, REG_NAME("status"));
  auto* has_uncaught_exception = builder_->CreateICmpEQ(
      status, builder_->getInt32(STATUS_EXCEPTION), REG_NAME("has_uncaught_exception"));
  builder_->CreateCondBr(has_uncaught_exception, exception_block, end_block);
}

void Compiler::StartFunction(const char* name) {
  function_ = CreateLambda(name);

  locals_block_ = CreateBasicBlock(BB_NAME("locals"));
  args_block_ = CreateBasicBlock(BB_NAME("args"));
  body_block_ = CreateBasicBlock(BB_NAME("body"));
  return_block_ = CreateBasicBlock(BB_NAME("return"));

  exec_context_ = function_->getArg(0);
  caps_ = function_->getArg(1);
  argc_ = function_->getArg(2);
  argv_ = function_->getArg(3);
  retv_ = function_->getArg(4);
  status_ = CreateAlloc1(builder_->getInt32Ty(), REG_NAME("status.ptr"));

  ClearScopeCleanupStack();

  builder_->SetInsertPoint(body_block_);

  CreateStoreUndefinedToValue(retv_);
  builder_->CreateStore(builder_->getInt32(STATUS_UNSET), status_);
}

void Compiler::EndFunction(bool optimize) {
  builder_->CreateBr(return_block_);
  return_block_->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(locals_block_);
  builder_->CreateBr(args_block_);
  args_block_->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(args_block_);
  builder_->CreateBr(body_block_);
  body_block_->moveAfter(builder_->GetInsertBlock());

  builder_->SetInsertPoint(return_block_);

  if (IsScopeCleanupCheckerEnabled()) {
    CreateAssertScopeCleanupStackIsEmpty();
  }

  auto* status = builder_->CreateLoad(builder_->getInt32Ty(), status_, REG_NAME("status"));
  // Convert STATUS_XXX into Status.
  auto* masked =
      builder_->CreateAnd(status, builder_->getInt32(STATUS_MASK), REG_NAME("status.masked"));
  builder_->CreateRet(masked);

  // DumpStack();

  locals_.clear();

  assert(stack_.empty());
  stack_.clear();

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

void Compiler::HandleReturnedThrown(bool returned, bool thrown, llvm::BasicBlock* block, llvm::BasicBlock* cleanup_block, llvm::BasicBlock* exception_block) {
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

void Compiler::AllocateLocals(uint16_t num_locals) {
  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(locals_block_);

  for (auto i = 0; i < num_locals; ++i) {
    auto* local = builder_->CreateAlloca(
        types_->CreateVariableType(), nullptr, REG_NAME("local" + llvm::Twine(i) + ".ptr"));
    locals_.push_back(local);
  }

  builder_->SetInsertPoint(backup);
}

void Compiler::InitLocal(Locator locator, llvm::BasicBlock* block) {
  assert(locator.kind == LocatorKind::Local);
  assert(block != nullptr);

  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(block);

  auto* variable_ptr = GetLocalVariablePtr(locator.index);
  CreateStoreFlagsToVariable(0, variable_ptr);

  builder_->SetInsertPoint(backup);
}

void Compiler::TidyLocal(Locator locator) {
  assert(locator.kind == LocatorKind::Local);

  UNUSED(locator);
  // TODO: GC
}

void Compiler::CreateCapture(Locator locator, llvm::BasicBlock* block) {
  assert(locator.kind != LocatorKind::Capture);
  assert(block != nullptr);

  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(block);

  llvm::Value* variable_ptr;
  switch (locator.kind) {
    case LocatorKind::Argument:
      variable_ptr = CreateGetArgumentVariablePtr(locator.index);
      break;
    case LocatorKind::Local:
      variable_ptr = GetLocalVariablePtr(locator.index);
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

void Compiler::CaptureVariable(llvm::BasicBlock* block) {
  // block may be nullptr.

  llvm::BasicBlock* backup = nullptr;
  if (block != nullptr) {
    backup = builder_->GetInsertBlock();
    builder_->SetInsertPoint(block);
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

  if (backup != nullptr) {
    builder_->SetInsertPoint(backup);
  }
}

void Compiler::EscapeVariable(Locator locator, llvm::BasicBlock* block) {
  assert(locator.kind != LocatorKind::Capture);
  assert(block != nullptr);

  llvm::BasicBlock* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(block);

  auto key = ComputeKeyFromLocator(locator);
  assert(captures_.find(key) != captures_.end());

  auto* capture_ptr = captures_[key];

  auto* escaped_ptr = CreateGetEscapedPtrOfCapture(capture_ptr);
  CreateStoreTargetToCapture(escaped_ptr, capture_ptr);
  auto* variable_ptr = CreateGetVariablePtr(locator);
  auto align = llvm::Align(sizeof(double));
  builder_->CreateMemCpy(
      escaped_ptr, align, variable_ptr, align, types_->GetWord(sizeof(Variable)));

  // The value of `locator.index` may be reused for another local variable.
  // The element identified by `locator.index` is removed from `captures_` here.
  captures_.erase(key);

  builder_->SetInsertPoint(backup);
}

void Compiler::Return(size_t n) {
  if (n > 0) {
    assert(n == 1);
    auto item = Dereference();
    CreateStoreItemToValue(item, retv_);
  }

  builder_->CreateStore(builder_->getInt32(STATUS_NORMAL), status_);
}

void Compiler::Throw() {
  auto item = Dereference();
  CreateStoreItemToValue(item, retv_);

  builder_->CreateStore(builder_->getInt32(STATUS_EXCEPTION), status_);
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

void Compiler::PrepareScopeCleanupChecker(uint32_t stack_size) {
  scope_cleanup_stack_type_ = llvm::ArrayType::get(builder_->getInt16Ty(), stack_size);
  scope_cleanup_stack_ =
      CreateAllocN(builder_->getInt16Ty(), stack_size, REG_NAME("scope_cleanup_stack"));
  scope_cleanup_stack_top_ =
      CreateAlloc1(builder_->getInt32Ty(), REG_NAME("scope_cleanup_stack_top"));
  builder_->CreateStore(builder_->getInt32(0), scope_cleanup_stack_top_);
  scope_cleanup_stack_size_ = stack_size;
}

void Compiler::DumpStack() {
  llvm::errs() << "### llvm-ir:compiler-stack\n";
  for (auto it = stack_.rbegin(); it != stack_.rend(); ++it) {
    const auto& item = *it;
    switch (item.type) {
      case Item::Undefined:
        llvm::errs() << "undefined";
        break;
      case Item::Null:
        llvm::errs() << "null";
        break;
      case Item::Boolean:
        llvm::errs() << "boolean: " << V2S(item.value);
        break;
      case Item::Number:
        llvm::errs() << "number: " << V2S(item.value);
        break;
      case Item::Function:
        llvm::errs() << "function: " << V2S(item.func);
        break;
      case Item::Closure:
        llvm::errs() << "closure: " << V2S(item.value);
        break;
      case Item::Any:
        llvm::errs() << "any: " << V2S(item.value);
        break;
      case Item::Reference:
        llvm::errs() << "reference: symbol=" << item.reference.symbol;
        switch (item.reference.locator.kind) {
          case LocatorKind::None:
            llvm::errs() << " locator=none";
            return;
          case LocatorKind::Argument:
            llvm::errs() << " locator=argument@";
            break;
          case LocatorKind::Local:
            llvm::errs() << " locator=local@";
            break;
          case LocatorKind::Capture:
            llvm::errs() << " locator=capture@";
            break;
        }
        llvm::errs() << item.reference.locator.index;
        break;
      case Item::Argv:
        llvm::errs() << "argv: " << V2S(item.value);
        break;
      case Item::Capture:
        llvm::errs() << "capture: " << V2S(item.value);
        break;
    }
#if defined(BEE_BUILD_DEBUG)
    if (item.label != nullptr) {
      llvm::errs() << " [" << item.label << "]";
    }
#endif
    llvm::errs() << '\n';
  }
  llvm::errs() << '\n';
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
  auto* new_value = op == '+' ? builder_->CreateFAdd(old_value, one, REG_NAME("incr"))
                              : builder_->CreateFSub(old_value, one, REG_NAME("decr"));
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
  auto* onum = builder_->CreateSIToFP(oint, builder_->getDoubleTy(), REG_NAME("si2fp"));
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
      CreateStoreValueToVariable(item.value, value_ptr);
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
      return builder_->CreateUIToFP(item.value, builder_->getDoubleTy(), REG_NAME("ui2fp"));
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

llvm::Value* Compiler::ToAny(const Item& item) {
  if (item.type == Item::Any) {
    return item.value;
  }
  auto* value_ptr = CreateAlloc1(types_->CreateValueType(), REG_NAME("any.ptr"));
  CreateStoreItemToValue(item, value_ptr);
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
  return builder_->CreateICmpUGT(
      kind, builder_->getInt8(kValueKindNull), REG_NAME("is_non_nullish"));
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
          item.value, llvm::ConstantFP::getZero(builder_->getDoubleTy()), REG_NAME("ne"));
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
  return builder_->CreateCall(func, {exec_context_, value_ptr}, REG_NAME("boolean"));
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
  return builder_->CreateCall(func, {exec_context_, x, y}, REG_NAME("is_loosely_equal.retval"));
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
      return builder_->CreateICmpEQ(lhs.value, rhs.value, REG_NAME("is_same_boolean"));
    case Item::Number:
      return builder_->CreateFCmpOEQ(lhs.value, rhs.value, REG_NAME("is_same_number"));
    case Item::Function:
      return builder_->CreateICmpEQ(lhs.value, rhs.value, REG_NAME("is_same_lambda"));
    case Item::Closure:
      return builder_->CreateICmpEQ(lhs.value, rhs.value, REG_NAME("is_same_closure"));
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

llvm::Value* Compiler::CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = CreateBasicBlock(BB_NAME("is_boolean.then"));
  auto* else_block = CreateBasicBlock(BB_NAME("is_boolean.else"));
  auto* merge_block = CreateBasicBlock(BB_NAME("is_same_boolean"));

  // if (value.kind == kValueKindBoolean)
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond =
      builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindBoolean), REG_NAME("is_boolean"));
  builder_->CreateCondBr(cond, then_block, else_block);
  // {
  builder_->SetInsertPoint(then_block);
  auto* boolean = CreateLoadBooleanFromValue(value_ptr);
  auto* then_value = builder_->CreateICmpEQ(boolean, value, REG_NAME("is_same_boolean_value"));
  builder_->CreateBr(merge_block);
  // } else {
  builder_->SetInsertPoint(else_block);
  auto* else_value = builder_->getFalse();
  builder_->CreateBr(merge_block);
  // }
  builder_->SetInsertPoint(merge_block);
  auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2, REG_NAME("is_same_boolean"));
  phi->addIncoming(then_value, then_block);
  phi->addIncoming(else_value, else_block);

  return phi;
}

llvm::Value* Compiler::CreateIsSameNumberValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = CreateBasicBlock(BB_NAME("is_number.then"));
  auto* else_block = CreateBasicBlock(BB_NAME("is_number.else"));
  auto* merge_block = CreateBasicBlock(BB_NAME("is_same_number"));

  // if (value.kind == kValueKindNumber)
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond =
      builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindNumber), REG_NAME("is_number"));
  builder_->CreateCondBr(cond, then_block, else_block);
  // {
  builder_->SetInsertPoint(then_block);
  auto* number = CreateLoadNumberFromValue(value_ptr);
  auto* then_value = builder_->CreateFCmpOEQ(number, value, REG_NAME("is_same_number_value"));
  builder_->CreateBr(merge_block);
  // } else {
  builder_->SetInsertPoint(else_block);
  auto* else_value = builder_->getFalse();
  builder_->CreateBr(merge_block);
  // }
  builder_->SetInsertPoint(merge_block);
  auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2, REG_NAME("is_same_number"));
  phi->addIncoming(then_value, then_block);
  phi->addIncoming(else_value, else_block);

  return phi;
}

llvm::Value* Compiler::CreateIsSameClosureValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = CreateBasicBlock(BB_NAME("is_closure.then"));
  auto* else_block = CreateBasicBlock(BB_NAME("is_closure.else"));
  auto* merge_block = CreateBasicBlock(BB_NAME("is_same_closure"));

  // if (value.kind == kValueKindClosure)
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond =
      builder_->CreateICmpEQ(kind, builder_->getInt8(kValueKindClosure), REG_NAME("is_closure"));
  builder_->CreateCondBr(cond, then_block, else_block);
  // {
  builder_->SetInsertPoint(then_block);
  auto* func_ptr = CreateLoadFunctionFromValue(value_ptr);
  auto* then_value = builder_->CreateICmpEQ(func_ptr, value, REG_NAME("is_same_closure_value"));
  builder_->CreateBr(merge_block);
  // } else {
  builder_->SetInsertPoint(else_block);
  auto* else_value = builder_->getFalse();
  builder_->CreateBr(merge_block);
  // }
  builder_->SetInsertPoint(merge_block);
  auto* phi = builder_->CreatePHI(builder_->getInt1Ty(), 2, REG_NAME("is_same_closure"));
  phi->addIncoming(then_value, then_block);
  phi->addIncoming(else_value, else_block);

  return phi;
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

llvm::Value* Compiler::CreateGetVariablePtr(Locator locator) {
  switch (locator.kind) {
    case LocatorKind::Argument:
      return CreateGetArgumentVariablePtr(locator.index);
    case LocatorKind::Local:
      return GetLocalVariablePtr(locator.index);
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
      return GetLocalValuePtr(locator.index);
    case LocatorKind::Capture:
      return CreateGetCaptureValuePtr(locator.index);
    default:
      // never reach here
      assert(false);
      return nullptr;
  }
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

void Compiler::CreateBasicBlockForDeadcode() {
  auto* block = CreateBasicBlock(BB_NAME("deadcode"));
  builder_->SetInsertPoint(block);
}

std::string Compiler::MakeBasicBlockName(const char* name) const {
  assert(enable_labels_);
  std::stringstream ss;
  ss << "bb";
  if (!basic_block_name_stack_.empty()) {
    ss << '.' << basic_block_name_stack_[0];
    for (size_t i = 1; i < basic_block_name_stack_.size(); ++i) {
      ss << '.' << basic_block_name_stack_[i];
    }
  }
  ss << '.' << name;
  return ss.str();
}
