use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum InterfaceDesc {
    Wireless,
    Ethernet,
}

impl fmt::Display for InterfaceDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterfaceDesc::Ethernet => write!(f, "Ethernet"),
            InterfaceDesc::Wireless => write!(f, "WiFi"),
        }
    }
}
