BEE_TOOLS := ../../../tools/bin/bee-tools

.PHONY: codegen
codegen: tables.rs

tables.rs: tables.rs.hbs lexer.dfa.json
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) codegen tables.rs.hbs lexer.dfa.json | rustfmt --emit=stdout >$@

.PRECIOUS: lexer.dfa.json
lexer.dfa.json: ecma262.lex.yaml
	@echo 'Generating $(abspath $@)...'
	@$(BEE_TOOLS) lexgen $< | jq '.' >$@
