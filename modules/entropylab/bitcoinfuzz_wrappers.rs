// ── Bitcoinfuzz integration wrappers ──────────────────────────────────────────
// Appended to entropylab-wasm/src/lib.rs by the module Makefile.

use std::ffi::CString;
use std::os::raw::c_char;

unsafe fn str_to_c_string(input: &str) -> *mut c_char {
    CString::new(input).unwrap().into_raw()
}

/// Frees a C string created by the wrapper functions.
#[no_mangle]
pub unsafe extern "C" fn entropylab_free_c_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_bip32_master_keygen(
    data: *const u8,
    len: usize,
) -> *mut c_char {
    let mut out = [0u8; 78];
    let result = el_hd_master(data, len, out.as_mut_ptr());
    if result != 78 {
        return std::ptr::null_mut();
    }
    let mut encoded = base58ck::encode_check(&out);
    let cstr = str_to_c_string(&encoded);
    wipe_string(&mut encoded);
    cstr
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_bip32_deserialize_extended_key(
    data: *const u8,
    len: usize,
) -> *mut c_char {
    let data_slice = read(data, len);
    let text = match std::str::from_utf8(data_slice) {
        Ok(t) => t,
        Err(_) => return str_to_c_string("INVALID"),
    };

    let mut decoded = match base58ck::decode_check(text) {
        Ok(d) => d,
        Err(_) => return str_to_c_string("INVALID"),
    };

    if decoded.len() != 78 {
        wipe_bytes(&mut decoded);
        return str_to_c_string("INVALID");
    }

    let validated = el_hd_validate(decoded.as_ptr());
    if validated == 0 {
        wipe_bytes(&mut decoded);
        return str_to_c_string("INVALID");
    }

    let depth = decoded[4];
    let fingerprint = [decoded[5], decoded[6], decoded[7], decoded[8]];
    let child_number = u32::from_be_bytes([decoded[9], decoded[10], decoded[11], decoded[12]]);
    let chain_code = &decoded[13..45];
    let key_bytes = &decoded[45..78];

    let chain_hex: String = chain_code.iter().map(|b| format!("{:02x}", b)).collect();
    let key_hex: String = key_bytes.iter().map(|b| format!("{:02x}", b)).collect();

    let result = format!(
        "depth={:02x};fp={:02x}{:02x}{:02x}{:02x};child={:08x};chaincode={};key={}",
        depth,
        fingerprint[0],
        fingerprint[1],
        fingerprint[2],
        fingerprint[3],
        child_number,
        chain_hex,
        key_hex
    );

    wipe_bytes(&mut decoded);
    str_to_c_string(&result)
}

fn parse_bip32_path(path_str: &str) -> Option<Vec<u32>> {
    let mut parts = path_str.split('/');
    let first = parts.next()?;

    if first != "m" && !first.is_empty() {
        return None;
    }

    let mut path = Vec::new();
    for part in parts {
        if part.is_empty() {
            return None;
        }

        let (num_str, hardened) = if let Some(stripped) = part.strip_suffix('\'') {
            (stripped, true)
        } else if let Some(stripped) = part.strip_suffix('h') {
            (stripped, true)
        } else {
            (part, false)
        };

        let num = num_str.parse::<u32>().ok()?;
        if num > 0x7FFF_FFFF {
            return None;
        }
        if hardened {
            path.push(num | 0x8000_0000);
        } else {
            path.push(num);
        }
    }

    Some(path)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_bip32_derive_from_path(
    data: *const u8,
    len: usize,
) -> *mut c_char {
    let path_str = match std::str::from_utf8(read(data, len)) {
        Ok(s) => s,
        Err(_) => return str_to_c_string("INVALID"),
    };

    if path_str.as_bytes().iter().any(|&b| b == b'+') {
        return std::ptr::null_mut();
    }
    if path_str.as_bytes().last() == Some(&b'/') {
        return std::ptr::null_mut();
    }
    if path_str.chars().any(|c| c.is_whitespace()) {
        return std::ptr::null_mut();
    }

    let path = match parse_bip32_path(path_str) {
        Some(p) => p,
        None => return str_to_c_string("INVALID"),
    };

    if path.is_empty() {
        return str_to_c_string("INVALID");
    }

    let seed: [u8; 32] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
        0x1d, 0x1e, 0x1f, 0x20,
    ];

    let mut master = [0u8; 78];
    if el_hd_master(seed.as_ptr(), seed.len(), master.as_mut_ptr()) != 78 {
        return str_to_c_string("INVALID");
    }

    let mut current = master;
    for &index in &path {
        let mut child = [0u8; 78];
        let result = el_hd_ckd_priv(current.as_ptr(), index, child.as_mut_ptr());
        if result == 1 {
            let result2 = el_hd_ckd_priv(current.as_ptr(), index.wrapping_add(1), child.as_mut_ptr());
            if result2 != 78 {
                wipe_bytes(&mut master);
                return str_to_c_string("INVALID");
            }
        } else if result != 78 {
            wipe_bytes(&mut master);
            return str_to_c_string("INVALID");
        }
        current = child;
    }

    let mut encoded = base58ck::encode_check(&current);
    let cstr = str_to_c_string(&encoded);
    wipe_string(&mut encoded);
    wipe_bytes(&mut master);
    cstr
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_pubkey_parse(data: *const u8, len: usize) -> *mut c_char {
    let mut out = [0u8; 33];
    let result = secp_point_parse_serialize(data, len, out.as_mut_ptr(), 1);
    if result == 33 {
        let hex: String = out.iter().map(|b| format!("{:02x}", b)).collect();
        str_to_c_string(&format!("OK:{}", hex))
    } else {
        str_to_c_string("ERR")
    }
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_private_to_public_key(
    data: *const u8,
    len: usize,
) -> *mut c_char {
    if len < 32 {
        return std::ptr::null_mut();
    }
    let mut out = [0u8; 33];
    let result = secp_pubkey_create(data, out.as_mut_ptr(), 1);
    if result == 33 {
        let hex: String = out.iter().map(|b| format!("{:02x}", b)).collect();
        str_to_c_string(&hex)
    } else {
        std::ptr::null_mut()
    }
}

// ── sign_compact ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_sign_compact(
    msg32: *const u8,
    seckey: *const u8,
) -> *mut c_char {
    let mut out = [0u8; 64];
    let result = secp_sign(msg32, seckey, std::ptr::null(), out.as_mut_ptr());
    if result != 64 {
        return std::ptr::null_mut();
    }
    let hex: String = out.iter().map(|b| format!("{:02x}", b)).collect();
    str_to_c_string(&hex)
}

// ── sign_verify ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_sign_verify(
    msg32: *const u8,
    seckey: *const u8,
    sig64: *const u8,
) -> i32 {
    let mut pubkey = [0u8; 33];
    if secp_pubkey_create(seckey, pubkey.as_mut_ptr(), 1) != 33 {
        return 0;
    }
    secp_verify(msg32, pubkey.as_ptr(), 33, sig64)
}

// ── descriptor_parse ────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_descriptor_parse(
    desc: *const u8,
    desc_len: usize,
) -> i32 {
    let mut out = [0u8; 4096];
    let result = crate::descriptor::el_desc_derive(desc, desc_len, 0, 0, out.as_mut_ptr(), 4096);
    if result == -1 {
        0
    } else {
        1
    }
}

