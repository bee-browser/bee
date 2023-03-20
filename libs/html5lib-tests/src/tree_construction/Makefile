BEE_TOOLS := ../../../../tools/bin/bee-tools
DATA_DIR := ../../data/tree_construction
DATA_FILES := $(wildcard $(DATA_DIR)/*.dat)
TEST_NAMES := $(subst .dat,,$(notdir $(DATA_FILES)))
TEST_FILES := $(addsuffix _test.rs,$(TEST_NAMES))
SRCS := mod.rs $(TEST_FILES)

.PHONY: codegen
codegen: $(SRCS)

mod.rs: mod.rs.hbs $(DATA_FILES)
	@echo 'Generating $(abspath $@)...'
	@echo -n $(TEST_NAMES) | \
	  jq -Rsc 'split(" ")' | \
	  $(BEE_TOOLS) codegen --no-escape --input-stdin $< | \
	  rustfmt --emit=stdout >$@

%_test.rs: %.json test.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen --no-escape test.rs.hbs $< | rustfmt --emit=stdout >$@

.PRECIOUS: %.json
%.json: $(DATA_DIR)/%.dat make-json.js
	@echo 'Generating $(abspath $@)...'
	@deno run -A make-json.js $(subst .dat,,$@) <$< >$@
