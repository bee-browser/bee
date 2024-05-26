#include "compiler.hh"

#include <cstdint>
#include <cstdlib>
#include <limits>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Analysis/CGSCCPassManager.h>
#include <llvm/Analysis/LoopAnalysisManager.h>
#include <llvm/IR/DerivedTypes.h>
#include <llvm/IR/PassInstrumentation.h>
#include <llvm/IR/PassManager.h>
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
  auto* prototype = types_->CreateFunctionType();
  auto* func = llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, name, *module_);
  functions_[name] = func;
  PushFunction(func);
}

void Compiler::Reference(uint32_t symbol, Locator locator) {
  PushReference(symbol, locator);
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
  auto* func = else_tail_block->getParent();

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

  auto* block = llvm::BasicBlock::Create(*context_, "bl", func);

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
  auto item = PopItem();
  auto ref = PopReference();

  auto* binding_ptr = CreateGetBindingPtr(ref.locator);
  // TODO: check the mutable flag
  // auto* flags_ptr = CreateGetFlagsPtr(binding_ptr);

  CreateStoreItemToBinding(item, binding_ptr);

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

void Compiler::Bindings(uint16_t n) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  bindings_type_ = llvm::ArrayType::get(types_->CreateBindingType(), n);
  function_scope_type_ = llvm::StructType::create(*context_, "FunctionScope");
  function_scope_type_->setBody(
      {builder_->getPtrTy(), types_->GetWordType(), builder_->getPtrTy(), bindings_type_});
  function_scope_ = builder_->CreateAlloca(function_scope_type_);
  CreateStoreOuterScopeToScope(outer_scope_, function_scope_);
  CreateStoreArgcToScope(argc_, function_scope_);
  CreateStoreArgvToScope(argv_, function_scope_);
  bindings_ = CreateGetBindingsPtrOfScope(function_scope_);
  builder_->CreateMemSet(bindings_, builder_->getInt8(0), builder_->getInt32(n * sizeof(Binding)),
      llvm::MaybeAlign());
  builder_->SetInsertPoint(backup);
}

void Compiler::DeclareImmutable() {
  static constexpr uint8_t FLAGS = BINDING_INITIALIZED;

  auto item = PopItem();
  auto ref = PopReference();

  assert(ref.locator.offset == 0);

  auto* binding_ptr = CreateGetBindingPtr(ref.locator);
  CreateStoreFlagsToBinding(FLAGS, binding_ptr);
  CreateStoreSymbolToBinding(ref.symbol, binding_ptr);
  CreateStoreItemToBinding(item, binding_ptr);
}

void Compiler::DeclareMutable() {
  static constexpr uint8_t FLAGS = BINDING_INITIALIZED | BINDING_MUTABLE;

  auto item = Dereference();
  auto ref = PopReference();

  assert(ref.locator.offset == 0);

  auto* binding_ptr = CreateGetBindingPtr(ref.locator);
  CreateStoreFlagsToBinding(FLAGS, binding_ptr);
  CreateStoreSymbolToBinding(ref.symbol, binding_ptr);
  CreateStoreItemToBinding(item, binding_ptr);
}

void Compiler::DeclareFunction() {
  static constexpr uint8_t FLAGS = BINDING_INITIALIZED | BINDING_MUTABLE;

  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);

  auto item = Dereference();
  auto ref = PopReference();

  assert(ref.locator.offset == 0);

  auto* binding_ptr = CreateGetBindingPtr(ref.locator);
  CreateStoreFlagsToBinding(FLAGS, binding_ptr);
  CreateStoreSymbolToBinding(ref.symbol, binding_ptr);
  CreateStoreItemToBinding(item, binding_ptr);

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
  llvm::Value* scope = function_scope_;
  auto item = Dereference(nullptr, &scope);
  llvm::Value* func;
  if (item.type == Item::Function) {
    func = item.value;  // IIFE
  } else {
    assert(item.type == Item::Any);
    auto* kind = CreateLoadValueKindFromValue(item.value);
    UNUSED(kind);  // TODO: check kind
    func = CreateLoadFunctionFromValue(item.value);
  }
  auto* prototype = types_->CreateFunctionType();
  auto* ret =
      builder_->CreateCall(prototype, func, {exec_context_, scope, types_->GetWord(argc), argv});
  auto* value_ptr = CreateAllocaInEntryBlock(types_->CreateValueType());
  auto* kind = CreateExtractValueKindFromValue(ret);
  CreateStoreValueKindToValue(kind, value_ptr);
  auto* holder = CreateExtractValueHolderFromValue(ret);
  CreateStoreValueHolderToValue(holder, value_ptr);
  PushAny(value_ptr);
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
  PushBlock(current_block);

  // Push a newly created block.
  // This will be used in ConditionalExpression() in order to build a branch instruction.
  auto* func = current_block->getParent();
  auto* block = llvm::BasicBlock::Create(*context_, "bl", func);
  PushBlock(block);

  builder_->SetInsertPoint(block);
}

