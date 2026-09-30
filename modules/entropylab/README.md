# Entropylab module

Differential-fuzzing module for [EntropyLab](https://github.com/OogaBoogaX/entropylab)'s Bitcoin cryptographic primitives.

## Supported targets

| Target | Description |
|--------|-------------|
| `bip32_master_keygen` | BIP32 master key generation from seed |
| `bip32_deserialize_extended_key` | Base58 extended key deserialization |
| `bip32_derive_from_path` | BIP32 derivation from a path string |
| `pubkey_parse` | secp256k1 public key parsing (compressed) |
| `private_to_public_key` | secp256k1 private-to-public key derivation |

## Build

The module copies source files from `external/entropylab/entropylab-wasm/src/` at build time and appends bitcoinfuzz-specific C FFI wrappers.

```bash
CXXFLAGS="-DENTROPYLAB" make
```

Or using the auto-build script:

```bash
CXXFLAGS="-DENTROPYLAB" ./scripts/auto_build.py
```

## Dependencies

- Rust nightly (for `-Z sanitizer=address`)
- `cargo`
- The `external/entropylab` submodule (run `git submodule update --init`)
