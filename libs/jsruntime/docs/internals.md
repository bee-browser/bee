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
