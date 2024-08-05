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

enum class FlowKind {
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
struct Flow {
  FlowKind kind;
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

  inline Flow(const FunctionFlow& function) : kind(FlowKind::kFunction), function(function) {}
  inline Flow(const ScopeFlow& scope) : kind(FlowKind::kScope), scope(scope) {}
  inline Flow(const BranchFlow& branch) : kind(FlowKind::kBranch), branch(branch) {}
  inline Flow(const LoopInitFlow& loop_init) : kind(FlowKind::kLoopInit), loop_init(loop_init) {}
  inline Flow(const LoopTestFlow& loop_test) : kind(FlowKind::kLoopTest), loop_test(loop_test) {}
  inline Flow(const LoopNextFlow& loop_next) : kind(FlowKind::kLoopNext), loop_next(loop_next) {}
  inline Flow(const LoopBodyFlow& loop_body) : kind(FlowKind::kLoopBody), loop_body(loop_body) {}
  inline Flow(const SelectFlow& select) : kind(FlowKind::kSelect), select(select) {}
  inline Flow(const CaseEndFlow& case_end) : kind(FlowKind::kCaseEnd), case_end(case_end) {}
  inline Flow(const ExceptionFlow& exception) : kind(FlowKind::kException), exception(exception) {}
  Flow(const Flow& flow) = default;
  ~Flow() = default;
};

struct BranchTarget {
  llvm::BasicBlock* block;
  uint32_t symbol;

  BranchTarget(llvm::BasicBlock* block, uint32_t symbol) : block(block), symbol(symbol) {}
  ~BranchTarget() = default;
};

class FlowStack {
 public:
  FlowStack() = default;
  ~FlowStack() = default;

  inline bool IsEmpty() const {
    return stack_.empty() && break_stack_.empty() && continue_stack_.empty();
  }

