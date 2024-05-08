#include "compiler.hh"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Analysis/CGSCCPassManager.h>
#include <llvm/Analysis/LoopAnalysisManager.h>
#include <llvm/IR/PassInstrumentation.h>
#include <llvm/IR/PassManager.h>
#include <llvm/IR/Verifier.h>
#include <llvm/Passes/PassBuilder.h>
#include <llvm/Passes/StandardInstrumentations.h>
#include <llvm/Transforms/InstCombine/InstCombine.h>
#include <llvm/Transforms/Scalar/GVN.h>
#include <llvm/Transforms/Scalar/Reassociate.h>
#include <llvm/Transforms/Scalar/SimplifyCFG.h>
#include <llvm/Transforms/Utils/Mem2Reg.h>
#pragma GCC diagnostic pop

#include "macros.hh"
#include "module.hh"
#include "runtime.hh"

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
  fpm_->addPass(llvm::GVNPass());
  fpm_->addPass(llvm::SimplifyCFGPass());

  llvm::PassBuilder pb;
  pb.registerModuleAnalyses(*mam_);
  pb.registerFunctionAnalyses(*fam_);
  pb.crossRegisterProxies(*lam_, *fam_, *cgam_, *mam_);
}

void Compiler::SetDataLayout(const char* data_layout) {
  module_->setDataLayout(data_layout);
}

void Compiler::SetTargetTriple(const char* triple) {
  module_->setTargetTriple(triple);
}

