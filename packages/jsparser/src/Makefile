# Specify `-d` if you want to see debug logs.
TRANSPILE_ARGS ?=

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

ECMA262_VERSION := es2022

CODEGEN_TARGETS := \
  lexer/grammar.yaml \
  parser/grammar.yaml

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)
	@$(MAKE) -s -C lexer codegen
	@$(MAKE) -s -C parser codegen

# Usually, we define targets in descending order in the dependency tree order.  However, we define
# targets here in the reverse order in order to explain the code generation steps.

# 1. Download an ECMA-262 specification written in HTML.

.PRECIOUS: ecma262.html
ecma262.html:
	@echo 'Updating $(abspath $@)...'
	@curl https://raw.githubusercontent.com/tc39/ecma262/$(ECMA262_VERSION)/spec.html -sG | \
	  deno run -qA $(TOOLS_BIN)/update_file.js $@

# 2. Extract the lexical and syntactic grammars from the specification
#
# The `bee-esgrammar` crate will be built and executed.  Texts contained in special tags will be
# output to STDOUT.
#
# We learned this approach from mozilla-spidermonkey/jsparagus.
# See js_parser/extract_es_grammar.py.

.PRECIOUS: lexer/grammar.txt
lexer/grammar.txt: ecma262.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -q --package bee-esgrammar extract lexical-grammar >$@

.PRECIOUS: parser/grammar.txt
parser/grammar.txt: ecma262.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -q --package bee-esgrammar extract syntactic-grammar >$@

# 3. Transpile the extracted grammars into an equivalent but more computer-friendly data
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

.PRECIOUS: lexer/grammar.yaml
lexer/grammar.yaml: lexer/grammar.txt transpile.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run transpile.js -g lexical $(TRANSPILE_ARGS) >$@

.PRECIOUS: parser/grammar.yaml
parser/grammar.yaml: parser/grammar.txt transpile.js lexer/tokens.json
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run --allow-read transpile.js -g syntactic -t lexer/tokens.json $(TRANSPILE_ARGS) >$@
