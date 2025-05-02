SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := evaluate.rs

EVALUATE_TESTS := $(wildcard scripts/*.js) $(wildcard modules/*.mjs)

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

evaluate.rs: evaluate.rs.njk $(EVALUATE_TESTS) evaluate.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. evaluate.js $(EVALUATE_TESTS) | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< | \
	  rustfmt --emit=stdout >$@
