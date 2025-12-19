SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LALR_JSON := ../parser/lalrgen/lalr.json

CODEGEN_TARGETS := actions.rs
CLEAN_TARGETS := $(CODEGEN_TARGETS)

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CLEAN_TARGETS)

actions.rs: actions.yaml actions.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@

actions.yaml: $(LALR_JSON) actions.js
	@echo 'Updating $(abspath $@)...'
	@deno run -q --allow-read=$(PROJ_DIR) actions.js $< $@ >$@.new
	@mv -f $@.new $@
