# Specify `-d` if you want to see debug logs.
TRANSPILE_ARGS ?=

BEE_TOOLS := ../../../tools/bin/bee-tools

CODEGEN_TARGETS := \
  lexer/es2022.lex.yaml \
  parser/es2022.syn.yaml

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)
	@$(MAKE) -s -C lexer codegen

# Usually, we define targets in descending order in the dependency tree order.
# However, we define targets here in the reverse order in order to explain the
# code generation steps.

# 1. Download an ECMA-262 specification written in HTML.

.PRECIOUS: es2022.spec.html
es2022.spec.html: FORCE
	@echo 'Updating $(abspath $@)...'
	@curl https://raw.githubusercontent.com/tc39/ecma262/es2022/spec.html -sG | \
	  $(BEE_TOOLS) update-file $@

# 2. Extract the lexical and syntactic grammars from the specification
#
# The `bee-esgrammar` crate will be built and executed.  Texts contained in
# special tags will be output to STDOUT.
#
# We learned this approach from mozilla-spidermonkey/jsparagus.
# See js_parser/extract_es_grammar.py.

.PRECIOUS: lexer/es2022.lex.txt
lexer/es2022.lex.txt: es2022.spec.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -q --package bee-esgrammar extract lexical-grammar >$@

.PRECIOUS: parser/es2022.syn.txt
parser/es2022.syn.txt: es2022.spec.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -q --package bee-esgrammar extract syntactic-grammar >$@

# 3. Transpile the extracted grammars into an equivalent but more
#    computer-friendly data
#
# The ECMA-262 specification defines the grammar with CFG and uses special
# notations like below:
#
#   DecimalDigits[Sep] ::
#     DecimalDigit
#     DecimalDigits[?Sep] DecimalDigit
#     [+Sep] DecimalDigits[+Sep] NumericLiteralSeparator DecimalDigit
#
# Those special notations are very useful for writing the specification but a
# little bit complicated as an input for our code generator.
#
# `transpile.js` interprets a grammar and translates production rules including
# special notations into equivalent production rules.  `transpile.js` adds some
# production rules for convenience in further processing.

.PRECIOUS: lexer/es2022.lex.yaml
lexer/es2022.lex.yaml: lexer/es2022.lex.txt transpile.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run transpile.js -g lexical $(TRANSPILE_ARGS) >$@

.PRECIOUS: parser/es2022.syn.yaml
parser/es2022.syn.yaml: parser/es2022.syn.txt transpile.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run transpile.js -g syntactic $(TRANSPILE_ARGS) >$@

.PHONY: FORCE
FORCE:
