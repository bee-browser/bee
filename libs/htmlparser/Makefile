.PHONY: all
all:

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src/localnames codegen
	@$(MAKE) -s -C src/treebuilder codegen
	@$(MAKE) -s -C tests codegen

.PHONY: update-data
update-data:
	@sh scripts/update-benches-data.sh
	@sh scripts/update-html5lib-tests.sh
	@rm -f tests/*_html5lib_tests.json
	@rm -f tests/*_html5lib_tests.rs
	@$(MAKE) -s -C tests codegen
