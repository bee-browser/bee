# Internals

## Pipeline

The following diagram shows perspective of data-flow on the pipeline.

```
+-------------+
| Data Source |
+-------------+
  | Byte Stream
  V
+--------------+
| Text Decoder |
+--------------+
  | UNICODE Character Stream
  V
+--------------------------+
| `jsparser::lexer::Lexer` |
+--------------------------+
  | Token Stream
  V
+----------------------------+
| `jsparser::parser::Parser` |
+----------------------------+
  | LALR(1) Shift/Reduce Event Stream
  V
+-------------------------------+
| `jsparser::syntax::Processor` |
+-------------------------------+
  | Flattened AST Node Stream
  V
+----------------------------------+
| `jsruntime::semantics::Analyzer` |
+----------------------------------+
  | Compile Command Stream
  V
+-------------------------------+
| `jsruntime::llvmir::Compiler` |
+-------------------------------+
  | LLVM IR Module
  V
+-------------------------------+
| `jsruntime::llvmir::Executor` |
+-------------------------------+
```

Currently, the pipeline is performed on a single thread, but it may be performed on multiple
threads in the future if the multi-threading improves the compilation performance.

## Dynamic code evaluation

In the first implementation, we won't support the dynamic code evaluation in the following built-in
functions:

* `eval()`
* `new Function()`
* `setTimeout()`
* `setInterval()`

It's recommended not to use the dynamic code evaluation due to security risks.  And major web
browsers support features to block the dynamic code evaluation nowadays.

We may implement the dynamic code evaluation in the future if there are many web sites and
applications depending on it, but our runtime will never be optimized for it.  This means that the
dynamic code evaluation works correctly but not efficiently.

The dynamic code evaluation support makes the runtime complicated and sacrifices performance of
other primary features such as resolving a reference to a lexical binding.  So, we believe that our
decision contributes to improvement of overall performance.

## Closures

The current implementation of closures is based on, but not the same as, an algorithm described in
the following paper and book:

* [Closures in Lua](https://www.cs.tufts.edu/~nr/cs257/archive/roberto-ierusalimschy/closures-draft.pdf)
* [Closures - Crafting Interpreters](https://craftinginterpreters.com/closures.html)

Differences from the original algorithm come from differences in the processing models.  One of
design goals of our compiler is to save runtime costs as much as possible.  Achieving this goal,
our compiler computes logical locations of free variables on the stack and generates LLVM IR
instructions to escape the free variables from the stack to the heap at runtime.  The generated
code does not use a list of open upvalues at all, which is used for sharing upvalues among
closures.

The implementation of our compiler is basically separated into two parts:

1. Detecting detect free variables and computing their locations
2. Generating LLVM IR instructions to perform runtime operations on *upvalues*

The first part is implemented in the `semantics` module.  A `semantics::Analyzer` builds a scope
tree for a JavaScript program.  A scope node in the scope tree holds a list of variables defined in
the scope.  When a variable referred in a scope is not found in the lexical scope chain from the
scope up to the function scope, the `semantics::Analyzer` postpones resolving the reference and
will try again in outer scopes enclosing the function.  Once the reference is resolved, the
`semantics::Analyzer` updates *placeholder* `semantics::CompileCommand`s to be interpreted in the
next phase to generate LLVM IR instructions for closures.

The second part is implemented in the `compiler` and `bridge` modules.  A `compiler::Compiler`
interprets the `semantics::CompileCommand`s and calls functions of `bridge::Compiler` in order to
build LLVM IR instructions for closures.  The logical location of each free variable on the stack
is computed and the runtime part of the original algorithm are coded in this phase.
