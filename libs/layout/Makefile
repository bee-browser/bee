EMPTY :=
SPACE := $(EMPTY) $(EMPTY)
SEP := ","

TESTS_DIR := $(abspath tests)
RESOURCES_DIR := $(TESTS_DIR)/resources
TESTGEN_DIR := $(BEE_CARGO_CODEGEN_DIR)/libs/layout/tests

LAYOUT_TEST_VIEWPORT_WIDTH := 1000
LAYOUT_TEST_VIEWPORT_HEIGHT := 500
LAYOUT_TEST_VIEWPORT_SIZE := $(LAYOUT_TEST_VIEWPORT_WIDTH)x$(LAYOUT_TEST_VIEWPORT_HEIGHT)
LAYOUT_TEST_SRC_FILES := $(wildcard $(RESOURCES_DIR)/*.test.yml.hbs)
LAYOUT_TEST_SRC_NAMES := $(subst .test.yml.hbs,,$(notdir $(LAYOUT_TEST_SRC_FILES)))
LAYOUT_TEST_HTML_FILES := $(addprefix $(TESTGEN_DIR)/,$(addsuffix .test.html,$(LAYOUT_TEST_SRC_NAMES)))
LAYOUT_TEST_JSON_FILES := $(addprefix $(TESTGEN_DIR)/,$(addsuffix .test.json,$(LAYOUT_TEST_SRC_NAMES)))
LAYOUT_TEST_EXPECTED_FILES := $(wildcard $(RESOURCES_DIR)/*.expected.yml)
LAYOUT_TEST_NAMES := $(subst .expected.yml,,$(notdir $(LAYOUT_TEST_EXPECTED_FILES)))
LAYOUT_TEST_SCENARIO_FILES := $(addprefix $(TESTGEN_DIR)/,$(addsuffix .scenario.jsonl,$(LAYOUT_TEST_NAMES)))
LAYOUT_TEST_JSON_FILES := $(addprefix $(TESTGEN_DIR)/,$(addsuffix .test.json,$(LAYOUT_TEST_SRC_NAMES)))

DEPS := $(addprefix $(TESTGEN_DIR)/,$(addsuffix .test.yml.d,$(LAYOUT_TEST_SRC_NAMES)))

.PHONY: testgen
testgen: $(TESTGEN_DIR)/index.html $(TESTGEN_DIR)/layout_test.rs

-include $(DEPS)

$(TESTGEN_DIR)/index.html: $(RESOURCES_DIR)/index.html.hbs $(TESTGEN_DIR)/index.json $(TESTGEN_DIR)/tests.js | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-codegen $< $(TESTGEN_DIR)/index.json >$@

$(TESTGEN_DIR)/index.json: $(LAYOUT_TEST_SRC_FILES) | $(TESTGEN_DIR)
	@echo Generating $@...
	@echo '{"names":["$(subst $(SPACE),$(SEP),$(LAYOUT_TEST_SRC_NAMES))"],"viewport":{"width":$(LAYOUT_TEST_VIEWPORT_WIDTH),"height":$(LAYOUT_TEST_VIEWPORT_HEIGHT)}}' >$@

$(TESTGEN_DIR)/tests.js: $(LAYOUT_TEST_JSON_FILES) $(LAYOUT_TEST_HTML_FILES) | $(TESTGEN_DIR)
	@echo Generating $@...
	@echo 'const TESTS = [' >$@
	@for file in $(LAYOUT_TEST_JSON_FILES); do cat $$file >>$@; echo ',' >>$@; done
	@echo '];' >>$@

$(TESTGEN_DIR)/layout_test.rs: $(RESOURCES_DIR)/layout_test.rs.hbs $(TESTGEN_DIR)/layout_test.json $(LAYOUT_TEST_SCENARIO_FILES) | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-codegen $< $(TESTGEN_DIR)/layout_test.json >$@

$(TESTGEN_DIR)/layout_test.json: $(LAYOUT_TEST_EXPECTED_FILES) | $(TESTGEN_DIR)
	@echo Generating $@...
	@echo '{"codegen_dir":"$(TESTGEN_DIR)","resources_dir":"$(RESOURCES_DIR)","names":["$(subst $(SPACE),$(SEP),$(LAYOUT_TEST_NAMES))"]}' >$@

$(TESTGEN_DIR)/%.scenario.jsonl: $(TESTGEN_DIR)/%.dom.json | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-layout-builder $< >$@

.PRECIOUS: $(TESTGEN_DIR)/%.dom.json
$(TESTGEN_DIR)/%.dom.json: $(TESTGEN_DIR)/%.test.html | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-dom-scraper --viewport=$(LAYOUT_TEST_VIEWPORT_SIZE) $< >$@

.PRECIOUS: $(TESTGEN_DIR)/%.test.html
$(TESTGEN_DIR)/%.test.html: $(TESTGEN_DIR)/%.test.json $(RESOURCES_DIR)/test.html.hbs | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-codegen $(RESOURCES_DIR)/test.html.hbs $< >$@

.PRECIOUS: $(TESTGEN_DIR)/%.test.json
$(TESTGEN_DIR)/%.test.json: $(TESTGEN_DIR)/%.test.yml | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-deepmerge -i yaml -o json $< >$@

.PRECIOUS: $(TESTGEN_DIR)/%.test.yml
$(TESTGEN_DIR)/%.test.yml: $(RESOURCES_DIR)/%.test.yml.hbs | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-codegen -p $(RESOURCES_DIR)/partials $< >$@

.PRECIOUS: $(TESTGEN_DIR)/%.test.yml.d
$(TESTGEN_DIR)/%.test.yml.d: $(RESOURCES_DIR)/%.test.yml.hbs | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-tools-codegen -p $(RESOURCES_DIR)/partials --deps $(subst .d,,$@) $< >$@

$(TESTGEN_DIR):
	@mkdir -p $@
