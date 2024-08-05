#pragma once

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <vector>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <llvm/Support/raw_ostream.h>
#pragma GCC diagnostic pop

namespace llvm {
class BasicBlock;
}

enum class ControlFlowKind {
  kFunction,
  kScope,
  kBranch,
  kLoopInit,
  kLoopTest,
  kLoopNext,
  kLoopBody,
  kSelect,
  kCaseEnd,
  kException,
};

// Contains data used for building the root region of a function.
struct FunctionFlow {
  llvm::BasicBlock* locals_block;
  llvm::BasicBlock* args_block;
  llvm::BasicBlock* body_block;
  llvm::BasicBlock* return_block;
};

// Contains data used for building a region representing a lexical scope.
struct ScopeFlow {
  // The entry block of the scope flow.
  llvm::BasicBlock* init_block;

  // A block containing instructions for hoisted function and variable declarations.
  llvm::BasicBlock* hoisted_block;

  // The first block of the container region of the scope flow.
  llvm::BasicBlock* block;

  // A block containing instructions for cleanup.
  // The container region will be always connected to this block.
  llvm::BasicBlock* cleanup_block;

  // The index of the enclosing outer scope flow.
  size_t outer_index;

  // `true` if the scope flow contains return statements.
  bool returned = false;

  // `true` if the scope flow has uncaught exceptions.
  bool thrown = false;
};

struct BranchFlow {
  llvm::BasicBlock* before_block;
  llvm::BasicBlock* after_block;
};

struct LoopInitFlow {
  llvm::BasicBlock* branch_block;
  llvm::BasicBlock* insert_point;
};

struct LoopTestFlow {
  llvm::BasicBlock* then_block;
  llvm::BasicBlock* else_block;
  llvm::BasicBlock* insert_point;
};

struct LoopNextFlow {
  llvm::BasicBlock* branch_block;
  llvm::BasicBlock* insert_point;
};

struct LoopBodyFlow {
  llvm::BasicBlock* branch_block;
  llvm::BasicBlock* insert_point;
};

struct SelectFlow {
  llvm::BasicBlock* end_block;
  size_t outer_index;
  llvm::BasicBlock* default_case_block = nullptr;
};

struct CaseEndFlow {
  llvm::BasicBlock* block;
};

struct ExceptionFlow {
  llvm::BasicBlock* try_block;
  llvm::BasicBlock* catch_block;
  llvm::BasicBlock* finally_block;
  llvm::BasicBlock* end_block;

  // The index of the enclosing outer exception flow.
  size_t outer_index;

  // `true` if the scope flow has uncaught exceptions.
  bool thrown = false;

  bool caught = false;

  bool ended = false;
};

// A `Flow` object contains basic blocks that will construct a region in the control flow graph
// (CFG) of a function.
struct ControlFlow {
  ControlFlowKind kind;
  union {
    FunctionFlow function;
    ScopeFlow scope;
    BranchFlow branch;
    LoopInitFlow loop_init;
    LoopTestFlow loop_test;
    LoopNextFlow loop_next;
    LoopBodyFlow loop_body;
    SelectFlow select;
    CaseEndFlow case_end;
    ExceptionFlow exception;
  };

  ControlFlow(const FunctionFlow& function)
      : kind(ControlFlowKind::kFunction), function(function) {}
  ControlFlow(const ScopeFlow& scope) : kind(ControlFlowKind::kScope), scope(scope) {}
  ControlFlow(const BranchFlow& branch) : kind(ControlFlowKind::kBranch), branch(branch) {}
  ControlFlow(const LoopInitFlow& loop_init)
      : kind(ControlFlowKind::kLoopInit), loop_init(loop_init) {}
  ControlFlow(const LoopTestFlow& loop_test)
      : kind(ControlFlowKind::kLoopTest), loop_test(loop_test) {}
  ControlFlow(const LoopNextFlow& loop_next)
      : kind(ControlFlowKind::kLoopNext), loop_next(loop_next) {}
  ControlFlow(const LoopBodyFlow& loop_body)
      : kind(ControlFlowKind::kLoopBody), loop_body(loop_body) {}
  ControlFlow(const SelectFlow& select) : kind(ControlFlowKind::kSelect), select(select) {}
  ControlFlow(const CaseEndFlow& case_end) : kind(ControlFlowKind::kCaseEnd), case_end(case_end) {}
  ControlFlow(const ExceptionFlow& exception)
      : kind(ControlFlowKind::kException), exception(exception) {}
  ControlFlow(const ControlFlow& flow) = default;
  ~ControlFlow() = default;
};

struct BranchTarget {
  llvm::BasicBlock* block;
  uint32_t symbol;

  BranchTarget(llvm::BasicBlock* block, uint32_t symbol) : block(block), symbol(symbol) {}
  ~BranchTarget() = default;
};

class ControlFlowStack {
 public:
  ControlFlowStack() = default;
  ~ControlFlowStack() = default;

  bool IsEmpty() const {
    return stack_.empty() && break_stack_.empty() && continue_stack_.empty();
  }

  void PushFunctionFlow(llvm::BasicBlock* locals_block,
      llvm::BasicBlock* args_block,
      llvm::BasicBlock* body_block,
      llvm::BasicBlock* return_block) {
    assert(locals_block != nullptr);
    assert(args_block != nullptr);
    assert(body_block != nullptr);
    assert(return_block != nullptr);
    assert(stack_.empty());
    stack_.emplace_back(FunctionFlow{locals_block, args_block, body_block, return_block});
  }

  FunctionFlow PopFunctionFlow() {
    auto flow = top();
    assert(flow.kind == ControlFlowKind::kFunction);
    stack_.pop_back();
    assert(stack_.empty());
    assert(scope_index_ == 0);
    assert(exception_index_ == 0);
    return flow.function;
  }

  void PushScopeFlow(llvm::BasicBlock* init_block,
      llvm::BasicBlock* hoisted_block,
      llvm::BasicBlock* block,
      llvm::BasicBlock* cleanup_block) {
    assert(init_block != nullptr);
    assert(hoisted_block != nullptr);
    assert(block != nullptr);
    assert(cleanup_block != nullptr);
    auto index = stack_.size();
    stack_.emplace_back(ScopeFlow{init_block, hoisted_block, block, cleanup_block, scope_index_});
    scope_index_ = index;
  }

  ScopeFlow PopScopeFlow() {
    assert(top().kind == ControlFlowKind::kScope);
    auto scope = top().scope;

    stack_.pop_back();
    // The `scope_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `scope_flow()` work properly.
    scope_index_ = scope.outer_index;

    // Propagate flags to the outer flow.
    auto& outer = top_mut();
    switch (outer.kind) {
      case ControlFlowKind::kFunction:
        // Nothing to do.
        break;
      case ControlFlowKind::kScope:
        if (scope.returned) {
          outer.scope.returned = true;
        }
        if (scope.thrown) {
          outer.scope.thrown = true;
        }
        break;
      case ControlFlowKind::kBranch:
      case ControlFlowKind::kLoopInit:  // TODO
      case ControlFlowKind::kLoopTest:  // TODO
      case ControlFlowKind::kLoopNext:  // TODO
      case ControlFlowKind::kLoopBody:  // TODO
      case ControlFlowKind::kSelect:    // TODO
      case ControlFlowKind::kCaseEnd:   // TODO
        if (scope.returned) {
          scope_flow_mut().returned = true;
        }
        if (scope.thrown) {
          scope_flow_mut().thrown = true;
        }
        break;
      case ControlFlowKind::kException:
        if (scope.thrown) {
          outer.exception.thrown = true;
        }
        break;
    }

    return scope;
  }

  void PushBranchFlow(llvm::BasicBlock* before_block, llvm::BasicBlock* after_block) {
    assert(before_block != nullptr);
    assert(after_block != nullptr);
    stack_.emplace_back(BranchFlow{before_block, after_block});
  }

  BranchFlow PopBranchFlow() {
    assert(top().kind == ControlFlowKind::kBranch);
    auto branch = top().branch;
    stack_.pop_back();
    return branch;
  }

  void PushLoopInitFlow(llvm::BasicBlock* branch_block, llvm::BasicBlock* insert_point) {
    assert(branch_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopInitFlow{branch_block, insert_point});
  }

  LoopInitFlow PopLoopInitFlow() {
    assert(top().kind == ControlFlowKind::kLoopInit);
    auto loop_init = top().loop_init;
    stack_.pop_back();
    return loop_init;
  }

  void PushLoopTestFlow(llvm::BasicBlock* then_block,
      llvm::BasicBlock* else_block,
      llvm::BasicBlock* insert_point) {
    assert(then_block != nullptr);
    assert(else_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopTestFlow{then_block, else_block, insert_point});
  }

  LoopTestFlow PopLoopTestFlow() {
    assert(top().kind == ControlFlowKind::kLoopTest);
    auto loop_test = top().loop_test;
    stack_.pop_back();
    return loop_test;
  }

  void PushLoopNextFlow(llvm::BasicBlock* branch_block, llvm::BasicBlock* insert_point) {
    assert(branch_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopNextFlow{branch_block, insert_point});
  }

  LoopNextFlow PopLoopNextFlow() {
    assert(top().kind == ControlFlowKind::kLoopNext);
    auto loop_next = top().loop_next;
    stack_.pop_back();
    return loop_next;
  }

