BEE_TOOLS := ../../../../../tools/bin/bee-tools
SPEC_YAML := ../spec.yaml
COMPILE_JS := ../compile.js

TAGS := $(shell cat tags.txt | tr '\n' ' ')
SRCS := mod.rs any_other.rs $(addsuffix .rs,$(TAGS))
DEPS := $(SPEC_YAML) $(COMPILE_JS)

.PHONY: codegen
codegen: $(SRCS)

mod.rs: mod.rs.hbs tags.txt
	@echo 'Generating $(abspath $@)...'
	@echo -n $(TAGS) | jq -Rsc 'split(" ")' | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin mod.rs.hbs | \
	  rustfmt --emit=stdout >$@

any_other.rs: tag.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read $(COMPILE_JS) -s $(SPEC_YAML) any_other '<>' '</>' | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@

%.rs: tag.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read $(COMPILE_JS) -s $(SPEC_YAML) $(subst .rs,,$@) \
	  '<$(subst .rs,,$@)>' '</$(subst .rs,,$@)>' | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@
