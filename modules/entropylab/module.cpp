#include "module.h"
#include "entropylab_fuzz_lib/entropylab_fuzz_lib.h"

namespace bitcoinfuzz {
namespace module {
Entropylab::Entropylab(void) : BaseModule("Entropylab") {}

std::optional<std::string>
Entropylab::bip32_master_keygen(std::span<const uint8_t> buffer) const {
  auto result_ptr =
      entropylab_bip32_master_keygen(buffer.data(), buffer.size());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string> Entropylab::bip32_deserialize_extended_key(
    std::span<const uint8_t> buffer) const {
  auto result_ptr =
      entropylab_bip32_deserialize_extended_key(buffer.data(), buffer.size());
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::bip32_derive_from_path(std::span<const uint8_t> buffer) const {
  auto result_ptr =
      entropylab_bip32_derive_from_path(buffer.data(), buffer.size());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::pubkey_parse(std::span<const uint8_t> buffer) const {
  auto result_ptr = entropylab_pubkey_parse(buffer.data(), buffer.size());
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::private_to_public_key(std::span<const uint8_t> buffer) const {
  auto result_ptr =
      entropylab_private_to_public_key(buffer.data(), buffer.size());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::sign_compact(std::span<const uint8_t> buffer,
                         std::span<const uint8_t> hash) const {
  auto result_ptr = entropylab_sign_compact(hash.data(), buffer.data());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<bool>
Entropylab::sign_verify(std::span<const uint8_t> buffer,
                        std::span<const uint8_t> hash,
                        std::span<const uint8_t> sign) const {
  if (sign.size() != 64)
    return false;
  return entropylab_sign_verify(hash.data(), buffer.data(), sign.data()) == 1;
}

std::optional<bool> Entropylab::descriptor_parse(std::string str) const {
  return entropylab_descriptor_parse(
             reinterpret_cast<const uint8_t *>(str.c_str()), str.size()) == 1;
}

std::optional<std::string>
Entropylab::transaction_eval(std::span<const uint8_t> buffer) const {
  std::vector<uint8_t> out(128);
  int result =
      entropylab_tx_eval(buffer.data(), buffer.size(), out.data(), out.size());
  if (result < 0)
    return "0";
  return std::string(reinterpret_cast<const char *>(out.data()), result);
}

std::optional<std::string>
Entropylab::sign_der(std::span<const uint8_t> buffer,
                     std::span<const uint8_t> hash) const {
  auto result_ptr = entropylab_sign_der(hash.data(), buffer.data());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<bool> Entropylab::miniscript_parse(std::string str) const {
  return entropylab_miniscript_parse(
             reinterpret_cast<const uint8_t *>(str.c_str()), str.size()) == 1;
}

std::optional<std::string> Entropylab::address_parse(std::string str) const {
  auto result_ptr = entropylab_address_parse(
      reinterpret_cast<const uint8_t *>(str.c_str()), str.size());
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::sighash_compute(const SighashComputeInput &input) const {
  if (!input.is_segwit_v0)
    return std::nullopt;
  std::vector<uint8_t> out(32);
  int result = entropylab_sighash_compute(
      input.tx_bytes.data(), input.tx_bytes.size(), input.input_index,
      input.script.data(), input.script.size(), input.amount, out.data());
  if (result != 32)
    return std::nullopt;
  std::string hex;
  hex.reserve(64);
  static constexpr char kDigits[] = "0123456789abcdef";
  for (const uint8_t b : out) {
    hex.push_back(kDigits[b >> 4]);
    hex.push_back(kDigits[b & 0x0f]);
  }
  return hex;
}

std::optional<std::string>
Entropylab::bech32_segwit_roundtrip(const Bech32SegwitInput &input) const {
  std::vector<uint8_t> out(256);
  int result = entropylab_bech32_roundtrip(
      reinterpret_cast<const uint8_t *>(input.hrp.data()), input.hrp.size(),
      input.witver, input.program.data(), input.program.size(), out.data(),
      out.size());
  if (result < 0)
    return "ENC:FAIL";
  return std::string(reinterpret_cast<const char *>(out.data()), result);
}

std::optional<std::string>
Entropylab::point_add(std::span<const uint8_t> a,
                      std::span<const uint8_t> b) const {
  auto result_ptr =
      entropylab_point_add(a.data(), a.size(), b.data(), b.size());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::point_mul(std::span<const uint8_t> point,
                      std::span<const uint8_t> scalar) const {
  if (scalar.size() != 32)
    return std::nullopt;
  auto result_ptr =
      entropylab_point_mul(point.data(), point.size(), scalar.data());
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string> Entropylab::hd_ckd_pub(std::span<const uint8_t> node,
                                                  uint32_t index) const {
  if (node.size() != 78)
    return std::nullopt;
  auto result_ptr = entropylab_hd_ckd_pub(node.data(), index);
  if (result_ptr == nullptr)
    return std::nullopt;
  std::string result(result_ptr);
  entropylab_free_c_string(result_ptr);
  return result;
}

std::optional<std::string>
Entropylab::bip39_mnemonic_roundtrip(std::span<const uint8_t> entropy) const {
  std::vector<uint8_t> out(512);
  int result = entropylab_bip39_mnemonic_roundtrip(
      entropy.data(), entropy.size(), out.data(), out.size());
  if (result < 0)
    return std::nullopt;
  return std::string(reinterpret_cast<const char *>(out.data()), result);
}

std::optional<bool> Entropylab::bip39_validate(std::string mnemonic) const {
  return entropylab_bip39_validate(
             reinterpret_cast<const uint8_t *>(mnemonic.c_str()),
             mnemonic.size()) == 1;
}

std::optional<std::string>
Entropylab::aezeed_decipher(std::span<const uint8_t> seed33,
                            std::string passphrase) const {
  if (seed33.size() != 33)
    return std::nullopt;
  std::vector<uint8_t> out(64);
  int result = entropylab_aezeed_decipher(
      seed33.data(), reinterpret_cast<const uint8_t *>(passphrase.c_str()),
      passphrase.size(), out.data(), out.size());
  if (result < 0)
    return std::nullopt;
  return std::string(reinterpret_cast<const char *>(out.data()), result);
}

} // namespace module
} // namespace bitcoinfuzz
