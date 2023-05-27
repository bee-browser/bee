BEE_TOOLS := ../../../../tools/bin/bee-tools

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

# Usually, we define targets in descending order in the dependency tree order.
# However, we define targets here in the reverse order in order to explain the
# code generation steps.

# 1. Build a minimized DFA for each goal symbol defined in the lexical grammar
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
	@cat $< | deno run dfagen.js $(LEXER_DFAGEN_ARGS) $(shell $(LIST_TOKENS)) | \
	  jq '.' >$@

# 2. Generate code for each DFA
dfa/%.rs: dfa/%.json dfa/dfa.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen dfa/dfa.rs.hbs $< | rustfmt --emit=stdout >$@

# 3. Generate a wrapper function
#
# A wrapper function that selects a DFA according to the specified goal symbol.
# Goal symbols are defined in `tokens.yaml` and extracted from it by using `jq`.
dfa/mod.rs: dfa/mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | $(BEE_TOOLS) y2j | jq -r '.goals | keys' | \
	  $(BEE_TOOLS) codegen --input-stdin $< | rustfmt --emit=stdout >$@

# 4. Generate an enum type for goal symbols
goals.rs: goals.rs.hbs tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | $(BEE_TOOLS) y2j | jq -r '.goals | keys' | \
	  $(BEE_TOOLS) codegen --input-stdin $< | rustfmt --emit=stdout >$@

# 5. Collect tokens from tokens.yaml
#
# We collect union of token used in each goal symbols.  The `unique` filter
# doesn't preserve the definition order of tokens, but it's OK because
# `tokens.json` is used only for generating an enum type.
.PRECIOUS: tokens.json
tokens.json: tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat $< | $(BEE_TOOLS) y2j | jq '[.goals[]] | flatten | unique' >$@

# 6. Generate an enum type for tokens.
tokens.rs: tokens.json tokens.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen tokens.rs.hbs $< | rustfmt --emit=stdout >$@
