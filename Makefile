SHELL := $(shell which bash) -eu -o pipefail -c

export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)

CODEGEN_PATHS := \
  libs/logging \
  libs/htmltokenizer \
  libs/htmlparser \
  libs/jsparser \
  libs/jsruntime \
  libs/layout \
  bins/estree

BUILD_TARGETS := $(addprefix build-,\
  webui \
)

CLEAN_TARGETS := $(addprefix clean-,\
  $(CODEGEN_PATHS) \
  webui \
)

# The order must be determined by dependencies between packages.
CODEGEN_TARGETS := $(addprefix codegen-,$(CODEGEN_PATHS))

.PHONY: all
all: build

.PHONY: list
list:
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | \
	  awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | \
	  sort | \
	  grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: check
check: check-rust check-cxx check-js

.PHONY: check-rust
check-rust:
	cargo fmt --all --check
	cargo check --workspace --all-targets --all-features
	cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: check-cxx
# TODO
check-cxx:

.PHONY: check-js
# TODO
check-js:

.PHONY: build
build: $(BUILD_TARGETS)
	cargo build

.PHONY: test
test:
	cargo nextest run --all-features

# TODO: remove '-' once we've fixed all failures.
.PHONY: test262
test262: ARGS ?= --progress
test262:
	-sh bins/estree/scripts/test262_parser_tests.sh $(ARGS)
	-sh bins/estree/scripts/test262.sh $(ARGS)

# DO NOT REMOVE '-'.
# Continue the execution in order to generate the report even if test commands fail.
.PHONY: coverage
coverage: LLVM_COV_ARGS ?= --html
coverage: TEST262_ARGS ?= --progress
coverage:
	cargo llvm-cov clean --workspace
	-cargo llvm-cov nextest --no-report --all-features
	-$(MAKE) test262 ARGS='--mode=coverage $(TEST262_ARGS)'
	cargo llvm-cov report $(LLVM_COV_ARGS)

.PHONY: bench
bench:
	cargo bench

.PHONY: clean
clean: $(CLEAN_TARGETS)
	@bash libs/logging/scripts/loggergen.sh --rm
	cargo clean --profile=dev
	cargo clean --profile=profiling
	cargo clean --profile=release
	cargo clean --profile=release-lto

.PHONY: clean-all
clean-all: $(CLEAN_TARGETS)
	cargo clean

.PHONY: release-build
release-build: $(BUILD_TARGETS)
	cargo build --release

.PHONY: release-test
release-test:
	cargo nextest run --release --all-features

# TODO: `make -j $(nproc) codegen` does not work properly...
.PHONE: codegen
codegen:
	@bash libs/logging/scripts/loggergen.sh
	@$(MAKE) -s codegen-modules

.PHONY: codegen-modules
codegen-modules: $(CODEGEN_TARGETS)

.PHONY: update-deps
update-deps: update-deps-crates update-deps-deno

# Specify `CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git` if `make update-deps-crates` gets stuck.
.PHONY: update-deps-crates
update-deps-crates:
	cargo upgrade -i allow

.PHONY: update-deps-deno
update-deps-deno:
	@deno task update

.PHONY: update-devcontainer
update-devcontainer:
	@sh .devcontainer/update-dockerfile-env.sh -c

.PHONY: doc
doc:
	cargo doc --workspace --all-features --document-private-items

.PHONY: format
format: format-rust format-cxx format-js

.PHONY: format-rust
format-rust:
	@echo 'Formatting *.rs...'
	@cargo fmt --all

.PHONY: format-cxx
format-cxx:
	@echo 'Formatting *.[cc|hh]...'
	@find . -name '*.cc' -o -name '*.hh' | grep -v './target/' | grep -v './vendor/' | \
	  xargs clang-format -i

.PHONY: format-js
format-js:
	@echo 'Formatting *.js...'
	@deno fmt -q 2>/dev/null

.PHONY: vendor
vendor:
	@$(MAKE) -s -C vendor clean
	@$(MAKE) -s -C vendor install
	@mv vendor/src/llvm/llvm-project/compile_commands.json ./

.PHONY: $(BUILD_TARGETS)
$(BUILD_TARGETS):
	@$(MAKE) -s -C $(subst build-,,$@) build

.PHONY: $(CODEGEN_TARGETS)
$(CODEGEN_TARGETS):
	@$(MAKE) -s -C $(subst codegen-,,$@) codegen

.PHONY: $(CLEAN_TARGETS)
$(CLEAN_TARGETS):
	@$(MAKE) -s -C $(subst clean-,,$@) clean
