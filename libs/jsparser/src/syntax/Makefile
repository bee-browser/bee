SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LALR_JSON := $(PROJ_DIR)/target/codegen/libs/jsparser/lalrgen/lalr.json

CODEGEN_TARGETS := actions.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

actions.rs: actions.yaml actions.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js actions.rs.njk | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

actions.yaml: actions.js $(LALR_JSON)
	@echo 'Updating $(abspath $@)...'
	@deno run -q --allow-read=$(PROJ_DIR) actions.js update $(LALR_JSON) $@ | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
