PROJ_DIR := $(realpath ../../../..)

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

lalr/%.rs: lalr/%.rs.hbs lalr.json.xz lalr.js ../lexer/tokens.json $(PROJ_DIR)/target/release/bee-handlebars
	@echo 'Generating $(abspath $@)...'
	@xzcat lalr.json.xz | \
	  deno run --allow-read lalr.js | \
	  cargo run -q -r -p bee-handlebars -- $< | \
	  rustfmt --emit=stdout >$@

# Generate a temporal intermediate JSON data in order to avoid re-running bee-lalrgen when lalr.js
# or lalr.rs.hbs changes.
#
# TODO: Remove this target.
lalr.json.xz: es2022.syn.yaml $(PROJ_DIR)/target/release/bee-lalrgen
	@echo 'Generating $(abspath $@)...'
	@cargo run -q -r -p bee-lalrgen -- $< Script | jq -c . | xz >$@

include $(PROJ_DIR)/target/release/bee-handlebars.d
$(PROJ_DIR)/target/release/bee-handlebars:
	@cargo build -r -p bee-handlebars

include $(PROJ_DIR)/target/release/bee-lalrgen.d
$(PROJ_DIR)/target/release/bee-lalrgen:
	@cargo build -r -p bee-lalrgen
