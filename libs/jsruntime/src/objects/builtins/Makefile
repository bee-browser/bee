SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
ECMA262_SPEC_HTML := $(PROJ_DIR)/vendor/src/tc39/ecma262/spec.html

NATIVE_ERROR_NAMES := \
  aggregate_error \
  eval_error \
  internal_error \
  range_error \
  reference_error \
  syntax_error \
  type_error \
  uri_error

BUILTINS := promise string
BUILTINS_README_MD_FILES := $(addsuffix /README.md,$(BUILTINS))
BUILTINS_MOD_RS_FILES := $(addsuffix /mod.rs,$(BUILTINS))
BUILTINS_IMP_JSON_FILES := $(addsuffix /imp.json,$(BUILTINS))

CODEGEN_TARGETS := $(addsuffix .rs,$(NATIVE_ERROR_NAMES)) $(BUILTINS_MOD_RS_FILES)
UPDATE_TARGETS := $(addprefix update-,$(BUILTINS_README_MD_FILES))
CLEAN_TARGETS := $(CODEGEN_TARGETS) $(BUILTINS_IMP_JSON_FILES)

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: update
update: $(UPDATE_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CLEAN_TARGETS)

aggregate_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "aggregate_error" }' | \
	  rustfmt --emit=stdout >$@

eval_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "eval_error" }' | \
	  rustfmt --emit=stdout >$@

# https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/InternalError
# Non-standard error type.  We use this error type for throwing internal errors.
internal_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "internal_error" }' | \
	  rustfmt --emit=stdout >$@

range_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "range_error" }' | \
	  rustfmt --emit=stdout >$@

reference_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "reference_error" }' | \
	  rustfmt --emit=stdout >$@

syntax_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "syntax_error" }' | \
	  rustfmt --emit=stdout >$@

type_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "type_error" }' | \
	  rustfmt --emit=stdout >$@

uri_error.rs: native_error.rs.njk $(TOOLS_BIN)/nunjucks.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $< '{ "name": "uri_error" }' | \
	  rustfmt --emit=stdout >$@

%/mod.rs: %/imp.json builtin_mod.rs.mustache
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. mustache $< builtin_mod.rs.mustache | rustfmt --emit=stdout >$@

# `jsdom` reads
# `$DENO_DIR/npm/registry.npmjs.org/jsdom/<version>/lib/jsdom/browser/default-stylesheet.css`.
# This is a file inside the `jsdom` package but `deno` does NOT allow reading it by default...
#
# `jsdom` uses the `debug` package which reads `process.env`.
.PRECIOUS: %/imp.json
%/imp.json: %/imp.rs imp.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run -q --allow-env --allow-read imp.js >$@

.PHONY: update-%/README.md
update-%/README.md: %/imp.json
	@echo 'Updating $(abspath $(<D)/README.md)...'
	@deno run -q --allow-read=$(<D) --allow-write=$(<D) update_readme_md.js $(<D)/README.md $<
