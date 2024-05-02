#include "compiler.hh"
#include <llvm/IR/DerivedTypes.h>

#include <cassert>
#include <cstdint>

#include "macros.hh"
#include "module.hh"

Compiler::Compiler() {
  context_ = std::make_unique<llvm::LLVMContext>();
  module_ = std::make_unique<llvm::Module>("<main>", *context_);
  // TODO: module_->setDataLayout(data_layout);
  builder_ = std::make_unique<llvm::IRBuilder<>>(*context_);
}

void Compiler::SetSourceFileName(const char* input) {
  module_->setSourceFileName(input);
}

void Compiler::DeclareTypes() {
  DeclareValueType();
  DeclareRuntimeDeclareConst();
  DeclareRuntimeDeclareVariable();
  DeclareRuntimeDeclareFunction();
  DeclareRuntimeGetArgument();
  DeclareRuntimeGetLocal();
  DeclareRuntimePutArgument();
  DeclareRuntimePutLocal();
  DeclareRuntimePushArg();
  DeclareRuntimeCall();
  DeclareRuntimeRet();
  DeclareRuntimeAllocateBindings();
  DeclareRuntimeReleaseBindings();
  DeclareRuntimeInspectNumber();
  DeclareRuntimeInspectAny();
}

Module* Compiler::TakeModule() {
  llvm::orc::ThreadSafeModule mod(std::move(module_), std::move(context_));
  return new Module(std::move(mod));
}

void Compiler::Number(double number) {
  auto* value = llvm::ConstantFP::get(*context_, llvm::APFloat(number));
  PushNumber(value);
}

void Compiler::Function(uint32_t func_id) {
  auto* value = builder_->getInt32(func_id);
  PushFunction(value);
}

void Compiler::ArgumentRef(uint32_t symbol, uint16_t index) {
  PushArgumentRef(symbol, index);
}

void Compiler::LocalRef(uint32_t symbol, uint16_t stack, uint16_t index) {
  PushLocalRef(symbol, stack, index);
}

void Compiler::Add() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs);
  PushNumber(v);
}

void Compiler::Sub() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs);
  PushNumber(v);
}

void Compiler::Mul() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFMul(lhs, rhs);
  PushNumber(v);
}

void Compiler::Div() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs);
  PushNumber(v);
}

void Compiler::Rem() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs);
  PushNumber(v);
}

void Compiler::Lt() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Gt() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Lte() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Gte() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Eq() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOEQ(lhs, rhs);
  PushBoolean(v);
}

void Compiler::Ne() {
  Swap();
  auto* lhs = ToNumber(Dereference());
  auto* rhs = ToNumber(Dereference());
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpONE(lhs, rhs);
  PushBoolean(v);
}

void Compiler::DeclareConst() {
  auto* value = PopValue();
  auto ref = PopLocalRef();
  CreateCallRuntimeDeclareConst(ref, value);
}

void Compiler::DeclareVariable() {
  auto* value = PopValue();
  auto ref = PopLocalRef();
  CreateCallRuntimeDeclareVariable(ref, value);
}

void Compiler::DeclareFunction() {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  auto* func = PopValue();
  auto ref = PopLocalRef();
  CreateCallRuntimeDeclareFunction(ref, func);
  builder_->SetInsertPoint(backup);
}

void Compiler::GetArgument() {
  auto argument_ref = PopArgumentRef();
  auto cache = argument_cache_.find(argument_ref.index);
  if (cache != argument_cache_.end()) {
    PushAny(cache->second);
  } else {
    // TODO: use a global variable to hold the execution context.
    auto* context = exec_context();
    auto* symbol = builder_->getInt32(argument_ref.symbol);
    auto* index = builder_->getInt16(argument_ref.index);
    auto* value = builder_->CreateAlloca(value_type_);
    auto* ret = builder_->CreateCall(runtime_get_argument_, {context, symbol, index, value});
    UNUSED(ret);
    PushAny(value);
    argument_cache_[argument_ref.index] = value;
  }
}

