SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

NATIVE_ERROR_NAMES := eval_error range_error reference_error syntax_error type_error uri_error

CODEGEN_TARGETS := $(addsuffix .rs,$(NATIVE_ERROR_NAMES))

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

eval_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "eval_error" }' | \
	  rustfmt --emit=stdout >$@

range_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "range_error" }' | \
	  rustfmt --emit=stdout >$@

reference_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "reference_error" }' | \
	  rustfmt --emit=stdout >$@

syntax_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "syntax_error" }' | \
	  rustfmt --emit=stdout >$@

type_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "type_error" }' | \
	  rustfmt --emit=stdout >$@

uri_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "uri_error" }' | \
	  rustfmt --emit=stdout >$@
