BEE_TOOLS := ../../../tools/bin/bee-tools

EMPTY :=
SPACE := $(EMPTY) $(EMPTY)
SEP := ","

LAYOUT_TEST_VIEWPORT_WIDTH := 1000
LAYOUT_TEST_VIEWPORT_HEIGHT := 500
LAYOUT_TEST_VIEWPORT_SIZE := $(LAYOUT_TEST_VIEWPORT_WIDTH)x$(LAYOUT_TEST_VIEWPORT_HEIGHT)
LAYOUT_TEST_SRC_FILES := $(wildcard data/*.test.yaml.hbs)
LAYOUT_TEST_SRC_NAMES := $(subst .test.yaml.hbs,,$(notdir $(LAYOUT_TEST_SRC_FILES)))
LAYOUT_TEST_HTML_FILES := $(addprefix html/,$(addsuffix .test.html,$(LAYOUT_TEST_SRC_NAMES)))
LAYOUT_TEST_EXPECTED_FILES := $(wildcard data/*.expected.yaml)
LAYOUT_TEST_NAMES := $(subst .expected.yaml,,$(notdir $(LAYOUT_TEST_EXPECTED_FILES)))
LAYOUT_TEST_SCENARIO_FILES := $(addsuffix .scenario.jsonl,$(LAYOUT_TEST_NAMES))
LAYOUT_TEST_JSON_FILES := $(addsuffix .test.json,$(LAYOUT_TEST_SRC_NAMES))

DEPS := $(addprefix src/,$(addsuffix .test.yaml.d,$(LAYOUT_TEST_SRC_NAMES)))

.PHONY: clean
clean:
	@rm -f html/*.html html/*.js html/*.json
	@rm -f src/*.json src/*.jsonl src/*.yaml src/*.yaml.d

.PHONY: tests
tests: clean
	@$(MAKE) -s -j $(shell nproc) codegen

.PHONY: codegen
codegen: codegen-html codegen-rust

.PHONY: codegen-html
codegen-html: html/index.html

.PHONY: codegen-rust
codegen-rust: layout_tests.rs

-include $(DEPS)

html/index.html: html/index.html.hbs html/index.json html/tests.js
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen $< html/index.json >$@

html/index.json: $(LAYOUT_TEST_SRC_FILES)
	@echo 'Generating $(abspath $@)...'
	@echo '{"names":["$(subst $(SPACE),$(SEP),$(LAYOUT_TEST_SRC_NAMES))"],"viewport":{"width":$(LAYOUT_TEST_VIEWPORT_WIDTH),"height":$(LAYOUT_TEST_VIEWPORT_HEIGHT)}}' >$@

html/tests.js: $(LAYOUT_TEST_JSON_FILES) $(LAYOUT_TEST_HTML_FILES)
	@echo 'Generating $(abspath $@)...'
	@echo 'const TESTS = [' >$@
	@for file in $(LAYOUT_TEST_JSON_FILES); do cat $$file >>$@; echo ',' >>$@; done
	@echo '];' >>$@

layout_tests.rs: layout_test.rs.hbs layout_tests.json $(LAYOUT_TEST_SCENARIO_FILES)
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen $< layout_tests.json >$@

layout_tests.json: $(LAYOUT_TEST_EXPECTED_FILES)
	@echo 'Generating $(abspath $@)...'
	@echo '{"names":["$(subst $(SPACE),$(SEP),$(LAYOUT_TEST_NAMES))"]}' >$@

%.scenario.jsonl: %.dom.json
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) layout-builder $< >$@

.PRECIOUS: %.dom.json
%.dom.json: html/%.test.html
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) dom-scraper --viewport=$(LAYOUT_TEST_VIEWPORT_SIZE) $< >$@

.PRECIOUS: html/%.test.html
html/%.test.html: %.test.json html/test.html.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen html/test.html.hbs $< >$@

.PRECIOUS: %.test.json
%.test.json: %.test.yaml
	@echo 'Generating $(abspath $@)...'
	@cat $< | $(BEE_TOOLS) y2j >$@

.PRECIOUS: %.test.yaml
%.test.yaml: data/%.test.yaml.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen -p data/partials $< >$@

.PRECIOUS: %.test.yaml.d
%.test.yaml.d: data/%.test.yaml.hbs
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen -p data/partials --deps $(subst .d,,$@) $< >$@
