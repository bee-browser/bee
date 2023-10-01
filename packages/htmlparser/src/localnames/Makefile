PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

SRCS := tags.rs svgattrs.rs

.PHONY: codegen
codegen: $(SRCS)

%.rs: %.yaml %.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  yq -o=json | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --no-escape --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@
