# jsruntime

> A JavaScript runtime

## Features

* TODO: Support ECMA-262 including Annex B
* Lazy JIT compilation
  * A JavaScript function is initially compiled into some kind of *script* that consists of
    in-memory, non-optimized naive instructions
  * Instructions will be performed on *a stack machine* when the JavaScript function is called for
    the first time
    * Unused JavaScript functions won't be compiled to native functions
  * The stack machine will generate [Cranelift IR] representation of the JavaScript function
  * The Cranelift IR representation will be compiled into a native function which can be called
    directly

[Cranelift IR]: https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/docs/ir.md
