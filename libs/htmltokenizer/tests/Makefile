SHELL := $(shell which bash) -eu -o pipefail -c

PROJ_DIR := ../../..
TOOLS_BIN := $(PROJ_DIR)/tools/bin

DATA_DIR := $(PROJ_DIR)/vendor/src/html5lib/html5lib-tests/tokenizer
EXCLUDES := xmlViolation
NAMES := $(filter-out $(EXCLUDES),$(subst .test,,$(notdir $(wildcard $(DATA_DIR)/*.test))))

define html5lib_tests
html5lib_tests_$(shell echo $1 | sed -e 's/\([A-Z]\)/_\L\1/g').rs: $$(DATA_DIR)/$1.test html5lib_tests.js html5lib_tests.rs.hbs
	@echo 'Generating $$(abspath $$@)...'
	@deno run -q --allow-read=$$< html5lib_tests.js $$< | \
	  deno run -q --allow-read=. $$(TOOLS_BIN)/codegen.js --input-stdin --no-escape html5lib_tests.rs.hbs $$< | \
	  rustfmt --emit=stdout | \
	  deno run -q --allow-read=. --allow-write=$$@ $(TOOLS_BIN)/update_file.js $$@
SRCS += html5lib_tests_$(shell echo $1 | sed -e 's/\([A-Z]\)/_\L\1/g').rs
endef

$(foreach NAME,$(NAMES),$(eval $(call html5lib_tests,$(NAME))))

.PHONY: codegen
codegen: $(SRCS)

.PHONY: clean
clean:
	@rm -f $(SRCS)
