SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin
CODEGEN_DIR := $(PROJ_DIR)/target/codegen/libs/jsparser/lalrgen

CODEGEN_FILES := \
  action.rs \
  debug.rs \
  goal_symbols.rs \
  goto.rs \
  lexical_goal.rs \
  non_terminals.rs \
  auto_semicolon.rs

CODEGEN_TARGETS := $(addprefix lalr/,$(CODEGEN_FILES))

# The syntactic grammar in the ECMA-262 specification adopts *permissive* production rules in order
# to avoid shift-shift/reduce conflicts.  There are some production rules that we have to parsing
# it again with a correct production rule.  See the supplemental syntax for cover rules such as
# `CoverParenthesizedExpressionAndArrowParameterList`.
#
# The following variable holds true goal symbols (Script and Module) and others which are used for
# refinements of permissive production rules.
GOAL_SYMBOLS := Script Module #ArrowFormalParameters

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

lalr/%.rs: lalr/%.rs.hbs lalr.js $(CODEGEN_DIR)/lalr.json
	@echo 'Generating $(abspath $@)...'
	@cat $(CODEGEN_DIR)/lalr.json | \
	  deno run --allow-read lalr.js | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --input-stdin --no-escape $< | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

# Generate an intermediate JSON data in order to avoid re-running lalrgen when lalr.js or
# lalr.rs.hbs changes.
.PRECIOUS: $(CODEGEN_DIR)/lalr.json
$(CODEGEN_DIR)/lalr.json: grammar.yaml | $(CODEGEN_DIR)
	@echo 'Generating $(abspath $@)...'
	@cargo run -rqp lalrgen -- -r $(CODEGEN_DIR) $< $(GOAL_SYMBOLS) | jq '.' | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

$(CODEGEN_DIR):
	@mkdir -p $@
