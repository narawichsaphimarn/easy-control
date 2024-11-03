#[derive(Debug, Clone, Copy)]
pub struct Mouse {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: f64,
    pub y: f64,
    pub edge: String,
}
