BEE_TOOLS := ../../tools/bin/bee-tools

.PHONY: all
all: update

.PHONY: test
test:
	@cargo nextest run

# TODO: use `git submodule`
.PHONY: update
update:
	@rm -f src/tokenizer/*.json
	@rm -f src/tokenizer/*_test.rs
	@rm -f src/tree_construction/*.json
	@rm -f src/tree_construction/*_test.rs
	@sh scripts/update.sh
	@$(MAKE) -s -j $(shell nproc) codegen

.PHONY: codegen
codegen: codegen-tokenizer codegen-tree-construction
	@cargo fmt

# tokenizer tests

TOKENIZER_TEST_NAMES := $(subst .test,,$(notdir $(wildcard data/tokenizer/*.test)))
TOKENIZER_TESTS := $(addprefix src/tokenizer/,$(addsuffix _test.rs,$(TOKENIZER_TEST_NAMES)))

.PHONY: codegen-tokenizer
codegen-tokenizer: src/tokenizer/mod.rs $(TOKENIZER_TESTS)

src/tokenizer/mod.rs: FORCE
	@echo 'Generating $(abspath $@)...'
	@echo -n $(TOKENIZER_TEST_NAMES) | jq -Rsc 'split(" ")' | $(BEE_TOOLS) codegen --input-stdin src/tokenizer/mod.rs.hbs | $(BEE_TOOLS) update-file $@

src/tokenizer/%_test.rs: src/tokenizer/%.json scripts/tokenizer.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS_CODEGEN) scripts/tokenizer.rs.hbs $< >$@

.PRECIOUS: src/tokenizer/%.json
src/tokenizer/%.json: data/tokenizer/%.test scripts/tokenizer.json.js
	@echo 'Generating $(abspath $@)...'
	@deno run -A scripts/tokenizer.json.js $(subst .test,,$(notdir $<)) <$< >$@

# tree-construction tests

TREE_CONSTRUCTION_TEST_NAMES := \
  comments01 \
  doctype01 \
  domjs_unsafe \
  entities01 \
  entities02 \
  html5test_com \
  plain_text_unsafe
TREE_CONSTRUCTION_TESTS := $(addprefix src/tree_construction/,$(addsuffix _test.rs,$(TREE_CONSTRUCTION_TEST_NAMES)))

.PHONY: codegen-tree-construction
codegen-tree-construction: src/tree_construction/mod.rs $(TREE_CONSTRUCTION_TESTS)

src/tree_construction/mod.rs: FORCE
	@echo 'Generating $(abspath $@)...'
	@echo -n $(TREE_CONSTRUCTION_TEST_NAMES) | jq -Rsc 'split(" ")' | $(BEE_TOOLS) codegen --input-stdin src/tree_construction/mod.rs.hbs | $(BEE_TOOLS) update-file $@

src/tree_construction/%_test.rs: src/tree_construction/%.json scripts/tree_construction.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen scripts/tree_construction.rs.hbs $< >$@

.PRECIOUS: src/tree_construction/%.json
src/tree_construction/%.json: data/tree_construction/%.dat scripts/tree_construction.json.js
	@echo 'Generating $(abspath $@)...'
	@deno run -A scripts/tree_construction.json.js $(subst .dat,,$(notdir $<)) <$< >$@

# helper targets

.PHONY: FORCE
FORCE:
