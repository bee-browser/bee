BEE_TOOLS := ../../../../tools/bin/bee-tools
SRCS := doctype.rs comment.rs text.rs eof.rs
DEPS := spec.yaml compile.js

.PHONY: codegen
codegen: $(SRCS)
	@$(MAKE) -s -C tags codegen

text.rs: text.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read compile.js -s spec.yaml text nul whitespace character | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@

%.rs: %.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read compile.js -s spec.yaml $(subst .rs,,$@) $(subst .rs,,$@) | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@
