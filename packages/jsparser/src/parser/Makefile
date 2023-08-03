CODEGEN_TARGETS := lalr.rs

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

lalr.rs: lalr.json.xz lalr.js lalr.rs.hbs ../lexer/tokens.json
	@echo 'Generating $(abspath $@)...'
	@xzcat $< | \
	  deno run --allow-read lalr.js | \
	  cargo run -q -r -p bee-handlebars -- lalr.rs.hbs | \
	  rustfmt --emit=stdout >$@

# Generate a temporal intermediate JSON data in order to avoid re-running bee-lalrgen when lalr.js
# or lalr.rs.hbs changes.
lalr.json.xz: es2022.syn.yaml
	@echo 'Generating $(abspath $@)...'
	@cargo run -q -r -p bee-lalrgen -- $< Script | jq -c . | xz >$@
