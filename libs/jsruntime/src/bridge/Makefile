SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := $(realpath ../../../..)
TOOLS_BIN := $(PROJ_DIR)/tools/bin

CODEGEN_TARGETS := \
  runtime.hh \
  llvmir/executor.codegen.cc \
  llvmir/type_holder.cc \
  llvmir/type_holder.hh

.PHONY: codegen
codegen: $(CODEGEN_TARGETS)

%:: runtime.yaml %.njk runtime.js $(TOOLS_BIN)/nunjucks.js $(TOOLS_BIN)/update_file.js
	@echo 'Generating $(abspath $@)...'
	@deno run -q --allow-read=. runtime.js $< | \
	  deno run -q --allow-read=. $(TOOLS_BIN)/nunjucks.js $@.njk | \
	  clang-format | \
	  deno run -q --allow-read=. --allow-write=$@ $(TOOLS_BIN)/update_file.js $@
