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
make codegen TRANSPILE_ARGS=-d LEXER_DFAGEN_ARGS=-d
```

See `src/lexer/Makefile` for details of the code generation steps.

## TODO

* [ ] Support `ID_Start` and `ID_Continue` Unicode properties
  * Currently, we support only ASCII character identifier names
* [ ] Streaming
* [ ] Text encoding
* [ ] Source location
* [ ] Fuzz testing