// ── transaction_eval ────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_tx_eval(
    input: *const u8,
    input_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    // Exercise el_tx_parse (size query first, then full parse)
    let size = el_tx_parse(input, input_len, std::ptr::null_mut(), 0);
    if size < 0 {
        return size;
    }
    let mut flat = vec![0u8; size as usize];
    let parsed = el_tx_parse(input, input_len, flat.as_mut_ptr(), flat.len());
    if parsed < 0 {
        return parsed;
    }

    // Compute the bitcoinfuzz expected output format
    let bytes = read(input, input_len);
    let mut cursor: &[u8] = bytes;
    let tx = match Transaction::consensus_decode_from_finite_reader(&mut cursor) {
        Ok(tx) => tx,
        Err(_) => return -1,
    };
    if !cursor.is_empty() {
        return -2;
    }
    let result = format!("{}{}", tx.compute_wtxid(), tx.total_size());
    if result.len() > cap {
        return -3;
    }
    std::ptr::copy_nonoverlapping(result.as_ptr(), out, result.len());
    result.len() as i32
}

// ── sign_der ────────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_sign_der(
    msg32: *const u8,
    seckey: *const u8,
) -> *mut c_char {
    let mut out64 = [0u8; 64];
    let result = secp_sign(msg32, seckey, std::ptr::null(), out64.as_mut_ptr());
    if result != 64 {
        return std::ptr::null_mut();
    }
    let sig = match secp256k1::ecdsa::Signature::from_compact(&out64) {
        Ok(sig) => sig,
        Err(_) => return std::ptr::null_mut(),
    };
    let der = sig.serialize_der();
    let hex: String = der.iter().map(|b| format!("{:02x}", b)).collect();
    str_to_c_string(&hex)
}

