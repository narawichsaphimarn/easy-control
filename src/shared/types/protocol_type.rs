#[derive(Debug, Clone)]
pub struct ProtocolEvent {
    pub mac: String,
    pub ip: String,
    pub edge: String,
    pub source_width: i32,
    pub source_height: i32,
    pub target_width: i32,
    pub target_height: i32,
    pub x: f64,
    pub y: f64,
}
