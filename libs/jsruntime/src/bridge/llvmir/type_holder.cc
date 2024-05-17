// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsruntime/src/bridge/llvmir/type_holder.cc.njk

#include "type_holder.hh"

llvm::StructType* TypeHolder::CreateValueType() {
  if (value_type_ == nullptr) {
    value_type_ = llvm::StructType::create(context_, "Value");
    value_type_->setBody({
        // kind
        builder_.getInt8Ty(),
        // holder
        builder_.getInt64Ty(),
    });
  }
  return value_type_;
}

llvm::StructType* TypeHolder::CreateBindingType() {
  if (binding_type_ == nullptr) {
    binding_type_ = llvm::StructType::create(context_, "Binding");
    binding_type_->setBody({
        // kind
        builder_.getInt8Ty(),
        // flags
        builder_.getInt8Ty(),
        // reserved
        builder_.getInt16Ty(),
        // holder
        builder_.getInt64Ty(),
    });
  }
  return binding_type_;
}

llvm::Function* TypeHolder::CreateRuntimeToBoolean() {
  if (runtime_to_boolean_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_.getInt1Ty(), {builder_.getPtrTy(), builder_.getPtrTy()}, false);
    runtime_to_boolean_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_to_boolean", module_);
  }
  return runtime_to_boolean_;
}

llvm::Function* TypeHolder::CreateRuntimeToNumeric() {
  if (runtime_to_numeric_ == nullptr) {
    auto* prototype = llvm::FunctionType::get(
        builder_.getDoubleTy(), {builder_.getPtrTy(), builder_.getPtrTy()}, false);
    runtime_to_numeric_ = llvm::Function::Create(
        prototype, llvm::Function::ExternalLinkage, "runtime_to_numeric", module_);
  }
  return runtime_to_numeric_;
}