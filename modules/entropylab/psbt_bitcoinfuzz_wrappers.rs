fn get_last_error() -> String {
    unsafe {
        let len = crate::psbt::psbt_last_error(std::ptr::null_mut(), 0);
        if len <= 0 {
            return String::new();
        }
        let mut buf = vec![0u8; len as usize];
        let ret = crate::psbt::psbt_last_error(buf.as_mut_ptr(), buf.len());
        if ret < 0 {
            return String::new();
        }
        String::from_utf8_lossy(&buf[..ret as usize]).into_owned()
    }
}

#[no_mangle]
pub unsafe extern "C" fn entropylab_psbt_parse(data: *const u8, len: usize) -> *mut c_char {
    let data_slice = std::slice::from_raw_parts(data, len);
    let size = crate::psbt::psbt_inspect(data_slice.as_ptr(), data_slice.len(), std::ptr::null_mut(), 0);
    if size < 0 {
        let err = get_last_error();
        if err.contains("incompatible time and height locktimes") {
            return str_to_c_string("CONFLICTING_LOCKTIME");
        }
        return str_to_c_string("INVALID");
    }
    let mut json_buf = vec![0u8; size as usize];
    let written = crate::psbt::psbt_inspect(data_slice.as_ptr(), data_slice.len(), json_buf.as_mut_ptr(), json_buf.len());
    if written < 0 {
        let err = get_last_error();
        if err.contains("incompatible time and height locktimes") {
            return str_to_c_string("CONFLICTING_LOCKTIME");
        }
        return str_to_c_string("INVALID");
    }
    let json_str = match std::str::from_utf8(&json_buf[..written as usize]) {
        Ok(s) => s,
        Err(_) => return str_to_c_string("INVALID"),
    };
    let doc: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(d) => d,
        Err(_) => return str_to_c_string("INVALID"),
    };
    match format_psbt(&doc) {
        Some(s) => str_to_c_string(&s),
        None => str_to_c_string("INVALID"),
    }
}

fn format_psbt(doc: &serde_json::Value) -> Option<String> {
    let tx = doc.get("tx")?;
    let lock_time = tx.get("locktime")?.as_u64()? as u32;
    let inputs_arr = tx.get("inputs")?.as_array()?;
    let outputs_arr = tx.get("outputs")?.as_array()?;
    let input_maps = doc.get("inputs")?.as_array()?;
    let output_maps = doc.get("outputs")?.as_array()?;
    let version = doc.get("psbtVersion")?.as_u64()? as u32;
    let mut result = String::new();
    result.push_str(&format!("lock_time={};", lock_time));
    result.push_str(&format!("inputs={};", inputs_arr.len()));
    result.push_str(&format!("outputs={};", outputs_arr.len()));
    for (i, (tx_input, input_map)) in inputs_arr.iter().zip(input_maps.iter()).enumerate() {
        let txid = tx_input.get("txid")?.as_str()?;
        let vout = tx_input.get("vout")?.as_u64()? as u32;
        let pairs = input_map.as_array()?;
        let mut has_utxo = false;
        let mut partial_sigs = 0usize;
        let mut redeem_script = String::new();
        let mut witness_script = String::new();
        let mut sighash_type = 0u32;
        let mut bip32_count = 0usize;
        let mut finalized = false;
        let mut has_explicit_sequence = false;
        for pair in pairs {
            let name = pair.get("name")?.as_str()?;
            match name {
                "PSBT_IN_NON_WITNESS_UTXO" | "PSBT_IN_WITNESS_UTXO" => has_utxo = true,
                "PSBT_IN_PARTIAL_SIG" => partial_sigs += 1,
                "PSBT_IN_REDEEM_SCRIPT" => { redeem_script = pair.get("value")?.as_str()?.to_string(); }
                "PSBT_IN_WITNESS_SCRIPT" => { witness_script = pair.get("value")?.as_str()?.to_string(); }
                "PSBT_IN_SIGHASH_TYPE" => {
                    if let Some(decoded) = pair.get("decoded") {
                        if let Some(n) = decoded.get("sighashType").and_then(|v| v.as_u64()) {
                            sighash_type = n as u32;
                        }
                    }
                }
                "PSBT_IN_BIP32_DERIVATION" | "PSBT_IN_TAP_BIP32_DERIVATION" => bip32_count += 1,
                "PSBT_IN_FINAL_SCRIPTSIG" => {
                    if let Some(val) = pair.get("value")?.as_str() {
                        if !val.is_empty() { finalized = true; }
                    }
                }
                "PSBT_IN_FINAL_SCRIPTWITNESS" => {
                    if let Some(val) = pair.get("value")?.as_str() {
                        if !val.is_empty() { finalized = true; }
                    }
                }
                "PSBT_IN_SEQUENCE" => has_explicit_sequence = true,
                _ => {}
            }
        }
        result.push_str(&format!("input{}previous_output={}:{};", i, txid, vout));
        if version == 2 && !has_explicit_sequence {
            result.push_str(&format!("input{}sequence={};", i, ""));
        } else {
            let sequence = tx_input.get("sequence")?.as_u64()? as u32;
            result.push_str(&format!("input{}sequence={};", i, sequence));
        }
        if has_utxo { result.push_str(&format!("input{}utxo=1;", i)); }
        result.push_str(&format!("input{}partial_signatures={};", i, partial_sigs));
        result.push_str(&format!("input{}redeem_script={};", i, redeem_script));
        result.push_str(&format!("input{}witness_script={};", i, witness_script));
        result.push_str(&format!("input{}sighash_type={};", i, sighash_type));
        result.push_str(&format!("input{}bip32={};", i, bip32_count));
        if finalized { result.push_str(&format!("input{}finalized=1;", i)); }
    }
    for (i, (tx_output, output_map)) in outputs_arr.iter().zip(output_maps.iter()).enumerate() {
        let value_str = tx_output.get("value")?.as_str()?;
        let value: i64 = value_str.parse().ok()?;
        let script_hex = tx_output.get("scriptPubKey")?.as_str()?;
        let pairs = output_map.as_array()?;
        let mut redeem_script = String::new();
        let mut witness_script = String::new();
        let mut bip32_count = 0usize;
        for pair in pairs {
            let name = pair.get("name")?.as_str()?;
            match name {
                "PSBT_OUT_REDEEM_SCRIPT" => { redeem_script = pair.get("value")?.as_str()?.to_string(); }
                "PSBT_OUT_WITNESS_SCRIPT" => { witness_script = pair.get("value")?.as_str()?.to_string(); }
                "PSBT_OUT_BIP32_DERIVATION" | "PSBT_OUT_TAP_BIP32_DERIVATION" => bip32_count += 1,
                _ => {}
            }
        }
        result.push_str(&format!("output{}val={};", i, value));
        result.push_str(&format!("output{}script={};", i, script_hex));
        result.push_str(&format!("output{}redeem_script={};", i, redeem_script));
        result.push_str(&format!("output{}witness_script={};", i, witness_script));
        result.push_str(&format!("output{}bip32={};", i, bip32_count));
    }
    Some(result)
}
