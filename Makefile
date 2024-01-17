SHELL := $(shell which bash) -eu -o pipefail -c

export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)

BUILD_TARGETS = $(addprefix build-,\
  webui \
)

CLEAN_TARGETS = $(addprefix clean-,\
  webui \
)

# The order must be determined by dependencies between packages.
CODEGEN_TARGETS = $(addprefix codegen-,\
  libs/logging \
  libs/htmltokenizer \
  libs/htmlparser \
  libs/jsparser \
  libs/layout \
  bins/estree \
)

.PHONY: all
all: build

.PHONY: list
list:
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | \
	  awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | \
	  sort | \
	  grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: check
check:
	cargo fmt --all --check
	cargo check --workspace --all-targets --all-features
	cargo clippy --workspace --all-targets --all-features -- -D warnings

.PHONY: build
build: format $(BUILD_TARGETS)
	cargo build

.PHONY: test
test: format
	cargo nextest run --all-features

# TODO: remove '-' once we've fixed all failures.
.PHONY: test262
test262: ARGS ?= --progress
test262:
	-sh bins/estree/scripts/test262_parser_tests.sh $(ARGS)
	-sh bins/estree/scripts/test262.sh $(ARGS)

# DO NOT REMOVE '-'.
# Continue the execution in order to generate the report even if a command fails.
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
	cargo clean

.PHONY: release-build
release-build: $(BUILD_TARGETS)
	cargo build --release

.PHONY: release-test
release-test:
	cargo nextest run --release --all-features

.PHONE: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: loggergen
loggergen:
	@sh libs/logging/scripts/loggergen.sh

.PHONY: update-deps
update-deps: update-deps-crates update-deps-deno

# Specify `CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git` if `make update-deps-crates` gets stuck.
.PHONY: update-deps-crates
update-deps-crates:
	cargo upgrade -i allow

.PHONY: update-deps-deno
update-deps-deno:
	@find -name '*.js' -not -path './vendor/*' | \
	  xargs deno run --allow-net --allow-read --allow-write \
	    https://raw.githubusercontent.com/masnagam/deno-udd/fix-issue-86/main.ts

.PHONY: update-devcontainer
update-devcontainer:
	@sh .devcontainer/update-dockerfile-env.sh -c

.PHONY: doc
doc: format
	cargo doc --workspace --all-features

.PHONY: format
format:
	cargo fmt --all

.PHONY: $(BUILD_TARGETS)
$(BUILD_TARGETS):
	@make -s -C $(subst build-,,$@) build

.PHONY: $(CODEGEN_TARGETS)
$(CODEGEN_TARGETS):
	@make -s -C $(subst codegen-,,$@) codegen

.PHONY: $(CLEAN_TARGETS)
$(CLEAN_TARGETS):
	@make -s -C $(subst clean-,,$@) clean
