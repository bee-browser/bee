export PROJDIR := $(abspath ..)

# TODO: Is there any way to manage versions of packages installed by `cargo install`?
ADDR2LINE_VERSION := 0.24.2

.PHONY: all
all: install

.PHONY: install
install: addr2line bytehound v8

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

.PHONY: v8
v8:
	@sh $(PROJDIR)/tools/bin/install_v8.sh -c .
