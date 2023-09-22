.PHONY: all
all:

.PHONY: test
test: codegen
	cargo nextest run --all-features

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src codegen
	@$(MAKE) -s -C logview codegen

.PHONY: metrics-cdnjs
metrics-cdnjs:
	@$(MAKE) -s -C metrics cdnjs

.PHONY: metrics-clean
metrics-clean:
	@$(MAKE) -s -C metrics clean
