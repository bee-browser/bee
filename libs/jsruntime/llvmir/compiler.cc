#include "compiler.hh"

#include <cassert>
#include <ostream>

#include "macros.hh"

Compiler::Compiler(const llvm::DataLayout& data_layout) {
  context_ = std::make_unique<llvm::LLVMContext>();
  module_ = std::make_unique<llvm::Module>("<main>", *context_);
  module_->setDataLayout(data_layout);
  builder_ = std::make_unique<llvm::IRBuilder<>>(*context_);
}

void Compiler::SetSourceFileName(const char* input) {
  module_->setSourceFileName(input);
}

void Compiler::StartMain() {
  auto* main_func = CreateMainFunction();
  auto* entry = llvm::BasicBlock::Create(*context_, "entry", main_func);
  builder_->SetInsertPoint(entry);
  auto* exec_context = main_func->getArg(0);
  // TODO: use a global variable to hold the execution context.
  PushValue(exec_context);
}

void Compiler::EndMain() {
  builder_->CreateRet(builder_->getInt32(0));
}

void Compiler::Number(double value) {
  auto* v = llvm::ConstantFP::get(*context_, llvm::APFloat(value));
  PushValue(v);
}

void Compiler::String(const char* data, size_t size) {
  auto* v = llvm::ConstantDataArray::getString(*context_, llvm::StringRef(data, size));
  PushValue(v);
}

void Compiler::Symbol(uint32_t symbol_id) {
  auto* v = builder_->getInt32(symbol_id);
  PushSymbol(v);
}

void Compiler::Add() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs);
  PushValue(v);
}

void Compiler::Sub() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs);
  PushValue(v);
}

void Compiler::Mul() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFMul(lhs, rhs);
  PushValue(v);
}

void Compiler::Div() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs);
  PushValue(v);
}

void Compiler::Rem() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs);
  PushValue(v);
}

void Compiler::Lt() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs);
  PushValue(v);
}

void Compiler::Gt() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs);
  PushValue(v);
}

void Compiler::Lte() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs);
  PushValue(v);
}

void Compiler::Gte() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs);
  PushValue(v);
}

void Compiler::Eq() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOEQ(lhs, rhs);
  PushValue(v);
}

void Compiler::Ne() {
  auto* rhs = Dereference();
  auto* lhs = Dereference();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpONE(lhs, rhs);
  PushValue(v);
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

void Compiler::Declare() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* value = PopValue();
  auto* symbol = PopSymbol();
  auto* declare = CreateRuntimeDeclare();
  builder_->CreateCall(declare, {context, symbol, value});
}

void Compiler::SetUndefined() {
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  auto* symbol = PopSymbol();
  auto* set = CreateRuntimeSetUndefined();
  builder_->CreateCall(set, {context, symbol});
}

void Compiler::Call(size_t argc) {
  UNUSED(argc);
  // TODO: use a global variable to hold the execution context.
  auto* context = exec_context();
  // TODO: argv
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

void Compiler::StartFunction(const char* name, size_t len) {
  // Push the current block.
  auto* current_block = builder_->GetInsertBlock();
  assert(current_block != nullptr);
  PushBlock(current_block);

  PushFunctionData();

  // Create a function.
  auto* prototype =
      llvm::FunctionType::get(builder_->getDoubleTy(), {builder_->getPtrTy()}, false);
  auto* func = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, llvm::StringRef(name, len), *module_);
  auto* block = llvm::BasicBlock::Create(*context_, "entry", func);

  // TODO: arguments

  // Switch the insertion point.
  builder_->SetInsertPoint(block);

  auto* exec_context = func->getArg(0);
  // TODO: use a global variable to hold the execution context.
  PushValue(exec_context);

  function_ = func;
}

void Compiler::EndFunction() {
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
  llvm::Value* value = Dereference();
  builder_->CreateRet(value);
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

void Compiler::DumpModule() {
  llvm::errs() << "<llvm-ir:module>\n";
  module_->print(llvm::errs(), nullptr);
  llvm::errs() << "</llvm-ir:module>\n";
}

llvm::orc::ThreadSafeModule Compiler::TakeModule() {
  return llvm::orc::ThreadSafeModule(std::move(module_), std::move(context_));
}

llvm::Function* Compiler::CreateMainFunction() {
  auto* prototype = llvm::FunctionType::get(builder_->getInt32Ty(), {builder_->getPtrTy()}, false);
  return llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, "main", module_.get());
}

llvm::Function* Compiler::CreatePrintStrFunction() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype =
        llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getInt8PtrTy()}, false);
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

llvm::Function* Compiler::CreateRuntimeDeclare() {
  static llvm::Function* func = nullptr;
  if (func == nullptr) {
    auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(),
        {builder_->getPtrTy(), builder_->getInt32Ty(), builder_->getDoubleTy()}, false);
    func = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_declare", module_.get());
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
