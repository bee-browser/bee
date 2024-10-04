export PROJDIR := $(abspath ..)

# TODO: Is there any way to manage versions of packages installed by `cargo install`?
ADDR2LINE_VERSION := 0.24.1

# On my environment, the maximum size of memory usage of a linker is smaller than 10GB.
LLVM_LINK_MAX_RAM_USAGE := $(shell deno eval -p '10 * 1024 * 1024 * 1024')
DEFAULT_LLVM_PARALLEL_LINK_JOBS := \
  $(shell deno eval -p 'Math.floor(Deno.systemMemoryInfo().total / $(LLVM_LINK_MAX_RAM_USAGE))')

.PHONY: all
all: install

.PHONY: install
install: addr2line bytehound llvm v8

.PHONY: clean
clean:
	@rm -rf bin include lib share

.PHONY: addr2line
addr2line:
	@mkdir -p bin
	@cargo install --root=. --bin=addr2line --features=bin addr2line@$(ADDR2LINE_VERSION)

.PHONY: bytehound
bytehound:
	@sh $(PROJDIR)/tools/bin/install_bytehound.sh -c .

# TODO: Build LLVM w/ Clang using docker like bytehound.  GCC causes many warnings...
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
	  -D CMAKE_EXPORT_COMPILE_COMMANDS=ON \
	  -D CMAKE_INSTALL_PREFIX=. \
	  -D LLVM_TARGETS_TO_BUILD=X86 \
	  -D LLVM_ENABLE_ZLIB=OFF \
	  -D LLVM_ENABLE_ZSTD=OFF \
	  -D LLVM_PARALLEL_LINK_JOBS=$(LLVM_PARALLEL_LINK_JOBS)
	@cmake --build src/llvm/llvm-project/build
	@cmake --install src/llvm/llvm-project/build
	@mv src/llvm/llvm-project/build/compile_commands.json src/llvm/llvm-project/
	@rm -rf src/llvm/llvm-project/build

.PHONY: v8
v8:
	@sh $(PROJDIR)/tools/bin/install_v8.sh -c .
