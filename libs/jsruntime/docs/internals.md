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

## async/await

There are multiple feasible ways to implement the async/await features, but we decided not to use
[the LLVM coroutines](https://llvm.org/docs/Coroutines.html).  The reasons are as follows:

* We have a plan to switch IR to others written in Rust, such as cranelift IR
  * It's better to use only instructions that commonly used in others
* The optimizer for llvm.coro.* is great but it does not improve instructions generated from a
  JavaScript program
  * In many cases, we cannot do inlining JavaScript functions because these are bound to mutable
    variables and can be changed at runtime
  * Of course, functions bound to immutable variables declared with const can be done inlining at
    compile time, but it's probably minority cases

Instead, we implement the async/await features in a way similar to Rust:

* [Async fn as a Future](https://tokio.rs/tokio/tutorial/async#async-fn-as-a-future)

An async function:

```javascript
async function func() {
  <async-function-body>
}
```

is ideally translated into:

```javascript
// The ramp function translated from the original async function.
function func() {
  // The inner coroutine is registered to some kind of job system in order to perform resuming the
  // coroutine asynchronously.
  const ##promise = runtime.register_promise(
    runtime.create_coroutine(
      // The arguments of the original async function are captured by the inner closure wrapping
      // the coroutine.
      //
      // A special variable called ##coroutine can be used inside the coroutine.
      //
      // The lambda function of the coroutine will be called with the following arguments the
      // coroutine resumes:
      //
      //   ##promise: The promise returned from the ramp function.
      //   ##result: Available if the promise is fulfilled.
      //   ##error: Available if the promise is rejected.
      //
      (##promise, ##result, ##error) => {
        // Load local, temporal and captured variables from ##coroutine.

        // Jump to the entry basic block for each coroutine state.
        <jump-table for ##coroutine.state>

        {
          // Values of variables must be held over suspend points will be saved into ##coroutine
          // before the coroutine suspends.
          <modified async-function-body>
        }
      },
      // Local variables used in the coroutine must be held over suspend points.
      // So, those will be placed in the heap memory.
      NUM_LOCALS,
      // Temporal variables used in expressions must be held over suspend points, too.
      // Those will be placed in the special memory called the scratch buffer allocated in the heap
      // memory.
      SCRATCH_BUFFER_LEN,
    ),
  );

  // Perform the function body of the coroutine until the first suspend point (or the entire
  // function body if there is no suspend point in the original function body).
  runtime.resume(##promise);

  return ##promise;
}
```

The async function is translated into a *regular* function that returns *a promise* (it's not a
`Promise` object at this point because there is no `Object` implementation in the runtime).  This
kind of function is generally called a *ramp* function in a coroutine implementation methodology.

Inside the ramp function, a [*coroutine*](https://en.wikipedia.org/wiki/Coroutine) is created.
Like the Rust compiler does, we implements the coroutine with a state machine.  In the `semantics`
module, compile commends for creating a jump table and suspending/resuming the coroutine execution
are generated and these commands are processed in the `llvmir::compiler` module in order to build
corresponding LLVM IR instructions.

A closure for the lambda function of the coroutine is created in order to capture the arguments of
the original async function.  The coroutine lambda function takes special *internal* arguments
instead of the original ones.  Inside the coroutine lambda function, the original arguments can be
accessible via **an environment** that contains the captured original arguments.

A `Coroutine` data which is passed as the environment to the coroutine lambda function is created.
The pointer to the `Coroutine` data is specified in the `context` formal parameter of the coroutine
lambda function.  An important member variable of the `Coroutine` is `Coroutine::state`.  This
member variable keeps and tracks the current state of the state machine used for the coroutine
implementation and used as the index for the jump table of the state machine implementation.

In addition, the `Coroutine` data has enough memory area for local and temporal variables used in
the coroutine lambda function.  Some of these variables must be held over suspend points.  Values
of such variables will be saved into the memory area of the `Coroutine` data before the coroutine
suspends and loaded from it when the coroutine resumes.  The size of the memory area can be
computed at compile time.

### Generating LLVM IR instructions implementing the state machine for a coroutine

Here we are going to explain how `await` expressions are translated into [LLVM IR instructions].
For simplicity, we treat only the case of the following simple `await` expression:

```javascript
await 0
```

The same strategy can work for any kind of JavaScript control structure including conditional and
loop statements.

LLVM IR instructions for `await` expressions can be divided into two groups:

* Instructions building the jump table for the state machine
* Instructions to suspend and resume the execution

As you saw above, the instructions for the jump table is placed before instructions for the
modified function body.  The jump table can be built with an LLVM IR `switch` instruction:

```llvm
bb.body:                                          ; preds = %bb.args
  %co.state.ptr = getelementptr inbounds %Coroutine, ptr %context, i32 0, i32 1
  %co.state = load i32, ptr %co.state.ptr, align 4
  switch i32 %co.state, label %bb.co.dormant [
    i32 0, label %bb.co.initial
    i32 1, label %bb.scope.2.resume
  ]
```

At the suspend point, the LLVM IR compiler generates a LLVM IR `store` instruction to update the
current state of the coroutine and a LLVM IR `ret` instruction to suspend the execution:

```llvm
  ; Save temporal variables to the scratch buffer.
  %co.state.ptr4 = getelementptr inbounds %Coroutine, ptr %context, i32 0, i32 1
  store i32 1, ptr %co.state.ptr4, align 4
  ret i32 2
```

A basic block from which the execution resumes is inserted after the basic block containing the
suspend point:

```llvm
bb.scope.2.resume:                                ; preds = %bb.body
  ; Load temporal variables the scratch buffer.
```

You can see the entire LLVM IR instructions by running the following command:

```shell
echo 'await 0' | cargo run --bin=jstb -- compile --as=module --no-optimize
```

### The scratch buffer

`await` is an expression.  So, we have to think about how to hold temporal values used in an
expression including `await` expressions over suspend points.

Given the following JavaScript expression:

```javascript
a + b + await c;
```

`a + b` is evaluated before `+ await c`.  The result of `a + b` must be held over the suspend point
at `await c`.  Then, the value of `c` is added to the result of `a + b`.  The result of `a + b` is
a temporal value and it's never assigned to any local variable.

Holding temporal values over suspend points, we introduce a special memory area in the `Coroutine`
data, called *the scratch buffer*.  Values saved into the scratch buffer are available only before
the next suspend point.  Existing values in the scratch buffer will always be **overwritten** with
new temporal values used in an expression which is being evaluated just before the next suspend
point.

The following LLVM IR instructions store the result of `a + b` into the scratch buffer and load it
from the scratch buffer:

```llvm
  %co.scratch_buffer.ptr = getelementptr inbounds i8, ptr %context, i64 %co.scratch_buffer.offsetof
  %scratch.number.ptr = getelementptr inbounds i8, ptr %co.scratch_buffer.ptr, i32 0
  store double %add, ptr %scratch.number.ptr, align 8
  ...

bb.scope.2.resume:                                ; preds = %bb.body
  ...
  %co.scratch_buffer.ptr26 = getelementptr inbounds i8, ptr %context, i64 %co.scratch_buffer.offsetof25
  %scratch.number.ptr27 = getelementptr inbounds i8, ptr %co.scratch_buffer.ptr26, i32 0
  %scratch.number = load double, ptr %scratch.number.ptr27, align 8
```

Run the following command if you want to see the entire LLVM IR instructions:

```shell
echo 'const a = 1, b = 2, c = 3; a + b + await c' | \
  cargo run --bin=jstb -- compile --no-optimize --as=module
```

[LLVM IR instructions]: https://llvm.org/docs/LangRef.html
