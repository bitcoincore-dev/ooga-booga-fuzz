#include <cstdint>

extern "C" char *entropylab_bip32_master_keygen(const uint8_t *data, size_t len);
extern "C" char *
entropylab_bip32_deserialize_extended_key(const uint8_t *data, size_t len);
extern "C" char *
entropylab_bip32_derive_from_path(const uint8_t *data, size_t len);
extern "C" char *entropylab_pubkey_parse(const uint8_t *data, size_t len);
extern "C" char *entropylab_private_to_public_key(const uint8_t *data,
                                                   size_t len);
extern "C" void entropylab_free_c_string(char *ptr);
