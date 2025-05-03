PROJ_DIR := ../../..
ECMA262_DIR := $(PROJ_DIR)/vendor/src/tc39/ecma262

# targets

.PHONY: all
all: codegen

.PHONY: codegen
codegen:
	@$(MAKE) -s -C lexer codegen
	@$(MAKE) -s -C parser codegen
	@$(MAKE) -s -C symbol codegen
	@$(MAKE) -s -C syntax codegen

.PHONY: clean
clean:
	@rm -f $(CODEGEN_TARGETS)
	@$(MAKE) -s -C lexer clean
	@$(MAKE) -s -C parser clean
	@$(MAKE) -s -C symbol clean
	@$(MAKE) -s -C syntax clean
