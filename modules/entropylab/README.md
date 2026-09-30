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
| `sign_compact` | ECDSA compact signature |
| `sign_der` | ECDSA DER-encoded signature |
| `sign_verify` | ECDSA signature verification |
| `descriptor_parse` | Output descriptor parsing |
| `miniscript_parse` | Miniscript parsing |
| `transaction_eval` | Transaction consensus decode + wtxid/size |
| `sighash_compute` | BIP143 SegWit v0 sighash |
| `address_parse` | Address string parsing (bech32 + base58) |
| `bech32_roundtrip` | Bech32/bech32m encode + decode |
| `bech32_convert_bits` | 5 ↔ 8 bit regrouping (bech32 primitive) |
| `point_add` | secp256k1 point addition |
| `point_mul` | secp256k1 point multiplication |
| `hd_ckd_pub` | BIP32 public child key derivation |
| `bip39_mnemonic_roundtrip` | BIP39 entropy → mnemonic → entropy |
| `bip39_validate` | BIP39 mnemonic checksum validation |
| `aezeed_decipher` | LND aezeed cipher-seed decipher |
| `scrypt_kdf` | scrypt key derivation |
| `script_build_roundtrip` | Smoke-test of all script/address builders |

`script_build_roundtrip` is a module-specific catch-all target that exercises every scriptPubKey builder and `addr_from_script` in one run. It does not participate in differential comparison — it is a smoke test to ensure the Rust exports do not panic on arbitrary input.

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
