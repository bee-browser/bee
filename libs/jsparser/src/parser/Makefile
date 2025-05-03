SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LOGGERGEN_SH := $(PROJ_DIR)/libs/logging/scripts/loggergen.sh
LOGGERGEN_DEPS := $(shell sh $(LOGGERGEN_SH) deps)
ECMA262_DIR := $(PROJ_DIR)/vendor/src/tc39/ecma262
TOKENS_JSON := ../lexer/tokens.json

CODEGEN_FILES := \
  grammar.txt \
  grammar.yaml \
  lalr/action.rs \
  lalr/debug.rs \
  lalr/goal_symbols.rs \
  lalr/goto.rs \
  lalr/lexical_goal.rs \
  lalr/non_terminals.rs \
  lalr/auto_semicolon.rs

CODEGEN_TARGETS := $(filter %.rs,$(CODEGEN_FILES))

LOGGERGEN_TARGETS := logger.rs

# The syntactic grammar in the ECMA-262 specification adopts *permissive* production rules in order
# to avoid shift-shift/reduce conflicts.  There are some production rules that we have to parsing
# it again with a correct production rule.  See the supplemental syntax for cover rules such as
# `CoverParenthesizedExpressionAndArrowParameterList`.
#
# The following variable holds true goal symbols (Script and Module) and others which are used for
# refinements of permissive production rules.
GOAL_SYMBOLS := \
  Script \
  Module \
  ArrowFormalParameters \
  ArrowFormalParameters_Yield \
  ArrowFormalParameters_Await \
  ArrowFormalParameters_Yield_Await \
  AsyncArrowHead

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: loggergen
loggergen: $(LOGGERGEN_TARGETS)

.PHONY: clean
clean:
	@rm -fr $(CODEGEN_FILES) $(LOGGERGEN_TARGETS) lalrgen

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
	@cat $< | cargo run -rqp esgrammar -- extract syntactic-grammar >$@

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
grammar.yaml: grammar.txt ../transpile.js $(TOKENS_JSON)
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  deno run --allow-read ../transpile.js -g syntactic -t $(TOKENS_JSON) $(EXTRA_ARGS) >$@

# 3. Generate an intermediate JSON data in order to avoid re-running lalrgen when lalr.js or
# lalr.rs.hbs changes.
.PRECIOUS: lalrgen/lalr.json
lalrgen/lalr.json: grammar.yaml
	@echo 'Generating $(abspath $@)...'
	@mkdir -p lalrgen
	@cargo run -rqp lalrgen -- -r lalrgen $< $(GOAL_SYMBOLS) | jq '.' >$@

# 4. Generate LALR parsing tables.
lalr/%.rs: lalrgen/lalr.json lalr/%.rs.hbs lalr.js $(TOKENS_JSON)
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  deno run --allow-read=.. lalr.js $(TOKENS_JSON) | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --input-stdin --no-escape $@.hbs | \
	  rustfmt --emit=stdout >$@

logger.rs: $(LOGGERGEN_DEPS)
	@echo 'Generating $(abspath $@)...'
	@sh $(LOGGERGEN_SH) codegen '{"module":"parser","target":"bee::jsparser::parser"}' >$@
