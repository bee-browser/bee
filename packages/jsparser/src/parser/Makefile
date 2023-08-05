CODEGEN_FILES := action.rs debug.rs goto.rs lexical_goal.rs non_terminals.rs
CODEGEN_TARGETS := $(addprefix lalr/,$(CODEGEN_FILES))

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

lalr/%.rs: lalr/%.rs.hbs lalr.json.xz lalr.js ../lexer/tokens.json
	@echo 'Generating $(abspath $@)...'
	@xzcat lalr.json.xz | \
	  deno run --allow-read lalr.js | \
	  cargo run -q -r -p bee-handlebars -- $< | \
	  rustfmt --emit=stdout >$@

# Generate a temporal intermediate JSON data in order to avoid re-running bee-lalrgen when lalr.js
# or lalr.rs.hbs changes.
#
# TODO: Remove this target.
lalr.json.xz: es2022.syn.yaml
	@echo 'Generating $(abspath $@)...'
	@cargo run -q -r -p bee-lalrgen -- $< Script | jq -c . | xz >$@
