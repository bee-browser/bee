PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin
CODEGEN_DIR := $(PROJ_DIR)/target/codegen/packages/jsparser/dfagen

GOAL_SYMBOLS := $(shell \
  cat tokens.yaml | \
  yq -o=json | \
  jq -r '[.goals[].name] | join(" ")' \
)

CODEGEN_TARGETS := \
  goals.rs \
  tokens.rs \
  dfa/mod.rs \
  $(patsubst %,dfa/%.rs,$(GOAL_SYMBOLS))

# Lazy evaluation.
# The following variable will be evaluated in substitution time.
# The definition order of tokens will be preserved.
LIST_TOKENS = \
  cat tokens.yaml | \
  yq -o=json | \
  jq -r '.goals[] | select(.name == "$(patsubst $(CODEGEN_DIR)/%.json,%,$@)") | .tokens | flatten | join(" ")'

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

# Usually, we define targets in descending order in the dependency tree order.  However, we define
# targets here in the reverse order in order to explain the code generation steps.

# 1. Build a minimized DFA for each goal symbol defined in the lexical grammar
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
# `bee-dfagen` reads the lexical grammar and builds an intermediate large NFA which recognizes
# specified tokens.  Then, `bee-dfagen` converts the NFA to an equivalent minimized DFA by using
# well-known algorithms.  Finally, `bee-dfagen` outputs the DFA in a JSON format which contains
# enough information for code generation.
.PRECIOUS: $(CODEGEN_DIR)/%.json
$(CODEGEN_DIR)/%.json: grammar.yaml tokens.yaml | $(CODEGEN_DIR)
	@echo 'Generating $(abspath $@)...'
	@cargo run -rqp bee-dfagen -- -g $< $(shell $(LIST_TOKENS)) | jq -c '.' >$@

# 2. Generate code for each DFA
dfa/%.rs: $(CODEGEN_DIR)/%.json dfa.js dfa/dfa.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  deno run dfa.js | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --no-escape --input-stdin dfa/dfa.rs.hbs | \
	  rustfmt --emit=stdout >$@

# 3. Generate a wrapper function
#
# A wrapper function that selects a DFA according to the specified goal symbol.  Goal symbols are
# defined in `tokens.yaml` and extracted from it by using `jq`.
dfa/mod.rs: dfa/mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | yq -o=json | jq -r '[.goals[].name]' | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --input-stdin $< | \
	  rustfmt --emit=stdout >$@

# 4. Generate an enum type for goal symbols
goals.rs: goals.rs.hbs tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | yq -o=json | jq -r '[.goals[].name]' | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --input-stdin $< | \
	  rustfmt --emit=stdout >$@

# 5. Collect tokens from tokens.yaml
#
# We collect union of token used in each goal symbols.  The `unique` filter doesn't preserve the
# definition order of tokens, but it's OK because `tokens.json` is used only for generating an enum
# type.
.PRECIOUS: tokens.json
tokens.json: tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | jq '[.goals[].tokens] | flatten | unique' >$@

# 6. Generate an enum type for tokens.
tokens.rs: tokens.json tokens.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@deno run -qA $(TOOLS_BIN)/codegen.js tokens.rs.hbs $< | \
	  rustfmt --emit=stdout >$@

$(CODEGEN_DIR):
	@mkdir -p $@
