use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum SystemDetail {
    Prefix,
    Path,
    Port = 3000,
    Timeout = 6000,
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
