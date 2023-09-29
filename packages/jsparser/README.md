# bee-jsparser

> A JavaScript parser compliant with ECMA-262

`bee-jsparser` provides an implementation of a JavaScript parser compliant with ECMA-262.

## Current supported version

* [ECMA-262 13th edition (ES2022)](https://262.ecma-international.org/13.0/)

## How is bee-jsparser implemented?

There are many JavaScript parser implementations and many of them are hand-written.  We definitely
agree that this is a reasonable decision, but we decided to implement bee-jsparser using our own
parser generator tools, [bee-dfagen] and [bee-lalrgen].

The ECMA-262 specification defines lexical and syntactic grammars for ECMAScript (JavaScript).  It
uses [special notations](https://262.ecma-international.org/13.0/#sec-notational-conventions) in
the grammars.  For example, `lookahead` terms are used to restrict some of production rules for a
non-terminal symbol following it.

`bison` is one of the widely used and famous parser generator.  However, `bison` has no notation to
express terms such as `lookahead`.  Therefore, it's not easy to convert the ECMA-262 syntactic
grammar into the corresponding `bison`'s grammar in an automatic way.  We think that this is one of
main reasons that the most existing JavaScript parser implementations are hand-written.

We should implement bee-jsparser by hand like as those, but we don't.  Instead, we attempt to
develop our own parser generator tools which support the special notations used in the ECMA-262
specification.  This is an experimental attempt and for a fun.

## Generating code using the parser generator tools

You don't you don't need to generate the code manually if you want to develop modules using
bee-jsparser.  The code has already been generated and committed to the repository.  So, what you
need to do is just fetching the source files from the repository.

When you change files affecting the generated code, you need to perform one of the following
command:

```shell
make codegen

# Or enforce re-generating files.
make -B codegen

# Enable debug logs.
RUST_LOG=debug make codegen
```

See [src/lexer/Makefile](./src/lexer/Makefile) and [src/parser/Makefile](./src/parser/Makefile) for
details of the code generation.

## Acknowledgments

[mozilla-spidermonkey/jsparagus] is a JavaScript parser written in Rust and we learned the basic
idea to generate code using grammars extracted from the ECMA-262 specification.

[cdn.js] is one of CDN services.  We use JavaScript files delivered from it for testing purposes.

## Known issues

JavaScript engines used in major web browsers seems to recognized a different grammar than the one
defined by the ECMA-262 specification.

For example, major web browsers can parse the following JavaScript code without any syntax errors:

```js
if (condition)
  function x() {}
```

Where the function `x` will be defined only if the `condition` is met.  This is not allowed in the
ECMA-262 specification and bee-jsparser stops due to an syntax error.  Because
`FunctionDeclaration` is not included in production rules for `Statement` in the ECMA-262 grammar.

## TODO

* [ ] Generate own tables for `UnicodeSet` instead of use [unicode-id-start]
* [ ] Streaming
* [ ] Text encoding
* [ ] Source location
* [ ] Fuzz testing

[bee-dfagen]: ../dfagen
[bee-lalrgen]: ../lalrgen
[unicode-id-start]: https://crates.io/crates/unicode-id-start
[mozilla-spidermonkey/jsparagus]: https://github.com/mozilla-spidermonkey/jsparagus
[cdn.js]: https://cdnjs.com/
