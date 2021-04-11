NODE_MODULES = $(abspath node_modules)

.PHONY: all
all:

.PHONY: prepare
prepare:
	@npm install

.PHONY: update-deps
update-deps:
	@deno run --allow-net --allow-read --allow-write https://raw.githubusercontent.com/bee-browser/deno-udd/master/main.ts deps.js
