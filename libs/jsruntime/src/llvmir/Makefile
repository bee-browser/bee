SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := \
  runtime.hh \
  executor.codegen.cc \
  type_holder.cc \
  type_holder.hh

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)

# Specify --Wno-error=unknown in clang-format in order to avoid errors caused by unknown options
# in //.clang-format.  clang-format used in CI jobs may be older than the local one.
%:: runtime.yaml %.njk runtime.js $(TOOLS_BIN)/nunjucks.js $(TOOLS_BIN)/update_file.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. runtime.js $< | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $@.njk | \
	  clang-format --Wno-error=unknown | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
