export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)
export BEE_VENDOR_DIR := $(PROJDIR)/vendor
export BEE_CARGO_CODEGEN_DIR := $(PROJDIR)/target/codegen

# These are used in the "lldb.launch.sourceMap" property in //.vscode/settings.json.
export BEE_DEV_RUSTC_COMMIT_HASH := $(shell rustc -vV | grep 'commit-hash' | cut -d ' ' -f 2)
export BEE_DEV_RUST_TOOLCHAIN_PATH := $(shell rustup toolchain list -v | grep '(default)' | cut -f 2)

# NOTES
# -----
# In this project, commands in //tools/bin are used in build scripts.  For example, bee-codegen is
# used for generating source files.  So, we need to add the path to the PATH enviroment before
# building.
#
# The `env` property in //.vscode/launch.json doesn't work for this purpose as you expected.  See:
# https://github.com/vadimcn/vscode-lldb/blob/v1.6.0/extension/cargo.ts#L204
#
# The `lldb.adapterEnv` property works, but it doesn't support the variable substitusion like
# below:
#
#   "lldb.adapterEnv": {
#     "PATH": "${workspaceFolder}/tools/bin:${env:PATH}"
#   }
#
# We've solved this issue by exporting enviroments before launching VSCode as you can see in the
# `launch-vscode` task below.
#BEE_DEV_CONTAINER_RUSTC_COMMIT_HASH=$(docker run --rm mcr.microsoft.com/vscode/devcontainers/rust rustc -vV | grep 'commit-hash' | cut -d ' ' -f 2)
#BEE_DEV_CONTAINER_RUST_TOOLCHAIN_PATH=$(docker run --rm mcr.microsoft.com/vscode/devcontainers/rust rustup toolchain list -v | grep '(default)' | cut -f 2)

BUILD_TARGETS = $(addprefix build-,\
  webui \
)

CLEAN_TARGETS = $(addprefix clean-,\
  webui \
)

# TODO: Enable codegen for lib/layout.
# bee-tools-dom-scrape doesn't work with deno/1.33.0 + Chrome/113.
# puppeteer.launch() fails due to "NetworkError: failed to connect to WebSocket: Invalid status code".
CODEGEN_TARGETS = $(addprefix codegen-,\
  libs/htmltokenizer \
  libs/htmlparser \
  libs/jsparser \
)

COVERAGE_TEST_ENV_VARS = \
  RUSTC_BOOTSTRAP=1 \
  CARGO_INCREMENTAL=0 \
  RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
  RUSTDOCFLAGS="-Cpanic=abort"

GRCOV_COMMON_ARGS = \
  $(PROJDIR)/target/debug \
  --branch --llvm --ignore-not-existing -s $(PROJDIR) \
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
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print $$1}}' | sort | grep -E -v -e '^[^[:alnum:]]' -e '^$@$$'

.PHONY: check
check: format
	cargo check --release --all-features

.PHONY: build
build: format codegen $(BUILD_TARGETS)
	cargo build --release

.PHONY: test
test: format codegen install-nextest
	cargo nextest run --all-features
	deno test --parallel --shuffle

.PHONY: bench
bench:
	cargo bench

.PHONY: clean
clean: $(CLEAN_TARGETS)
	cargo clean

.PHONY: debug-build
debug-build: format codegen $(BUILD_TARGETS)
	cargo build

.PHONY: coverage-test
coverage-test: format codegen
	env $(COVERAGE_TEST_ENV_VARS) cargo test --all-features

.PHONY: coverage-lcov
coverage-lcov: coverage-test install-grcov | $(PROJDIR)/target/coverage
	grcov $(GRCOV_COMMON_ARGS) -t lcov -o $(PROJDIR)/target/coverage/lcov.info

.PHONY: coverage-html
coverage-html: coverage-test install-grcov | $(PROJDIR)/target/coverage
	grcov $(GRCOV_COMMON_ARGS) -t html -o $(PROJDIR)/target/coverage

.PHONE: codegen
codegen: $(CODEGEN_TARGETS)

.PHONY: update-deps
update-deps: update-deps-crates update-deps-deno

.PHONY: update-deps-crates
update-deps-crates:
	cargo upgrade -i allow

.PHONY: update-deps-deno
update-deps-deno:
	@find -name '*.js' | xargs deno run --allow-net --allow-read --allow-write https://raw.githubusercontent.com/masnagam/deno-udd/fix-issue-86/main.ts
	@ls -1 tools/bin/* | xargs deno run --allow-net --allow-read --allow-write https://raw.githubusercontent.com/masnagam/deno-udd/fix-issue-86/main.ts

.PHONY: doc
doc: format
	cargo doc --workspace --all-features

.PHONY: format
format:
	cargo fmt --all

.PHONY: install-grcov
install-grcov:
	cargo install grcov

.PHONY: install-nextest
install-nextest:
	cargo install cargo-nextest

.PHONY: github-ci
github-ci:
	@echo "$GITHUB_WORKSPACE/tools/bin" >>$GITHUB_PATH

.PHONY: github-workflows
github-workflows:
	@sh .github/workflows/update.sh

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
