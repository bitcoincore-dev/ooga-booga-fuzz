# OOGA-BOOGA: Entropylab Full Fuzz Coverage Plan

This document tracks the complete mapping between `entropylab-wasm` exports and
`bitcoinfuzz` targets, with a roadmap to 100 % coverage.

## Current state (23 / ~35 exports covered)

| # | bitcoinfuzz target | entropylab-wasm export | JS facade | Status |
|---|-------------------|------------------------|-----------|--------|
| 1 | `bip32_master_keygen` | `el_hd_master` | `hdkey.js` | ✅ |
| 2 | `bip32_deserialize_extended_key` | `el_b58check_decode` + `el_hd_validate` | `hdkey.js` + `base58.js` | ✅ |
| 3 | `bip32_derive_from_path` | `el_hd_master` + `el_hd_ckd_priv` | `hdkey.js` | ✅ |
| 4 | `pubkey_parse` | `secp_point_parse_serialize` | `secp256k1.js` | ✅ |
| 5 | `private_to_public_key` | `secp_pubkey_create` | `secp256k1.js` | ✅ |
| 6 | `sign_compact` | `secp_sign` | `secp256k1.js` | ✅ |
| 7 | `sign_verify` | `secp_verify` (via derived pubkey) | `secp256k1.js` | ✅ |
| 8 | `sign_der` | `secp_sign` + DER serialize | `secp256k1.js` | ✅ |
| 9 | `point_add` | `secp_point_add` | `secp256k1.js` | ✅ |
| 10 | `point_mul` | `secp_point_mul` | `secp256k1.js` | ✅ |
| 11 | `descriptor_parse` | `el_desc_derive` | `core-importdescriptors.js` | ✅ |
| 12 | `miniscript_parse` | `miniscript::Miniscript::from_str` | `core-importdescriptors.js` | ✅ |
| 13 | `transaction_eval` | `el_tx_eval` | `tx.js` | ✅ |
| 14 | `sighash_compute` | `el_sighash_segwit_v0` | `tx.js` | ✅ |
| 15 | `address_parse` | `bech32::decode` + `el_b58check_decode` | `addresses.js` | ✅ |
| 16 | `bech32_roundtrip` | `bech32::encode` + `bech32::decode` | `bech32.js` | ✅ |
| 17 | `bech32_convert_bits` | Manual 5 ↔ 8 bit regrouping | `bech32.js` | ✅ |
| 18 | `hd_ckd_pub` | `el_hd_ckd_pub` | `hdkey.js` | ✅ |
| 19 | `bip39_mnemonic_roundtrip` | `el_bip39_entropy_to_mnemonic` ↔ `el_bip39_mnemonic_to_entropy` | `bip39.js` | ✅ |
| 20 | `bip39_validate` | `el_bip39_validate` | `bip39.js` | ✅ |
| 21 | `aezeed_decipher` | `el_aezeed_decipher` | `aezeed.js` | ✅ |
| 22 | `scrypt_kdf` | `el_scrypt` | `aezeed.js` | ✅ |
| 23 | `script_build_roundtrip` | `el_spk_*` + `el_script_*` + `el_addr_from_script` | `addresses.js` | ✅ |

> **`script_build_roundtrip`** is a module-specific smoke-test target (no differential partner). It stitches together all 10 script/address builders plus `addr_from_script` in a single run. It catches panics and validates that every builder accepts/rejects fuzzer-generated input correctly, but it does not compare outputs across implementations.

## Remaining gaps

### Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Implemented and running |
| 📝 | Planned — good differential-fuzz value, needs a partner module or driver target |
| ❌ | Intentionally omitted — deterministic / no differential partner / trivial |

### Hashes (`hashes.js`) — ❌ intentionally omitted

| Export | Status | Rationale |
|--------|--------|-----------|
| `el_sha256` | ❌ | Deterministic; no differential partner |
| `el_sha512` | ❌ | — |
| `el_ripemd160` | ❌ | — |
| `el_hash160` | ❌ | — |
| `el_hmac_sha512` | ❌ | — |
| `el_pbkdf2_hmac_sha512` | ❌ | — |

