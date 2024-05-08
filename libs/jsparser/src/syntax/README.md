# syntax

The `syntax` module performs:

* Static analyses on an [AST] of a JavaScript program according to *static semantic rules* defined
  in the ECMA-262 specification
* Refinements of *permissive* production rules using supplemental syntax
* Generating a stream of ordered nodes visited in a depth-first tree traversal on the AST of the
  JavaScript program

The structure of the ordered nodes in the stream is somewhat similar, but not same as a sequence of
the [flattened AST] of the JavaScript program.  It has no links to child nodes (the indexes of the
child nodes).  It's intended that there is a stack machine to interpret nodes in the stream and
translate the stream into another representation such as the [ESTree] of the JavaScript program, or
instructions that is executable on a target machine.

[AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[flattened AST]: https://www.cs.cornell.edu/~asampson/blog/flattening.html
[ESTree]: https://github.com/estree/estree
