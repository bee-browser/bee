SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LOGGERGEN_SH := $(PROJ_DIR)/libs/logging/scripts/loggergen.sh
LOGGERGEN_DEPS := $(shell sh $(LOGGERGEN_SH) deps)
LALR_JSON := ../parser/lalrgen/lalr.json

CODEGEN_TARGETS := actions.rs

LOGGERGEN_TARGETS := logger.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: loggergen
loggergen: $(LOGGERGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS) $(LOGGERGEN_TARGETS)

actions.rs: actions.rs.njk actions.yaml $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@cat actions.yaml | yq -o=json | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< | \
	  rustfmt --emit=stdout >$@

actions.yaml: $(LALR_JSON) actions.js
	@echo 'Updating $(abspath $@)...'
	@deno run -q --allow-read=$(PROJ_DIR) actions.js $< $@ >$@.new
	@mv -f $@.new $@

logger.rs: $(LOGGERGEN_DEPS)
	@echo 'Generating $(abspath $@)...'
	@sh $(LOGGERGEN_SH) codegen '{"module":"syntax","target":"bee::jsparser::syntax"}' >$@