  void PushLoopBodyFlow(llvm::BasicBlock* branch_block, llvm::BasicBlock* insert_point) {
    assert(branch_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopBodyFlow{branch_block, insert_point});
  }

  LoopBodyFlow PopLoopBodyFlow() {
    assert(top().kind == ControlFlowKind::kLoopBody);
    auto loop_body = top().loop_body;
    stack_.pop_back();
    return loop_body;
  }

  void PushSelectFlow(llvm::BasicBlock* end_block) {
    assert(end_block != nullptr);
    auto index = stack_.size();
    stack_.emplace_back(SelectFlow{end_block, select_index_});
    select_index_ = index;
  }

  SelectFlow PopSelectFlow() {
    assert(top().kind == ControlFlowKind::kSelect);
    auto select = top().select;

    stack_.pop_back();
    // The `select_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `select_flow()` work properly.
    select_index_ = select.outer_index;

    return select;
  }

  void PushCaseEndFlow(llvm::BasicBlock* block) {
    assert(block != nullptr);
    stack_.emplace_back(CaseEndFlow{block});
  }

  CaseEndFlow PopCaseEndFlow() {
    assert(top().kind == ControlFlowKind::kCaseEnd);
    auto case_end = top().case_end;
    stack_.pop_back();
    return case_end;
  }

  void PushExceptionFlow(llvm::BasicBlock* try_block,
      llvm::BasicBlock* catch_block,
      llvm::BasicBlock* finally_block,
      llvm::BasicBlock* end_block) {
    assert(try_block != nullptr);
    assert(catch_block != nullptr);
    assert(finally_block != nullptr);
    assert(end_block != nullptr);
    auto index = stack_.size();
    stack_.emplace_back(
        ExceptionFlow{try_block, catch_block, finally_block, end_block, exception_index_});
    exception_index_ = index;
  }

  ExceptionFlow PopExceptionFlow() {
    assert(top().kind == ControlFlowKind::kException);
    auto exception = top().exception;

    stack_.pop_back();
    // The `exception_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `exception_flow()` work properly.
    exception_index_ = exception.outer_index;

    // Any exception flow is enclosed by a scope flow.
    assert(top().kind == ControlFlowKind::kScope);

    // Propagate flags to the outer flow.
    if (exception.thrown) {
      top_mut().scope.thrown = true;
    }

    return exception;
  }

  void SetReturned() {
    auto& flow = top_mut();
    switch (flow.kind) {
      case ControlFlowKind::kFunction:
        // Nothing to do.
        break;
      case ControlFlowKind::kScope:
        flow.scope.returned = true;
        break;
      case ControlFlowKind::kBranch:
        scope_flow_mut().returned = true;
        break;
      default:
        // never reach here
        assert(false);
        break;
    }
  }

  void SetThrown() {
    assert(top().kind == ControlFlowKind::kScope);
    top_mut().scope.thrown = true;
  }

  void SetCaught(bool nominal) {
    assert(top().kind == ControlFlowKind::kException);
    top_mut().exception.caught = true;
    if (!nominal) {
      top_mut().exception.thrown = false;
    }
  }

  void SetEnded() {
    assert(top().kind == ControlFlowKind::kException);
    top_mut().exception.ended = true;
  }

  void SetDefaultCaseBlock(llvm::BasicBlock* block) {
    assert(block != nullptr);
    select_flow_mut().default_case_block = block;
  }

  void PushBreakTarget(llvm::BasicBlock* block, uint32_t symbol = 0) {
    break_stack_.emplace_back(block, symbol);
  }

  BranchTarget PopBreakTarget() {
    auto target = break_stack_.back();
    break_stack_.pop_back();
    return target;
  }

  void PushContinueTarget(llvm::BasicBlock* block, uint32_t symbol = 0) {
    continue_stack_.emplace_back(block, symbol);
  }

  void PopContinueTarget() {
    continue_stack_.pop_back();
  }

  void SetContinueTarget(llvm::BasicBlock* block) {
    assert(block != nullptr);
    for (auto it = continue_stack_.rbegin(); it != continue_stack_.rend(); ++it) {
      if (it->symbol == 0) {
        assert(it->block != nullptr);
        return;
      }
      assert(it->block == nullptr);
      it->block = block;
    }
  }

  void Clear() {
    stack_.clear();
    break_stack_.clear();
    continue_stack_.clear();
  }