void Compiler::GetLocal() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto local_ref = PopLocalRef();
  auto* symbol = builder_->getInt32(local_ref.symbol);
  auto* stack = builder_->getInt16(local_ref.stack);
  auto* index = builder_->getInt16(local_ref.index);
  auto* value = builder_->CreateAlloca(value_type_);
  auto* ret = builder_->CreateCall(runtime_get_local_, {context, symbol, stack, index, value});
  UNUSED(ret);
  PushAny(value);
  // TODO: caching the value may improve the performance
}

void Compiler::Set() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* value = PopValue();
  auto item = PopItem();
  switch (item.type) {
    case Item::ArgumentRef: {
      auto* symbol = builder_->getInt32(item.argument_ref.symbol);
      auto* index = builder_->getInt16(item.argument_ref.index);
      builder_->CreateCall(runtime_put_argument_, {context, symbol, index, value});
      argument_cache_[item.argument_ref.index] = value;
    } break;
    case Item::LocalRef: {
      auto* symbol = builder_->getInt32(item.local_ref.symbol);
      auto* stack = builder_->getInt16(item.local_ref.stack);
      auto* index = builder_->getInt16(item.local_ref.index);
      builder_->CreateCall(runtime_put_local_, {context, symbol, stack, index, value});
    } break;
    default:
      assert(false);
      break;
  }
  PushAny(value);
}

void Compiler::PushArg() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto arg = Dereference();
  assert(arg.IsValue());
  builder_->CreateCall(runtime_push_arg_, {context, arg.value});
}

void Compiler::Call() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto func = Dereference();
  assert(func.type == Item::Any);
  // TODO: check value type
  auto* value = builder_->CreateCall(runtime_call_, {context, func.value});
  PushNumber(value);  // TODO: any value
}

void Compiler::ToBoolean() {
  auto item = Dereference();
  assert(item.IsValue());
  llvm::Value* value;
  switch (item.type) {
    case Item::Undefined:
      value = builder_->getFalse();
      break;
    case Item::Boolean:
      value = item.value;
      break;
    case Item::Number:
      value = builder_->CreateFCmpONE(
          item.value, llvm::ConstantFP::get(*context_, llvm::APFloat(0.0)));
      break;
    default:
      // TODO
      assert(false);
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

  auto* block = llvm::BasicBlock::Create(*context_, "bl", func);

  auto else_item = Dereference();
  assert(else_item.type == Item::Number);  // TODO
  auto* else_value = else_item.value;
  builder_->CreateBr(block);

  auto* else_head_block = PopBlock();
  auto* then_tail_block = PopBlock();

  builder_->SetInsertPoint(then_tail_block);
  auto then_item = Dereference();
  assert(then_item.type == Item::Number);  // TODO
  auto* then_value = then_item.value;
  builder_->CreateBr(block);

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();

  builder_->SetInsertPoint(cond_block);
  auto* cond_value = PopValue();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  builder_->SetInsertPoint(block);
  auto* phi = builder_->CreatePHI(llvm::Type::getDoubleTy(*context_), 2);  // TODO
  phi->addIncoming(then_value, then_tail_block);
  phi->addIncoming(else_value, else_tail_block);

  PushNumber(phi);  // TODO
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

void Compiler::EndFunction() {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  builder_->CreateBr(body_);
  builder_->SetInsertPoint(backup);

  PopItem();  // exec_conext

  argument_cache_.clear();
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
    // TODO: use a global variable to hold the execution context.
    auto* context = exec_context();
    auto item = Dereference();
    assert(item.IsValue());
    auto* value = ToNumber(item);  // TODO
    builder_->CreateCall(runtime_ret_, {context, value});
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
      case Item::ArgumentRef:
        llvm::errs() << "argument-ref: " << item.argument_ref.symbol << "("
                     << item.argument_ref.index << ")"
                     << "\n";
        break;
      case Item::LocalRef:
        llvm::errs() << "local-ref: " << item.local_ref.symbol << "(" << item.local_ref.stack
                     << ":" << item.local_ref.index << ")"
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

void Compiler::DeclareValueType() {
  value_type_ = llvm::StructType::create(*context_, "Value");
  value_type_->setBody({builder_->getInt64Ty(), builder_->getInt64Ty()});
}

void Compiler::DeclareRuntimeDeclareConst() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getDoubleTy()}, false);
  runtime_declare_const_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_declare_const", module_.get());
}

