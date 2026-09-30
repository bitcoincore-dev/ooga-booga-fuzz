#include "module.h"
#include "entropylab_fuzz_lib/entropylab_fuzz_lib.h"
#include <fuzzer/FuzzedDataProvider.h>

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

std::optional<std::string>
Entropylab::scrypt_kdf(std::span<const uint8_t> password,
                       std::span<const uint8_t> salt, uint32_t log_n,
                       uint32_t r, uint32_t p, size_t out_len) const {
  std::vector<uint8_t> out(out_len);
  int result = entropylab_scrypt_kdf(password.data(), password.size(),
                                     salt.data(), salt.size(), log_n, r, p,
                                     out_len, out.data(), out.size());
  if (result < 0)
    return std::nullopt;
  std::string hex;
  hex.reserve(out.size() * 2);
  static constexpr char kDigits[] = "0123456789abcdef";
  for (const uint8_t b : out) {
    hex.push_back(kDigits[b >> 4]);
    hex.push_back(kDigits[b & 0x0f]);
  }
  return hex;
}

std::optional<std::string>
Entropylab::script_build_roundtrip(std::span<const uint8_t> buffer) const {
  FuzzedDataProvider provider(buffer.data(), buffer.size());

  auto pubkey = provider.ConsumeBytes<uint8_t>(33);
  auto xonly = provider.ConsumeBytes<uint8_t>(32);

  uint8_t net_sel = provider.ConsumeIntegral<uint8_t>();
  uint8_t script_len = provider.ConsumeIntegralInRange<uint8_t>(0, 64);
  uint8_t leaf_len = provider.ConsumeIntegralInRange<uint8_t>(0, 64);
  uint8_t ms_n = provider.ConsumeIntegralInRange<uint8_t>(1, 16);
  uint8_t ms_m = provider.ConsumeIntegralInRange<uint8_t>(1, 16);
  uint8_t tr_ms_n = provider.ConsumeIntegralInRange<uint8_t>(1, 10);
  uint8_t tr_ms_m = provider.ConsumeIntegralInRange<uint8_t>(1, 10);
  uint8_t addr_script_len = provider.ConsumeIntegralInRange<uint8_t>(0, 64);

  auto script = provider.ConsumeBytes<uint8_t>(script_len);
  auto leaf = provider.ConsumeBytes<uint8_t>(leaf_len);
  auto ms_pubs = provider.ConsumeBytes<uint8_t>(ms_n * 33);
  auto tr_ms_pubs = provider.ConsumeBytes<uint8_t>(tr_ms_n * 32);
  auto addr_script = provider.ConsumeBytes<uint8_t>(addr_script_len);

  std::string result;
  result.reserve(1024);
  static constexpr char kDigits[] = "0123456789abcdef";
  uint8_t out[256];
  uint8_t addr_buf[128];

  auto append_hex = [&](const char *label, int n) {
    result += label;
    result += ':';
    if (n > 0) {
      for (int i = 0; i < n; ++i) {
        result.push_back(kDigits[out[i] >> 4]);
        result.push_back(kDigits[out[i] & 0x0f]);
      }
    } else {
      result += "ERR";
    }
  };

  auto append_addr = [&](int n) {
    result += '|';
    int len = entropylab_addr_from_script(
        out, std::max(n, 0), net_sel, addr_buf, sizeof(addr_buf));
    result += "ADDR:";
    if (len > 0) {
      result.append(reinterpret_cast<const char *>(addr_buf), len);
    } else {
      result += "ERR";
    }
    result += ';';
  };

  int n;

  n = entropylab_spk_p2pkh(pubkey.data(), pubkey.size(), out, sizeof(out));
  append_hex("P2PKH", n);
  append_addr(n);

  n = entropylab_spk_p2wpkh(pubkey.data(), pubkey.size(), out, sizeof(out));
  append_hex("P2WPKH", n);
  append_addr(n);

  n = entropylab_spk_p2sh_p2wpkh(pubkey.data(), pubkey.size(), out,
                                 sizeof(out));
  append_hex("P2SHWPKH", n);
  append_addr(n);

  n = xonly.size() == 32
          ? entropylab_spk_p2tr_key(xonly.data(), out, sizeof(out))
          : -1;
  append_hex("P2TRK", n);
  append_addr(n);

  n = xonly.size() == 32
          ? entropylab_spk_p2tr_leaf(xonly.data(), leaf.data(), leaf.size(),
                                     out, sizeof(out))
          : -1;
  append_hex("P2TRL", n);
  append_addr(n);

  n = entropylab_spk_p2sh(script.data(), script.size(), out, sizeof(out));
  append_hex("P2SH", n);
  append_addr(n);

  n = entropylab_spk_p2wsh(script.data(), script.size(), out, sizeof(out));
  append_hex("P2WSH", n);
  append_addr(n);

  n = entropylab_script_multisig(ms_m, ms_pubs.data(), ms_pubs.size(), out,
                                 sizeof(out));
  append_hex("MULTI", n);
  append_addr(n);

  n = entropylab_script_multisig_tr(tr_ms_m, tr_ms_pubs.data(),
                                    tr_ms_pubs.size(), out, sizeof(out));
  append_hex("TRMUL", n);
  append_addr(n);

  n = entropylab_addr_from_script(addr_script.data(), addr_script.size(),
                                  net_sel, addr_buf, sizeof(addr_buf));
  result += "AFSCR:";
  if (n > 0) {
    result.append(reinterpret_cast<const char *>(addr_buf), n);
  } else {
    result += "ERR";
  }
  result += ';';

  return result;
}

std::optional<std::string>
Entropylab::bech32_convert_bits(const Bech32ConvertBitsInput &input) const {
  std::vector<uint8_t> out(256);
  int result = entropylab_bech32_convert_bits(
      input.data.data(), input.data.size(), input.from_bits, input.to_bits,
      input.pad ? 1 : 0, out.data(), out.size());
  if (result < 0)
    return std::nullopt;
  return std::string(reinterpret_cast<const char *>(out.data()), result);
}

} // namespace module
} // namespace bitcoinfuzz