void Compiler::SetSourceFileName(const char* input) {
  module_->setSourceFileName(input);
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

void Compiler::Undefined() {
  PushUndefined();
}

void Compiler::Boolean(bool value) {
  PushBoolean(llvm::ConstantInt::getBool(*context_, value));
}

void Compiler::Number(double value) {
  PushNumber(llvm::ConstantFP::get(*context_, llvm::APFloat(value)));
}

void Compiler::Function(uint32_t func_id) {
  PushFunction(builder_->getInt32(func_id));
}

void Compiler::Reference(uint32_t symbol, uint32_t locator) {
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

void Compiler::DeclareImmutable() {
  auto item = PopItem();
  auto ref = PopReference();
  llvm::Function* call;
  switch (item.type) {
    case Item::Undefined:
      call = types_->CreateRuntimeDeclareImmutableUndefined();
      break;
    case Item::Boolean:
      call = types_->CreateRuntimeDeclareImmutableBoolean();
      break;
    case Item::Number:
      call = types_->CreateRuntimeDeclareImmutableNumber();
      break;
    case Item::Any:
      call = types_->CreateRuntimeDeclareImmutable();
      break;
    default:
      assert(false);
      call = nullptr;
      break;
  }
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  auto* locator = builder_->getInt32(ref.locator);
  if (item.type == Item::Undefined) {
    builder_->CreateCall(call, {context, symbol, locator});
  } else {
    builder_->CreateCall(call, {context, symbol, locator, item.value});
  }
}

void Compiler::DeclareMutable() {
  auto item = Dereference();
  auto ref = PopReference();
  llvm::Function* call;
  switch (item.type) {
    case Item::Undefined:
      call = types_->CreateRuntimeDeclareMutableUndefined();
      break;
    case Item::Boolean:
      call = types_->CreateRuntimeDeclareMutableBoolean();
      break;
    case Item::Number:
      call = types_->CreateRuntimeDeclareMutableNumber();
      break;
    case Item::Any:
      call = types_->CreateRuntimeDeclareMutable();
      break;
    default:
      assert(false);
      call = nullptr;
      break;
  }
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  auto* locator = builder_->getInt32(ref.locator);
  if (item.type == Item::Undefined) {
    builder_->CreateCall(call, {context, symbol, locator});
  } else {
    builder_->CreateCall(call, {context, symbol, locator, item.value});
  }
}

void Compiler::DeclareFunction() {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  auto* func = PopValue();
  auto ref = PopReference();
  auto* call = types_->CreateRuntimeDeclareFunction();
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  auto* locator = builder_->getInt32(ref.locator);
  builder_->CreateCall(call, {context, symbol, locator, func});
  builder_->SetInsertPoint(backup);
}

void Compiler::Set() {
  auto item = PopItem();
  auto ref = PopReference();
  llvm::Function* call;
  switch (item.type) {
    case Item::Undefined:
      call = types_->CreateRuntimePutBindingUndefined();
      break;
    case Item::Boolean:
      call = types_->CreateRuntimePutBindingBoolean();
      break;
    case Item::Number:
      call = types_->CreateRuntimePutBindingNumber();
      break;
    case Item::Any:
      call = types_->CreateRuntimePutBinding();
      break;
    default:
      assert(false);
      call = nullptr;
      break;
  }
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  auto* locator = builder_->getInt32(ref.locator);
  if (item.type == Item::Undefined) {
    builder_->CreateCall(call, {context, symbol, locator});
  } else {
    builder_->CreateCall(call, {context, symbol, locator, item.value});
  }
  stack_.push_back(item);
}

void Compiler::PushArgument() {
  auto item = Dereference();
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  llvm::Function* call;
  switch (item.type) {
    case Item::Undefined:
      call = types_->CreateRuntimePushArgumentUndefined();
      break;
    case Item::Boolean:
      call = types_->CreateRuntimePushArgumentBoolean();
      break;
    case Item::Number:
      call = types_->CreateRuntimePushArgumentNumber();
      break;
    case Item::Any:
      call = types_->CreateRuntimePushArgument();
      break;
    default:
      assert(false);
      call = nullptr;
      break;
  }
  if (item.type == Item::Undefined) {
    builder_->CreateCall(call, {context});
  } else {
    builder_->CreateCall(call, {context, item.value});
  }
}

void Compiler::Call() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto func = Dereference();
  assert(func.type == Item::Any);
  // TODO: check value type
  auto* value = builder_->CreateAlloca(types_->CreateValueType());
  builder_->CreateCall(types_->CreateRuntimeCall(), {context, func.value, value});
  PushAny(value);
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
      auto* call = types_->CreateToBoolean();
      value = builder_->CreateCall(call, {item.value});
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
  // Create a function.
  auto* sig = llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy()}, false);
  function_ = llvm::Function::Create(sig, llvm::Function::ExternalLinkage, name, *module_);
  prologue_ = llvm::BasicBlock::Create(*context_, "prologue", function_);
  body_ = llvm::BasicBlock::Create(*context_, "body", function_);

  // TODO: arguments

  // Switch the insertion point.
  builder_->SetInsertPoint(body_);

  auto* exec_context = function_->getArg(0);
  // TODO: use a global variable to hold the execution context.
  PushExecContext(exec_context);
}

void Compiler::EndFunction(bool optimize) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  builder_->CreateBr(body_);
  builder_->SetInsertPoint(backup);

  PopItem();  // exec_conext

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
  auto* backup = builder_->GetInsertBlock();
  if (prologue) {
    builder_->SetInsertPoint(prologue_);
  }
  CreateCallRuntimeAllocateBindings(n);
  ++scope_depth_;
  builder_->SetInsertPoint(backup);
}

void Compiler::ReleaseBindings(uint16_t n) {
  if (builder_->GetInsertBlock()->getTerminator() == nullptr) {
    CreateCallRuntimeReleaseBindings(n);
  }
  --scope_depth_;
}

