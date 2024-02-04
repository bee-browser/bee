# jsruntime

> A JavaScript runtime

## Features

* Multi-Tier compilation
  * Tier-1: in-memory, non-optimized naive instructions for a stack machine
  * Tier-2: tier-1 instructions will be translated into in-memory LLVM IR
* LLVM ORC JIT
  * Tier-2 LLVM IR will be compiled into machine instructions of the target architecture

## Memo

* A stack machine naturally appears in a compiler implementation based on bottom-up parsing
  This is the reason why we select a stack machine in the tier-1
  * Tier-1 instructions are interpreted by an interpreter for quick execution of the top-level instructions of JavaScript programs (or modules)
  * Tier-1 instructions in function bodies may be JIT compiled into the tier-2 LLVM IR