void Compiler::IfElseStatement() {
  auto* else_tail_block = builder_->GetInsertBlock();
  auto* func = else_tail_block->getParent();

  llvm::BasicBlock* block = nullptr;

  if (else_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    block = llvm::BasicBlock::Create(*context_, "bl", func);
    builder_->CreateBr(block);
  }

  auto* else_head_block = PopBlock();
  auto* then_tail_block = PopBlock();

  if (then_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    if (block == nullptr) {
      block = llvm::BasicBlock::Create(*context_, "bl", func);
    }
    builder_->SetInsertPoint(then_tail_block);
    builder_->CreateBr(block);
  }

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();
  auto* cond_value = PopValue();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  if (block != nullptr) {
    builder_->SetInsertPoint(block);
  }
}

void Compiler::IfStatement() {
  auto* then_tail_block = builder_->GetInsertBlock();
  auto* func = then_tail_block->getParent();

  auto* block = llvm::BasicBlock::Create(*context_, "bl", func);

  if (then_tail_block->getTerminator() != nullptr) {
    // We should not append any instructions after a terminator instruction such as `ret`.
  } else {
    builder_->CreateBr(block);
  }

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();
  auto* cond_value = PopValue();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_head_block, block);

  builder_->SetInsertPoint(block);
}

void Compiler::StartFunction(const char* name) {
  const auto& found = functions_.find(name);
  if (found != functions_.end()) {
    function_ = found->second;
  } else {
    // Create a function.
    auto* prototype = types_->CreateFunctionType();
    function_ = llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, name, *module_);
    functions_[name] = function_;
  }
  prologue_ = llvm::BasicBlock::Create(*context_, "prologue", function_);
  body_ = llvm::BasicBlock::Create(*context_, "body", function_);

  exec_context_ = function_->getArg(0);
  outer_scope_ = function_->getArg(1);
  argc_ = function_->getArg(2);
  argv_ = function_->getArg(3);

  // Switch the insertion point.
  builder_->SetInsertPoint(body_);
}

void Compiler::EndFunction(bool optimize) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  builder_->CreateBr(body_);
  builder_->SetInsertPoint(backup);

  // DumpStack();
  // assert(stack_.empty());

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

void Compiler::AllocateBindings(uint16_t n, bool prologue) {
  UNUSED(prologue);
  assert(static_cast<size_t>(allocated_bindings_) + static_cast<size_t>(n) <
      std::numeric_limits<uint16_t>::max());
  allocated_bindings_ += n;
}

void Compiler::ReleaseBindings(uint16_t n) {
  assert(allocated_bindings_ >= n);
  if (builder_->GetInsertBlock()->getTerminator() == nullptr) {
    auto start = allocated_bindings_ - n;
    while (start < allocated_bindings_) {
      // TODO: CG
      auto* binding_ptr = CreateGetBindingPtrOfScope(function_scope_, start);
      CreateStoreFlagsToBinding(0, binding_ptr);
      start++;
    }
  }
  allocated_bindings_ -= n;
}

void Compiler::Return(size_t n) {
  Item item(Item::Undefined);
  if (n > 0) {
    item = Dereference();
  }
  auto* value = ToAny(item);
  auto* ret = builder_->CreateLoad(types_->CreateValueType(), value);
  auto backup = allocated_bindings_;
  ReleaseBindings(backup);  // release all bindings
  allocated_bindings_ = backup;
  builder_->CreateRet(ret);
}

void Compiler::Discard() {
  if (stack_.size() > 1) {
    PopItem();
  }
}

void Compiler::DumpStack() {
  llvm::errs() << "<llvm-ir:compiler-stack>\n";
  for (auto it = stack_.rbegin(); it != stack_.rend(); ++it) {
    const auto& item = *it;
    switch (item.type) {
      case Item::Undefined:
        llvm::errs() << "value: undefined\n";
        break;
      case Item::Null:
        llvm::errs() << "value: null\n";
        break;
      case Item::Boolean:
        llvm::errs() << "boolean: " << item.value << "\n";
        break;
      case Item::Number:
        llvm::errs() << "number: " << item.value << "\n";
        break;
      case Item::Function:
        llvm::errs() << "function: " << item.func << "\n";
        break;
      case Item::Any:
        llvm::errs() << "any: " << item.value << "\n";
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
        }
        // static_cast<uint16_t>() is needed for printing uint8_t values.
        llvm::errs() << static_cast<uint16_t>(item.reference.locator.offset) << ", "
                     << item.reference.locator.index << ")\n";
        break;
      case Item::Argv:
        llvm::errs() << "argv: " << item.value << "\n";
        break;
      case Item::Block:
        llvm::errs() << "block: " << item.block << "\n";
        break;
    }
  }
  llvm::errs() << "</llvm-ir:compiler-stack>\n";
}

