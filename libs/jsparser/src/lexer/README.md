# lexer

The `lexer` module implements a JavaScript lexer.

The ECMA-262 specification defines multiple goal symbols.  Each goal symbol is used in a particular
syntactic context.  `bee-dfagen` is used for generating a DFA for each goal symbol.

`dfa.js` converts the generated DFA into more convenient data for Rust source file generation.  The
generated Rust source files are placed in the `dfa` module inside the `lexer` module.  The current
goal symbol of the lexer can be switched by `Lexer::set_goal()`.

The `Lexer` is implemented in a pull-style.  `Lexer::next_token()` returns a token which is
available until `Lexer::consume_token()` is called.  `Lexer::consume_token()` must be called before
the next `Lexer::next_token()` call.

The `Token` has no location information.  Its location information can be obtained from the `Lexer`
by using `Lexer::location()`.  This is somewhat unergonomic but efficient because location
information is not always necessary.

## TODO

* [ ] Read characters from byte stream
  * Currently, the lexer uses a pre-converted UTF-8 string
  * Wrong surrogate pairs must be handled in the lexer (or the input stream implementation)
* [ ] Compress DFA tables
  * For example, we can simulate a DFA using the two NFAs
    * NFA recognizing tokens commonly used in the goal symbols
    * NFA recognizing tokens used in a particular goal symbol
* [ ] Consider replacing `Option<TokenKind>` with `TokenKind` in `ACCEPT_TABLE`
  * Adding `TokenKind::None` is needed for representing `None`
* [ ] Consider using bit flags in `LOOKAHEAD_TABLE`
  * Bitwise operations may increase the processing time
* [ ] Use some kind of pre-calculated table for non-ASCII characters in `UnicodeSet::from()`
