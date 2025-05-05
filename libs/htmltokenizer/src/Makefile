SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_FILES := \
  charref/trie.codegen.json \
  charref/trie.rs

CODEGEN_TARGETS := $(filter %.rs,$(CODEGEN_FILES))

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_FILES)

charref/trie.rs: charref/trie.codegen.json charref/trie.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/codegen.js --no-escape $@.hbs $< | \
	  rustfmt --emit=stdout >$@

# TODO: Use JSON5 and add comments at the beginning of the file like other
#       generated files.
.PRECIOUS: charref/trie.codegen.json
charref/trie.codegen.json: charref/entities.json charref/trie.codegen.json.js
	@echo 'Generating $(abspath $@)...'
	@deno run charref/trie.codegen.json.js <$< >$@
