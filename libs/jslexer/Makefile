.PHONY: all
all:

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src codegen

.PHONY: test
test: codegen
	cargo nextest run --all-features
