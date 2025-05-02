SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := builtin.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

builtin.rs: builtin.yaml builtin.rs.njk builtin.js $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=$< builtin.js $< | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js builtin.rs.njk | \
	  rustfmt --emit=stdout >$@