> These are well-tested in their upstream crates (`bitcoin_hashes`, `bip39`). Without a second implementation to compare against, fuzzing only verifies that the same code produces the same output.

### Base58Check (`base58.js`) — 📝 planned

| Export | Status | Notes |
|--------|--------|-------|
| `el_b58check_encode` | 📝 | No standalone driver target; only exercised transitively via `bip32_deserialize_extended_key` decode path |
| `el_b58check_decode` | 📝 | Could add a dedicated `base58_roundtrip` target (encode → decode → compare) |

### Scripts / Addresses (`addresses.js`) — ✅ fully covered

All 10 script/address builders and `addr_from_script` are exercised by `script_build_roundtrip`.

### Misc — ❌ intentionally omitted

| Export | Status | Rationale |
|--------|--------|-----------|
| `secp_seckey_valid` | ❌ | Trivial 32-byte range check |
| `secp_sig_normalize` | ❌ | Only meaningful as post-processing of `sign_compact`; no standalone target needed |
| `el_bip39_word_at` | ❌ | Pure wordlist lookup; upstream crate test coverage is sufficient |

## GH Pages build → fuzz target matrix

| JS facade | WASM exports | Fuzz target(s) | Coverage % |
|-----------|--------------|----------------|------------|
| `secp256k1.js` | `secp_*` | `pubkey_parse`, `private_to_public_key`, `sign_compact`, `sign_verify`, `sign_der`, `point_add`, `point_mul` | ~90 % |
| `hashes.js` | `el_sha256`, `el_sha512`, `el_hash160`, `el_ripemd160`, `el_hmac_sha512`, `el_pbkdf2_hmac_sha512` | *None yet* | 0 % |
| `hdkey.js` | `el_hd_master`, `el_hd_ckd_priv`, `el_hd_ckd_pub`, `el_hd_validate` | `bip32_master_keygen`, `bip32_deserialize_extended_key`, `bip32_derive_from_path`, `hd_ckd_pub` | ~95 % |
| `bip39.js` | `el_bip39_*` | `bip39_mnemonic_roundtrip`, `bip39_validate` | ~75 % |
| `base58.js` | `el_b58check_*` | Used internally only | 0 % |
| `bech32.js` | `el_bech32m_*` | `bech32_roundtrip`, `bech32_convert_bits` | ~95 % |
| `addresses.js` | `el_spk_*`, `el_script_*`, `el_addr_from_script` | `script_build_roundtrip`, `descriptor_parse`, `miniscript_parse`, `address_parse` | ~95 % |
| `tx.js` | `el_tx_parse`, `el_sighash_segwit_v0` | `transaction_eval`, `sighash_compute` | ~90 % |
| `aezeed.js` | `el_aezeed_decipher`, `el_scrypt` | `aezeed_decipher`, `scrypt_kdf` | ~90 % |
| `core-importdescriptors.js` | `el_desc_derive` | `descriptor_parse`, `miniscript_parse` | ✅ 100 % |

## Files touched for new targets

When adding a target, the edit surface is always:

1. `modules/entropylab/bitcoinfuzz_wrappers.rs` — Rust FFI shim
2. `modules/entropylab/entropylab_fuzz_lib/entropylab_fuzz_lib.h` — C declaration
3. `modules/entropylab/module.h` — `BaseModule` override declaration
4. `modules/entropylab/module.cpp` — C++ implementation
5. `Makefile` — add to `run-entropylab` loop (if doing a smoke-test run)

If the target is **new to bitcoinfuzz core**, also edit:
- `include/bitcoinfuzz/basemodule.h` — add virtual method
- `driver.cpp` — add `Driver::*Target()` method + `Run()` branch
- `include/bitcoinfuzz/module_defs.h` — only if registering a new module class
