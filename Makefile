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
  libs/htmltokenizer \
  libs/htmlparser \
  libs/jsparser \
  libs/layout \
  bins/estree \
)

COVERAGE_TEST_ENV_VARS = \
  RUSTC_BOOTSTRAP=1 \
  CARGO_INCREMENTAL=0 \
  RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
  RUSTDOCFLAGS="-Cpanic=abort"

GRCOV_COMMON_ARGS = \
  $(PROJDIR) -s $(PROJDIR) \
  --binary-path $(PROJDIR)/target/debug \
  --branch --llvm --ignore-not-existing \
  --ignore '*/src/main.rs' \
  --excl-line '<coverage:exclude/>|unimplemented!|unreachable!' \
  --excl-start '<coverage:exclude>' \
  --excl-stop '</coverage:exclude>' \
  --excl-br-start '<coverage:exclude>' \
  --excl-br-stop '</coverage:exclude>'

.PHONY: all
all: build

.PHONY: list
list:
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | \
	  awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | \
	  sort | \
	  grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: check
check: format
	cargo check --release --all-features

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

.PHONY: coverage-test
coverage-test:
	env $(COVERAGE_TEST_ENV_VARS) $(MAKE) -s test

.PHONY: coverage-test262
coverage-test262: ARGS ?=
coverage-test262:
	env $(COVERAGE_TEST_ENV_VARS) $(MAKE) -s test262 ARGS=$(ARGS)

.PHONY: coverage-lcov
coverage-lcov: | $(PROJDIR)/target/coverage
	grcov $(GRCOV_COMMON_ARGS) -t lcov -o $(PROJDIR)/target/coverage/lcov.info

.PHONY: coverage-html
coverage-html: | $(PROJDIR)/target/coverage
	grcov $(GRCOV_COMMON_ARGS) -t html -o $(PROJDIR)/target/coverage

.PHONE: codegen
codegen: $(CODEGEN_TARGETS)

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

$(PROJDIR)/target/coverage:
	@mkdir -p $@
