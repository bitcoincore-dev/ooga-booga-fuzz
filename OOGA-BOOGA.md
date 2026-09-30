# OOGA-BOOGA: Entropylab Full Fuzz Coverage Plan

This document tracks the complete mapping between `entropylab-wasm` exports and
`bitcoinfuzz` targets, with a roadmap to 100 % coverage.

## Current state (9 / ~35 exports covered)

| # | bitcoinfuzz target | entropylab-wasm export | JS facade | Status |
|---|-------------------|------------------------|-----------|--------|
| 1 | `bip32_master_keygen` | `el_hd_master` | `hdkey.js` | ✅ |
| 2 | `bip32_deserialize_extended_key` | `el_b58check_decode` + `el_hd_validate` | `hdkey.js` + `base58.js` | ✅ |
| 3 | `bip32_derive_from_path` | `el_hd_master` + `el_hd_ckd_priv` | `hdkey.js` | ✅ |
| 4 | `pubkey_parse` | `secp_point_parse_serialize` | `secp256k1.js` | ✅ |
| 5 | `private_to_public_key` | `secp_pubkey_create` | `secp256k1.js` | ✅ |
| 6 | `sign_compact` | `secp_sign` | `secp256k1.js` | ✅ |
| 7 | `sign_verify` | `secp_verify` (via derived pubkey) | `secp256k1.js` | ✅ |
| 8 | `descriptor_parse` | `el_desc_derive` | `core-importdescriptors.js` | ✅ |
| 9 | `transaction_eval` | `el_tx_eval` (new wrapper) | `tx.js` | ✅ |

## Gap analysis: uncovered exports

### secp256k1 curve ops (`secp256k1.js`)
| Export | Existing bitcoinfuzz target? | Blocker / notes |
|--------|------------------------------|-----------------|
| `secp_seckey_valid` | No direct target | Could add `seckey_valid` target, or wire into `pubkey_parse` pre-check |
| `secp_point_add` | No direct target | Needs new driver target |
| `secp_point_mul` | No direct target | Needs new driver target |
| `secp_sig_normalize` | No direct target | Could be tested under `sign_compact` post-processing |

### Hashes (`hashes.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_sha256` | No | No driver target for raw hash round-trips |
| `el_sha512` | No | — |
| `el_ripemd160` | No | — |
| `el_hash160` | No | — |
| `el_hmac_sha512` | No | — |
| `el_pbkdf2_hmac_sha512` | No | — |

> **Recommendation:** Hashes are deterministic and well-tested in their upstream
> crates. Fuzzing value is low unless we find a differential partner module that
> also exposes the same hash functions. De-prioritize.

### Base58Check (`base58.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_b58check_encode` | No | No driver target for base58 encode |
| `el_b58check_decode` | Partial (used by `bip32_deserialize_extended_key`) | Could add `base58_decode` target |

### BIP32 HD (`hdkey.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_hd_ckd_pub` | No | Needs new driver target for public child derivation |
| `el_hd_master` | ✅ (`bip32_master_keygen`) | — |
| `el_hd_ckd_priv` | ✅ (used internally) | — |
| `el_hd_validate` | ✅ (used internally) | — |

### BIP39 (`bip39.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_bip39_entropy_to_mnemonic` | No | Needs new driver target |
| `el_bip39_mnemonic_to_entropy` | No | — |
| `el_bip39_validate` | No | — |
| `el_bip39_word_at` | No | — |

### Scripts / Addresses (`addresses.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_spk_p2pkh` | Partial (`script_parse` exists in driver) | Could map to `script_parse` if driver output format aligns |
| `el_spk_p2wpkh` | No | — |
| `el_spk_p2sh_p2wpkh` | No | — |
| `el_spk_p2tr_key` | No | — |
| `el_spk_p2tr_leaf` | No | — |
| `el_spk_p2sh` | No | — |
| `el_spk_p2wsh` | No | — |
| `el_script_multisig` | No | — |
| `el_script_multisig_tr` | No | — |
| `el_addr_from_script` | No (`address_parse` goes the other direction) | Needs new driver target or module-specific target |

### bech32m (`bech32.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_bech32m_encode` | Partial (`bech32_roundtrip` / `bech32_convert_bits`) | `bech32_roundtrip` is segwit-specific; entropylab's bech32m is generic (BIP352) |
| `el_bech32m_decode` | Partial | — |

### Transactions / Sighash (`tx.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_tx_parse` | ✅ (wrapped by `el_tx_eval` for `transaction_eval`) | — |
| `el_sighash_segwit_v0` | Partial (`sighash_compute` exists) | `sighash_compute` driver target has a complex structured input; needs a dedicated wrapper |

### aezeed / scrypt (`aezeed.js`)
| Export | Existing target? | Blocker |
|--------|------------------|---------|
| `el_scrypt` | No | No driver target |
| `el_aezeed_decipher` | No | No driver target |

## Roadmap to full coverage

### Phase 1 — Low-hanging fruit (existing driver targets)

These can be added immediately because the driver already supports them.

