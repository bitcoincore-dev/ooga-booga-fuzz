all: module.a

override CXXFLAGS += -Wall -Wextra -fsanitize=address,fuzzer -std=c++20 -I ../../include

UNAME_S := $(shell uname -s)

# macOS: Apple Clang does not ship libfuzzer. Auto-detect Homebrew LLVM.
ifeq ($(UNAME_S), Darwin)
	ifeq ($(origin CXX),default)
		HOMEBREW_LLVM := $(wildcard /opt/homebrew/opt/llvm/bin/clang++)
		ifeq ($(HOMEBREW_LLVM),)
			HOMEBREW_LLVM := $(wildcard /usr/local/opt/llvm/bin/clang++)
		endif
		ifneq ($(HOMEBREW_LLVM),)
			CXX := $(HOMEBREW_LLVM)
		else
			$(warning Apple Clang does not support -fsanitize=fuzzer. Run: brew install llvm)
		endif
	endif
	export CXX
endif

CARGO_TARGET_DIR := $(shell pwd)/entropylab_fuzz_lib/target
RUST_LIB_PATH := $(CARGO_TARGET_DIR)/release/libentropylab_fuzz_lib.a

SYMBOL_PREFIX ?= entropylab

# Copy and sync entropylab-wasm source before building
sync-source:
	rm -rf entropylab_fuzz_lib/src
	mkdir -p entropylab_fuzz_lib/src/aez
	cp ../../external/entropylab/entropylab-wasm/src/lib.rs entropylab_fuzz_lib/src/lib.rs
	cp ../../external/entropylab/entropylab-wasm/src/descriptor.rs entropylab_fuzz_lib/src/descriptor.rs
	cp ../../external/entropylab/entropylab-wasm/src/aez/*.rs entropylab_fuzz_lib/src/aez/
	cat bitcoinfuzz_wrappers.rs >> entropylab_fuzz_lib/src/lib.rs

$(RUST_LIB_PATH): sync-source
	cd entropylab_fuzz_lib && \
	CARGO_TARGET_DIR="$(CARGO_TARGET_DIR)" \
	RUSTFLAGS="\
		-Z sanitizer=address \
		-C passes=sancov-module \
		-C llvm-args=-sanitizer-coverage-inline-8bit-counters \
		-C llvm-args=-sanitizer-coverage-trace-compares \
		-C llvm-args=-sanitizer-coverage-pc-table \
		-C llvm-args=-sanitizer-coverage-level=4 \
		-C llvm-args=-simplifycfg-branch-fold-threshold=0" \
	cargo +nightly build --release

cargo: $(RUST_LIB_PATH)

module.a: module.o $(RUST_LIB_PATH)
	bash ../../scripts/merge.sh module.a $(RUST_LIB_PATH) module.o
	bash ../../scripts/rename_rust_secp_symbols.sh module.a $(SYMBOL_PREFIX)
	ranlib module.a

module.o: module.cpp module.h
	$(CXX) $(CXXFLAGS) -I . -fPIC -c module.cpp -o module.o

format:
	$(CLANG_FORMAT) -i ./module.cpp ./module.h ./entropylab_fuzz_lib/entropylab_fuzz_lib.h
	cd entropylab_fuzz_lib && cargo fmt

check-format:
	$(CLANG_FORMAT) -Werror --fail-on-incomplete-format -n ./module.cpp ./module.h ./entropylab_fuzz_lib/entropylab_fuzz_lib.h
	cd entropylab_fuzz_lib && cargo fmt --check

clean:
	rm -rf *.o module.a
	rm -rf entropylab_fuzz_lib/src
	rm -rf entropylab_fuzz_lib/target

.PHONY: all cargo clean format check-format $(RUST_LIB_PATH)
