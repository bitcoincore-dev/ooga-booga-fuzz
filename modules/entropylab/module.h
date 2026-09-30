#pragma once

#include <bitcoinfuzz/basemodule.h>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <span>
#include <string>

namespace bitcoinfuzz {
namespace module {
class Entropylab : public BaseModule {
public:
  Entropylab(void);
  std::optional<std::string>
  bip32_master_keygen(std::span<const uint8_t> buffer) const override;
  std::optional<std::string> bip32_deserialize_extended_key(
      std::span<const uint8_t> buffer) const override;
  std::optional<std::string>
  bip32_derive_from_path(std::span<const uint8_t> buffer) const override;
  std::optional<std::string>
  pubkey_parse(std::span<const uint8_t> buffer) const override;
  std::optional<std::string>
  private_to_public_key(std::span<const uint8_t> buffer) const override;
  std::optional<std::string>
  sign_compact(std::span<const uint8_t> buffer,
               std::span<const uint8_t> hash) const override;
  std::optional<bool> sign_verify(std::span<const uint8_t> buffer,
                                  std::span<const uint8_t> hash,
                                  std::span<const uint8_t> sign) const override;
  std::optional<bool> descriptor_parse(std::string str) const override;
  std::optional<std::string>
  transaction_eval(std::span<const uint8_t> buffer) const override;
  std::optional<std::string>
  sign_der(std::span<const uint8_t> buffer,
           std::span<const uint8_t> hash) const override;
  std::optional<bool> miniscript_parse(std::string str) const override;
  std::optional<std::string> address_parse(std::string str) const override;
  std::optional<std::string>
  sighash_compute(const SighashComputeInput &input) const override;
  std::optional<std::string>
  bech32_segwit_roundtrip(const Bech32SegwitInput &input) const override;
  std::optional<std::string>
  point_add(std::span<const uint8_t> a,
            std::span<const uint8_t> b) const override;
  std::optional<std::string>
  point_mul(std::span<const uint8_t> point,
            std::span<const uint8_t> scalar) const override;
  std::optional<std::string> hd_ckd_pub(std::span<const uint8_t> node,
                                        uint32_t index) const override;
  std::optional<std::string>
  bip39_mnemonic_roundtrip(std::span<const uint8_t> entropy) const override;
  std::optional<bool> bip39_validate(std::string mnemonic) const override;
  std::optional<std::string>
  aezeed_decipher(std::span<const uint8_t> seed33,
                  std::string passphrase) const override;
  std::optional<std::string> scrypt_kdf(std::span<const uint8_t> password,
                                        std::span<const uint8_t> salt,
                                        uint32_t log_n, uint32_t r, uint32_t p,
                                        size_t out_len) const override;
  std::optional<std::string>
  script_build_roundtrip(std::span<const uint8_t> buffer) const override;
  std::optional<std::string>
  bech32_convert_bits(const Bech32ConvertBitsInput &input) const override;
  std::optional<std::string>
  base58_roundtrip(std::span<const uint8_t> payload) const override;
  ~Entropylab() noexcept override = default;
};

} // namespace module
} // namespace bitcoinfuzz
