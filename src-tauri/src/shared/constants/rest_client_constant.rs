use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum SystemDetail {
    Prefix,
    Path,
    Port = 3000,
    Timeout = 500,
}

impl fmt::Display for SystemDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SystemDetail::Path => write!(f, "/api/v1/system-detail?ip_addr="),
            SystemDetail::Port => write!(f, ""),
            SystemDetail::Timeout => write!(f, ""),
            SystemDetail::Prefix => write!(f, "http://"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScreenMappingMatrix {
    Prefix,
    Path,
    Port = 3000,
    Timeout = 6000,
}

impl fmt::Display for ScreenMappingMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScreenMappingMatrix::Path => write!(f, "/api/v1/screen-matrix"),
            ScreenMappingMatrix::Port => write!(f, ""),
            ScreenMappingMatrix::Timeout => write!(f, ""),
            ScreenMappingMatrix::Prefix => write!(f, "http://"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEvent {
    Prefix,
    Path,
    Port = 3000,
    Timeout = 6000,
}

impl fmt::Display for MouseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MouseEvent::Path => write!(f, "/api/v1/mouse/event"),
            MouseEvent::Port => write!(f, ""),
            MouseEvent::Timeout => write!(f, ""),
            MouseEvent::Prefix => write!(f, "http://"),
        }
    }
}
