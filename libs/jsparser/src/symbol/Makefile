SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := builtins.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

builtins.rs: builtins.yaml builtins.rs.njk $(TOOLS_BIN)/nunjucks.js $(TOOLS_BIN)/update_file.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js builtins.rs.njk | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
