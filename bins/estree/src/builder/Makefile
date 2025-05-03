SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LALR_JSON := $(PROJ_DIR)/libs/jsparser/src/parser/lalrgen/lalr.json

CODEGEN_TARGETS := actions.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

actions.rs: actions.yaml actions.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --input-stdin --no-escape $@.hbs | \
	  rustfmt --emit=stdout >$@

actions.yaml: actions.js $(LALR_JSON)
	@echo 'Updating $(abspath $@)...'
	@deno run -A actions.js $(LALR_JSON) $@ >$@.new
	@mv -f $@.new $@

$(LALR_JSON):
	@$(MAKE) -s -C $(PROJ_DIR)/libs/jsparser codegen
