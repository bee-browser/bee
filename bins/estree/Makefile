.PHONY: all
all:

.PHONY: test
test: codegen
	cargo nextest run --all-features

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src codegen

.PHONY: loggergen
loggergen:
	@$(MAKE) -s -C src loggergen

.PHONY: clean
clean:
	@$(MAKE) -s -C src clean
