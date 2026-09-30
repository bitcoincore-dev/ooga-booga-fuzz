all: help

help:
	@echo "bitcoinfuzz - Differential fuzzing of Bitcoin implementations"
	@echo ""
	@echo "Usage:"
	@echo "  make bitcoinfuzz          Build the fuzzer binary"
	@echo "  make help                 Show this help message"
	@echo "  make clean                Remove build artifacts"
	@echo "  make setup-macos          Install Homebrew LLVM (macOS only)"
	@echo "  make run-macos            Run the fuzzer (requires FUZZ=target)"
	@echo "  make run-entropylab       Run all Entropylab fuzz targets (auto-detects OS)"
	@echo "  make run-macos-entropylab Run all Entropylab fuzz targets (macOS)"
	@echo "  make format               Format C++ code"
	@echo "  make check-format         Check C++ code formatting"
	@echo ""
	@echo "Examples:"
	@echo "  CXXFLAGS=\"-DENTROPYLAB\" make bitcoinfuzz"
	@echo "  FUZZ=bip32_master_keygen ./bitcoinfuzz"
	@echo ""
	@echo "For more info, see README.md and RUNNING.md"

BASE_CXXFLAGS := -fsanitize=address,fuzzer -Wall -Wextra -std=c++20 -I include -I .
UNAME_S := $(shell uname -s)
BITCOINFUZZ_SRC := basemodule modulelogger
BITCOINFUZZ_OBJS := $(addprefix include/bitcoinfuzz/, $(addsuffix .o, $(BITCOINFUZZ_SRC)))
HELPERS_SRC := jvmloader
HELPERS_OBJS := $(addprefix helpers/, $(addsuffix .o, $(HELPERS_SRC)))

BITCOINFUZZ_DIR = $(shell pwd)
CXXFLAGS += -DBITCOINFUZZ_DIR=\"$(BITCOINFUZZ_DIR)\"

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
			$(warning Apple Clang does not support -fsanitize=fuzzer. Run: make setup-macos)
		endif
	endif
	LDFLAGS += -framework CoreFoundation
	export CXX
endif

