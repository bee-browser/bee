SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := src/targets.rs

.PHONY: all
all:

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

src/targets.rs: FORCE
	@echo 'Updating $(abspath $@) if any of logging.yaml files changes...'
	@deno run -q --allow-read=$(PROJ_DIR) scripts/targets.js | \
	  deno run -q --allow-read=$(PROJ_DIR) $(TOOLS_BIN)/codegen.js --input-stdin --no-escape \
	    $@.hbs | \
	  rustfmt --emit=stdout >$@

.PHONY: FORCE
FORCE:
