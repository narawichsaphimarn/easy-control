use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum InterfaceWinDesc {
    Wireless,
    Ethernet,
}

impl fmt::Display for InterfaceWinDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterfaceWinDesc::Ethernet => write!(f, "Ethernet"),
            InterfaceWinDesc::Wireless => write!(f, "WiFi"),
        }
    }
}
