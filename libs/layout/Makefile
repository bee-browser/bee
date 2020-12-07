TESTS_DIR = $(abspath tests)
RESOURCES_DIR = $(TESTS_DIR)/resources
TESTGEN_DIR = $(BEE_CARGO_CODEGEN_DIR)/libs/layout/tests

LAYOUT_TEST_VIEWPORT_SIZE = "1000x500"
LAYOUT_TEST_COMMON_FILES = $(wildcard $(RESOURCES_DIR)/common/*.html.njk)
LAYOUT_TEST_SRC_FILES = $(wildcard $(RESOURCES_DIR)/*.html.njk)
LAYOUT_TEST_NAMES = $(notdir $(basename $(basename $(LAYOUT_TEST_SRC_FILES))))
LAYOUT_TEST_SCENARIO_FILES = \
  $(addprefix $(TESTGEN_DIR)/,$(addsuffix .scenario.jsonl,$(LAYOUT_TEST_NAMES)))

.PHONY: testgen
testgen: $(TESTGEN_DIR)/layout_test.rs $(LAYOUT_TEST_SCENARIO_FILES)

$(TESTGEN_DIR)/layout_test.rs: $(RESOURCES_DIR)/layout_test.rs.njk $(LAYOUT_TEST_SRC_FILES) | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-codegen -v "names=$(LAYOUT_TEST_NAMES)" -v "resources_dir=$(RESOURCES_DIR)" \
	  -v "codegen_dir=$(TESTGEN_DIR)" $< >$@

$(TESTGEN_DIR)/%.scenario.jsonl: $(TESTGEN_DIR)/%.html | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-lms-html --no-sandbox --viewport=$(LAYOUT_TEST_VIEWPORT_SIZE) $< >$@

.PRECIOUS: $(TESTGEN_DIR)/%.html
$(TESTGEN_DIR)/%.html: $(RESOURCES_DIR)/%.html.njk $(LAYOUT_TEST_COMMON_FILES) | $(TESTGEN_DIR)
	@echo Generating $@...
	@bee-codegen -s $(RESOURCES_DIR)/common $< >$@

$(TESTGEN_DIR):
	@mkdir -p $@