// ── miniscript_parse ────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_miniscript_parse(
    body: *const u8,
    body_len: usize,
) -> i32 {
    let text = match std::str::from_utf8(read(body, body_len)) {
        Ok(t) => t,
        Err(_) => return 0,
    };
    if text == "1" || text == "0" {
        return 0;
    }
    use miniscript::{Miniscript, Segwitv0, Tap};
    use std::str::FromStr;
    if Miniscript::<String, Segwitv0>::from_str(text).is_ok() {
        return 1;
    }
    if Miniscript::<String, Tap>::from_str(text).is_ok() {
        return 1;
    }
    0
}

// ── address_parse ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_address_parse(
    addr: *const u8,
    addr_len: usize,
) -> *mut c_char {
    let text = match std::str::from_utf8(read(addr, addr_len)) {
        Ok(t) => t,
        Err(_) => return str_to_c_string("INVALID"),
    };

    // Try bech32 / bech32m decode
    match bech32::decode(text) {
        Ok((hrp, data)) => {
            if hrp.as_str() == "bc" && !data.is_empty() {
                let witver = data[0];
                let program = &data[1..];
                let prefix = if witver == 0 && program.len() == 20 {
                    "WPKH:"
                } else if witver == 0 && program.len() == 32 {
                    "WSH:"
                } else if witver == 1 && program.len() == 32 {
                    "TR:"
                } else {
                    let hex: String = program.iter().map(|b| format!("{:02x}", b)).collect();
                    let result = format!("WITNESS_UNKNOWN:v{}:{}", witver, hex);
                    return str_to_c_string(&result);
                };
                let result = format!("{}{}", prefix, text);
                return str_to_c_string(&result);
            }
        }
        Err(_) => {}
    }

    // Try base58
    let mut b58_out = [0u8; 64];
    let b58_res = el_b58check_decode(addr, addr_len, b58_out.as_mut_ptr(), b58_out.len());
    if b58_res >= 0 {
        let payload_len = b58_res as usize;
        if payload_len >= 1 {
            let version = b58_out[0];
            let prefix = if version == 0x00 && payload_len == 21 {
                "PKH:"
            } else if version == 0x05 && payload_len == 21 {
                "SH:"
            } else {
                return str_to_c_string("INVALID");
            };
            let result = format!("{}{}", prefix, text);
            return str_to_c_string(&result);
        }
    }

    str_to_c_string("INVALID")
}

// ── sighash_compute ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_sighash_compute(
    tx_ptr: *const u8,
    tx_len: usize,
    index: u32,
    script_code: *const u8,
    sc_len: usize,
    amount: u64,
    out: *mut u8,
) -> i32 {
    el_sighash_segwit_v0(tx_ptr, tx_len, index, script_code, sc_len, amount, out)
}

