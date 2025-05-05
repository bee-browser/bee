SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

.PHONY: codegen
codegen:
	@$(MAKE) -s -C backend/clir codegen

.PHONY: clean
clean:
	@$(MAKE) -s -C backend/clir clean
