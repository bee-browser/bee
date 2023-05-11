BEE_TOOLS := ../../../tools/bin/bee-tools

LIST_TOKENS = \
  cat tokens.yaml | \
  $(BEE_TOOLS) y2j | \
  jq -r '.goals.$(patsubst dfa/%.json,%,$@) | flatten | join(" ")'

GOAL_SYMBOLS := $(shell \
  cat tokens.yaml | \
  bee-tools y2j | jq -r '.goals | keys | join(" ")' \
)

DFA_FILES := dfa/mod.rs $(patsubst %,dfa/%.rs,$(GOAL_SYMBOLS))

# Specify `-d` if you want to see debug logs.
LEXGEN_ARGS ?=

.PHONY: codegen
codegen: goals.rs tokens.rs $(DFA_FILES)

goals.rs: goals.rs.hbs tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | $(BEE_TOOLS) y2j | jq -r '.goals | keys' | \
	  $(BEE_TOOLS) codegen --input-stdin $< | rustfmt --emit=stdout >$@

tokens.rs: tokens.json tokens.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen tokens.rs.hbs $< | rustfmt --emit=stdout >$@

dfa/mod.rs: dfa/mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat tokens.yaml | $(BEE_TOOLS) y2j | jq -r '.goals | keys' | \
	  $(BEE_TOOLS) codegen --input-stdin $< | rustfmt --emit=stdout >$@

dfa/%.rs: dfa/%.json dfa/dfa.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen dfa/dfa.rs.hbs $< | rustfmt --emit=stdout >$@

.PRECIOUS: tokens.json
tokens.json: tokens.yaml
	@echo 'Generating $(abspath $@)...'
	@cat $< | $(BEE_TOOLS) y2j | jq '[.goals[]] | flatten | unique' >$@

.PRECIOUS: dfa/%.json
dfa/%.json: es2022.lex.yaml tokens.yaml lexgen.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run lexgen.js $(LEXGEN_ARGS) $(shell $(LIST_TOKENS)) | jq '.' >$@

.PRECIOUS: es2022.lex.yaml
es2022.lex.yaml: es2022.lex.txt transpile.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run transpile.js >$@

es2022.lex.txt: es2022.spec.html
	@echo 'Generating $(abspath $@)...'
	@cat $< | cargo run -q --package xtask esgrammar >$@

.PRECIOUS: es2022.spec.html
es2022.spec.html:
	@echo 'Downloading $(abspath $@)...'
	@curl https://raw.githubusercontent.com/tc39/ecma262/es2022/spec.html -sG >$@
