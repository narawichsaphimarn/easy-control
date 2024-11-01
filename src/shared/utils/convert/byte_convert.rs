pub fn convert_option_byte_to_string(source: Option<&[u8]>, default: &String) -> String {
    match source {
        Some(bytes) => String::from_utf8_lossy(bytes).to_string(),
        None => default.to_string(),
    }
}

pub fn convert_option_byte_to_string_for_mac(source: Option<&[u8]>, join: &String) -> String {
    match source {
        Some(bytes) => bytes
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(join),
        None => "No MAC Address".to_string(),
    }
}