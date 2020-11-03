export PATH := $(abspath tools/bin):$(PATH)
export PROJDIR := $(abspath .)
export CARGO_TARGET_DIR := $(PROJDIR)/target

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

TARGETS=build test coverage clean
SUBDIRS=$(wildcard libs/*)

$(TARGETS): $(SUBDIRS)

github-workflows:
	@sh .github/workflows/update.sh

$(SUBDIRS):
	@make -C $@ $(MAKECMDGOALS)

.PHONY: $(TARGETS) $(SUBDIRS) github-workflows
