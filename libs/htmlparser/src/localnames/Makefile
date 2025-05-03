SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

SRCS := tags.rs svgattrs.rs

.PHONY: codegen
codegen: $(SRCS)

.PHONY: clean
clean:
	@rm -f $(SRCS)

%.rs: %.yaml %.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --no-escape --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@
