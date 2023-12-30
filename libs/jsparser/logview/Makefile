ASSETS_DIR := assets

JS_FILES := $(wildcard src/*.js)
CSS_FILES := src/main.css
HTML_FILES := src/index.html

BUNDLE_JS := $(ASSETS_DIR)/bundle.js
BUNDLE_CSS := $(ASSETS_DIR)/bundle.css
INDEX_HTML := $(ASSETS_DIR)/index.html

ASSETS_FILES := $(BUNDLE_JS) $(BUNDLE_CSS) $(INDEX_HTML)

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(ASSETS_FILES)

# TODO: minify the file
$(BUNDLE_JS): $(JS_FILES)
	@echo 'Generating $(abspath $@)...'
	deno bundle src/main.js $@

# TODO: minify the file
$(BUNDLE_CSS): $(CSS_FILES)
	@echo 'Generating $(abspath $@)...'
	@cp src/main.css $@

# TODO: minify the file
$(INDEX_HTML): $(HTML_FILES)
	@echo 'Generating $(abspath $@)...'
	@cp src/index.html $@