// ── bech32_roundtrip ────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_bech32_roundtrip(
    hrp: *const u8,
    hrp_len: usize,
    witver: u8,
    program: *const u8,
    program_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    let hrp_text = match std::str::from_utf8(read(hrp, hrp_len)) {
        Ok(t) => t,
        Err(_) => return -1,
    };
    let hrp = match bech32::Hrp::parse(hrp_text) {
        Ok(h) => h,
        Err(_) => return -1,
    };

    let prog = read(program, program_len);
    let mut data = Vec::with_capacity(1 + prog.len());
    data.push(witver);
    data.extend_from_slice(prog);

    let encoded = if witver == 0 {
        match bech32::encode::<bech32::Bech32>(hrp, &data) {
            Ok(s) => s,
            Err(_) => {
                let msg = b"ENC:FAIL";
                if msg.len() > cap {
                    return -1;
                }
                std::ptr::copy_nonoverlapping(msg.as_ptr(), out, msg.len());
                return msg.len() as i32;
            }
        }
    } else {
        match bech32::encode::<bech32::Bech32m>(hrp, &data) {
            Ok(s) => s,
            Err(_) => {
                let msg = b"ENC:FAIL";
                if msg.len() > cap {
                    return -1;
                }
                std::ptr::copy_nonoverlapping(msg.as_ptr(), out, msg.len());
                return msg.len() as i32;
            }
        }
    };

    match bech32::decode(&encoded) {
        Ok((dec_hrp, dec_data)) => {
            if dec_hrp.as_str() != hrp_text
                || dec_data.len() != data.len()
                || dec_data != data
            {
                let result = format!("ENC:{}|DEC:FAIL", encoded);
                if result.len() > cap {
                    return -1;
                }
                std::ptr::copy_nonoverlapping(result.as_ptr(), out, result.len());
                return result.len() as i32;
            }
            let dec_ver = dec_data[0];
            let dec_prog = &dec_data[1..];
            let hex: String = dec_prog.iter().map(|b| format!("{:02x}", b)).collect();
            let result = format!("ENC:{}|DEC:v{}:{}", encoded, dec_ver, hex);
            if result.len() > cap {
                return -1;
            }
            std::ptr::copy_nonoverlapping(result.as_ptr(), out, result.len());
            result.len() as i32
        }
        Err(_) => {
            let result = format!("ENC:{}|DEC:FAIL", encoded);
            if result.len() > cap {
                return -1;
            }
            std::ptr::copy_nonoverlapping(result.as_ptr(), out, result.len());
            result.len() as i32
        }
    }
}

// ── point_add ───────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_point_add(
    a: *const u8,
    a_len: usize,
    b: *const u8,
    b_len: usize,
) -> *mut c_char {
    let mut buf = [0u8; 33];
    let result = secp_point_add(a, a_len, b, b_len, buf.as_mut_ptr());
    if result != 33 {
        return std::ptr::null_mut();
    }
    let hex: String = buf.iter().map(|b| format!("{:02x}", b)).collect();
    str_to_c_string(&hex)
}

// ── point_mul ───────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_point_mul(
    point: *const u8,
    point_len: usize,
    scalar: *const u8,
) -> *mut c_char {
    let mut buf = [0u8; 33];
    let result = secp_point_mul(point, point_len, scalar, buf.as_mut_ptr(), 1);
    if result != 33 {
        return std::ptr::null_mut();
    }
    let hex: String = buf.iter().map(|b| format!("{:02x}", b)).collect();
    str_to_c_string(&hex)
}

// ── hd_ckd_pub ──────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_hd_ckd_pub(
    node: *const u8,
    index: u32,
) -> *mut c_char {
    let mut buf = [0u8; 78];
    let result = el_hd_ckd_pub(node, index, buf.as_mut_ptr());
    if result == 1 {
        let result2 = el_hd_ckd_pub(node, index.wrapping_add(1), buf.as_mut_ptr());
        if result2 != 78 {
            return std::ptr::null_mut();
        }
    } else if result != 78 {
        return std::ptr::null_mut();
    }
    let encoded = base58ck::encode_check(&buf);
    str_to_c_string(&encoded)
}

// ── bip39_mnemonic_roundtrip ────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_bip39_mnemonic_roundtrip(
    entropy: *const u8,
    entropy_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    let mut phrase_buf = vec![0u8; 512];
    let phrase_len = el_bip39_entropy_to_mnemonic(
        entropy,
        entropy_len,
        phrase_buf.as_mut_ptr(),
        phrase_buf.len(),
    );
    if phrase_len < 0 {
        return -1;
    }
    let phrase_len = phrase_len as usize;

    // Verify roundtrip: mnemonic -> entropy
    let mut entropy_back = vec![0u8; 64];
    let entropy_back_len = el_bip39_mnemonic_to_entropy(
        phrase_buf.as_ptr(),
        phrase_len,
        entropy_back.as_mut_ptr(),
        entropy_back.len(),
    );
    if entropy_back_len < 0 {
        return -1;
    }

    if phrase_len > cap {
        return -1;
    }
    std::ptr::copy_nonoverlapping(phrase_buf.as_ptr(), out, phrase_len);
    phrase_len as i32
}

// ── bip39_validate ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_bip39_validate(
    phrase: *const u8,
    phrase_len: usize,
) -> i32 {
    el_bip39_validate(phrase, phrase_len)
}