# Conditionally include module.a files based on compilation flags
MODULES :=
ifneq ($(findstring -DBITCOIN_CORE,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bitcoin/module.a
endif

ifneq ($(findstring -DRUST_PSBT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustpsbt/module.a
endif

ifneq ($(findstring -DRUST_BITCOIN,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustbitcoin/module.a
endif

ifneq ($(findstring -DRUST_MINISCRIPT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustminiscript/module.a
endif

ifneq ($(findstring -DTINY_MINISCRIPT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/tinyminiscript/module.a
endif

ifneq ($(findstring -DBITCOINERLAB_MINISCRIPT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bitcoinerlabminiscript/module.a
endif

ifneq ($(findstring -DBTCD,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/btcd/module.a
endif

ifneq ($(findstring -DGOCOIN,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/gocoin/module.a
endif

ifneq ($(filter -DNBITCOIN,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/nbitcoin/module.a
endif

ifneq ($(findstring -DLND,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/lnd/module.a
endif

ifneq ($(findstring -DLDK,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/ldk/module.a
endif

ifneq ($(findstring -DECLAIR,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/eclair/module.a
endif

ifneq ($(findstring -DNLIGHTNING,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/nlightning/module.a
endif

ifneq ($(findstring -DEMBIT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/embit/module.a
endif

ifneq ($(findstring -DPYBITCOINKERNEL,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/pybitcoinkernel/module.a
endif

ifneq ($(findstring -DRUSTBITCOINKERNEL,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustbitcoinkernel/module.a
endif

ifneq ($(findstring -DCLIGHTNING,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	SODIUM_LDLIBS = $(shell pkg-config --silence-errors --libs libsodium 2>/dev/null)
	MODULES += modules/clightning/module.a
endif

ifneq ($(findstring -DLIGHTNING_KMP,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/lightningkmp/module.a
endif

ifneq ($(findstring -DBITCOINJ,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bitcoinj/module.a
endif

ifneq ($(findstring -DBITCOINS,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bitcoins/module.a
endif

# Add custom mutator module
ifneq (,$(filter -DCUSTOM_MUTATOR%,$(BASE_CXXFLAGS) $(CXXFLAGS)))
	MODULES += custommutator/module.a
endif

ifneq ($(findstring -DDECRED_SECP256K1,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/decredsecp256k1/module.a
endif

ifneq ($(findstring -DSECP256K1,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/secp256k1/module.a
endif

ifneq ($(filter -DNBITCOIN_SECP256K1,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/nbitcoinsecp256k1/module.a
endif

ifneq ($(findstring -DRUST_K256,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustk256/module.a
endif

ifneq ($(findstring -DLIBWALLY_CORE,$(BASE_CXXFLAGS) $(CXXFLAGS)),)                                                                                                                                   
  MODULES += modules/libwallycore/module.a                                                                                                                                                           
endif   

ifneq ($(filter -DBITCOINKERNEL,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bitcoinkernel/module.a
endif

ifneq ($(filter -DBITCOINKERNEL_VARIANT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bitcoinkernelvariant/module.a
endif

ifneq ($(findstring -DPYCOIN,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/pycoin/module.a
endif

ifneq ($(findstring -DPYHDWALLET,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/pyhdwallet/module.a
endif

ifneq ($(findstring -DELECTRUM,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/electrum/module.a
endif

ifneq ($(findstring -DRUST_MUSIG2,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustmusig2/module.a
endif

ifneq ($(findstring -DRUSTCRYPTO_AES,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustcryptoaes/module.a
endif

ifneq ($(findstring -DBDK_SP,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bdksp/module.a
endif

ifneq ($(findstring -DBLUEWALLET_SP,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/bluewalletsp/module.a
endif

ifneq ($(findstring -DSPDK,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/spdk/module.a
endif

ifneq ($(findstring -DENTROPYLAB,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/entropylab/module.a
endif

ifeq ($(UNAME_S), Darwin)
	LIB_EXT := dylib
else
	LIB_EXT := so
endif

ifneq ($(filter -DNBITCOIN,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
  NBITCOIN_LIB := ./NBitcoin.CppBridge.$(LIB_EXT)
endif

ifneq ($(findstring -DNLIGHTNING,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
  NLIGHTNING_LIB := ./NLightning.CppBridge.$(LIB_EXT)
endif

ifneq ($(filter -DNBITCOIN_SECP256K1,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
  NBITCOIN_SECP256K1_LIB := ./NBitcoinSecp256k1.CppBridge.$(LIB_EXT)
endif

ifneq ($(findstring -DTINY_MINISCRIPT,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
  TINY_MINISCRIPT_LIB := ./libtiny_miniscript_lib.$(LIB_EXT)
endif

# Check for Python-based modules and add Python-related flags.
ifneq (,$(filter -DELECTRUM -DEMBIT -DPYBITCOINKERNEL -DPYCOIN -DPYHDWALLET,$(BASE_CXXFLAGS) $(CXXFLAGS)))
  PYTHON_LDFLAGS := $(shell python3-config --ldflags --embed)
endif

# Check that the Java modules are defined to add Java-related flags.
ifneq (,$(filter -DECLAIR -DLIGHTNING_KMP -DBITCOINJ -DBITCOINS,$(BASE_CXXFLAGS) $(CXXFLAGS)))
	ifeq ($(UNAME_S), Darwin)
		JAVA_HOME ?= $(shell /usr/libexec/java_home)
	else
		JAVA_HOME ?= $(shell dirname $$(dirname $$(readlink -f $$(which javac))))
	endif

  JAVA_CXXFLAGS := -I$(JAVA_HOME)/include -I$(JAVA_HOME)/include/$(shell uname -s | tr '[:upper:]' '[:lower:]') -L$(JAVA_HOME)/lib/server -ljvm -Wl,-rpath,$(JAVA_HOME)/lib/server
	JVM_LOADER := helpers/jvmloader.o
endif

ifneq ($(findstring -DRUSTREEXO,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/rustreexo/module.a
endif

ifneq ($(findstring -DUTREEXO,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/utreexo/module.a
endif

ifneq ($(findstring -DLIBBITCOIN_SYSTEM,$(BASE_CXXFLAGS) $(CXXFLAGS)),)
	MODULES += modules/libbitcoinsystem/module.a
	BOOST_ROOT ?= /usr
	LIBBITCOIN_CXXFLAGS := -I$(BOOST_ROOT)/include
	LIBBITCOIN_LDLIBS   := -L$(BOOST_ROOT)/lib -lboost_program_options
endif

override CXXFLAGS := $(BASE_CXXFLAGS) $(JAVA_CXXFLAGS) $(CXXFLAGS) $(PYTHON_LDFLAGS) $(LIBBITCOIN_CXXFLAGS)

# Auto-build any module.a that is missing when linking the final binary.
modules/%/module.a:
	$(MAKE) -C $(dir $@)

bitcoinfuzz: main.cpp driver.o $(BITCOINFUZZ_OBJS) $(JVM_LOADER) $(MODULES)
	$(CXX) $(CXXFLAGS) $(LDFLAGS) main.cpp driver.o $(BITCOINFUZZ_OBJS) $(JVM_LOADER) $(MODULES) $(NBITCOIN_LIB) $(NLIGHTNING_LIB) $(NBITCOIN_SECP256K1_LIB) $(TINY_MINISCRIPT_LIB) -o bitcoinfuzz $(PYTHON_LDFLAGS) $(SODIUM_LDLIBS) $(LIBBITCOIN_LDLIBS)

driver.o: driver.cpp driver.h
	$(CXX) $(CXXFLAGS) -c driver.cpp -o driver.o

include/bitcoinfuzz/%.o: include/bitcoinfuzz/%.cpp include/bitcoinfuzz/%.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

helpers/%.o: helpers/%.cpp helpers/%.h
	$(CXX) $(CXXFLAGS) -c $< -o $@

format:
	clang-format -i include/bitcoinfuzz/*.h include/bitcoinfuzz/*.cpp driver.cpp driver.h main.cpp helpers/*.cpp helpers/*.h

format-all: format
	$(MAKE) -C custommutator format
	@for dir in modules/*/; do $(MAKE) -C $$dir format; done

check-format:
	clang-format -Werror --fail-on-incomplete-format -n include/bitcoinfuzz/*.h include/bitcoinfuzz/*.cpp driver.cpp driver.h main.cpp helpers/*.cpp helpers/*.h

check-format-all:
	@EXIT_CODE=0; \
	$(MAKE) check-format || EXIT_CODE=1; \
	$(MAKE) -C custommutator check-format || EXIT_CODE=1; \
	for dir in modules/*/; do \
		$(MAKE) -C $$dir check-format || EXIT_CODE=1; \
	done; \
	exit $$EXIT_CODE

setup-macos:
	@echo "Installing Homebrew LLVM (required for -fsanitize=fuzzer on macOS)..."
	brew install llvm
	@echo ""
	@echo "Homebrew LLVM installed. Build with:"
	@echo "  make"
	@echo "Or with specific modules:"
	@echo "  CXXFLAGS=\"-DENTROPYLAB\" make"

run-macos:
	@if [ -z "$(FUZZ)" ]; then \
		echo "Usage: FUZZ=<target> make run-macos"; \
		echo "Example: FUZZ=bip32_master_keygen make run-macos"; \
		exit 1; \
	fi
	./bitcoinfuzz

_run-entropylab:
	@echo "Building bitcoinfuzz with ENTROPYLAB module..."
	@$(MAKE) clean >/dev/null 2>&1
	@$(MAKE) bitcoinfuzz CXXFLAGS="-DENTROPYLAB"
	@echo ""
	@echo "=== Running all Entropylab fuzz targets ==="
	@for target in bip32_master_keygen bip32_deserialize_extended_key bip32_derive_from_path pubkey_parse private_to_public_key; do \
		echo ""; \
		echo ">>> Fuzzing target: $$target <<<"; \
		FUZZ=$$target ./bitcoinfuzz -max_total_time=$(or $(FUZZ_TIME),10); \
	done

run-macos-entropylab: _run-entropylab

run-entropylab:
ifeq ($(UNAME_S), Darwin)
	$(MAKE) run-macos-entropylab
else
	$(MAKE) _run-entropylab
endif

clean:
	rm -rf *.o module.a bitcoinfuzz include/bitcoinfuzz/*.o helpers/*.o $(MODULES)
	rm -rf modules/eclair/eclair.zip modules/eclair/lib modules/eclair/eclair_extracted


.PHONY: all bitcoinfuzz setup-macos run-macos run-macos-entropylab run-entropylab _run-entropylab
