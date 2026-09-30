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
  ~Entropylab() noexcept override = default;
};

} // namespace module
} // namespace bitcoinfuzz
