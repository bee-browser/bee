BEE_TOOLS := ../../../../tools/bin/bee-tools
SRCS := doctype.rs comment.rs null.rs whitespace.rs text.rs eof.rs
DEPS := spec.yaml collect.js

.PHONY: codegen
codegen: $(SRCS)
	@$(MAKE) -s -C tags codegen

null.rs: null.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read collect.js rules -s spec.yaml nul nul | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@

%.rs: %.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read collect.js rules -s spec.yaml $(subst .rs,,$@) $(subst .rs,,$@) | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@
