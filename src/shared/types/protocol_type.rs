#[derive(Debug, Clone)]
pub struct ProtocolEvent {
    pub source_mac: String,
    pub source_ip: String,
    pub edge: String,
    pub source_width: i32,
    pub source_height: i32,
    pub target_width: i32,
    pub target_height: i32,
    pub target_mac: String,
    pub target_ip: String,
    pub x: f64,
    pub y: f64,
}

impl ProtocolEvent {
    pub fn new() -> ProtocolEvent {
        ProtocolEvent {
            source_mac: String::new(),
            source_ip: String::new(),
            edge: String::new(),
            source_width: 0,
            source_height: 0,
            target_width: 0,
            target_height: 0,
            target_mac: String::new(),
            target_ip: String::new(),
            x: 0.0,
            y: 0.0,
        }
    }
}
