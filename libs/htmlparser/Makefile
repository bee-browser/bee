.PHONY: all
all:

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src/localnames codegen
	@$(MAKE) -s -C src/treebuilder codegen

.PHONY: update-data
update-data:
	@sh scripts/update-benches-data.sh
