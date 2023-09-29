BEE_TOOLS := ../../../../tools/bin/bee-tools
SRCS := tags.rs svgattrs.rs

.PHONY: codegen
codegen: $(SRCS)

%.rs: %.yaml %.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  $(BEE_TOOLS) y2j | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@
