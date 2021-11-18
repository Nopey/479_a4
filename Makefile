# quick n dirty makefile to build rust binary
# based off the output of `cargo build --release --verbose` and `cargo build --verbose`

SHELL := /bin/bash

PROJECT := hfkb
ROOT := $(shell realpath .)
FLAGS = --edition=2018
COMMON = main.rs --crate-type bin
CAPI = capi.rs --crate-type=staticlib -C panic=abort
RUSTC = rustc $(FLAGS) --emit=dep-info,link

# build everything
.PHONY: all
all: $(PROJECT) $(PROJECT)_dbg c$(PROJECT) c$(PROJECT)_dbg doc

# C++ release build
c$(PROJECT): main.cpp KB.h lib$(PROJECT).a
	g++ $^ -o $@ -Wall -Wextra -Wpedantic -lpthread -ldl -O3

# C++ debug build
c$(PROJECT)_dbg: main.cpp KB.h lib$(PROJECT)_dbg.a
	g++ $^ -o $@ -Wall -Wextra -Wpedantic -lpthread -ldl -ggdb

# Rust lib release build
lib$(PROJECT).a:
	$(RUSTC) $(CAPI) --crate-name $(PROJECT) -C opt-level=3

# Rust lib debug build
lib$(PROJECT)_dbg.a:
	$(RUSTC) $(CAPI) --crate-name $(PROJECT)_dbg -C debuginfo=2

# Rust release build
$(PROJECT):
	$(RUSTC) $(COMMON) --crate-name $@ -C opt-level=3
-include $(PROJECT).d

# Rust debug build
$(PROJECT)_dbg:
	$(RUSTC) $(COMMON) --crate-name $@ -C debuginfo=2
-include $(PROJECT)_dbg.d

# Generate Documentation
# (Try opening doc/$(PROJECT)/index.html in a browser)
.PHONY: doc
doc:
	rustdoc $(COMMON) --crate-name $(PROJECT) --document-private-items

# Benchmark
# (Prefer hyperfine if it is present, but fall back to time)
.PHONY: bench
ifeq (, $(shell which hyperfine))
bench: $(PROJECT)
	time ./$(PROJECT)
else
bench: $(PROJECT)
	@# Hyperfine needs the command to be in quotes
	hyperfine './$(PROJECT)'
endif

# professor-proofing the makefile by adding aliases
.PHONY: docs build build_dbg debug dbg benchmark
docs: doc
build: $(PROJECT)
build_dbg: $(PROJECT)_dbg
debug: $(PROJECT)_dbg
dbg: $(PROJECT)_dbg
benchmark: bench


# Clean build dir
.PHONY: clean
clean:
	rm -rf $(PROJECT) $(PROJECT).d $(PROJECT)_dbg $(PROJECT)_dbg.d doc $(PROJECT)*.o lib$(PROJECT).a lib$(PROJECT)_dbg.a c$(PROJECT) c$(PROJECT)_dbg
