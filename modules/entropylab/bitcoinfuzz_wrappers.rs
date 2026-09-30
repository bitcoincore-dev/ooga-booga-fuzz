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
    let mut out = [0u8; 1];
    let result = crate::descriptor::el_desc_derive(desc, desc_len, 0, 0, out.as_mut_ptr(), 1);
    if result >= 0 {
        1
    } else {
        0
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