// ── aezeed_decipher ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_aezeed_decipher(
    seed33: *const u8,
    pass: *const u8,
    pass_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    if cap < 19 {
        return -1;
    }
    el_aezeed_decipher(seed33, pass, pass_len, 15, 8, 1, out)
}

// ── scrypt_kdf ──────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_scrypt_kdf(
    pass: *const u8,
    pass_len: usize,
    salt: *const u8,
    salt_len: usize,
    log_n: u32,
    r: u32,
    p: u32,
    out_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    if out_len > cap {
        return -1;
    }
    el_scrypt(pass, pass_len, salt, salt_len, log_n, r, p, out, out_len)
}

// ── script_build_roundtrip helpers ───────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2pkh(
    pubkey: *const u8,
    pubkey_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2pkh(pubkey, pubkey_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2wpkh(
    pubkey: *const u8,
    pubkey_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2wpkh(pubkey, pubkey_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2sh_p2wpkh(
    pubkey: *const u8,
    pubkey_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2sh_p2wpkh(pubkey, pubkey_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2tr_key(
    internal: *const u8,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2tr_key(internal, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2tr_leaf(
    internal: *const u8,
    leaf: *const u8,
    leaf_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2tr_leaf(internal, leaf, leaf_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2sh(
    script: *const u8,
    script_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2sh(script, script_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_spk_p2wsh(
    script: *const u8,
    script_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_spk_p2wsh(script, script_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_script_multisig(
    m: u32,
    pubs: *const u8,
    pubs_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_script_multisig(m, pubs, pubs_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_script_multisig_tr(
    m: u32,
    pubs: *const u8,
    pubs_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_script_multisig_tr(m, pubs, pubs_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_addr_from_script(
    script: *const u8,
    script_len: usize,
    net_sel: u8,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_addr_from_script(script, script_len, net_sel, out, cap)
}

// ── base58 roundtrip helpers ────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn entropylab_b58check_encode(
    input: *const u8,
    input_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_b58check_encode(input, input_len, out, cap)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_b58check_decode(
    input: *const u8,
    input_len: usize,
    out: *mut u8,
    cap: usize,
) -> i32 {
    el_b58check_decode(input, input_len, out, cap)
}

// ── bech32_convert_bits ─────────────────────────────────────────────────────

fn hex_encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(DIGITS[(b >> 4) as usize] as char);
        s.push(DIGITS[(b & 0x0f) as usize] as char);
    }
    s
}

fn convert_bits_8_to_5(data: &[u8], pad: bool) -> Option<Vec<u8>> {
    if !pad && (data.len() * 8) % 5 != 0 {
        return None;
    }
    let mut acc = 0u32;
    let mut bits = 0u8;
    let mut out = Vec::new();
    for &b in data {
        acc = (acc << 8) | u32::from(b);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(((acc >> bits) & 0x1f) as u8);
        }
    }
    if pad && bits > 0 {
        out.push(((acc << (5 - bits)) & 0x1f) as u8);
    }
    Some(out)
}

fn convert_bits_5_to_8(data: &[u8], pad: bool) -> Option<Vec<u8>> {
    if !pad && (data.len() * 5) % 8 != 0 {
        return None;
    }
    let mut acc = 0u32;
    let mut bits = 0u8;
    let mut out = Vec::new();
    for &b in data {
        if b > 0x1f {
            return None;
        }
        acc = (acc << 5) | u32::from(b);
        bits += 5;
        while bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }
    Some(out)
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_bech32_convert_bits(
    data: *const u8,
    data_len: usize,
    from_bits: u8,
    to_bits: u8,
    pad: i32,
    out: *mut u8,
    cap: usize,
) -> i32 {
    let input = read(data, data_len);
    let result = match (from_bits, to_bits) {
        (8, 5) => match convert_bits_8_to_5(input, pad != 0) {
            Some(v) => format!("OK:{}", hex_encode(&v)),
            None => "ERR".to_string(),
        },
        (5, 8) => match convert_bits_5_to_8(input, pad != 0) {
            Some(v) => format!("OK:{}", hex_encode(&v)),
            None => "ERR".to_string(),
        },
        _ => "ERR".to_string(),
    };
    if result.len() > cap {
        return -1;
    }
    std::ptr::copy_nonoverlapping(result.as_ptr(), out, result.len());
    result.len() as i32
}
