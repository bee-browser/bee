.PHONY: all
all:

.PHONY: test
test:
	@cargo nextest run

.PHONY: codegen
codegen:
	@$(MAKE) -s -C src codegen
	@$(MAKE) -s -C tests codegen

.PHONY: loggergen
loggergen:
	@$(MAKE) -s -C src loggergen

.PHONY: clean
clean:
	@$(MAKE) -s -C src clean
	@$(MAKE) -s -C tests clean
