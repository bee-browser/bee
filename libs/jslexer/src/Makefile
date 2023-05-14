# Specify `-d` if you want to see debug logs.
TRANSPILE_ARGS ?=
DFAGEN_ARGS ?=

BEE_TOOLS := ../../../tools/bin/bee-tools

GOAL_SYMBOLS := $(shell \
  cat tokens.yaml | \
  bee-tools y2j | jq -r '.goals | keys | join(" ")' \
)

DFA_FILES := dfa/mod.rs $(patsubst %,dfa/%.rs,$(GOAL_SYMBOLS))

# Lazy evaluation.
# The following variable will be evaluated in substitution time.
# The definition order of tokens will be preserved.
LIST_TOKENS = \
  cat tokens.yaml | \
  $(BEE_TOOLS) y2j | \
  jq -r '.goals.$(patsubst dfa/%.json,%,$@) | flatten | join(" ")'

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(DFA_FILES) goals.rs tokens.rs

# Usually, we define targets in descending order in the dependency tree order.
# However, we define targets here in the reverse order in order to explain the
# code generation steps.

# 1. Download an ECMA-262 specification
.PRECIOUS: es2022.spec.html
es2022.spec.html:
	@echo 'Downloading $(abspath $@)...'
	@curl https://raw.githubusercontent.com/tc39/ecma262/es2022/spec.html -sG >$@

# 2. Extract the lexical grammar from the specification
#
# The `xtask` crate will be built and executed.  Texts contained in special tags
# will be output to STDOUT.
#
# We learned this approach from mozilla-spidermonkey/jsparagus.
# See js_parser/extract_es_grammar.py.
es2022.lex.txt: es2022.spec.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -q --package xtask esgrammar >$@

# 3. Transpile the extracted lexical grammar into an equivalent but more
#    computer-friendly data
#
# The ECMA-262 specification defines the lexical grammar with CFG and uses
# special notations like below:
#
#   DecimalDigits[Sep] ::
#     DecimalDigit
#     DecimalDigits[?Sep] DecimalDigit
#     [+Sep] DecimalDigits[+Sep] NumericLiteralSeparator DecimalDigit
#
# Those special notations are very useful for writing the specification but a
# little bit complicated as an input for our code generator.
#
# `transpile.js` interprets the lexical grammar and translates production rules
# including special notations into equivalent production rules.  `transpile.js`
# adds some production rules for convenience in further processing.
.PRECIOUS: es2022.lex.yaml
es2022.lex.yaml: es2022.lex.txt transpile.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run transpile.js $(TRANSPILE_ARGS) >$@

# 4. Build a minimized DFA for each goal symbol defined in the lexical grammar
#
# Many existing lexer generators use regular expressions in order to define
# tokens.  However, the ECMA-262 specification uses CFG.  In general, CFG can
# define larger languages than regular grammar.  So, well-define CFG for a
# lexical grammar can be converted to a regular grammar that defines the same
# lexical grammar.  If we want to an existing lexer generator such as flex, we
# have to convert CFG to regular expressions in some way.  However, this is
# inefficient.
#
# It gets clear that CFG and NFA are very similar when comparing the structure
# of them.  This means that we can build a NFA directly from CFG without
# converting production rules to regular expressions.
#
# `dfagen.js` reads the lexical grammar and builds an intermediate large NFA
# which recognizes specified tokens.  Then, `dfagen.js` converts the NFA to an
# equivalent minimized DFA by using well-known algorithms.  Finally, `dfagen.js`
# outputs the DFA in a JSON format which contains enough information for code
# generation.
.PRECIOUS: dfa/%.json
dfa/%.json: es2022.lex.yaml tokens.yaml dfagen.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run dfagen.js $(DFAGEN_ARGS) $(shell $(LIST_TOKENS)) | jq '.' >$@

# 5. Generate code for each DFA
dfa/%.rs: dfa/%.json dfa/dfa.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen dfa/dfa.rs.hbs $< | rustfmt --emit=stdout >$@

# 6. Generate a wrapper function
#
# A wrapper function that selects a DFA according to the specified goal symbol.
# Goal symbols are defined in `tokens.yaml` and extracted from it by using `jq`.
dfa/mod.rs: dfa/mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | $(BEE_TOOLS) y2j | jq -r '.goals | keys' | \
	  $(BEE_TOOLS) codegen --input-stdin $< | rustfmt --emit=stdout >$@

# 7. Generate an enum type for goal symbols
goals.rs: goals.rs.hbs tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | $(BEE_TOOLS) y2j | jq -r '.goals | keys' | \
	  $(BEE_TOOLS) codegen --input-stdin $< | rustfmt --emit=stdout >$@

# 8. Collect tokens from tokens.yaml
#
# We collect union of token used in each goal symbols.  The `unique` filter
# doesn't preserve the definition order of tokens, but it's OK because
# `tokens.json` is used only for generating an enum type.
.PRECIOUS: tokens.json
tokens.json: tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat $< | $(BEE_TOOLS) y2j | jq '[.goals[]] | flatten | unique' >$@

# 9. Generate an enum type for tokens.
tokens.rs: tokens.json tokens.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen tokens.rs.hbs $< | rustfmt --emit=stdout >$@
