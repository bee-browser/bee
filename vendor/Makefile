# On my environment, the maximum size of memory usage of a linker is smaller than 10GB.
LLVM_LINK_MAX_RAM_USAGE := $(shell deno eval -p '10 * 1024 * 1024 * 1024')
DEFAULT_LLVM_PARALLEL_LINK_JOBS := \
  $(shell deno eval -p 'Math.floor(Deno.systemMemoryInfo().total / $(LLVM_LINK_MAX_RAM_USAGE))')

.PHONY: install
install: addr2line llvm

.PHONY: addr2line
addr2line:
	@mkdir -p bin
	@cargo install --root=. --example=addr2line addr2line

.PHONY: llvm
# See //vendor/src/llvm/llvm-project/llvm/docs/GettingStarted.rst
#
# Note: `make` creates more link jobs than `LLVM_PARALLEL_LINK_JOBS` when specifying the number
# of jobs by using `-j` in `cmake --build` command.  Without `-j`, `make` uses only a single
# process.  On the other hand, `ninja` works fine without `-j` in `cmake --build`.
#
# TODO: debug fission
# TODO: CPUs other than x86/x64
llvm: LLVM_PARALLEL_LINK_JOBS ?= $(DEFAULT_LLVM_PARALLEL_LINK_JOBS)
llvm:
	@cmake -G Ninja -S src/llvm/llvm-project/llvm -B src/llvm/llvm-project/build \
	  -D CMAKE_BUILD_TYPE=RelWithDebInfo \
	  -D CMAKE_INSTALL_PREFIX=. \
	  -D LLVM_TARGETS_TO_BUILD=X86 \
	  -D LLVM_ENABLE_ZLIB=OFF \
	  -D LLVM_ENABLE_ZSTD=OFF \
	  -D LLVM_PARALLEL_LINK_JOBS=$(LLVM_PARALLEL_LINK_JOBS)
	@cmake --build src/llvm/llvm-project/build
	@cmake --install src/llvm/llvm-project/build
	@rm -rf src/llvm/llvm-project/build

.PHONY: clean
clean:
	@rm -rf bin include lib share
