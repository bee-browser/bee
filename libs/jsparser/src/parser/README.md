# parser

The `parser` module implements a JavaScript parser.

Unlike the `Lexer`, the `Parser` is implemented in a push-style.  An implementation of the
`SyntaxHandler` trait receives *events* from the `Parser`.  In many cases, the `SyntaxHandler` is
used for building an AST or directly generating code to be executed on a VM.

`lalrgen` is used to generate LALR(1) parsing tables.  It supports multiple goals and a single set
of parsing tables are generated.

`lalr.js` tweaks it for later RUST code generation.  The most important task of `lalr.js` is adding
properties to each state object, which are used for processing auto semicolon insertions in the
parser.  The generated Rust source files are placed in the `lalr` module inside the `parser`
module.

Supplemental syntax and static semantics defined in the ECMA-262 specification are not processed in
the parser.  For example, the `CoverParenthesizedExpressionAndArrowParameterList` may produce an
empty parenthesized expression `()` but it's not allowed in the `PrimaryExpression`.  This must be
handled as a syntax error in the caller side.

## TODO

* [ ] Consider changing to a pull-style
  * A pull-style parser is better than a push-style in a control flow point of view
  * Overhead of the `SyntaxHandler` method calls is minimized by static dispatches
* [ ] Replace the `phf` crate with other implementation
  * Currently it's one of bottlenecks in the performance
  * Using a simple binary search tree might be enough
* [ ] Compress the parsing tables
