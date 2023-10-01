PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_FILES := \
  action.rs \
  debug.rs \
  goto.rs \
  lexical_goal.rs \
  non_terminals.rs \
  auto_semicolon.rs

CODEGEN_TARGETS := $(addprefix lalr/,$(CODEGEN_FILES))

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

lalr/%.rs: lalr/%.rs.hbs lalr.json.xz lalr.js
	@echo 'Generating $(abspath $@)...'
	@xzcat lalr.json.xz | \
	  deno run --allow-read lalr.js | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --input-stdin --no-escape $< | \
	  rustfmt --emit=stdout >$@

# Generate an intermediate JSON data in order to avoid re-running bee-lalrgen when lalr.js or
# lalr.rs.hbs changes.
lalr.json.xz: grammar.yaml
	@echo 'Generating $(abspath $@)...'
	@cargo run -q -r -p bee-lalrgen -- $< Module | jq -c . | xz >$@
