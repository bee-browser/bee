BEE_TOOLS := ../../../../tools/bin/bee-tools
SRCS := tags.rs svgattrs.rs

.PHONY: codegen
codegen: $(SRCS)

%.rs: %.yaml %.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | \
	  yq -o=json | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $@.hbs | \
	  rustfmt --emit=stdout >$@