- [ ] **`sign_der`** → `secp_sign` + DER serialization
  - `secp_sign` returns compact (64 bytes). Need to convert to DER in Rust or C++.
  - Bitcoin Core's `secp256k1_ecdsa_signature_serialize_der` is not exposed by
    rust-secp256k1. We'd need to implement DER serialization or add a new Rust
    dependency.

- [ ] **`miniscript_parse`** → `el_desc_derive` with descriptor body only
  - rust-miniscript parses both descriptors and miniscripts. `el_desc_derive`
    calls `Descriptor::parse_descriptor`. We could call
    `miniscript::Miniscript::from_str` directly for a miniscript-only target.

- [ ] **`address_parse`** → composite of `el_bech32m_decode` + `el_b58check_decode`
  - The driver passes an address string. We'd need to detect bech32m vs base58,
    decode, and return a normalized classification string.
  - Format must match other modules (e.g., `bitcoin` module returns
    `P2WPKH:<program_hex>`, `P2TR:<program_hex>`, etc.).

- [ ] **`sighash_compute`** → `el_sighash_segwit_v0`
  - The driver target `sighash_compute` takes a structured `SighashComputeInput`.
  - Need a C++ wrapper that unpacks the struct and calls `el_sighash_segwit_v0`.

### Phase 2 — New driver targets required

These need new `Driver::Run()` branches + `BaseModule` virtual methods.

| New target | Exports to wrap | Priority |
|-----------|-----------------|----------|
| `seckey_valid` | `secp_seckey_valid` | Low |
| `point_add` | `secp_point_add` | Medium |
| `point_mul` | `secp_point_mul` | Medium |
| `bip39_mnemonic_roundtrip` | `el_bip39_entropy_to_mnemonic` ↔ `el_bip39_mnemonic_to_entropy` | Medium |
| `bip39_validate` | `el_bip39_validate` | Medium |
| `script_build_roundtrip` | `el_spk_*` builders + `el_addr_from_script` | Low |
| `bech32m_roundtrip` | `el_bech32m_encode` + `el_bech32m_decode` | Medium |
| `scrypt_kdf` | `el_scrypt` | Low |
| `aezeed_decipher` | `el_aezeed_decipher` | Low |
| `hd_ckd_pub` | `el_hd_ckd_pub` | Medium |

### Phase 3 — Module-specific targets (no differential partner)

If no other bitcoinfuzz module implements the same logic, we can still add
module-specific targets that run as smoke tests (no differential comparison).

| Target | Purpose |
|--------|---------|
| `entropylab_hash_roundtrip` | SHA-256, SHA-512, HASH160, RIPEMD-160 smoke test |
| `entropylab_hmac_pbkdf2` | HMAC-SHA-512 + PBKDF2 smoke test |
| `entropylab_base58_roundtrip` | Encode → decode identity check |

## GH Pages build → fuzz target matrix

Every JS facade in the browser app should have at least one fuzz target:

| JS facade | WASM exports | Fuzz target(s) | Coverage % |
|-----------|--------------|----------------|------------|
| `secp256k1.js` | `secp_*` | `pubkey_parse`, `private_to_public_key`, `sign_compact`, `sign_verify` | ~60 % |
| `hashes.js` | `el_sha256`, `el_sha512`, `el_hash160`, `el_ripemd160`, `el_hmac_sha512`, `el_pbkdf2_hmac_sha512` | *None yet* | 0 % |
| `hdkey.js` | `el_hd_master`, `el_hd_ckd_priv`, `el_hd_ckd_pub`, `el_hd_validate` | `bip32_master_keygen`, `bip32_deserialize_extended_key`, `bip32_derive_from_path` | ~75 % |
| `bip39.js` | `el_bip39_*` | *None yet* | 0 % |
| `base58.js` | `el_b58check_*` | Used internally only | 0 % |
| `bech32.js` | `el_bech32m_*` | *None yet* | 0 % |
| `addresses.js` | `el_spk_*`, `el_script_*`, `el_addr_from_script` | `descriptor_parse` (indirect) | ~10 % |
| `tx.js` | `el_tx_parse`, `el_sighash_segwit_v0` | `transaction_eval` | ~50 % |
| `aezeed.js` | `el_aezeed_decipher`, `el_scrypt` | *None yet* | 0 % |
| `core-importdescriptors.js` | `el_desc_derive` | `descriptor_parse` | ✅ 100 % |

## Quick wins (recommended next steps)

1. **`address_parse`** — High value, maps to existing driver target.
   - Decode bech32m / base58, classify output type, return normalized string.
   - Differential partner: `bitcoin` (Core) module already implements this.

2. **`sighash_compute`** — High value, maps to existing driver target.
   - Wrapper unpacks `SighashComputeInput` and calls `el_sighash_segwit_v0`.
   - Differential partners: `bitcoin`, `rustbitcoin`, `btcd` modules.

3. **`miniscript_parse`** — Medium value, maps to existing driver target.
   - Call `miniscript::Miniscript::from_str` directly.
   - Differential partner: `rustminiscript` module.

4. **`bech32_roundtrip`** — Medium value, existing driver target.
   - `el_bech32m_encode` + `el_bech32m_decode` with segwit parameters.
   - Differential partners: `bitcoin` (Core), `rustbitcoin` modules.

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
