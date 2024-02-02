#pragma once

#include <memory>

#include "compiler.hh"
#include "evaluator.hh"

class Runtime {
 public:
  static void Initialize();

  Runtime();
  ~Runtime() = default;

  void SetSourceFileName(const char* input);
  void Eval();

  Compiler* StartCompilation();
  void PopulateModule(Compiler* compiler);
  void EndCompilation(Compiler* compiler);

 private:
  std::unique_ptr<Evaluator> evaluator_ = nullptr;
  std::unique_ptr<Compiler> compiler_ = nullptr;
};
