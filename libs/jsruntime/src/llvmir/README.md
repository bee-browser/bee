# llvmir

The `llvmir` module implements a compiler and an executor targeting [LLVM IR].

## Stacks used in the LLVM IR compiler

The LLVM IR compiler uses multiple stacks for different purposes.

The operand stack (`operand_stack_`) holds LLVM IR values (concrete classes of `llvm::Value`).

The control flow stack (`control_flow_`) holds sets of basic blocks (`llvm::BasicBlock`) which
construct of a structured sub-graph in the control flow graph ([CFG]) finally built.  Nesting
relationships of structured sub-graphs can be represented as a graph of connected basic blocks.
However, the graph representation is not a program-friendly representation.  By contrast, a stack
can represent the nesting relationships easily as parent-child relationships between adjacent
elements of the stack.

## Scope cleanup checker

We introduced the scope cleanup checker.  At compile time, it inserts additional LLVM IR
instructions that check at runtime if the cleanup instructions for a scope are performed in the
correct order.  It immediately aborts the current program when detecting the cleanup for an
unexpected scope.

In the current implementation, the scope cleanup checker uses a stack to keep IDs of nested scopes.
And it checks:

* If a scope ID at the top of the stack is equals to the current scope ID before performing any
  instructions for the scope cleanup
* If the stack is empty before returning the current function

This feature is one of runtime preferences and we don't need rebuilding a binary for enabling this
feature.

[LLVM IR]: https://llvm.org/docs/LangRef.html
[CFG]: https://en.wikipedia.org/wiki/Control-flow_graph
