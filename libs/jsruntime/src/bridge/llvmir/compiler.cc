#include "compiler.hh"

#include <climits>
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

void Compiler::Add() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs);
  PushNumber(v);
}

void Compiler::Sub() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs);
  PushNumber(v);
}

void Compiler::Mul() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFMul(lhs, rhs);
  PushNumber(v);
}

void Compiler::Div() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs);
  PushNumber(v);
}

void Compiler::Rem() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs);
  PushNumber(v);
}

void Compiler::Lt() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Gt() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Lte() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Gte() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Eq() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOEQ(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Ne() {
  Swap();
  auto* lhs = ToNumeric(Dereference());
  auto* rhs = ToNumeric(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpONE(lhs, rhs);
  PushBoolean(v);
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

void Compiler::Set() {
  auto item = PopItem();
  auto ref = PopReference();

  auto* binding_ptr = CreateGetBindingPtr(ref.locator);
  // TODO: check the mutable flag
  // auto* flags_ptr = CreateGetFlagsPtr(binding_ptr);

  CreateStoreItemToBinding(item, binding_ptr);

  stack_.push_back(item);
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
  auto item = Dereference(&scope);
  assert(item.type == Item::Any);
  // TODO: check value type
  auto* holder_ptr = builder_->CreateStructGEP(types_->CreateValueType(), item.value, 1);
  auto* func = builder_->CreateLoad(builder_->getPtrTy(), holder_ptr);
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

void Compiler::ToBoolean() {
  const auto item = Dereference();
  llvm::Value* value;
  switch (item.type) {
    case Item::Undefined:
      value = builder_->getFalse();
      break;
    case Item::Boolean:
      value = item.value;
      break;
    case Item::Number:
      value =
          builder_->CreateFCmpUNE(item.value, llvm::ConstantFP::getZero(builder_->getDoubleTy()));
      break;
    case Item::Any: {
      auto* call = types_->CreateRuntimeToBoolean();
      value = builder_->CreateCall(call, {exec_context_, item.value});
      break;
    }
    default:
      // TODO
      assert(false);
      value = nullptr;
      break;
  }
  PushBoolean(value);
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

void Compiler::ConditionalExpression() {
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

void Compiler::Void() {
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
        llvm::errs() << "value: " << item.value << "\n";
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

Compiler::Item Compiler::Dereference(llvm::Value** scope) {
  const auto item = PopItem();
  switch (item.type) {
    case Item::Undefined:
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
          // TODO: item.reference.locator.offset
          // TODO: argc_
          auto* arg = builder_->CreateConstInBoundsGEP1_32(
              types_->CreateValueType(), argv_, item.reference.locator.index);
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

llvm::Value* Compiler::ToNumeric(const Item& item) {
  switch (item.type) {
    case Item::Undefined:
      return llvm::ConstantFP::getNaN(builder_->getDoubleTy());
    case Item::Boolean:
      return builder_->CreateUIToFP(item.value, builder_->getDoubleTy());
    case Item::Number:
      return item.value;
    case Item::Any: {
      auto* call = types_->CreateRuntimeToNumeric();
      return builder_->CreateCall(call, {exec_context_, item.value});
    }
    default:
      assert(false);
      return nullptr;
  }
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
