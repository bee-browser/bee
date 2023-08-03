# bee-lalrgen

> A LALR parsing tables generator

The main goal of this crate is generating a LALR(1) parsing tables for the `bee-jsparser` crate
from a syntactic grammar defined in the ECMA-262 specification.

## How does bee-lalrgen process lookahead restrictions in production rules?

One of special notations in the ECMA-262 specification is **lookahead restrictions**.  For example:

```text
LookaheadExample :
  `n` [lookahead ∉ { `1`, `3`, `5`, `7`, `9` }] DecimalDigits
  DecimalDigit [lookahead ∉ DecimalDigit]

DecimalDigits :
  DecimalDigit
  DecimalDigits DecimalDigit

DecimalDigit : one of
  `0` `1` `2` `3` `4` `5` `6` `7` `8` `9`
```

The `LookaheadExample` matches either the letter n followed by one or more decimal digits the first
of which is even, or a decimal digit not followed by another decimal digit.

In the LALR(1) parsing tables generation algorithm, a lookahead restriction at the tail of a
production can be processed straightforwardly.  On the other hand, one not at the end of a
production is worth considering.  We call it a non-tail lookahead restriction in this crate.

There are two directions to process non-tail lookahead restrictions:

1. Transform a grammar into another grammar which has no non-tail lookahead restrictions before
   performing the LALR parsing tables generation algorithm
2. Apply a non-tail lookahead restriction when build a LR item set in the LALR parsing tables
   generation algorithm

Eventually, we select the first direction in this crate.

In the above example, we transform the grammar like below:

1. Create a new production rule called `DecimalDigits#1` from `DecimalDegits`
2. Replace `DecimalDigits` in the first production of `LookaheadExample` with `DecimalDigits#1`
3. Move the non-tail lookahead restriction into the head of each production of `DecimalDigits#1`
4. Create a new production rule called `DecimalDigit#2` from `DecimalDegit`
5. Replace `DecimalDigit` in the first production of `DecimalDigits#2` with `DecimalDigit#2`
6. Move the non-tail lookahead restriction into the head of each production of `DecimalDigit#2`
7. Remove `1`, `3`, `5`, `7`, `9` from `DecimalDigit#2` which do not meet the non-tail lookahead
   restriction and remove the non-tail lookahead restriction from the remaining productions
8. Replace `DecimalDigits` in the second production with `DecimalDigits#1` and remove the non-tail
   lookahead restriction from the second production

Finally, the following grammar is obtained:

```text
LookaheadExample ::
  `n` DecimalDigits#1
  DecimalDigit [lookahead ∉ DecimalDigit]

DecimalDigits ::
  DecimalDigit
  DecimalDigits DecimalDigit

DecimalDigit :: one of
  `0` `1` `2` `3` `4` `5` `6` `7` `8` `9`

DecimalDigits#1 ::
  DecimalDigit#2
  DecimalDigits.1 DecimalDigit

DecimalDigit#2 :: one of
  `0` `2` `4` `6` `8`
```

Production rules for `DecimalDigits#1` and `DecimalDigit#2` are internal *variant* rules.  Variant
rules are used only in closure computations for restricted production rules.  And non-terminal
symbols in a variant rule of an LR item are converted to corresponding *grammatical* symbols before
adding the LR item to the closure item set.  So, non-terminal variants never appear in the list of
non-terminal symbols in [`LalrSpec`].  See the [`closure`] module for details.
