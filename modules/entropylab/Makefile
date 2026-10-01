all: module.a

override CXXFLAGS += -Wall -Wextra -fsanitize=address,fuzzer -std=c++20 -I ../../include

UNAME_S := $(shell uname -s)

# Default clang-format so this module never sees an empty variable.
ifeq ($(origin CLANG_FORMAT),undefined)
	CLANG_FORMAT := clang-format
endif
ifeq ($(strip $(CLANG_FORMAT)),)
	CLANG_FORMAT := clang-format
endif

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
	ifeq ($(CLANG_FORMAT),clang-format)
		HOMEBREW_CLANG_FORMAT := $(wildcard /opt/homebrew/opt/llvm/bin/clang-format)
		ifeq ($(HOMEBREW_CLANG_FORMAT),)
			HOMEBREW_CLANG_FORMAT := $(wildcard /usr/local/opt/llvm/bin/clang-format)
		endif
		ifneq ($(HOMEBREW_CLANG_FORMAT),)
			CLANG_FORMAT := $(HOMEBREW_CLANG_FORMAT)
		endif
	endif
	export CXX
	export CLANG_FORMAT
endif

CARGO_TARGET_DIR := $(shell pwd)/entropylab_fuzz_lib/target
RUST_LIB_PATH := $(CARGO_TARGET_DIR)/release/libentropylab_fuzz_lib.a

SYMBOL_PREFIX ?= entropylab

# Copy and sync entropylab-wasm source before building
sync-source:
	rm -rf entropylab_fuzz_lib/src
	mkdir -p entropylab_fuzz_lib/src/aez
	mkdir -p entropylab_fuzz_lib/src/psbt
	cp ../../external/entropylab/entropylab-wasm/src/lib.rs entropylab_fuzz_lib/src/lib.rs
	cp ../../external/entropylab/entropylab-wasm/src/descriptor.rs entropylab_fuzz_lib/src/descriptor.rs
	cp ../../external/entropylab/entropylab-wasm/src/aez/*.rs entropylab_fuzz_lib/src/aez/
	cp ../../external/entropylab/psbt-wasm/src/lib.rs entropylab_fuzz_lib/src/psbt/mod.rs
	cp ../../external/entropylab/psbt-wasm/src/verify.rs entropylab_fuzz_lib/src/psbt/verify.rs
	cp ../../external/entropylab/psbt-wasm/src/scriptcode.rs entropylab_fuzz_lib/src/psbt/scriptcode.rs
	cp ../../external/entropylab/psbt-wasm/src/sanitize.rs entropylab_fuzz_lib/src/psbt/sanitize.rs
	# Patch upstream psbt-wasm imports for nested-module layout
	sed -i.bak 's/use crate::{hex_encode, pair_type_name, read_varint, RawPair};/use super::{hex_encode, pair_type_name, read_varint, RawPair};/' entropylab_fuzz_lib/src/psbt/sanitize.rs
	sed -i.bak 's/use crate::scriptcode;/use super::scriptcode;/' entropylab_fuzz_lib/src/psbt/verify.rs
	sed -i.bak 's/use crate::{hex_encode, pair_utxo_claim, tx_sanity_error, RawPair};/use super::{hex_encode, pair_utxo_claim, tx_sanity_error, RawPair};/' entropylab_fuzz_lib/src/psbt/verify.rs
	sed -i.bak 's/crate::hex_decode/super::hex_decode/g' entropylab_fuzz_lib/src/psbt/verify.rs
	sed -i.bak 's/^fn hex_decode/pub(crate) fn hex_decode/' entropylab_fuzz_lib/src/psbt/mod.rs
	rm -f entropylab_fuzz_lib/src/psbt/*.bak
	echo 'mod psbt;' >> entropylab_fuzz_lib/src/lib.rs
	cat bitcoinfuzz_wrappers.rs >> entropylab_fuzz_lib/src/lib.rs
	cat psbt_bitcoinfuzz_wrappers.rs >> entropylab_fuzz_lib/src/lib.rs
	-cd entropylab_fuzz_lib && cargo fmt

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

format: sync-source
	$(CLANG_FORMAT) -i ./module.cpp ./module.h ./entropylab_fuzz_lib/entropylab_fuzz_lib.h

check-format:
	$(CLANG_FORMAT) -Werror --fail-on-incomplete-format -n ./module.cpp ./module.h ./entropylab_fuzz_lib/entropylab_fuzz_lib.h
	cd entropylab_fuzz_lib && cargo fmt --check

clean:
	rm -rf *.o module.a
	rm -rf entropylab_fuzz_lib/src
	rm -rf entropylab_fuzz_lib/target

.PHONY: all cargo clean format check-format $(RUST_LIB_PATH)
