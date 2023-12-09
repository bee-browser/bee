PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin
CODEGEN_DIR := $(PROJ_DIR)/target/codegen/packages/jsparser/lalrgen

CODEGEN_FILES := \
  action.rs \
  debug.rs \
  goal_symbols.rs \
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

lalr/%.rs: lalr/%.rs.hbs lalr.js $(CODEGEN_DIR)/lalr.json
	@echo 'Generating $(abspath $@)...'
	@cat $(CODEGEN_DIR)/lalr.json | \
	  deno run --allow-read lalr.js | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --input-stdin --no-escape $< | \
	  rustfmt --emit=stdout >$@

# Generate an intermediate JSON data in order to avoid re-running bee-lalrgen when lalr.js or
# lalr.rs.hbs changes.
.PRECIOUS: $(CODEGEN_DIR)/lalr.json
$(CODEGEN_DIR)/lalr.json: grammar.yaml | $(CODEGEN_DIR)
	@echo 'Generating $(abspath $@)...'
	@cargo run -rqp bee-lalrgen -- -r $(CODEGEN_DIR) $< Script Module | jq '.' >$@

$(CODEGEN_DIR):
	@mkdir -p $@
