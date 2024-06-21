PROJ_DIR := $(realpath ../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

JS_FILES := $(wildcard src/*.js)
CSS_FILES := src/main.css
HTML_FILES := src/index.html

BUNDLE_JS := assets/bundle.js
BUNDLE_CSS := assets/bundle.css
INDEX_HTML := assets/index.html

ASSETS_FILES := $(BUNDLE_JS) $(BUNDLE_CSS) $(INDEX_HTML)

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(ASSETS_FILES)

# It's necessary to allow reading entries outside this folder so that deno can read dependencies
# from DENO_DIR.
# TODO: minify the file
$(BUNDLE_JS): $(JS_FILES) | assets
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-env --allow-net='jsr.io' --allow-read --allow-write $(TOOLS_BIN)/deno_emit.js src/main.js >$@

# TODO: minify the file
$(BUNDLE_CSS): $(CSS_FILES) | assets
	@echo 'Generating $(abspath $@)...'
	@cp src/main.css $@

# TODO: minify the file
$(INDEX_HTML): $(HTML_FILES) | assets
	@echo 'Generating $(abspath $@)...'
	@cp src/index.html $@

assets:
	@mkdir -p assets
