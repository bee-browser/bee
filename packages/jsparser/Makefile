.PHONY: all
all:

.PHONY: test
test: codegen
	cargo nextest run --all-features

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src codegen
	@$(MAKE) -s -C logview codegen
