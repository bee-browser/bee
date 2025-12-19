SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := support.rs
CLEAN_TARGETS := $(CODEGEN_TARGETS) ../runtime.json

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CLEAN_TARGETS)

%.rs: ../runtime.json %.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: ../runtime.json
../runtime.json: ../runtime.js ../runtime.yaml
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=.. $^ >$@
