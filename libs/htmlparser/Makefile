.PHONY: all
all:

.PHONY: test
test:
	@cargo nextest run --all-features

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src/localnames codegen
	@$(MAKE) -s -C src/treebuilder codegen
	@$(MAKE) -s -C tests codegen

.PHONY: clean
clean:
	@$(MAKE) -s -C src/localnames clean
	@$(MAKE) -s -C src/treebuilder clean
	@$(MAKE) -s -C tests clean

.PHONY: update-data
update-data:
	@sh scripts/update-benches-data.sh
