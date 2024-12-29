# semantics

The `semantics` module reads a sequence of the flattened AST and performs:

* Analyzing variables and their scopes
* Computing the on-memory location of each variable
* Inferring the type of each variable as much as possible
* Generating a sequence of compile commands for each function
  * The compile commands can be processed on some kind of stack machine

Some of the above correspond to the content of the semantic analysis phase described in
[The Tiger Book].

[The Tiger Book]: https://www.cs.princeton.edu/~appel/modern/
