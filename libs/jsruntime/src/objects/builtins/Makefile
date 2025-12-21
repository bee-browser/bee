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
NATIVE_ERROR_IMP_RS_FILES := $(addsuffix /imp.rs,$(NATIVE_ERROR_NAMES))

BUILTINS := error promise string $(NATIVE_ERROR_NAMES)
BUILTINS_README_MD_FILES := $(addsuffix /README.md,$(BUILTINS))
BUILTINS_MOD_RS_FILES := $(addsuffix /mod.rs,$(BUILTINS))
BUILTINS_IMP_JSON_FILES := $(addsuffix /imp.json,$(BUILTINS))

CODEGEN_TARGETS := $(BUILTINS_MOD_RS_FILES) $(NATIVE_ERROR_IMP_RS_FILES)
UPDATE_TARGETS := $(addprefix update-,$(BUILTINS_README_MD_FILES))
CLEAN_TARGETS := $(CODEGEN_TARGETS) $(BUILTINS_IMP_JSON_FILES)

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: update
update: $(UPDATE_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CLEAN_TARGETS)

%/mod.rs: %/imp.json builtin_mod.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@cat $< | deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-stdin builtin_mod.rs.hbs | \
	  rustfmt --emit=stdout >$@

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

.PRECIOUS: aggregate_error/imp.rs
aggregate_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "aggregate_error", "class": "AggregateError" }' | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: eval_error/imp.rs
eval_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "eval_error", "class": "EvalError" }' | \
	  rustfmt --emit=stdout >$@

# https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/InternalError
# Non-standard error type.  We use this error type for throwing internal errors.
.PRECIOUS: internal_error/imp.rs
internal_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "internal_error", "class": "InternalError" }' | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: range_error/imp.rs
range_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "range_error", "class": "RangeError" }' | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: reference_error/imp.rs
reference_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "reference_error", "class": "ReferenceError" }' | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: syntax_error/imp.rs
syntax_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "syntax_error", "class": "SyntaxError" }' | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: type_error/imp.rs
type_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "type_error", "class": "TypeError" }' | \
	  rustfmt --emit=stdout >$@

.PRECIOUS: uri_error/imp.rs
uri_error/imp.rs: native_error_imp.rs.hbs $(TOOLS_BIN)/handlebars.js
	@echo 'Generating $(abspath $@)...'
	@mkdir -p $(@D)
	@deno run -q --allow-read=. $(TOOLS_BIN)/handlebars.js --no-escape --input-inline $< '{ "id": "uri_error", "class": "URIError" }' | \
	  rustfmt --emit=stdout >$@