Compiler::Item Compiler::Dereference(struct Reference* ref, llvm::Value** scope) {
  const auto item = PopItem();
  switch (item.type) {
    case Item::Undefined:
    case Item::Null:
    case Item::Boolean:
    case Item::Number:
    case Item::Function:
    case Item::Any:
      return item;
    case Item::Reference:
      switch (item.reference.locator.kind) {
        case LocatorKind::None:
          assert(false);
          return Item(Item::Undefined);
        case LocatorKind::Argument: {
          auto* scope_ptr = function_scope_;
          auto* argv = argv_;
          if (item.reference.locator.offset > 0) {
            scope_ptr = CreateGetScope(item.reference.locator);
            argv = CreateLoadArgvFromScope(scope_ptr);
          }
          auto* arg = builder_->CreateConstInBoundsGEP1_32(
              types_->CreateValueType(), argv, item.reference.locator.index);
          return Item(Item::Any, arg);
        }
        case LocatorKind::Local: {
          auto* scope_ptr = function_scope_;
          auto* bindings_ptr = bindings_;
          if (item.reference.locator.offset > 0) {
            scope_ptr = CreateGetScope(item.reference.locator);
            bindings_ptr = CreateGetBindingsPtrOfScope(scope_ptr);
          }
          auto* binding_ptr = builder_->CreateConstInBoundsGEP2_32(
              bindings_type_, bindings_ptr, 0, item.reference.locator.index);
          auto* value = CreateAllocaInEntryBlock(types_->CreateValueType());
          builder_->CreateMemCpy(value, llvm::MaybeAlign(), binding_ptr, llvm::MaybeAlign(),
              builder_->getInt32(sizeof(Value)));
          if (ref != nullptr) {
            *ref = item.reference;
          }
          if (scope != nullptr) {
            *scope = scope_ptr;
          }
          return Item(Item::Any, value);
        }
      }
      // fall through
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

llvm::Value* Compiler::CreateGetScope(const Locator& locator) {
  auto* scope_ptr = function_scope_;
  if (locator.offset > 0) {
    scope_ptr = outer_scope_;
    for (size_t i = 1; i < locator.offset; ++i) {
      scope_ptr = CreateLoadOuterScopeFromScope(scope_ptr);
    }
  }
  return scope_ptr;
}

void Compiler::CreateStoreItemToBinding(const Item& item, llvm::Value* binding_ptr) {
  switch (item.type) {
    case Item::Undefined:
      CreateStoreUndefinedToBinding(binding_ptr);
      break;
    case Item::Null:
      CreateStoreNullToBinding(binding_ptr);
      break;
    case Item::Boolean:
      CreateStoreBooleanToBinding(item.value, binding_ptr);
      break;
    case Item::Number:
      CreateStoreNumberToBinding(item.value, binding_ptr);
      break;
    case Item::Function:
      CreateStoreFunctionToBinding(item.value, binding_ptr);
      break;
    case Item::Any:
      CreateStoreValueToBinding(item.value, binding_ptr);
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
    case Item::Function:
      CreateStoreFunctionToValue(item.value, value_ptr);
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
  constexpr uint8_t kNullish = ValueKind::Null | ValueKind::Undefined;
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* nullish = builder_->CreateAnd(kind, builder_->getInt8(kNullish));
  return builder_->CreateICmpEQ(nullish, builder_->getInt8(0));
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
    case Item::Function:
      return CreateIsSameFunctionValue(value_ptr, item.value);
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
  return builder_->CreateICmpEQ(kind, builder_->getInt8(ValueKind::Undefined));
}

llvm::Value* Compiler::CreateIsNull(llvm::Value* value_ptr) {
  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  return builder_->CreateICmpEQ(kind, builder_->getInt8(ValueKind::Null));
}

llvm::Value* Compiler::CreateIsSameBooleanValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = llvm::BasicBlock::Create(*context_, "bb", function_);
  auto* else_block = llvm::BasicBlock::Create(*context_, "bb", function_);
  auto* merge_block = llvm::BasicBlock::Create(*context_, "bb", function_);

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond = builder_->CreateICmpEQ(kind, builder_->getInt8(ValueKind::Boolean));
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
  auto* then_block = llvm::BasicBlock::Create(*context_, "bb", function_);
  auto* else_block = llvm::BasicBlock::Create(*context_, "bb", function_);
  auto* merge_block = llvm::BasicBlock::Create(*context_, "bb", function_);

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond = builder_->CreateICmpEQ(kind, builder_->getInt8(ValueKind::Boolean));
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

llvm::Value* Compiler::CreateIsSameFunctionValue(llvm::Value* value_ptr, llvm::Value* value) {
  auto* then_block = llvm::BasicBlock::Create(*context_, "bb", function_);
  auto* else_block = llvm::BasicBlock::Create(*context_, "bb", function_);
  auto* merge_block = llvm::BasicBlock::Create(*context_, "bb", function_);

  auto* kind = CreateLoadValueKindFromValue(value_ptr);
  auto* cond = builder_->CreateICmpEQ(kind, builder_->getInt8(ValueKind::Boolean));
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
