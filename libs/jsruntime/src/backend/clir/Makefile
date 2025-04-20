SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := support.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

%:: ../runtime.yaml %.njk ../runtime.js $(TOOLS_BIN)/nunjucks.js $(TOOLS_BIN)/update_file.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=.. ../runtime.js $< | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $@.njk | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
