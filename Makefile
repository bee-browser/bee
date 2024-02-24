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
	cargo clean --profile=dev
	cargo clean --profile=profiling
	cargo clean --profile=release

.PHONY: clean-all
clean-all: $(CLEAN_TARGETS)
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
doc:
	cargo doc --workspace --all-features

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

# On my environment, the maximum size of memory usage of a linker is smaller than 10GB.
LLVM_LINK_MAX_RAM_USAGE := $(shell deno eval -p '10 * 1024 * 1024 * 1024')
DEFAULT_LLVM_PARALLEL_LINK_JOBS := \
  $(shell deno eval -p 'Math.floor(Deno.systemMemoryInfo().total / $(LLVM_LINK_MAX_RAM_USAGE))')

.PHONY: vendor
# See //vendor/src/llvm/llvm-project/llvm/docs/GettingStarted.rst
#
# Note: `make` creates more link jobs than `LLVM_PARALLEL_LINK_JOBS` when specifying the number
# of jobs by using `-j` in `cmake --build` command.  Without `-j`, `make` uses only a single
# process.  On the other hand, `ninja` works fine without `-j` in `cmake --build`.
#
# TODO: debug fission
# TODO: CPUs other than x86/x64
vendor: LLVM_PARALLEL_LINK_JOBS ?= $(DEFAULT_LLVM_PARALLEL_LINK_JOBS)
vendor:
	cmake -G Ninja -S vendor/src/llvm/llvm-project/llvm -B vendor/src/llvm/llvm-project/build \
	  -D CMAKE_BUILD_TYPE=RelWithDebInfo \
	  -D CMAKE_INSTALL_PREFIX=vendor \
	  -D LLVM_TARGETS_TO_BUILD=X86 \
	  -D LLVM_ENABLE_ZLIB=OFF \
	  -D LLVM_ENABLE_ZSTD=OFF \
	  -D LLVM_PARALLEL_LINK_JOBS=$(LLVM_PARALLEL_LINK_JOBS)
	cmake --build vendor/src/llvm/llvm-project/build
	cmake --install vendor/src/llvm/llvm-project/build

.PHONY: vendor-clean
vendor-clean:
	@rm -rf vendor/src/llvm/llvm-project/build
	@rm -rf vendor/bin vendor/include vendor/lib vendor/share

.PHONY: $(BUILD_TARGETS)
$(BUILD_TARGETS):
	@make -s -C $(subst build-,,$@) build

.PHONY: $(CODEGEN_TARGETS)
$(CODEGEN_TARGETS):
	@make -s -C $(subst codegen-,,$@) codegen

.PHONY: $(CLEAN_TARGETS)
$(CLEAN_TARGETS):
	@make -s -C $(subst clean-,,$@) clean
