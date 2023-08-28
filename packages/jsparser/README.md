# bee-jsparser

> A JavaScript parser compliant with ECMA-262 13th edition (ES2022)

`bee-jsparser` provides an implementation of a JavaScript parser compliant with
[ECMA-262 13th edition (ES2022)](https://262.ecma-international.org/13.0/).

## Generating DFAs

Run the following command:

```shell
make codegen

# Or enforce re-generating files.
make -B codegen

# Enable debug logs.
RUST_LOG=debug make codegen
```

See `src/lexer/Makefile` for details of the code generation steps.

## TODO

* [ ] Generate own tables for `UnicodeSet` instead of use [unicode-id-start]
* [ ] Streaming
* [ ] Text encoding
* [ ] Source location
* [ ] Fuzz testing

[unicode-id-start]: https://crates.io/crates/unicode-id-start
