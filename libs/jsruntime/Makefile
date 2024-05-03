UMLS := docs/ecma262_types.svg

.PHONY: all
all:

.PHONY: test
test:
	cargo nextest run --all-features

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src/llvmir codegen

.PHONY: bench
bench:
	cargo bench

.PHONY: uml
uml: $(UMLS)

%.svg: %.puml
	@echo 'Generating $(abspath $@)...'
	@cat $< | docker run --rm -i ghcr.io/plantuml/plantuml -charset UTF-8 -tsvg -pipe >$@