  void Dump() const {
    llvm::errs() << "<llvm-ir:flow-stack>\n";
    for (auto it = stack_.rbegin(); it != stack_.rend(); ++it) {
      const auto& flow = *it;
      switch (flow.kind) {
        case ControlFlowKind::kFunction:
          llvm::errs() << "function";
          break;
        case ControlFlowKind::kScope:
          llvm::errs() << "scope: ";
          if (flow.scope.returned) {
            llvm::errs() << 'R';
          }
          if (flow.scope.thrown) {
            llvm::errs() << 'E';
          }
          break;
        case ControlFlowKind::kBranch:
          llvm::errs() << "branch";
          break;
        case ControlFlowKind::kLoopInit:
          llvm::errs() << "loop-init";
          break;
        case ControlFlowKind::kLoopTest:
          llvm::errs() << "loop-test";
          break;
        case ControlFlowKind::kLoopNext:
          llvm::errs() << "loop-next";
          break;
        case ControlFlowKind::kLoopBody:
          llvm::errs() << "loop-body";
          break;
        case ControlFlowKind::kSelect:
          llvm::errs() << "select";
          break;
        case ControlFlowKind::kCaseEnd:
          llvm::errs() << "case-end";
          break;
        case ControlFlowKind::kException:
          llvm::errs() << "exception: ";
          if (flow.exception.thrown) {
            llvm::errs() << 'E';
          }
          if (flow.exception.caught) {
            llvm::errs() << 'C';
          }
          break;
      }
      llvm::errs() << '\n';
    }
    llvm::errs() << "</llvm-ir:flow-stack>\n";
  }

  const FunctionFlow& function_flow() const {
    assert(!stack_.empty());
    assert(stack_[0].kind == ControlFlowKind::kFunction);
    return stack_[0].function;
  }

  const ScopeFlow& scope_flow() const {
    assert(scope_index_ != 0);
    assert(stack_[scope_index_].kind == ControlFlowKind::kScope);
    return stack_[scope_index_].scope;
  }

  const SelectFlow& select_flow() const {
    assert(select_index_ != 0);
    assert(stack_[select_index_].kind == ControlFlowKind::kSelect);
    return stack_[select_index_].select;
  }

  const ExceptionFlow& exception_flow() const {
    assert(exception_index_ != 0);
    assert(stack_[exception_index_].kind == ControlFlowKind::kException);
    return stack_[exception_index_].exception;
  }

  llvm::BasicBlock* cleanup_block() const {
    const auto& flow = top();
    switch (flow.kind) {
      case ControlFlowKind::kFunction:
        return flow.function.return_block;
      case ControlFlowKind::kScope:
        return flow.scope.cleanup_block;
      case ControlFlowKind::kBranch:
      case ControlFlowKind::kException:
        return scope_flow().cleanup_block;
      default:
        // never reach here
        assert(false);
        return nullptr;
    }
  }

  llvm::BasicBlock* exception_block() const {
    const auto& flow = top();
    switch (flow.kind) {
      case ControlFlowKind::kFunction:
        return flow.function.return_block;
      case ControlFlowKind::kScope:
        return flow.scope.cleanup_block;
      case ControlFlowKind::kBranch:
        return scope_flow().cleanup_block;
      case ControlFlowKind::kException:
        if (flow.exception.ended) {
          return flow.exception.end_block;
        }
        if (flow.exception.caught) {
          return flow.exception.finally_block;
        }
        return flow.exception.catch_block;
      default:
        // never reach here
        assert(false);
        return nullptr;
    }
  }

  llvm::BasicBlock* break_target(uint32_t symbol) const {
    if (symbol == 0) {
      return break_stack_.back().block;
    }
    return FindBranchTarget(break_stack_, symbol);
  }

  llvm::BasicBlock* continue_target(uint32_t symbol) const {
    if (symbol == 0) {
      return continue_stack_.back().block;
    }
    return FindBranchTarget(continue_stack_, symbol);
  }

 private:
  static llvm::BasicBlock* FindBranchTarget(const std::vector<BranchTarget>& stack,
      uint32_t symbol) {
    assert(!stack.empty());
    for (auto it = stack.rbegin(); it != stack.rend(); ++it) {
      if (it->symbol == symbol) {
        return it->block;
      }
    }
    assert(false);  // never reach here
    return nullptr;
  }

  const ControlFlow& top() const {
    assert(!stack_.empty());
    return stack_.back();
  }

  ControlFlow& top_mut() {
    assert(!stack_.empty());
    return stack_.back();
  }

  ScopeFlow& scope_flow_mut() {
    assert(scope_index_ != 0);
    return stack_[scope_index_].scope;
  }

  SelectFlow& select_flow_mut() {
    assert(select_index_ != 0);
    return stack_[select_index_].select;
  }

  ExceptionFlow& exception_flow_mut() {
    assert(exception_index_ != 0);
    return stack_[exception_index_].exception;
  }

  std::vector<ControlFlow> stack_;
  std::vector<BranchTarget> break_stack_;
  std::vector<BranchTarget> continue_stack_;

  // The index of the top-most scope flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t scope_index_ = 0;

  // The index of the top-most select flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t select_index_ = 0;

  // The index of the top-most exception flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t exception_index_ = 0;
};
