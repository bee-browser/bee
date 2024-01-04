SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

.PHONY: all
all:

.PHONY: test
test:
	@cargo nextest run

.PHONY: codegen
codegen: src/charref/trie.rs
	@$(MAKE) -s -C tests codegen

.PHONY: update-data
update-data:
	@sh scripts/update-html5lib-tests.sh
	@rm -f tests/*_html5lib_tests.json
	@rm -f tests/*_html5lib_tests.rs
	@$(MAKE) -s -C tests codegen

src/charref/trie.rs: src/charref/trie.codegen.json src/charref/trie.rs.hbs
	@echo 'Generating $@'...
	@deno run -qA $(TOOLS_BIN)/codegen.js --no-escape src/charref/trie.rs.hbs $< | \
	  rustfmt --emit=stdout >$@

# TODO: Use JSON5 and add comments at the beginning of the file like other
#       generated files.
.PRECIOUS: src/charref/trie.codegen.json
src/charref/trie.codegen.json: src/charref/entities.json src/charref/trie.codegen.json.js
	@echo 'Generating $@'...
	@deno run src/charref/trie.codegen.json.js <$< >$@
