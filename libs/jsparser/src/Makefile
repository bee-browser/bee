.PHONY: all
all: codegen

.PHONY: codegen
codegen:
	@$(MAKE) -s -C lexer codegen
	@$(MAKE) -s -C parser codegen
	@$(MAKE) -s -C symbol codegen
	@$(MAKE) -s -C syntax codegen

.PHONY: loggergen
loggergen:
	@$(MAKE) -s -C lexer loggergen
	@$(MAKE) -s -C parser loggergen
	@$(MAKE) -s -C syntax loggergen

.PHONY: clean
clean:
	@$(MAKE) -s -C lexer clean
	@$(MAKE) -s -C parser clean
	@$(MAKE) -s -C symbol clean
	@$(MAKE) -s -C syntax clean