void Compiler::Return(size_t n) {
  if (n > 0) {
    assert(n == 1);
    auto item = Dereference();
    llvm::Function* call;
    switch (item.type) {
      case Item::Undefined:
        call = nullptr;
        break;
      case Item::Boolean:
        call = types_->CreateRuntimeReturnBoolean();
        break;
      case Item::Number:
        call = types_->CreateRuntimeReturnNumber();
        break;
      case Item::Any:
        call = types_->CreateRuntimeReturnValue();
        break;
      default:
        assert(false);
        call = nullptr;
        break;
    }
    if (call != nullptr) {
      // TODO: use a global variable to hold the execution context.
      auto* context = exec_context();
      builder_->CreateCall(call, {context, item.value});
    }
  }
  builder_->CreateRetVoid();
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
        llvm::errs() << "function: " << item.value << "\n";
        break;
      case Item::Any:
        llvm::errs() << "any: " << item.value << "\n";
        break;
      case Item::Reference:
        llvm::errs() << "reference: " << item.reference.symbol << "(" << item.reference.locator
                     << ")"
                     << "\n";
        break;
      case Item::Block:
        llvm::errs() << "block: " << item.block << "\n";
        break;
      case Item::ExecContext:
        llvm::errs() << "exec-context: " << item.value << "\n";
    }
  }
  llvm::errs() << "</llvm-ir:compiler-stack>\n";
}

void Compiler::CreateCallRuntimeAllocateBindings(uint16_t n) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  builder_->CreateCall(types_->CreateRuntimeAllocateBindings(), {context, builder_->getInt16(n)});
}

void Compiler::CreateCallRuntimeReleaseBindings(uint16_t n) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  builder_->CreateCall(types_->CreateRuntimeReleaseBindings(), {context, builder_->getInt16(n)});
}

void Compiler::CreateCallRuntimeInspectNumber(llvm::Value* value) {
  // TODO: static dispatch
  auto* context = exec_context();
  builder_->CreateCall(types_->CreateRuntimeInspectNumber(), {context, value});
}

void Compiler::CreateCallRuntimeInspect(llvm::Value* value) {
  // TODO: static dispatch
  auto* context = exec_context();
  builder_->CreateCall(types_->CreateRuntimeInspect(), {context, value});
}

Compiler::Item Compiler::Dereference() {
  const auto item = PopItem();
  switch (item.type) {
    case Item::Undefined:
    case Item::Boolean:
    case Item::Number:
    case Item::Function:
    case Item::Any:
      return item;
    case Item::Reference: {
      // TODO: use a global variable to hold the execution context.
      auto* context = exec_context();
      auto* symbol = builder_->getInt32(item.reference.symbol);
      auto* locator = builder_->getInt32(item.reference.locator);
      auto* value = builder_->CreateAlloca(types_->CreateValueType());
      auto* ret = builder_->CreateCall(
          types_->CreateRuntimeGetBinding(), {context, symbol, locator, value});
      UNUSED(ret);
      return Item(Item::Any, value);
    }
    default:
      // never reach here
      assert(false);
      return Item(Item::Undefined);
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
      auto* call = types_->CreateToNumeric();
      return builder_->CreateCall(call, {item.value});
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

  auto* value = builder_->CreateAlloca(types_->CreateValueType());
  auto* kind_ptr = builder_->CreateStructGEP(types_->CreateValueType(), value, 0);
  llvm::Value* holder_ptr;
  switch (item.type) {
    case Item::Undefined:
      builder_->CreateStore(builder_->getInt64(ValueKind::Undefined), kind_ptr);
      holder_ptr = builder_->CreateStructGEP(types_->CreateValueType(), value, 1);
      builder_->CreateStore(builder_->getInt64(0), holder_ptr);
      return value;
    case Item::Boolean:
      builder_->CreateStore(builder_->getInt64(ValueKind::Boolean), kind_ptr);
      holder_ptr = builder_->CreateStructGEP(types_->CreateValueType(), value, 1);
      builder_->CreateStore(item.value, holder_ptr);
      return value;
    case Item::Number:
      builder_->CreateStore(builder_->getInt64(ValueKind::Number), kind_ptr);
      holder_ptr = builder_->CreateStructGEP(types_->CreateValueType(), value, 1);
      builder_->CreateStore(item.value, holder_ptr);
      return value;
    default:
      // TODO
      assert(false);
      return nullptr;
  }
}
