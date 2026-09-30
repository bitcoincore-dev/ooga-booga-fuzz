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

} // namespace module
} // namespace bitcoinfuzz
