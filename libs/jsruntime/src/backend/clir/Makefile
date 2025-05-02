SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := support.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

%.rs: %.rs.njk ../runtime.yaml ../runtime.js $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=.. ../runtime.js ../runtime.yaml | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< | \
	  rustfmt --emit=stdout >$@
