BEE_TOOLS := ../../../tools/bin/bee-tools
SRCS := mod.rs

.PHONY: codegen
codegen: $(SRCS)

mod.rs: local_names.yaml mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) deepmerge -i yaml -o json $< | \
	  $(BEE_TOOLS) codegen --input-stdin mod.rs.hbs | \
	  rustfmt --emit=stdout >$@
