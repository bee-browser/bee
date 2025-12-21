SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := builtin.rs
CLEAN_TARGETS := $(CODEGEN_TARGETS) builtin.json

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CLEAN_TARGETS)

builtin.rs: builtin.json builtin.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape $@.hbs $< | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: builtin.json
builtin.json: builtin.js builtin.yaml
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $^ >$@
