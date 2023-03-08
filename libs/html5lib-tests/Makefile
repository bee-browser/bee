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
codegen:
	@$(MAKE) -s -C src/tokenizer codegen
	@$(MAKE) -s -C src/tree_construction codegen
