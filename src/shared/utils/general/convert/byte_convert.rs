pub fn convert_option_byte_to_string(source: Option<&[u8]>, join: &String) -> String {
    match source {
        Some(bytes) => bytes
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(join),
        None => "No MAC Address".to_string(),
    }
}
