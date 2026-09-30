# ooga-booga-fuzz

[![Test](https://github.com/bitcoincore-dev/ooga-booga-fuzz/actions/workflows/test.yml/badge.svg)](https://github.com/bitcoincore-dev/ooga-booga-fuzz/actions/workflows/test.yml)

Differential fuzzing for [entropylab](https://github.com/OogaBoogaX/entropylab) via the
bitcoinfuzz harness. This repo builds entropylab's WebAssembly crypto layer as a
native static library and fuzzes it against other Bitcoin implementations.

## Quick start (macOS)

```bash
# 1. Install Homebrew LLVM (Apple Clang does not ship libFuzzer)
make setup-macos

# 2. Build the fuzzer with the Entropylab module
make bitcoinfuzz CXXFLAGS="-DENTROPYLAB"

# 3. Run a single target
FUZZ=bip32_master_keygen ./bitcoinfuzz -max_total_time=30

# 4. Or run all Entropylab targets
make run-entropylab
```

## Make targets

| Target | Description |
|--------|-------------|
| `make` | Show help |
| `make bitcoinfuzz` | Build the fuzzer binary |
| `make setup-macos` | Install Homebrew LLVM |
| `make run-macos` | Run fuzzer (`FUZZ=` required) |
| `make run-macos-entropylab` | Run all Entropylab targets (macOS) |
| `make run-entropylab` | Run all Entropylab targets (auto-detects OS) |
| `make clean` | Remove build artifacts |

## Running a specific target

```bash
# Build
make clean
CXXFLAGS="-DENTROPYLAB" make bitcoinfuzz

# Run
FUZZ=<target> ./bitcoinfuzz [-max_total_time=N]
```

Available targets:

**BIP32 / HD keys**
- `bip32_master_keygen`
- `bip32_deserialize_extended_key`
- `bip32_derive_from_path`
- `hd_ckd_pub`

**secp256k1**
- `pubkey_parse`
- `private_to_public_key`
- `sign_compact`
- `sign_der`
- `sign_verify`
- `point_add`
- `point_mul`

**Descriptors / Scripts / Addresses**
- `descriptor_parse`
- `miniscript_parse`
- `address_parse`
- `script_build_roundtrip`
- `base58_roundtrip`

**Transactions**
- `transaction_eval`
- `sighash_compute`

**bech32 / BIP39 / aezeed**
- `bech32_roundtrip`
- `bech32_convert_bits`
- `bip39_mnemonic_roundtrip`
- `bip39_validate`
- `aezeed_decipher`
- `scrypt_kdf`

## Platform notes

**macOS** — The root `Makefile` auto-detects Homebrew LLVM at
`/opt/homebrew/opt/llvm/bin/clang++` (Apple Silicon) or
`/usr/local/opt/llvm/bin/clang++` (Intel), and also auto-detects `clang-format`
from the same installation for `make check-format`. The deprecated `-ld_classic`
linker flag has been removed.

**Linux** — Should work with the standard LLVM toolchain (`clang++`, `lld`,
`compiler-rt`). No special setup required.

## Coverage roadmap

See [`OOGA-BOOGA.md`](./OOGA-BOOGA.md) for the full gap analysis and roadmap to
100 % coverage of entropylab's WASM exports.

## Repository layout

```
modules/entropylab/          # Entropylab bitcoinfuzz module
  module.cpp / module.h      # C++ wrapper implementing BaseModule
  bitcoinfuzz_wrappers.rs    # Rust FFI shims appended to entropylab-wasm
  entropylab_fuzz_lib/       # Cargo crate building the staticlib
external/entropylab/         # git submodule: entropylab source
```

## License

Same as upstream bitcoinfuzz — see [`LICENSE`](./LICENSE).
