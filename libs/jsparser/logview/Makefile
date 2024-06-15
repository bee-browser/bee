PROJ_DIR := $(realpath ../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

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

# It's necessary to allow reading entries outside this folder so that deno can read dependencies
# from DENO_DIR.
# TODO: minify the file
$(BUNDLE_JS): $(JS_FILES)
	@echo 'Generating $(abspath $@)...'
	@deno run --allow-env --allow-net='jsr.io' --allow-read --allow-write $(TOOLS_BIN)/deno_emit.js src/main.js >$@

# TODO: minify the file
$(BUNDLE_CSS): $(CSS_FILES)
	@echo 'Generating $(abspath $@)...'
	@cp src/main.css $@

# TODO: minify the file
$(INDEX_HTML): $(HTML_FILES)
	@echo 'Generating $(abspath $@)...'
	@cp src/index.html $@
