.PHONY: all
all:

.PHONY: test
test: codegen
	cargo nextest run --all-features

.PHONY: codegen
codegen: update-data
	@$(MAKE) -s -C src codegen

.PHONY: update-data
update-data:
	@$(MAKE) -s -C data update
