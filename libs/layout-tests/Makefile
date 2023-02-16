BEE_TOOLS_CODEGEN := ../../tools/bin/bee-tools-codegen
BEE_TOOLS_DEEPMERGE := ../../tools/bin/bee-tools-deepmerge
BEE_TOOLS_DOM_SCRAPER := ../../tools/bin/bee-tools-dom-scraper
BEE_TOOLS_LAYOUT_BUILDER := ../../tools/bin/bee-tools-layout-builder

EMPTY :=
SPACE := $(EMPTY) $(EMPTY)
SEP := ","

LAYOUT_TEST_VIEWPORT_WIDTH := 1000
LAYOUT_TEST_VIEWPORT_HEIGHT := 500
LAYOUT_TEST_VIEWPORT_SIZE := $(LAYOUT_TEST_VIEWPORT_WIDTH)x$(LAYOUT_TEST_VIEWPORT_HEIGHT)
LAYOUT_TEST_SRC_FILES := $(wildcard data/*.test.yml.hbs)
LAYOUT_TEST_SRC_NAMES := $(subst .test.yml.hbs,,$(notdir $(LAYOUT_TEST_SRC_FILES)))
LAYOUT_TEST_HTML_FILES := $(addprefix html/,$(addsuffix .test.html,$(LAYOUT_TEST_SRC_NAMES)))
LAYOUT_TEST_EXPECTED_FILES := $(wildcard data/*.expected.yml)
LAYOUT_TEST_NAMES := $(subst .expected.yml,,$(notdir $(LAYOUT_TEST_EXPECTED_FILES)))
LAYOUT_TEST_SCENARIO_FILES := $(addprefix src/,$(addsuffix .scenario.jsonl,$(LAYOUT_TEST_NAMES)))
LAYOUT_TEST_JSON_FILES := $(addprefix src/,$(addsuffix .test.json,$(LAYOUT_TEST_SRC_NAMES)))

DEPS := $(addprefix src/,$(addsuffix .test.yml.d,$(LAYOUT_TEST_SRC_NAMES)))

.PHONY: all
all: tests

.PHONY: test
test:
	@cargo nextest run

.PHONY: clean
clean:
	@rm -f html/*.html html/*.js html/*.json
	@rm -f src/*.json src/*.jsonl src/*.yml src/*.yml.d

.PHONY: tests
tests: clean
	@$(MAKE) -s -j $(shell nproc) codegen

.PHONY: codegen
codegen: html/index.html src/tests.rs

-include $(DEPS)

html/index.html: html/index.html.hbs html/index.json html/tests.js
	@echo Generating $@...
	@$(BEE_TOOLS_CODEGEN) $< html/index.json >$@

html/index.json: $(LAYOUT_TEST_SRC_FILES)
	@echo 'Generating $@...'
	@echo '{"names":["$(subst $(SPACE),$(SEP),$(LAYOUT_TEST_SRC_NAMES))"],"viewport":{"width":$(LAYOUT_TEST_VIEWPORT_WIDTH),"height":$(LAYOUT_TEST_VIEWPORT_HEIGHT)}}' >$@

html/tests.js: $(LAYOUT_TEST_JSON_FILES) $(LAYOUT_TEST_HTML_FILES)
	@echo 'Generating $@...'
	@echo 'const TESTS = [' >$@
	@for file in $(LAYOUT_TEST_JSON_FILES); do cat $$file >>$@; echo ',' >>$@; done
	@echo '];' >>$@

src/tests.rs: data/layout_test.rs.hbs src/tests.json $(LAYOUT_TEST_SCENARIO_FILES)
	@echo Generating $@...
	@$(BEE_TOOLS_CODEGEN) $< src/tests.json >$@

src/tests.json: $(LAYOUT_TEST_EXPECTED_FILES)
	@echo 'Generating $@...'
	@echo '{"codegen_dir":"src","resources_dir":"data","names":["$(subst $(SPACE),$(SEP),$(LAYOUT_TEST_NAMES))"]}' >$@

src/%.scenario.jsonl: src/%.dom.json
	@echo 'Generating $@...'
	@$(BEE_TOOLS_LAYOUT_BUILDER) $< >$@

.PRECIOUS: src/%.dom.json
src/%.dom.json: html/%.test.html
	@echo 'Generating $@...'
	@$(BEE_TOOLS_DOM_SCRAPER) --viewport=$(LAYOUT_TEST_VIEWPORT_SIZE) $< >$@

.PRECIOUS: html/%.test.html
html/%.test.html: src/%.test.json html/test.html.hbs
	@echo 'Generating $@...'
	@$(BEE_TOOLS_CODEGEN) html/test.html.hbs $< >$@

.PRECIOUS: src/%.test.json
src/%.test.json: src/%.test.yml
	@echo 'Generating $@...'
	@$(BEE_TOOLS_DEEPMERGE) -i yaml -o json $< >$@

.PRECIOUS: src/%.test.yml
src/%.test.yml: data/%.test.yml.hbs
	@echo 'Generating $@...'
	@$(BEE_TOOLS_CODEGEN) -p data/partials $< >$@

.PRECIOUS: src/%.test.yml.d
src/%.test.yml.d: data/%.test.yml.hbs
	@echo 'Generating $@...'
	@$(BEE_TOOLS_CODEGEN) -p data/partials --deps $(subst .d,,$@) $< >$@
