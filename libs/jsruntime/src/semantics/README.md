# semantics

The `semantics` module reads a sequence of the flattened AST and performs:

* Analyzing bindings and their scopes
* Computing the on-memory location of each binding
* Inferring the type of each binding as much as possible
* Generating a sequence of compile commands for each function
  * The compile commands can be processed on some kind of stack machine

Some of the above correspond to the content of the semantic analysis phase described in
[Modern Compiler Implementation in ML].

[Modern Compiler Implementation in ML](https://www.cambridge.org/core/books/modern-compiler-implementation-in-ml/C2A59C37468AA8AAD0ADDCE080E3CB5D)