  inline void PushFunctionFlow(llvm::BasicBlock* locals_block,
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

  inline FunctionFlow PopFunctionFlow() {
    auto flow = top();
    assert(flow.kind == FlowKind::kFunction);
    stack_.pop_back();
    assert(stack_.empty());
    assert(scope_index_ == 0);
    assert(exception_index_ == 0);
    return flow.function;
  }

  inline void PushScopeFlow(llvm::BasicBlock* init_block,
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

  inline ScopeFlow PopScopeFlow() {
    assert(top().kind == FlowKind::kScope);
    auto flow = top().scope;

    stack_.pop_back();
    // The `scope_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `scope_flow()` work properly.
    scope_index_ = flow.outer_index;

    // Propagate flags to the outer flow.
    auto& outer = top_mut();
    switch (outer.kind) {
      case FlowKind::kFunction:
        // Nothing to do.
        break;
      case FlowKind::kScope:
        if (flow.returned) {
          outer.scope.returned = true;
        }
        if (flow.thrown) {
          outer.scope.thrown = true;
        }
        break;
      case FlowKind::kBranch:
      case FlowKind::kLoopInit:  // TODO
      case FlowKind::kLoopTest:  // TODO
      case FlowKind::kLoopNext:  // TODO
      case FlowKind::kLoopBody:  // TODO
      case FlowKind::kSelect:    // TODO
      case FlowKind::kCaseEnd:   // TODO
        if (flow.returned) {
          scope_flow_mut().returned = true;
        }
        if (flow.thrown) {
          scope_flow_mut().thrown = true;
        }
        break;
      case FlowKind::kException:
        if (flow.thrown) {
          outer.exception.thrown = true;
        }
        break;
    }

    return flow;
  }

  inline void PushBranchFlow(llvm::BasicBlock* before_block, llvm::BasicBlock* after_block) {
    assert(before_block != nullptr);
    assert(after_block != nullptr);
    stack_.emplace_back(BranchFlow{before_block, after_block});
  }

  inline BranchFlow PopBranchFlow() {
    assert(top().kind == FlowKind::kBranch);
    auto branch = top().branch;
    stack_.pop_back();
    return branch;
  }

  inline void PushLoopInitFlow(llvm::BasicBlock* branch_block, llvm::BasicBlock* insert_point) {
    assert(branch_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopInitFlow{branch_block, insert_point});
  }

  inline LoopInitFlow PopLoopInitFlow() {
    assert(top().kind == FlowKind::kLoopInit);
    auto loop_init = top().loop_init;
    stack_.pop_back();
    return loop_init;
  }

  inline void PushLoopTestFlow(llvm::BasicBlock* then_block,
      llvm::BasicBlock* else_block,
      llvm::BasicBlock* insert_point) {
    assert(then_block != nullptr);
    assert(else_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopTestFlow{then_block, else_block, insert_point});
  }

  inline LoopTestFlow PopLoopTestFlow() {
    assert(top().kind == FlowKind::kLoopTest);
    auto loop_test = top().loop_test;
    stack_.pop_back();
    return loop_test;
  }

  inline void PushLoopNextFlow(llvm::BasicBlock* branch_block, llvm::BasicBlock* insert_point) {
    assert(branch_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopNextFlow{branch_block, insert_point});
  }

  inline LoopNextFlow PopLoopNextFlow() {
    assert(top().kind == FlowKind::kLoopNext);
    auto loop_next = top().loop_next;
    stack_.pop_back();
    return loop_next;
  }

  inline void PushLoopBodyFlow(llvm::BasicBlock* branch_block, llvm::BasicBlock* insert_point) {
    assert(branch_block != nullptr);
    assert(insert_point != nullptr);
    stack_.emplace_back(LoopBodyFlow{branch_block, insert_point});
  }

  inline LoopBodyFlow PopLoopBodyFlow() {
    assert(top().kind == FlowKind::kLoopBody);
    auto loop_body = top().loop_body;
    stack_.pop_back();
    return loop_body;
  }

  inline void PushSelectFlow(llvm::BasicBlock* end_block) {
    assert(end_block != nullptr);
    auto index = stack_.size();
    stack_.emplace_back(SelectFlow{end_block, select_index_});
    select_index_ = index;
  }

  inline SelectFlow PopSelectFlow() {
    assert(top().kind == FlowKind::kSelect);
    auto select = top().select;

    stack_.pop_back();
    // The `select_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `select_flow()` work properly.
    select_index_ = select.outer_index;

    return select;
  }

  inline void PushCaseEndFlow(llvm::BasicBlock* block) {
    assert(block != nullptr);
    stack_.emplace_back(CaseEndFlow{block});
  }

  inline CaseEndFlow PopCaseEndFlow() {
    assert(top().kind == FlowKind::kCaseEnd);
    auto case_end = top().case_end;
    stack_.pop_back();
    return case_end;
  }

  inline void PushExceptionFlow(llvm::BasicBlock* try_block,
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

  inline ExceptionFlow PopExceptionFlow() {
    assert(top().kind == FlowKind::kException);
    auto flow = top().exception;

    stack_.pop_back();
    // The `exception_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `exception_flow()` work properly.
    exception_index_ = flow.outer_index;

    // Any exception flow is enclosed by a scope flow.
    assert(top().kind == FlowKind::kScope);

    // Propagate flags to the outer flow.
    if (flow.thrown) {
      top_mut().scope.thrown = true;
    }

    return flow;
  }

  inline void SetReturned() {
    auto& flow = top_mut();
    switch (flow.kind) {
      case FlowKind::kFunction:
        // Nothing to do.
        break;
      case FlowKind::kScope:
        flow.scope.returned = true;
        break;
      case FlowKind::kBranch:
        scope_flow_mut().returned = true;
        break;
      default:
        // never reach here
        assert(false);
        break;
    }
  }

  inline void SetThrown() {
    assert(top().kind == FlowKind::kScope);
    top_mut().scope.thrown = true;
  }

  inline void SetCaught(bool nominal) {
    assert(top().kind == FlowKind::kException);
    top_mut().exception.caught = true;
    if (!nominal) {
      top_mut().exception.thrown = false;
    }
  }

  inline void SetEnded() {
    assert(top().kind == FlowKind::kException);
    top_mut().exception.ended = true;
  }

  inline void SetDefaultCaseBlock(llvm::BasicBlock* block) {
    assert(block != nullptr);
    select_flow_mut().default_case_block = block;
  }

  inline void PushBreakTarget(llvm::BasicBlock* block, uint32_t symbol = 0) {
    break_stack_.emplace_back(block, symbol);
  }

  inline BranchTarget PopBreakTarget() {
    auto target = break_stack_.back();
    break_stack_.pop_back();
    return target;
  }

  inline void PushContinueTarget(llvm::BasicBlock* block, uint32_t symbol = 0) {
    continue_stack_.emplace_back(block, symbol);
  }

  inline void PopContinueTarget() {
    continue_stack_.pop_back();
  }

  inline void SetContinueTarget(llvm::BasicBlock* block) {
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

  inline void Clear() {
    stack_.clear();
    break_stack_.clear();
    continue_stack_.clear();
  }

  void Dump() const {
    llvm::errs() << "<llvm-ir:flow-stack>\n";
    for (auto it = stack_.rbegin(); it != stack_.rend(); ++it) {
      const auto& flow = *it;
      switch (flow.kind) {
        case FlowKind::kFunction:
          llvm::errs() << "function";
          break;
        case FlowKind::kScope:
          llvm::errs() << "scope: ";
          if (flow.scope.returned) {
            llvm::errs() << 'R';
          }
          if (flow.scope.thrown) {
            llvm::errs() << 'E';
          }
          break;
        case FlowKind::kBranch:
          llvm::errs() << "branch";
          break;
        case FlowKind::kLoopInit:
          llvm::errs() << "loop-init";
          break;
        case FlowKind::kLoopTest:
          llvm::errs() << "loop-test";
          break;
        case FlowKind::kLoopNext:
          llvm::errs() << "loop-next";
          break;
        case FlowKind::kLoopBody:
          llvm::errs() << "loop-body";
          break;
        case FlowKind::kSelect:
          llvm::errs() << "select";
          break;
        case FlowKind::kCaseEnd:
          llvm::errs() << "case-end";
          break;
        case FlowKind::kException:
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

  inline const Flow& top() const {
    assert(!stack_.empty());
    return stack_.back();
  }

  inline const FunctionFlow& function_flow() const {
    assert(!stack_.empty());
    assert(stack_[0].kind == FlowKind::kFunction);
    return stack_[0].function;
  }

  inline const ScopeFlow& scope_flow() const {
    assert(scope_index_ != 0);
    assert(stack_[scope_index_].kind == FlowKind::kScope);
    return stack_[scope_index_].scope;
  }

  inline const SelectFlow& select_flow() const {
    assert(select_index_ != 0);
    assert(stack_[select_index_].kind == FlowKind::kSelect);
    return stack_[select_index_].select;
  }

  inline const ExceptionFlow& exception_flow() const {
    assert(exception_index_ != 0);
    assert(stack_[exception_index_].kind == FlowKind::kException);
    return stack_[exception_index_].exception;
  }

  inline llvm::BasicBlock* cleanup_block() const {
    const auto& flow = top();
    switch (flow.kind) {
      case FlowKind::kFunction:
        return flow.function.return_block;
      case FlowKind::kScope:
        return flow.scope.cleanup_block;
      case FlowKind::kBranch:
      case FlowKind::kException:
        return scope_flow().cleanup_block;
      default:
        // never reach here
        assert(false);
        return nullptr;
    }
  }

  inline llvm::BasicBlock* exception_block() const {
    const auto& flow = top();
    switch (flow.kind) {
      case FlowKind::kFunction:
        return flow.function.return_block;
      case FlowKind::kScope:
        return flow.scope.cleanup_block;
      case FlowKind::kBranch:
        return scope_flow().cleanup_block;
      case FlowKind::kException:
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

  inline llvm::BasicBlock* break_target(uint32_t symbol) const {
    if (symbol == 0) {
      return break_stack_.back().block;
    }
    assert(!break_stack_.empty());
    for (auto it = break_stack_.rbegin(); it != break_stack_.rend(); ++it) {
      if (it->symbol == symbol) {
        return it->block;
      }
    }
    assert(false);  // never reach here
    return nullptr;
  }

  inline llvm::BasicBlock* continue_target(uint32_t symbol) const {
    if (symbol == 0) {
      return continue_stack_.back().block;
    }
    assert(!continue_stack_.empty());
    for (auto it = continue_stack_.rbegin(); it != continue_stack_.rend(); ++it) {
      if (it->symbol == symbol) {
        return it->block;
      }
    }
    assert(false);  // never reach here
    return nullptr;
  }

 private:
  inline Flow& top_mut() {
    assert(!stack_.empty());
    return stack_.back();
  }

  inline ScopeFlow& scope_flow_mut() {
    assert(scope_index_ != 0);
    return stack_[scope_index_].scope;
  }

  inline SelectFlow& select_flow_mut() {
    assert(select_index_ != 0);
    return stack_[select_index_].select;
  }

  inline ExceptionFlow& exception_flow_mut() {
    assert(exception_index_ != 0);
    return stack_[exception_index_].exception;
  }

  std::vector<Flow> stack_;
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
