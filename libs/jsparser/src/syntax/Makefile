SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LALR_JSON := ../parser/lalr.json

CODEGEN_TARGETS := actions.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

actions.rs: actions.rs.njk actions.yaml $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@cat actions.yaml | yq -o=json | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< | \
	  rustfmt --emit=stdout >$@

actions.yaml: $(LALR_JSON) actions.js
	@echo 'Updating $(abspath $@)...'
	@deno run -q --allow-read=$(PROJ_DIR) actions.js $< $@ >$@.new
	@mv -f $@.new $@
