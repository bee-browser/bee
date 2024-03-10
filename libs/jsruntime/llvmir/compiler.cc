#include "compiler.hh"
#include <llvm/ExecutionEngine/Orc/ThreadSafeModule.h>

#include <cassert>
#include <ostream>

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

void Compiler::StartMain() {
  function_ = CreateMainFunction();
  prologue_ = llvm::BasicBlock::Create(*context_, "prologue", function_);
  body_ = llvm::BasicBlock::Create(*context_, "body", function_);
  builder_->SetInsertPoint(body_);
  auto* exec_context = function_->getArg(0);
  // TODO: use a global variable to hold the execution context.
  PushValue(exec_context);
}

void Compiler::EndMain() {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  builder_->CreateBr(body_);
  builder_->SetInsertPoint(backup);
  builder_->CreateRetVoid();
}

Module* Compiler::TakeModule() {
  llvm::orc::ThreadSafeModule mod(std::move(module_), std::move(context_));
  return new Module(std::move(mod));
}

void Compiler::Number(double value) {
  auto* v = llvm::ConstantFP::get(*context_, llvm::APFloat(value));
  PushValue(v);
}

void Compiler::Symbol(uint32_t symbol_id) {
  auto* v = builder_->getInt32(symbol_id);
  PushSymbol(v);
}

void Compiler::Add() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs);
  PushValue(v);
}

void Compiler::Sub() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs);
  PushValue(v);
}

void Compiler::Mul() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFMul(lhs, rhs);
  PushValue(v);
}

void Compiler::Div() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs);
  PushValue(v);
}

void Compiler::Rem() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs);
  PushValue(v);
}

void Compiler::Lt() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs);
  PushValue(v);
}

void Compiler::Gt() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs);
  PushValue(v);
}

void Compiler::Lte() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs);
  PushValue(v);
}

void Compiler::Gte() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs);
  PushValue(v);
}

void Compiler::Eq() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOEQ(lhs, rhs);
  PushValue(v);
}

void Compiler::Ne() {
  Swap();
  auto* lhs = Dereference();
  auto* rhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpONE(lhs, rhs);
  PushValue(v);
}

void Compiler::DeclareConst() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* value = PopValue();
  auto* symbol = PopSymbol();
  auto* declare = CreateRuntimeDeclareConst();
  builder_->CreateCall(declare, {context, symbol, value});
}

void Compiler::DeclareVariable() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* value = PopValue();
  auto* symbol = PopSymbol();
  auto* declare = CreateRuntimeDeclareVariable();
  builder_->CreateCall(declare, {context, symbol, value});
}

void Compiler::DeclareUndefined() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = PopSymbol();
  auto* declare = CreateRuntimeDeclareUndefined();
  builder_->CreateCall(declare, {context, symbol});
}

void Compiler::DeclareFunction(uint32_t symbol_id, uint32_t func_id) {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = builder_->getInt32(symbol_id);
  auto* func = builder_->getInt32(func_id);
  auto* declare = CreateRuntimeDeclareFunction();
  builder_->CreateCall(declare, {context, symbol, func});
  builder_->SetInsertPoint(backup);
}

void Compiler::Get() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = PopSymbol();
  auto* get = CreateRuntimeGet();
  auto* value = builder_->CreateCall(get, {context, symbol});
  PushValue(value);
}

void Compiler::Set() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* value = PopValue();
  auto* symbol = PopSymbol();
  auto* set = CreateRuntimeSet();
  builder_->CreateCall(set, {context, symbol, value});
}

void Compiler::SetUndefined() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = PopSymbol();
  auto* set = CreateRuntimeSetUndefined();
  builder_->CreateCall(set, {context, symbol});
}

void Compiler::PushArgs() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* push_args = CreateRuntimePushArgs();
  builder_->CreateCall(push_args, {context});
}

void Compiler::PushArg() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* arg = Dereference();
  auto* push_arg = CreateRuntimePushArg();
  builder_->CreateCall(push_arg, {context, arg});
}

void Compiler::Call() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = PopSymbol();
  auto* call = CreateRuntimeCall();
  auto* value = builder_->CreateCall(call, {context, symbol});
  PushValue(value);
}

