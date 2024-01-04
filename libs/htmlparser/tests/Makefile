SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

DATA_DIR := html5lib-tests
DATA_FILES := $(wildcard $(DATA_DIR)/*.dat)
TEST_NAMES := $(subst .dat,,$(notdir $(DATA_FILES)))
TEST_FILES := $(addsuffix _html5lib_tests.rs,$(TEST_NAMES))
SRCS := $(TEST_FILES)

.PHONY: codegen
codegen: $(SRCS)

%_html5lib_tests.rs: %_html5lib_tests.json html5lib_tests.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@deno run -qA $(TOOLS_BIN)/codegen.js --no-escape html5lib_tests.rs.hbs $< | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: %_html5lib_tests.json
%_html5lib_tests.json: $(DATA_DIR)/%.dat html5lib-tests.js
	@echo 'Generating $(abspath $@)...'
	@deno run -A html5lib-tests.js $(subst .json,,$@) <$< >$@
