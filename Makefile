SHELL := $(shell which bash) -eu -o pipefail -c

export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)

BUILD_TARGETS := $(addprefix build-,\
  webui \
)

CODEGEN_PATHS := \
  bins/estree \
  libs/htmltokenizer \
  libs/htmlparser \
  libs/jsparser \
  libs/jsruntime \
  libs/layout

UPDATE_PATHS := \
  libs/jsruntime

CLEAN_TARGETS := $(addprefix clean-,\
  $(CODEGEN_PATHS) \
  webui \
)

CODEGEN_TARGETS := $(addprefix codegen-,\
  $(CODEGEN_PATHS) \
)

UPDATE_TARGETS := $(addprefix update-,\
  $(UPDATE_PATHS) \
)

.PHONY: all
all: build

.PHONY: list-targets
list-targets:
	@grep -E '^\.PHONY: ' $(MAKEFILE_LIST) | cut -d ' ' -f 2 | grep -v '^\$$' | sort

.PHONY: check
check: check-rust check-js

.PHONY: check-rust
check-rust:
	cargo fmt --all --check
	cargo check --workspace --all-targets --all-features
	cargo clippy --workspace --all-targets --all-features -- -D warnings -A 'clippy::collapsible_if'

.PHONY: check-js
# TODO
check-js:

.PHONY: build
build: OPTIONS ?=
build: $(BUILD_TARGETS)
	cargo build $(OPTIONS)

.PHONY: test
test: OPTIONS ?= --all-features
test: TESTNAME ?=
test:
	cargo nextest run $(OPTIONS) $(TESTNAME)

.PHONY: test262
test262: PROFILE ?= release
test262: ARGS ?= --progress
test262: OOP ?=
test262:
ifdef OOP
	cargo build --bin=bjs --profile=$(PROFILE) --all-features
	cargo run -r --bin=test262 --all-features -- --test262-dir=vendor/src/tc39/test262 $(ARGS) launch -- /bin/sh bins/test262/launchers/bjs.sh --profile=$(PROFILE) >test262.json
else
	cargo run --bin=test262 --profile=$(PROFILE) --all-features -- --test262-dir=vendor/src/tc39/test262 $(ARGS) run >test262.json
endif

# DO NOT REMOVE '-'.
# Continue the execution in order to generate the report even if test commands fail.
# TODO(test): Add test cases for jsruntime.
.PHONY: coverage
coverage: LLVM_COV_ARGS ?= --html
coverage: TEST262_ARGS ?=
coverage:
	cargo llvm-cov clean --workspace
	cargo llvm-cov nextest --no-report --all-features
	-sh bins/estree/scripts/test262_parser_tests.sh --profile=coverage $(TEST262_ARGS)
	-sh bins/estree/scripts/test262.sh --profile=coverage $(TEST262_ARGS)
	cargo llvm-cov report $(LLVM_COV_ARGS)

# TODO(test): Very slow...
# This takes nearly an hour on a high performance PC.
# This takes several hours in the GitHub Actions.
.PHONY: coverage-jsruntime
coverage-jsruntime: LLVM_COV_ARGS ?= --html
coverage-jsruntime: TEST262_ARGS ?=
coverage-jsruntime:
	cargo llvm-cov clean --workspace
	cargo llvm-cov run --bin=test262 --no-report --all-features -- --test262-dir=vendor/src/tc39/test262 $(TEST262_ARGS) run >/dev/null
	cargo llvm-cov report $(LLVM_COV_ARGS)

.PHONY: bench
bench: OPTIONS ?=
bench: BENCHNAME ?=
bench:
	cargo bench $(OPTIONS) $(BENCHNAME)

.PHONY: clean
clean: $(CLEAN_TARGETS)
	cargo clean --profile=dev
	cargo clean --profile=release
	cargo clean --profile=release-lto
	cargo clean --profile=release-symbols

.PHONY: clean-all
clean-all: $(CLEAN_TARGETS)
	cargo clean

# The order must be determined by dependencies between packages.
.PHONE: codegen
codegen:
	@$(MAKE) -s codegen-libs/htmltokenizer
	@$(MAKE) -s codegen-libs/htmlparser
	@$(MAKE) -s codegen-libs/jsparser
	@$(MAKE) -s codegen-libs/jsruntime
	@$(MAKE) -s codegen-libs/layout
	@$(MAKE) -s codegen-bins/estree

.PHONY: update
update:
	@$(MAKE) -s update-libs/jsruntime

.PHONY: update-deps
update-deps: update-deps-crates update-deps-deno

# Specify `CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git` if `make update-deps-crates` gets stuck.
# Perform `cargo update` after `cargo upgrade` in order to update `Cargo.lock`.
.PHONY: update-deps-crates
update-deps-crates:
	cargo upgrade -i allow
	cargo update

.PHONY: update-deps-deno
update-deps-deno:
	@deno upgrade
	@deno task update
	@deno eval "console.log('deno', Deno.version.deno)" >.tool-versions

.PHONY: update-devcontainer
update-devcontainer:
	@sh tools/bin/update_devcontainer_dockerfile.sh -c

.PHONY: doc
doc:
	cargo doc --workspace --all-features --document-private-items

.PHONY: format
format: format-rust format-js

.PHONY: format-rust
format-rust:
	@echo 'Formatting *.rs...'
	@cargo fmt --all

.PHONY: format-js
format-js:
	@echo 'Formatting *.js...'
	@deno fmt -q 2>/dev/null

.PHONY: vendor
vendor:
	@$(MAKE) -s -C vendor clean
	@$(MAKE) -s -C vendor install

.PHONY: $(BUILD_TARGETS)
$(BUILD_TARGETS):
	@$(MAKE) -s -C $(subst build-,,$@) build

.PHONY: $(CODEGEN_TARGETS)
$(CODEGEN_TARGETS):
	@$(MAKE) -s -C $(subst codegen-,,$@) codegen

.PHONY: $(UPDATE_TARGETS)
$(UPDATE_TARGETS):
	@$(MAKE) -s -C $(subst update-,,$@) update

.PHONY: $(CLEAN_TARGETS)
$(CLEAN_TARGETS):
	@$(MAKE) -s -C $(subst clean-,,$@) clean
