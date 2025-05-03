SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LOGGERGEN_SH := $(PROJ_DIR)/libs/logging/scripts/loggergen.sh
LOGGERGEN_DEPS := $(shell sh $(LOGGERGEN_SH) deps)
ECMA262_DIR := $(PROJ_DIR)/vendor/src/tc39/ecma262

GOAL_SYMBOLS := $(shell \
  cat tokens.yaml | \
  yq -o=json | \
  jq -r '[.goals[].name] | join(" ")' \
)

CODEGEN_FILES := \
  grammar.txt \
  grammar.yaml \
  goals.rs \
  tokens.json \
  tokens.rs \
  dfa/mod.rs \
  $(patsubst %,dfa/%.rs,$(GOAL_SYMBOLS))

CODEGEN_TARGETS := $(filter %.rs,$(CODEGEN_FILES))

LOGGERGEN_TARGETS := logger.rs

# Lazy evaluation.
# The following variable will be evaluated in substitution time.
# The definition order of tokens will be preserved.
LIST_TOKENS = \
  cat tokens.yaml | \
  yq -o=json | \
  jq -r '.goals[] | select(.name == "$(patsubst dfagen/%.json,%,$@)") | .tokens | flatten | join(" ")'

# targets

.PHONY: all
all: codegen

# Generate DFAs.
#
# We don't use build.rs for the code generation.  We generate DFAs in development time and commit
# the generated files to the repository.
#
# DFAs has no compilation flag at this time.  If we use build.rs for the code generation, always
# the same DFAs will be generated on every build as long as the lexical grammar and/or scripts to
# generate code are changed.  Therefore, using build.rs is inefficient in our use case.
.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: loggergen
loggergen: $(LOGGERGEN_TARGETS)

.PHONY: clean
clean:
	@rm -fr $(CODEGEN_FILES) $(LOGGERGEN_TARGETS) dfagen

# Usually, we define targets in descending order in the dependency tree order.  However, we define
# targets here in the reverse order in order to explain the code generation steps.

# 1. Extract the lexical and syntactic grammars from the specification
#
# The `esgrammar` crate will be built and executed.  Texts contained in special tags will be output
# to STDOUT.
#
# We learned this approach from mozilla-spidermonkey/jsparagus.
# See js_parser/extract_es_grammar.py.
.PRECIOUS: grammar.txt
grammar.txt: $(ECMA262_DIR)/spec.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -rqp esgrammar -- extract lexical-grammar >$@

# 2. Transpile the extracted grammars into an equivalent but more computer-friendly data
#
# The ECMA-262 specification defines the grammar with CFG and uses special notations like below:
#
#   DecimalDigits[Sep] ::
#     DecimalDigit
#     DecimalDigits[?Sep] DecimalDigit
#     [+Sep] DecimalDigits[+Sep] NumericLiteralSeparator DecimalDigit
#
# Those special notations are very useful for the programming language designers but a little bit
# complicated as an input for code generators.
#
# `transpile.js` interprets a grammar and translates production rules including special notations
# into equivalent production rules.  `transpile.js` adds some production rules for convenience in
# further processing.
.PRECIOUS: grammar.yaml
grammar.yaml: EXTRA_ARGS := $(BEE_BUILD_JSPARSER_TRANSPILE_EXTRA_ARGS)
grammar.yaml: grammar.txt ../transpile.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run ../transpile.js -g lexical $(EXTRA_ARGS) >$@

# 3. Build a minimized DFA for each goal symbol defined in the lexical grammar
#
# Many existing lexer generators use regular expressions in order to define tokens.  However, the
# ECMA-262 specification uses CFG.  In general, CFG can define larger languages than regular
# grammar.  So, well-define CFG for a lexical grammar can be converted to a regular grammar that
# defines the same lexical grammar.  If we want to an existing lexer generator such as flex, we
# have to convert CFG to regular expressions in some way.  However, this is inefficient.
#
# It gets clear that CFG and NFA are very similar when comparing the structure of them.  This
# means that we can build a NFA directly from CFG without converting production rules to regular
# expressions.
#
# `dfagen` reads the lexical grammar and builds an intermediate large NFA which recognizes
# specified tokens.  Then, `dfagen` converts the NFA to an equivalent minimized DFA by using
# well-known algorithms.  Finally, `dfagen` outputs the DFA in a JSON format which contains enough
# information for code generation.
.PRECIOUS: dfagen/%.json
dfagen/%.json: grammar.yaml tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@mkdir -p dfagen
	@cargo run -rqp dfagen -- -g $< $(shell $(LIST_TOKENS)) | jq '.' >$@

# 4. Generate code for each DFA
dfa/%.rs: dfagen/%.json dfa/dfa.rs.hbs dfa.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run dfa.js | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --no-escape --input-stdin dfa/dfa.rs.hbs | \
	  rustfmt --emit=stdout >$@

# 5. Generate a wrapper function
#
# A wrapper function that selects a DFA according to the specified goal symbol.  Goal symbols are
# defined in `tokens.yaml` and extracted from it by using `jq`.
dfa/mod.rs: tokens.yaml dfa/mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | jq -r '[.goals[].name]' | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@

# 6. Generate an enum type for goal symbols
goals.rs: tokens.yaml goals.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | jq -r '[.goals[].name]' | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@

# 7. Collect tokens from tokens.yaml
#
# We collect union of token used in each goal symbols.  The `unique` filter doesn't preserve the
# definition order of tokens, but it's OK because `tokens.json` is used only for generating an enum
# type.
.PRECIOUS: tokens.json
tokens.json: tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | jq '[.goals[].tokens] | flatten | unique' >$@

# 8. Generate an enum type for tokens.
tokens.rs: tokens.json tokens.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@

logger.rs: $(LOGGERGEN_DEPS)
	@echo 'Generating $(abspath $@)...'
	@sh $(LOGGERGEN_SH) codegen '{"module":"lexer","target":"bee::jsparser::lexer"}' >$@
