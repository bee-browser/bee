SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin
LOGGERGEN_SH := $(PROJ_DIR)/libs/logging/scripts/loggergen.sh
LOGGERGEN_DEPS := $(shell sh $(LOGGERGEN_SH) deps)

LOGGERGEN_TARGETS := src/logger.rs

.PHONY: all
all:

.PHONY: loggergen
loggergen: $(LOGGERGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(LOGGERGEN_TARGETS)

src/logger.rs: $(LOGGERGEN_DEPS)
	@echo 'Generating $(abspath $@)...'
	@sh $(LOGGERGEN_SH) codegen '{"module":"","target":"bee::lalrgen"}' >$@
