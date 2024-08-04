#pragma once

#include <cassert>
#include <cstddef>
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
  kLoop,
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

struct LoopFlow {
  size_t outer_index;

  // For LoopTest()
  llvm::BasicBlock* test_then_block;
  llvm::BasicBlock* test_else_block;
  llvm::BasicBlock* test_insert_point;

  // For LoopBody()
  llvm::BasicBlock* body_branch_block;
  llvm::BasicBlock* body_insert_point;

  // For LoopInit()
  llvm::BasicBlock* init_branch_block = nullptr;
  llvm::BasicBlock* init_insert_point = nullptr;

  // For LoopNext()
  llvm::BasicBlock* next_branch_block = nullptr;
  llvm::BasicBlock* next_insert_point = nullptr;
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
    LoopFlow loop;
    SelectFlow select;
    CaseEndFlow case_end;
    ExceptionFlow exception;
  };

  inline Flow(const FunctionFlow& function) : kind(FlowKind::kFunction), function(function) {}
  inline Flow(const ScopeFlow& scope) : kind(FlowKind::kScope), scope(scope) {}
  inline Flow(const BranchFlow& branch) : kind(FlowKind::kBranch), branch(branch) {}
  inline Flow(const LoopFlow& loop) : kind(FlowKind::kLoop), loop(loop) {}
  inline Flow(const SelectFlow& select) : kind(FlowKind::kSelect), select(select) {}
  inline Flow(const CaseEndFlow& case_end) : kind(FlowKind::kCaseEnd), case_end(case_end) {}
  inline Flow(const ExceptionFlow& exception) : kind(FlowKind::kException), exception(exception) {}
  Flow(const Flow& flow) = default;
  ~Flow() = default;
};

class FlowStack {
 public:
  FlowStack() = default;
  ~FlowStack() = default;

  inline bool IsEmpty() const {
    return stack_.empty();
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
      case FlowKind::kLoop:     // TODO
      case FlowKind::kSelect:   // TODO
      case FlowKind::kCaseEnd:  // TODO
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

  inline void PushDoWhileLoopFlow(llvm::BasicBlock* body_branch_block,
      llvm::BasicBlock* body_insert_point,
      llvm::BasicBlock* test_then_block,
      llvm::BasicBlock* test_else_block,
      llvm::BasicBlock* test_insert_point) {
    auto index = stack_.size();
    stack_.emplace_back(LoopFlow{loop_index_, test_then_block, test_else_block, test_insert_point,
        body_branch_block, body_insert_point});
    loop_index_ = index;
  }

  inline void PushWhileLoopFlow(llvm::BasicBlock* test_then_block,
      llvm::BasicBlock* test_else_block,
      llvm::BasicBlock* test_insert_point,
      llvm::BasicBlock* body_branch_block,
      llvm::BasicBlock* body_insert_point) {
    auto index = stack_.size();
    stack_.emplace_back(LoopFlow{loop_index_, test_then_block, test_else_block, test_insert_point,
        body_branch_block, body_insert_point});
    loop_index_ = index;
  }

  inline void PushForLoopFlow(llvm::BasicBlock* init_branch_block,
      llvm::BasicBlock* init_insert_point,
      llvm::BasicBlock* test_then_block,
      llvm::BasicBlock* test_else_block,
      llvm::BasicBlock* test_insert_point,
      llvm::BasicBlock* next_branch_block,
      llvm::BasicBlock* next_insert_point,
      llvm::BasicBlock* body_branch_block,
      llvm::BasicBlock* body_insert_point) {
    auto index = stack_.size();
    stack_.emplace_back(LoopFlow{loop_index_, test_then_block, test_else_block, test_insert_point,
        body_branch_block, body_insert_point, init_branch_block, init_insert_point,
        next_branch_block, next_insert_point});
    loop_index_ = index;
  }

  inline LoopFlow PopLoopFlow() {
    assert(top().kind == FlowKind::kLoop);
    auto loop = top().loop;

    stack_.pop_back();
    // The `loop_index_` must be updated just after stack_.pop_back() so that other instance
    // methods such as `loop_flow()` work properly.
    loop_index_ = loop.outer_index;

    return loop;
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

  inline void Clear() {
    stack_.clear();
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
        case FlowKind::kLoop:
          llvm::errs() << "loop";
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

  inline const LoopFlow& loop_flow() const {
    assert(loop_index_ != 0);
    assert(stack_[loop_index_].kind == FlowKind::kLoop);
    return stack_[loop_index_].loop;
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

 private:
  inline Flow& top_mut() {
    assert(!stack_.empty());
    return stack_.back();
  }

  inline ScopeFlow& scope_flow_mut() {
    assert(scope_index_ != 0);
    return stack_[scope_index_].scope;
  }

  inline LoopFlow& loop_flow_mut() {
    assert(loop_index_ != 0);
    return stack_[loop_index_].loop;
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

  // The index of the top-most scope flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t scope_index_ = 0;

  // The index of the top-most loop flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t loop_index_ = 0;

  // The index of the top-most select flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t select_index_ = 0;

  // The index of the top-most exception flow on the stack.
  // It's used for building the flow chain from the top-most to the bottom-most.
  size_t exception_index_ = 0;
};
