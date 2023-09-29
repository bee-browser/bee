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

1. Create a new production rule called `DecimalDigits.1` from `DecimalDegits`
2. Replace `DecimalDigits` in the first production of `LookaheadExample` with `DecimalDigits.1`
3. Move the non-tail lookahead restriction into the head of each production of `DecimalDigits.1`
4. Create a new production rule called `DecimalDigit.2` from `DecimalDegit`
5. Replace `DecimalDigit` in the first production of `DecimalDigits.2` with `DecimalDigit.2`
6. Move the non-tail lookahead restriction into the head of each production of `DecimalDigit.2`
7. Remove `1`, `3`, `5`, `7`, `9` from `DecimalDigit.2` which do not meet the non-tail lookahead
   restriction and remove the non-tail lookahead restriction from the remaining productions
8. Replace `DecimalDigits` in the second production with `DecimalDigits.1` and remove the non-tail
   lookahead restriction from the second production

Finally, the following grammar is obtained:

```text
LookaheadExample ::
  `n` DecimalDigits.1
  DecimalDigit [lookahead ∉ DecimalDigit]

DecimalDigits ::
  DecimalDigit
  DecimalDigits DecimalDigit

DecimalDigit :: one of
  `0` `1` `2` `3` `4` `5` `6` `7` `8` `9`

DecimalDigits.1 ::
  DecimalDigit.2
  DecimalDigits.1 DecimalDigit

DecimalDigit.2 :: one of
  `0` `2` `4` `6` `8`
```

Production rules for `DecimalDigits.1` and `DecimalDigit.2` are internal *variant* rules.  Variant
rules are used only in closure computations for restricted production rules.  And non-terminal
symbols in a variant rule of an LR item are converted to corresponding *original* symbols before
adding the LR item to the closure item set.  So, non-terminal variants never appear in the list of
non-terminal symbols in [`LalrSpec`].

## How does bee-lalrgen process [no LineTerminator here] restrictions in production rules?

`[no LineTerminator here]` is a special notation in the ECMA-262 specification.  It's used in a
production rule in order to indicate that `LineTerminator` (and `LineTerminatorSequence`) tokens
cannot be acceptable at this location in the production rule.  If such a situation happens, the
production rule should be ignored.  This requires a LR parser to switch to another state which
doesn't contain the LR item for the production rule.

For representing `[no LineTerminator here]`, the [`Term::Disallow`] variant is added.  For example,
the following production rule:

```text
ThrowStatement :
  throw [no LineTerminator here] Expression ;
```

should be translated into:

```yaml
name: ThrowStatement
production:
  - type: token
    data: THROW
  - type: disallow
    data: LineTerminatorSequence
  - type: non-terminal
    data: Expression
  - type: token
    data: SEMICOLON
```

A [`Term::Disallow`] term is treated as an empty term in the closure computation.  It affects the
LR(0) automaton and the LALR parsing table generation processes.

In the LR(0) automaton generation process, a state having *restricted* LR items will generate a
special transition caused by a disallowed token.  This transition creates a new state which has no
*restricted* LR item regarding the disallowed token.  As a result, the size of LR(0) automaton
increases.

In the LALR parsing table generation process, a transition caused by a disallowed token will
generate an [`LalrAction::Replace`] action.  An [`LalrAction::Replace`] action will replace the
state on the top of the parsing state stack of a LALR parser.  Unlike an `LalrShift` action, it
doesn't change the size of the parsing state stack.  A lookahead token sequence for each
*non-restricted* LR(0) item in a *restricted* state should be propagated to the corresponding LR(0)
item in the state transitioned by the disallowed token.  See [`lalr::build_lookahead_tables()`] for
details.

An [`LalrAction::Replace`] action must be implicitly performed if a state after an
[`LalrAction::Reduce`] action has a transition caused by the last processed token and the token is
one of disallowed tokens in the state.
