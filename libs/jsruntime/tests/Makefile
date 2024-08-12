SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := evaluate.rs

EVALUATE_TESTS := $(wildcard evaluate_*.js) $(wildcard evaluate_*.mjs)

.PHONY: codegen
codegen: evaluate.rs

evaluate.rs: evaluate.rs.njk $(EVALUATE_TESTS) collect_evaluate_tests.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. collect_evaluate_tests.js $(EVALUATE_TESTS) | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $@.njk | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