void Compiler::ToBoolean() {
  auto* value = Dereference();
  if (value->getType()->isDoubleTy()) {
    value = builder_->CreateFCmpONE(value, llvm::ConstantFP::get(*context_, llvm::APFloat(0.0)));
  }
  PushValue(value);
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

  auto* else_value = Dereference();
  builder_->CreateBr(block);

  auto* else_head_block = PopBlock();
  auto* then_tail_block = PopBlock();

  builder_->SetInsertPoint(then_tail_block);
  auto* then_value = Dereference();
  builder_->CreateBr(block);

  auto* then_head_block = PopBlock();
  auto* cond_block = PopBlock();

  builder_->SetInsertPoint(cond_block);
  auto* cond_value = PopValue();

  builder_->SetInsertPoint(cond_block);
  builder_->CreateCondBr(cond_value, then_head_block, else_head_block);

  builder_->SetInsertPoint(block);
  auto* phi = builder_->CreatePHI(llvm::Type::getDoubleTy(*context_), 2);
  phi->addIncoming(then_value, then_tail_block);
  phi->addIncoming(else_value, else_tail_block);

  PushValue(phi);
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
  // Push the current block.
  auto* current_block = builder_->GetInsertBlock();
  assert(current_block != nullptr);
  PushBlock(current_block);

  PushFunctionData();

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
  PushValue(exec_context);
}

void Compiler::EndFunction() {
  auto* backup = builder_->GetInsertBlock();
  builder_->SetInsertPoint(prologue_);
  builder_->CreateBr(body_);
  builder_->SetInsertPoint(backup);

  PopValue();  // exec_conext
  PopFunctionData();

  llvm::BasicBlock* block = PopBlock();
  // Switch the insertion point.
  builder_->SetInsertPoint(block);
}

void Compiler::StartScope() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* push_scope = CreateRuntimePushScope();
  builder_->CreateCall(push_scope, {context});
  ++scope_depth_;
}

void Compiler::EndScope() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* pop_scope = CreateRuntimePopScope();
  builder_->CreateCall(pop_scope, {context});
  --scope_depth_;
}

void Compiler::Return(size_t n) {
  UNUSED(n);
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* value = Dereference();
  auto* ret = CreateRuntimeRet();
  builder_->CreateCall(ret, {context, value});
  builder_->CreateRetVoid();
}

void Compiler::Print() {
  llvm::Value* value = Dereference();
  // TODO: function overloading
  llvm::Function* print;
  if (value->getType()->isDoubleTy()) {
    print = CreatePrintF64Function();
  } else {
    print = CreatePrintBoolFunction();
  }
  builder_->CreateCall(print, {value});
}

void Compiler::DumpStack() {
  llvm::errs() << "<llvm-ir:compiler-stack>\n";
  for (auto it = stack_.rbegin(); it != stack_.rend(); ++it) {
    const auto& item = *it;
    switch (item.type) {
      case Item::Value:
        llvm::errs() << "value: " << item.data.value << "\n";
        break;
      case Item::Symbol:
        llvm::errs() << "symbol: " << item.data.symbol << "\n";
        break;
      case Item::Block:
        llvm::errs() << "block: " << item.data.block << "\n";
        break;
      case Item::Function:
        llvm::errs() << "function" << item.data.function << "\n";
        break;
      case Item::Index:
        llvm::errs() << "index: " << item.data.index << "\n";
        break;
    }
  }
  llvm::errs() << "</llvm-ir:compiler-stack>\n";
}

llvm::Function* Compiler::CreateMainFunction() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy()}, false);
  return llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, "main", module_.get());
}

llvm::Function* Compiler::CreatePrintStrFunction() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype =
        llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "print_str", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreatePrintBoolFunction() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype =
        llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getInt1Ty()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "print_bool", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreatePrintF64Function() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype =
        llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "print_f64", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeDeclareConst() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
        {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_declare_const", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeDeclareVariable() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
        {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_declare_variable", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeDeclareUndefined() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
        {builder_->getPtrTy(), builder_->getInt32Ty()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_declare_undefined", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeDeclareFunction() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getInt32Ty()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_declare_function", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeGet() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getDoubleTy(),
        {
            builder_->getPtrTy(),
            builder_->getInt32Ty(),
        },
        false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_get", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeSet() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
        {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_set", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeSetUndefined() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
        {
            builder_->getPtrTy(),
            builder_->getInt32Ty(),
        },
        false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_set_undefined", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimePushArgs() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_->getVoidTy(), {builder_->getPtrTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_push_args", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimePushArg() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_push_arg", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeCall() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_->getDoubleTy(), {builder_->getPtrTy(), builder_->getInt32Ty()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_call", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimeRet() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_->getVoidTy(), {builder_->getPtrTy(), builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_ret", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimePushScope() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype =
        llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_push_scope", module_.get());
  }
  return func;
}

llvm::Function* Compiler::CreateRuntimePopScope() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype =
        llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getPtrTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_pop_scope", module_.get());
  }
  return func;
}