void Compiler::CreateCallRuntimeDeclareConst(const struct LocalRef& ref, llvm::Value* value) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  assert(ref.stack == 0);
  auto* index = builder_->getInt16(ref.index);
  builder_->CreateCall(runtime_declare_const_, {context, symbol, index, value});
}

void Compiler::DeclareRuntimeDeclareVariable() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getDoubleTy()}, false);
  runtime_declare_variable_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_declare_variable", module_.get());
}

void Compiler::CreateCallRuntimeDeclareVariable(const struct LocalRef& ref, llvm::Value* value) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  assert(ref.stack == 0);
  auto* index = builder_->getInt16(ref.index);
  builder_->CreateCall(runtime_declare_variable_, {context, symbol, index, value});
}

void Compiler::DeclareRuntimeDeclareFunction() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getInt32Ty()}, false);
  runtime_declare_function_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_declare_function", module_.get());
}

void Compiler::CreateCallRuntimeDeclareFunction(const struct LocalRef& ref, llvm::Value* value) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(ref.symbol);
  assert(ref.stack == 0);
  auto* index = builder_->getInt16(ref.index);
  builder_->CreateCall(runtime_declare_function_, {context, symbol, index, value});
}

void Compiler::DeclareRuntimeGetArgument() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getInt16Ty(), builder_->getPtrTy()},
      false);
  runtime_get_argument_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_get_argument", module_.get());
}

void Compiler::DeclareRuntimeGetLocal() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getInt16Ty(),
          builder_->getInt16Ty(), builder_->getPtrTy()},
      false);
  runtime_get_local_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_get_local", module_.get());
}

void Compiler::DeclareRuntimePutArgument() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getInt16Ty(),
          builder_->getDoubleTy()},
      false);
  runtime_put_argument_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_put_argument", module_.get());
}

void Compiler::DeclareRuntimePutLocal() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
      {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getInt16Ty(),
          builder_->getInt16Ty(), builder_->getDoubleTy()},
      false);
  runtime_put_local_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_put_local", module_.get());
}

void Compiler::DeclareRuntimePushArg() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getDoubleTy()}, false);
  runtime_push_arg_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_push_arg", module_.get());
}

void Compiler::DeclareRuntimeCall() {
  auto* value_ptr = llvm::PointerType::get(value_type_, 0);
  auto* prototype =
      llvm::FunctionType::get(builder_->getDoubleTy(), {builder_->getPtrTy(), value_ptr}, false);
  runtime_call_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_call", module_.get());
}

void Compiler::DeclareRuntimeRet() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getDoubleTy()}, false);
  runtime_ret_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_ret", module_.get());
}

void Compiler::DeclareRuntimeAllocateBindings() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getInt16Ty()}, false);
  runtime_allocate_bindings_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_allocate_bindings", module_.get());
}

void Compiler::CreateCallRuntimeAllocateBindings(uint16_t n) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  builder_->CreateCall(runtime_allocate_bindings_, {context, builder_->getInt16(n)});
}

void Compiler::DeclareRuntimeReleaseBindings() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getInt16Ty()}, false);
  runtime_release_bindings_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_release_bindings", module_.get());
}

void Compiler::CreateCallRuntimeReleaseBindings(uint16_t n) {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  builder_->CreateCall(runtime_release_bindings_, {context, builder_->getInt16(n)});
}

void Compiler::DeclareRuntimeInspectNumber() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getDoubleTy()}, false);
  runtime_inspect_number_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_inspect_number", module_.get());
}

void Compiler::CreateCallRuntimeInspectNumber(llvm::Value* value) {
  // TODO: static dispatch
  auto* context = exec_context();
  builder_->CreateCall(runtime_inspect_number_, {context, value});
}

void Compiler::DeclareRuntimeInspectAny() {
  auto* prototype =
      llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy(), value_type_}, false);
  runtime_inspect_any_ = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_inspect_any", module_.get());
}

void Compiler::CreateCallRuntimeInspectAny(llvm::Value* value) {
  // TODO: static dispatch
  auto* context = exec_context();
  builder_->CreateCall(runtime_inspect_any_, {context, value});
}
