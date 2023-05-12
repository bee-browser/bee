# bee-jslexer

> A JavaScript lexer compliant with ECMA-262 13th edition (ES2022)

`bee-jslexer` provides an implementation of a JavaScript lexer compliant with
[ECMA-262 13th edition (ES2022)](https://262.ecma-international.org/13.0/).

## Generating DFAs

`bee-jslexer` recognizes tokens defined in ES2022.  The lexical grammar of
ES2022 is defined in CFG and has multiple goal symbols.  We generate a DFA for
each goal symbol in the following steps:

1. Extract the lexical grammar from ES2022 specification
2. Transpile the extracted lexical grammar into an equivalent but more
   computer-friendly data
3. Define goal symbols in [src/tokens.yaml](./src/tokens.yaml)
4. Build a NFA recognizing tokens contained in each goal symbol in the data
   without converting production rules into a regular expression for each token
5. Build a minimized DFA for each goal symbol from a NFA

Details of each step are described in `src/Makefile`.

## TODO

* [ ] Support `ID_Start` and `ID_Continue` Unicode properties
  * Currently, we support only ASCII character identifier names
* [ ] Streaming
* [ ] Text encoding
* [ ] Source location
* [ ] Fuzz testing
