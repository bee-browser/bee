export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)
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

SUBDIR_TARGETS=testgen
SUBDIRS=$(wildcard apps/* libs/*)

COVERAGE_ENV_VARS = \
  CARGO_INCREMENTAL=0 \
  RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
  RUSTDOCFLAGS="-Cpanic=abort"

.PHONY: all
all: build

$(TARGETS): $(SUBDIRS)

.PHONY: build
build:
	cargo build --release

.PHONY: test
test: run-testgen
	cargo test --release --all-features

.PHONY: clean
clean:
	cargo clean

.PHONY: debug-build
debug-build:
	cargo build

.PHONY: debug-test
debug-test: run-testgen
	cargo test --all-features

.PHONY: coverage
coverage: run-testgen
	env $(COVERAGE_ENV_VARS) cargo +nightly test --all-features

.PHONY: coverage-html
coverage-html: coverage
	grcov --llvm --branch --ignore-not-existing \
	  -s $(PROJDIR) -t html -o $(PROJDIR)/target/coverage --excl-line "//<coverage:exclude/>" \
	  --excl-start "//<coverage:exclude>" --excl-stop "//</coverage:exclude>" \
	  $(PROJDIR)/target/debug

.PHONY: github-workflows
github-workflows:
	@sh .github/workflows/update.sh

.PHONE: run-testgen
run-testgen:
	@make testgen

.PHONY: $(SUBDIR_TARGETS)
$(SUBDIR_TARGETS): $(SUBDIRS)

.PHONY: $(SUBDIRS)
$(SUBDIRS):
	-@make -C $@ $(MAKECMDGOALS)
