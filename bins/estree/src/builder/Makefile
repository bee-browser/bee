PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LALR_JSON := $(PROJ_DIR)/target/codegen/packages/jsparser/lalrgen/lalr.json

CODEGEN_TARGETS := actions.rs

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

actions.rs: actions.yaml actions.rs.hbs
	@echo 'Generating $(abspath $@)...'
	@cat $< | yq -o=json | \
	  deno run -qA $(TOOLS_BIN)/codegen.js --input-stdin --no-escape actions.rs.hbs | \
	  rustfmt --emit=stdout >$@

actions.yaml: actions.js $(LALR_JSON)
	@echo 'Updating $(abspath $@)...'
	@deno run -A actions.js update $(LALR_JSON) $@ >$@.new
	@mv -f $@.new $@

$(LALR_JSON):
	@$(MAKE) -s -C $(PROJ_DIR)/packages/jsparser/src $@
