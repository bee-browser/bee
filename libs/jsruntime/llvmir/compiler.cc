#include "compiler.hh"

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
}

void Compiler::EndMain() {
  builder_->CreateRet(builder_->getInt32(0));
  DumpModule();
}

void Compiler::PushNumber(double value) {
  auto* v = llvm::ConstantFP::get(*context_, llvm::APFloat(value));
  stack_.push_back(v);
}

void Compiler::PushString(const char* data, size_t size) {
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

void Compiler::Print() {
  assert(stack_.size() > 0);
  llvm::Value* value = stack_.back();
  stack_.pop_back();
  // TODO: function overloading
  auto* print = CreatePrintF64Function();
  builder_->CreateCall(print, {value});
}

llvm::orc::ThreadSafeModule Compiler::TakeModule() {
  return llvm::orc::ThreadSafeModule(std::move(module_), std::move(context_));
}

llvm::Function* Compiler::CreateMainFunction() {
  auto* prototype = llvm::FunctionType::get(builder_->getInt32Ty(), false);
  return llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, "main", *module_);
}

llvm::Function* Compiler::CreatePrintStrFunction() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getInt8PtrTy()}, false);
  return llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, "print_str", module_.get());
}

llvm::Function* Compiler::CreatePrintF64Function() {
  auto* prototype = llvm::FunctionType::get(builder_->getVoidTy(), {builder_->getDoubleTy()}, false);
  return llvm::Function::Create(prototype, llvm::Function::ExternalLinkage, "print_f64", module_.get());
}

void Compiler::CompileHelloWorld() {
  auto* print = CreatePrintStrFunction();
  auto* hello_world = builder_->CreateGlobalStringPtr("hello, world!", "HELLO_WORLD");
  builder_->CreateCall(print, {hello_world});
}

void Compiler::DumpModule() {
  llvm::errs() << "<llvm-ir:module>\n";
  module_->print(llvm::errs(), nullptr);
  llvm::errs() << "</llvm-ir:module>\n";
}
