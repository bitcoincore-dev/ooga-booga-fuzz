#include <cstdint>

extern "C" char *entropylab_bip32_master_keygen(const uint8_t *data, size_t len);
extern "C" char *
entropylab_bip32_deserialize_extended_key(const uint8_t *data, size_t len);
extern "C" char *
entropylab_bip32_derive_from_path(const uint8_t *data, size_t len);
extern "C" char *entropylab_pubkey_parse(const uint8_t *data, size_t len);
extern "C" char *entropylab_private_to_public_key(const uint8_t *data,
                                                   size_t len);
extern "C" char *entropylab_sign_compact(const uint8_t *msg32,
                                          const uint8_t *seckey);
extern "C" int entropylab_sign_verify(const uint8_t *msg32,
                                        const uint8_t *seckey,
                                        const uint8_t *sig64);
extern "C" int entropylab_descriptor_parse(const uint8_t *desc, size_t desc_len);
extern "C" int entropylab_tx_eval(const uint8_t *input, size_t input_len,
                                  uint8_t *out, size_t cap);
extern "C" void entropylab_free_c_string(char *ptr);
