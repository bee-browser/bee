HTML5LIB_TESTS_DIR := tests/html5lib_tests

HTML5LIB_TESTS_NAMES := \
  comments01 \
  doctype01 \
  entities01

HTML5LIB_TESTS := $(addprefix $(HTML5LIB_TESTS_DIR)/,$(addsuffix _test.rs,$(HTML5LIB_TESTS_NAMES)))

TEMPLATE := $(HTML5LIB_TESTS_DIR)/test.rs.hbs
BEE_TOOLS_CODEGEN := ../../tools/bin/bee-tools-codegen

.PHONY: all
all: html5lib-tests

.PHONY: test
test:
	@cargo nextest run

.PHONY: html5lib-tests
html5lib-tests:
	@rm -f $(HTML5LIB_TESTS_DIR)/*.json
	@rm -f $(HTML5LIB_TESTS_DIR)/*_test.rs
	@sh $(HTML5LIB_TESTS_DIR)/update.sh
	@$(MAKE) -s -j $(shell nproc) html5lib-tests-codegen

.PHONY: html5lib-tests-codegen
html5lib-tests-codegen: $(HTML5LIB_TESTS_DIR)/main.rs $(HTML5LIB_TESTS)
	@cargo fmt

$(HTML5LIB_TESTS_DIR)/main.rs: FORCE
	@echo 'Generating $(abspath $@)...'
	@echo '//<coverage:exclude>' >$@
	@echo 'mod helper;' >>$@
	@for name in $(HTML5LIB_TESTS_NAMES); do echo "mod $${name}_test;" >>$@; done
	@echo '//</coverage:exclude>' >>$@

$(HTML5LIB_TESTS_DIR)/%_test.rs: $(HTML5LIB_TESTS_DIR)/%.json $(HTML5LIB_TESTS_DIR)/test.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS_CODEGEN) $(HTML5LIB_TESTS_DIR)/test.rs.hbs $< >$@

.PRECIOUS: $(HTML5LIB_TESTS_DIR)/%.json
$(HTML5LIB_TESTS_DIR)/%.json: $(HTML5LIB_TESTS_DIR)/data/%.dat $(HTML5LIB_TESTS_DIR)/to_json.js
	@echo 'Generating $(abspath $@)...'
	@deno run -A $(HTML5LIB_TESTS_DIR)/to_json.js $(subst .dat,,$(notdir $<)) <$< >$@

.PHONY: FORCE
FORCE:
