#pragma once

#include <cstdint>
#include <memory>

#include "compiler.hh"
#include "evaluator.hh"

struct Host;

class Runtime {
 public:
  static void Initialize();

  Runtime();
  ~Runtime() = default;

  void RegisterHost(const Host* host);
  void SetSourceFileName(const char* input);

  Compiler* StartCompilation();
  void PopulateModule(Compiler* compiler);
  void EndCompilation(Compiler* compiler);

  void DumpModule();
  void Eval(uintptr_t context);
  void Call(uintptr_t context, const char* name, size_t name_len, double* return_value);

 private:
  std::unique_ptr<Evaluator> evaluator_ = nullptr;
  std::unique_ptr<Compiler> compiler_ = nullptr;
};
