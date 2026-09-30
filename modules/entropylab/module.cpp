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

std::optional<bool> Entropylab::sign_verify(std::span<const uint8_t> buffer,
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

} // namespace module
} // namespace bitcoinfuzz
