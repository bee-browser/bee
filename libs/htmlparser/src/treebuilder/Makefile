SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

SRCS := doctype.rs comment.rs null.rs whitespace.rs text.rs eof.rs
DEPS := spec.yaml collect.js

.PHONY: codegen
codegen: $(SRCS)
	@$(MAKE) -s -C tags codegen

.PHONY: clean
clean:
	@rm -f $(SRCS)
	@$(MAKE) -s -C tags clean

null.rs: null.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read collect.js rules -s spec.yaml nul nul | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@

%.rs: %.rs.hbs $(DEPS)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-read collect.js rules -s spec.yaml $(subst .rs,,$@) $(subst .rs,,$@) | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
