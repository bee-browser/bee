TESTS_DIR = $(abspath tests)
RESOURCES_DIR = $(TESTS_DIR)/resources
TARGET_DIR = $(if $(CARGO_TARGET_DIR),$(CARGO_TARGET_DIR),$(abspath target))
CODEGEN_DIR = $(TARGET_DIR)/codegen/libs/layout

LAYOUT_TEST_VIEWPORT_SIZE = "1000x500"
LAYOUT_TEST_COMMON_FILES = $(wildcard $(RESOURCES_DIR)/common/*.html.njk)
LAYOUT_TEST_SRC_FILES = $(wildcard $(RESOURCES_DIR)/*.html.njk)
LAYOUT_TEST_NAMES = $(notdir $(basename $(basename $(LAYOUT_TEST_SRC_FILES))))
LAYOUT_TEST_SCENARIO_FILES = \
  $(addprefix $(CODEGEN_DIR)/,$(addsuffix .scenario.json,$(LAYOUT_TEST_NAMES)))

COVERAGE_ENV_VARS = \
  CARGO_INCREMENTAL=0 \
  RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
  RUSTDOCFLAGS="-Cpanic=abort"

build:
	cargo build

test: $(CODEGEN_DIR)/layout_test.rs $(LAYOUT_TEST_SCENARIO_FILES)
	cargo test --all-features

coverage: $(CODEGEN_DIR)/layout_test.rs $(LAYOUT_TEST_SCENARIO_FILES)
	env $(COVERAGE_ENV_VARS) cargo +nightly test --all-features
	grcov $(TARGET_DIR)/debug -s $(abspath .) -t html --llvm --branch --ignore-not-existing \
	  -o $(TARGET_DIR)/coverage --excl-line "//<coverage:exclude/>" \
	  --excl-start "//<coverage:exclude>" --excl-stop "//</coverage:exclude>"

clean:
	cargo clean

$(CODEGEN_DIR)/layout_test.rs: $(RESOURCES_DIR)/layout_test.rs.njk | $(CODEGEN_DIR)
	@echo Generating $@...
	@bee-codegen $(RESOURCES_DIR)/layout_test.rs.njk \
	  -v "names=$(LAYOUT_TEST_NAMES)" -v "resources_dir=$(RESOURCES_DIR)" \
	  -v "codegen_dir=$(CODEGEN_DIR)" >$@

$(CODEGEN_DIR)/%.scenario.json: $(CODEGEN_DIR)/%.html | $(CODEGEN_DIR)
	@echo Generating $@...
	@bee-lms-html --no-sandbox --viewport=$(LAYOUT_TEST_VIEWPORT_SIZE) $< >$@

$(CODEGEN_DIR)/%.html: $(RESOURCES_DIR)/%.html.njk $(LAYOUT_TEST_COMMON_FILES) | $(CODEGEN_DIR)
	@echo Generating $@...
	@bee-codegen -s $(RESOURCES_DIR)/common $< >$@

$(CODEGEN_DIR):
	@mkdir -p $@

.PRECIOUS: $(CODEGEN_DIR)/%.html
