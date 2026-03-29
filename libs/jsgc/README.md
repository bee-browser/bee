# jsgc

> A garbage collector for JavaScript

## Features

* Tracing
* Precise

## Basic Design

### `Managed` and `Unmanaged`

From the `jsgc` crate perspective, all memory blocks can be classified into the following two
groups:

* Managed memory blocks
* Unmanaged memory blocks

Managed memory blocks are allocated from a heap memory managed by the `jsgc` crate.  Any managed
memory blocks that are unreachable from the root set are reclaimed when a garbage collection is
performed.

Unmanaged memory blocks are allocated outside the heap memory managed by the `jsgc` crate.  None of
unmanaged memory blocks are reclaimed by a garbage collection.  The root set always contains
unmanaged memory blocks that contain references to managed memory blocks.

### `Trace`

All types that have fields of `Handle<T>` must implement the `Trace` trait.
