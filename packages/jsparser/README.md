# bee-jsparser

> A JavaScript parser compliant with ECMA-262 13th edition (ES2022)

`bee-jsparser` provides an implementation of a JavaScript parser compliant with
[ECMA-262 13th edition (ES2022)](https://262.ecma-international.org/13.0/).

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

We should implement bee-jsparser by hand like as those, we didn't.  Because our browser engine is
experimental and for fun.

## Generating code using the parser generator tools

Run the following command:

```shell
make codegen

# Or enforce re-generating files.
make -B codegen

# Enable debug logs.
RUST_LOG=debug make codegen
```

See [src/lexer/Makefile](./src/lexer/Makefile) and [src/parser/Makefile](./src/parser/Makefile) for
details of the code generation steps.

## Acknowledgments

[mozilla-spidermonkey/jsparagus] is a JavaScript parser written in Rust and we learned the basic
idea to generate code using grammars extracted from the ECMA-262 specification.

[cdn.js] is one of CDN services.  We use JavaScript files delivered from it for testing purposes.

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
