SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := evaluate.rs
CLEAN_TARGETS := $(CODEGEN_TARGETS) evaluate.json

EVALUATE_TESTS := $(wildcard scripts/*.js) $(wildcard modules/*.mjs)

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CLEAN_TARGETS)

evaluate.rs: evaluate.rs.hbs evaluate.json $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape $< evaluate.json | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: evaluate.json
evaluate.json: $(EVALUATE_TESTS) evaluate.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. evaluate.js $(EVALUATE_TESTS) >$@
