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
  kException,
};

struct FunctionFlow {
  llvm::BasicBlock* locals_block;
  llvm::BasicBlock* args_block;
  llvm::BasicBlock* body_block;
  llvm::BasicBlock* return_block;
};

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
  bool returned;

  // `true` if the scope flow has uncaught exceptions.
  bool thrown;
};

struct ExceptionFlow {
  llvm::BasicBlock* try_block;
  llvm::BasicBlock* catch_block;
  llvm::BasicBlock* finally_block;
  llvm::BasicBlock* end_block;

  // The index of the enclosing outer exception flow.
  size_t outer_index;

  // `true` if the scope flow has uncaught exceptions.
  bool thrown;

  bool caught;

  bool ended;
};

// A `Flow` object contains basic blocks that will construct a region in the control flow graph
// (CFG) of a function.
struct Flow {
  FlowKind kind;
  union {
    FunctionFlow function;
    ScopeFlow scope;
    ExceptionFlow exception;
  };

  inline Flow(FlowKind kind, llvm::BasicBlock* locals_block, llvm::BasicBlock* args_block, llvm::BasicBlock* body_block, llvm::BasicBlock* return_block)
      : kind(kind) {
    function.locals_block = locals_block;
    function.args_block = args_block;
    function.body_block = body_block;
    function.return_block = return_block;
  }

  inline Flow(FlowKind kind, llvm::BasicBlock* init, llvm::BasicBlock* hoisted, llvm::BasicBlock* block, llvm::BasicBlock* cleanup, size_t outer_index)
      : kind(kind) {
    scope.init_block = init;
    scope.hoisted_block = hoisted;
    scope.block = block;
    scope.cleanup_block = cleanup;
    scope.outer_index = outer_index;
    scope.returned = false;
    scope.thrown = false;
  }

  inline Flow(FlowKind kind, llvm::BasicBlock* try_block, llvm::BasicBlock* catch_block, llvm::BasicBlock* finally_block, llvm::BasicBlock* end_block, size_t outer_index, bool thrown)
      : kind(kind) {
    exception.try_block = try_block;
    exception.catch_block = catch_block;
    exception.finally_block = finally_block;
    exception.end_block = end_block;
    exception.outer_index = outer_index;
    exception.thrown = thrown;
    exception.caught = false;
    exception.ended = false;
  }

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

  inline void PushFunctionFlow(llvm::BasicBlock* locals_block, llvm::BasicBlock* args_block, llvm::BasicBlock* body_block, llvm::BasicBlock* return_block) {
    assert(stack_.empty());
    stack_.emplace_back(FlowKind::kFunction, locals_block, args_block, body_block, return_block);
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

  inline void PushScopeFlow(llvm::BasicBlock* init, llvm::BasicBlock* hoisted, llvm::BasicBlock* block, llvm::BasicBlock* cleanup) {
    auto index = stack_.size();
    stack_.emplace_back(FlowKind::kScope, init, hoisted, block, cleanup, scope_index_);
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
          scope_flow_mut().returned = true;
        }
        if (flow.thrown) {
          outer.scope.thrown = true;
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

  inline void PushExceptionFlow(llvm::BasicBlock* try_block, llvm::BasicBlock* catch_block, llvm::BasicBlock* finally_block, llvm::BasicBlock* end_block) {
    auto index = stack_.size();
    stack_.emplace_back(FlowKind::kException, try_block, catch_block, finally_block, end_block, exception_index_, false);
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
    return stack_[0].function;
  }

  inline const ScopeFlow& scope_flow() const {
    assert(scope_index_ != 0);
    return stack_[scope_index_].scope;
  }

  inline const ExceptionFlow& exception_flow() const {
    assert(exception_index_ != 0);
    return stack_[exception_index_].exception;
  }

  inline llvm::BasicBlock* cleanup_block() const {
    const auto& flow = top();
    switch (flow.kind) {
      case FlowKind::kFunction:
        return flow.function.return_block;
      case FlowKind::kScope:
        return flow.scope.cleanup_block;
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

  inline ExceptionFlow& exception_flow_mut() {
    assert(exception_index_ != 0);
    return stack_[exception_index_].exception;
  }

  std::vector<Flow> stack_;

  // The index of the top-most scope flow on the stack.
  // It's used for building the scope chain from the top-most to the bottom-most.
  size_t scope_index_ = 0;

  // The index of the top-most exception flow on the stack.
  // It's used for building the scope chain from the top-most to the bottom-most.
  size_t exception_index_ = 0;
};
