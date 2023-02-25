BEE_TOOLS := ../../tools/bin/bee-tools

.PHONY: all
all:

.PHONY: codegen
codegen: codegen-local-names
	@cargo fmt

.PHONY: codegen-local-names
codegen-local-names: src/local_names/mod.rs

src/local_names/mod.rs: src/local_names/local_names.yaml src/local_names/mod.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) deepmerge -i yaml -o json $< | $(BEE_TOOLS) codegen --input-stdin src/local_names/mod.rs.hbs >$@
