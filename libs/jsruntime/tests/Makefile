SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := evaluate.rs

EVALUATE_TESTS := $(wildcard scripts/*.js) $(wildcard modules/*.mjs)

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

evaluate.rs: evaluate.rs.njk $(EVALUATE_TESTS) collect_evaluate_tests.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. collect_evaluate_tests.js $(EVALUATE_TESTS) | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $@.njk | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
