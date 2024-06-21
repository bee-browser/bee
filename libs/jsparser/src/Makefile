# Specify `-d` if you want to see debug logs.
TRANSPILE_ARGS ?=

PROJ_DIR := $(realpath ../../..)
ECMA262_DIR := $(PROJ_DIR)/vendor/src/tc39/ecma262

ECMA262_VERSION := es2023
ifneq ($(shell git -C $(ECMA262_DIR) describe),$(ECMA262_VERSION))
  $(error ECMA-262 specification version must be $(ECMA262_VERSION))
endif

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen:
	@$(MAKE) -s -C lexer codegen TRANSPILE_ARGS=$(TRANSPILE_ARGS)
	@$(MAKE) -s -C parser codegen TRANSPILE_ARGS=$(TRANSPILE_ARGS)
	@$(MAKE) -s -C symbol codegen
	@$(MAKE) -s -C syntax codegen

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)
	@$(MAKE) -s -C lexer clean
	@$(MAKE) -s -C parser clean
	@$(MAKE) -s -C symbol clean
	@$(MAKE) -s -C syntax clean
