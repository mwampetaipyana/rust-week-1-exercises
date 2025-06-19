pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    //since the raw tx hex is string I change it to bytes
    let tx_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    // ensuring there are at least 4 bytes for the version field
    if tx_bytes.len() < 4 {
        return Err("Transaction data too short ".to_string());
    }

    // converting the first 4 bytes from little-endian to u32
    let version_bytes: [u8; 4] = tx_bytes[0..4]
        .try_into()
        .map_err(|_| "Failed to extract 4 bytes for version.".to_string())?;

    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
