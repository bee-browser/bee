SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

SPEC_YAML := ../spec.yaml
COLLECT_JS := ../collect.js

TAGS := $(shell cat tags.txt | tr '\n' ' ')
SRCS := mod.rs any_other.rs $(addsuffix .rs,$(TAGS))
DEPS := $(SPEC_YAML) $(COLLECT_JS)

.PHONY: codegen
codegen: $(SRCS)

mod.rs: mod.rs.hbs tags.txt
	@echo 'Generating $(abspath $@)...'
	@echo -n $(TAGS) | jq -Rsc 'split(" ")' | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --no-escape --input-stdin mod.rs.hbs | \
	  rustfmt --emit=stdout >$@

any_other.rs: tag.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read $(COLLECT_JS) rules -s $(SPEC_YAML) any_other '<_>' '</_>' | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@

%.rs: tag.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read $(COLLECT_JS) rules -s $(SPEC_YAML) $(subst .rs,,$@) \
	  '<$(subst .rs,,$@)>' '</$(subst .rs,,$@)>' | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@
