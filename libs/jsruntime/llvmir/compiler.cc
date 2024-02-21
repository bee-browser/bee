#include "compiler.hh"
#include "macros.hh"

#include <cassert>

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
  stack_.push_back(exec_context);
}

void Compiler::EndMain() {
  builder_->CreateRet(builder_->getInt32(0));
}

void Compiler::Number(double value) {
  auto* v = llvm::ConstantFP::get(*context_, llvm::APFloat(value));
  stack_.push_back(v);
}

void Compiler::String(const char* data, size_t size) {
  auto* v = llvm::ConstantDataArray::getString(*context_, llvm::StringRef(data, size));
  stack_.push_back(v);
}

void Compiler::Add() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFAdd(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Sub() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFSub(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Mul() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFMul(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Div() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFDiv(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Rem() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFRem(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Lt() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLT(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Gt() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGT(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Lte() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOLE(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Gte() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOGE(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Eq() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpOEQ(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Ne() {
  assert(stack_.size() > 1);
  llvm::Value* rhs = stack_.back();
  stack_.pop_back();
  llvm::Value* lhs = stack_.back();
  stack_.pop_back();
  // TODO: static dispatch
  auto* v = builder_->CreateFCmpONE(lhs, rhs);
  stack_.push_back(v);
}

void Compiler::Call(uint32_t symbol_id, size_t argc) {
  assert(stack_.size() >= argc);
  // TODO: use a global variable to hold the execution context.
  auto* exec_context = stack_[0];
  auto* symbol = builder_->getInt32(symbol_id);
  // TODO: argv
  auto* call = CreateRuntimeCall();
  auto* value = builder_->CreateCall(call, {exec_context, symbol});
  stack_.push_back(value);
}

void Compiler::StartFunction(size_t id, const char* name, size_t len) {
  // Push the current block.
  auto* current_block = builder_->GetInsertBlock();
  assert(current_block != nullptr);
  stack_.push_back(current_block);

  // Create a function.
  auto* prototype = llvm::FunctionType::get(builder_->getDoubleTy(), {}, false);
  auto* func = llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, llvm::StringRef(name, len), *module_);
  auto* block = llvm::BasicBlock::Create(*context_, "entry", func);

  // Switch the insertion point.
  builder_->SetInsertPoint(block);

  // Keep the function for recursive calls.
  funcs_[id] = func;
}

void Compiler::EndFunction() {
  assert(stack_.size() > 0);
  llvm::BasicBlock* block = static_cast<llvm::BasicBlock*>(stack_.back());
  stack_.pop_back();

  // Switch the insertion point.
  builder_->SetInsertPoint(block);
}

void Compiler::Return(size_t n) {
  assert(stack_.size() >= n);
  UNUSED(n);
  llvm::Value* value = stack_.back();
  stack_.pop_back();
  builder_->CreateRet(value);
}

void Compiler::Print() {
  assert(stack_.size() > 0);
  llvm::Value* value = stack_.back();
  stack_.pop_back();
  // TODO: function overloading
  llvm::Function* print = nullptr;
  if (value->getType()->isDoubleTy()) {
    print = CreatePrintF64Function();
  } else {
    print = CreatePrintBoolFunction();
  }
  builder_->CreateCall(print, {value});
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
  auto* prototype = llvm::FunctionType::get(
      builder_->getInt32Ty(), {builder_->getPtrTy()}, false);
  return llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "main", module_.get());
}

llvm::Function* Compiler::CreatePrintStrFunction() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getInt8PtrTy()}, false);
  return llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "print_str", module_.get());
}

llvm::Function* Compiler::CreatePrintBoolFunction() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getInt1Ty()}, false);
  return llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "print_bool", module_.get());
}

llvm::Function* Compiler::CreatePrintF64Function() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getVoidTy(), {builder_->getDoubleTy()}, false);
  return llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "print_f64", module_.get());
}

llvm::Function* Compiler::CreateRuntimeCall() {
  auto* prototype = llvm::FunctionType::get(
      builder_->getDoubleTy(), {builder_->getPtrTy(), builder_->getInt64Ty()}, false);
  return llvm::Function::Create(
      prototype, llvm::Function::ExternalLinkage, "runtime_call", module_.get());
}

void Compiler::CompileHelloWorld() {
  auto* print = CreatePrintStrFunction();
  auto* hello_world = builder_->CreateGlobalStringPtr("hello, world!", "HELLO_WORLD");
  builder_->CreateCall(print, {hello_world});
}
