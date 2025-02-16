# Specify `-d` if you want to see debug logs.
TRANSPILE_ARGS ?=

SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin
ECMA262_DIR := $(PROJ_DIR)/vendor/src/tc39/ecma262
CODEGEN_DIR := $(PROJ_DIR)/target/codegen/libs/jsparser/lalrgen

CODEGEN_FILES := \
  action.rs \
  debug.rs \
  goal_symbols.rs \
  goto.rs \
  lexical_goal.rs \
  non_terminals.rs \
  auto_semicolon.rs

CODEGEN_TARGETS := \
  grammar.txt \
  grammar.yaml \
  $(addprefix lalr/,$(CODEGEN_FILES))

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

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

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
	@cat $< | cargo run -rqp esgrammar -- extract syntactic-grammar | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

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
grammar.yaml: grammar.txt ../transpile.js ../lexer/tokens.json
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  deno run --allow-read ../transpile.js -d -g syntactic -t ../lexer/tokens.json \
	    $(TRANSPILE_ARGS) | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

# 3. Generate an intermediate JSON data in order to avoid re-running lalrgen when lalr.js or
# lalr.rs.hbs changes.
.PRECIOUS: $(CODEGEN_DIR)/lalr.json
$(CODEGEN_DIR)/lalr.json: grammar.yaml | $(CODEGEN_DIR)
	@echo 'Generating $(abspath $@)...'
	@cargo run -rqp lalrgen -- -r $(CODEGEN_DIR) $< $(GOAL_SYMBOLS) | jq '.' | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

# 4. Generate LALR parsing tables.
lalr/%.rs: lalr/%.rs.hbs lalr.js $(CODEGEN_DIR)/lalr.json
	@echo 'Generating $(abspath $@)...'
	@cat $(CODEGEN_DIR)/lalr.json | \
	  deno run --allow-read lalr.js | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --input-stdin --no-escape $< | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

$(CODEGEN_DIR):
	@mkdir -p $@
