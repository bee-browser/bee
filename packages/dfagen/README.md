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
writing a small script or template files processed by a template engine such as `Jinja`.

You don't need to use it in most cases, but this package supplementarily provides a library crate.

## How to process lookahead terms in lexical production rules

Generally, there are two approaches:

1. Use a sequence of characters (to be precise, a sequence of Unicode character sets) instead of a
   single character as an index of the state transition table of a DFA
2. Take lookahead terms into account when generating a DFA

`bee-dfagen` generates a DFA in the fist approach.

The first approach is intuitive, but it normally generates a larger state transition table than the
second one.  Because the number of states in a DFA is normally larger than the number of possible
sequences of Unicode character sets.  The first one increases the number of indexes of the state
transition table and it doesn't change the number of states in a DFA.  On the other hand, the
second one increases the number of states in a DFA and keeps the number of indexes of the state
transition table.

For example, the ES2022 lexical grammar contains the following production rules:

```text
TemplateMiddle ::
  `}` TemplateCharacters? `${`

TemplateCharacters ::
  TemplateCharacter TemplateCharacters?

TemplateCharacter ::
  `$` [lookahead != `{`]
  `\` TemplateEscapeSequence
  `\` NotEscapeSequence
  LineContinuation
  LineTerminatorSequence
  SourceCharacter but not one of ``` or `\` or `$` or LineTerminator
```

In the first approach, the generated DFA requires two characters as the input in order to
transition from a state for:

```text
TemplateMiddle -> [}] TemplateCharacters? [$] . [{]
TemplateCharacter -> [$] . ?![{]
```

to a next state for:

```text
# if the second character is `{`
TemplateMiddle -> [}] TemplateCharacters? [$] [{] .
```

or:

```text
# if the second character is not `{`
TemplateCharacter -> [$] . ?![{]
...
TemplateCharacter -> [(any) -` -\ -$ -LineTerminator] .
```

In the second approach, before building a NFA, the production rules are conceptually converted like
this:

```text
TemplateMiddle ::
  `}` TemplateCharacters#1? `${`

TemplateCharacters#1 ::
  TemplateCharacter#2 TemplateCharacters#3?

TemplateCharacter#2 ::
  `$`
  `\` TemplateEscapeSequence
  `\` NotEscapeSequence
  LineContinuation
  LineTerminatorSequence
  SourceCharacter but not one of ``` or `\` or `$` or LineTerminator

TemplateCharacters#3 ::
  TemplateCharacter#4 TemplateCharacters#3?

TemplateCharacter#4 ::
  `$`
  `\` TemplateEscapeSequence
  `\` NotEscapeSequence
  LineContinuation
  LineTerminatorSequence
  SourceCharacter but not one of ``` or `\` or `$` or LineTerminator or `{`
```

In this example, the lookahead term is completed removed, but generally lookahead terms remain in
the converted grammar.

There is no such term in the ES2022 lexical specification, but lookbehind terms cannot be processed
in the second approach in general.  Imagine what if there is a lookbehind term at the beginning of
the production rule of a token.  Probably, we need to introduce a new mechanism to process such
lookbehind terms.  For example, we have to use multiple DFAs or a DFA having multiple start states.
