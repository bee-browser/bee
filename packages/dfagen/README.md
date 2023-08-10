# bee-dfagen

> A deterministic finite automaton generator for lexer generators

The main goal of this crate is generating a DFA (deterministic finite automaton) recognizing tokens
defined in the ECMA-262 specification.

The lexical grammar defined in the ECMA-262 specification is written in some kind of a CFG
(context-free grammar).  Most popular existing lexer generators such as flex define tokens by using
regular expressions.  Therefore, we have to convert the ECMA-262 CFG into a corresponding regular
expressions that cab be acceptable for a lexer generator to use.

Eventually, we decided to build a new DFA generator for the following reasons:

* Manually converting the grammar tends to make hard-to-find mistakes easily
  * This implies that we should write a script for the conversion, but it's not an efficient way
    for the next reason
* We can easily build a NFA (non-deterministic finite automaton) from a production rule in a CFG

`bee-dfagen` generates a DFA recognizing tokens defined in a CFG.  The CFG is written in an YAML
format so that we don't need to build another parser for the input lexical grammar.

`bee-dfagen` doesn't generate a lexer code directly.  Instead, it generates a definition of a DFA
in a JSON format.  Using the generated JSON data, you can easily generate a desired lexer code by
writing a small script or template files processed by a template engine such as `Junja`.

You don't need to use it in most cases, but this package supplementarily provides a library crate.
