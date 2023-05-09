.PHONY: update
update: es2022.lex.yaml

es2022.lex.yaml: es2022.lex.txt transpile.js
	@echo 'Updating $(abspath $@)...'
	@cat $< | deno run --allow-read transpile.js >$@

es2022.lex.txt: es2022.spec.html
	@echo 'Updating $(abspath $@)...'
	@cat $< | cargo run -q --package xtask esgrammar >$@

.PRECIOUS: es2022.html
es2022.spec.html:
	@echo 'Updating $(abspath $@)...'
	@curl https://raw.githubusercontent.com/tc39/ecma262/es2022/spec.html -sG >$@
