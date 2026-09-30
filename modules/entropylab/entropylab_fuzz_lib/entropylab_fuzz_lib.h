#include <cstdint>

extern "C" char *entropylab_bip32_master_keygen(const uint8_t *data,
                                                size_t len);
extern "C" char *entropylab_bip32_deserialize_extended_key(const uint8_t *data,
                                                           size_t len);
extern "C" char *entropylab_bip32_derive_from_path(const uint8_t *data,
                                                   size_t len);
extern "C" char *entropylab_pubkey_parse(const uint8_t *data, size_t len);
extern "C" char *entropylab_private_to_public_key(const uint8_t *data,
                                                  size_t len);
extern "C" char *entropylab_sign_compact(const uint8_t *msg32,
                                         const uint8_t *seckey);
extern "C" int entropylab_sign_verify(const uint8_t *msg32,
                                      const uint8_t *seckey,
                                      const uint8_t *sig64);
extern "C" int entropylab_descriptor_parse(const uint8_t *desc,
                                           size_t desc_len);
extern "C" int entropylab_tx_eval(const uint8_t *input, size_t input_len,
                                  uint8_t *out, size_t cap);
extern "C" int entropylab_miniscript_parse(const uint8_t *body,
                                           size_t body_len);
extern "C" char *entropylab_address_parse(const uint8_t *addr, size_t addr_len);
extern "C" int entropylab_sighash_compute(const uint8_t *tx_ptr, size_t tx_len,
                                          uint32_t index,
                                          const uint8_t *script_code,
                                          size_t sc_len, uint64_t amount,
                                          uint8_t *out);
extern "C" int entropylab_bech32_roundtrip(const uint8_t *hrp, size_t hrp_len,
                                           uint8_t witver,
                                           const uint8_t *program,
                                           size_t program_len, uint8_t *out,
                                           size_t cap);
extern "C" char *entropylab_sign_der(const uint8_t *msg32,
                                     const uint8_t *seckey);
extern "C" char *entropylab_point_add(const uint8_t *a, size_t a_len,
                                      const uint8_t *b, size_t b_len);
extern "C" char *entropylab_point_mul(const uint8_t *point, size_t point_len,
                                      const uint8_t *scalar);
extern "C" char *entropylab_hd_ckd_pub(const uint8_t *node, uint32_t index);
extern "C" int entropylab_bip39_mnemonic_roundtrip(const uint8_t *entropy,
                                                   size_t entropy_len,
                                                   uint8_t *out, size_t cap);
extern "C" int entropylab_bip39_validate(const uint8_t *phrase,
                                         size_t phrase_len);
extern "C" int entropylab_aezeed_decipher(const uint8_t *seed33,
                                          const uint8_t *pass, size_t pass_len,
                                          uint8_t *out, size_t cap);
extern "C" int entropylab_scrypt_kdf(const uint8_t *pass, size_t pass_len,
                                     const uint8_t *salt, size_t salt_len,
                                     uint32_t log_n, uint32_t r, uint32_t p,
                                     size_t out_len, uint8_t *out, size_t cap);
extern "C" void entropylab_free_c_string(char *ptr);

// ── script_build_roundtrip helpers ───────────────────────────────────────────

extern "C" int entropylab_spk_p2pkh(const uint8_t *pubkey, size_t pubkey_len,
                                    uint8_t *out, size_t cap);
extern "C" int entropylab_spk_p2wpkh(const uint8_t *pubkey, size_t pubkey_len,
                                     uint8_t *out, size_t cap);
extern "C" int entropylab_spk_p2sh_p2wpkh(const uint8_t *pubkey,
                                          size_t pubkey_len, uint8_t *out,
                                          size_t cap);
extern "C" int entropylab_spk_p2tr_key(const uint8_t *internal, uint8_t *out,
                                       size_t cap);
extern "C" int entropylab_spk_p2tr_leaf(const uint8_t *internal,
                                        const uint8_t *leaf, size_t leaf_len,
                                        uint8_t *out, size_t cap);
extern "C" int entropylab_spk_p2sh(const uint8_t *script, size_t script_len,
                                   uint8_t *out, size_t cap);
extern "C" int entropylab_spk_p2wsh(const uint8_t *script, size_t script_len,
                                    uint8_t *out, size_t cap);
extern "C" int entropylab_script_multisig(uint32_t m, const uint8_t *pubs,
                                          size_t pubs_len, uint8_t *out,
                                          size_t cap);
extern "C" int entropylab_script_multisig_tr(uint32_t m, const uint8_t *pubs,
                                             size_t pubs_len, uint8_t *out,
                                             size_t cap);
extern "C" int entropylab_addr_from_script(const uint8_t *script,
                                           size_t script_len, uint8_t net_sel,
                                           uint8_t *out, size_t cap);
