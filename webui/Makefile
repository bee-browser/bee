ASSETS_DIR = $(abspath assets)
JS_FILES = $(wildcard src/*.js)
CSS_FILES = src/main.css
HTML_FILES = src/index.html
BUNDLE_JS = $(ASSETS_DIR)/bundle.js
BUNDLE_CSS = $(ASSETS_DIR)/bundle.css
INDEX_HTML = $(ASSETS_DIR)/index.html
ASSETS_FILES = $(BUNDLE_JS) $(BUNDLE_CSS) $(INDEX_HTML)

.PHONY: all
all: build

.PHONY: build
build: $(ASSETS_FILES)

.PHONE: clean
clean:
	@rm -rf $(ASSETS_DIR)

.PHONE: prepare
prepare:
	@npm install

$(BUNDLE_JS): $(JS_FILES) | $(ASSETS_DIR)
	npx rollup -c rollup.config.js -i src/main.js -o $@

$(BUNDLE_CSS): $(CSS_FILES) | $(ASSETS_DIR)
	cp src/main.css $@

$(INDEX_HTML): $(HTML_FILES) | $(ASSETS_DIR)
	cp src/index.html $@

$(ASSETS_DIR):
	@mkdir -p $@
