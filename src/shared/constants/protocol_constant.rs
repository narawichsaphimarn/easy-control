use std::fmt;

#[cfg(target_os = "windows")]
#[derive(Debug, Clone, Copy)]
pub enum InterfaceDesc {
    Wireless,
    Ethernet,
}

#[cfg(target_os = "windows")]
impl fmt::Display for InterfaceDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterfaceDesc::Ethernet => write!(f, "Ethernet"),
            InterfaceDesc::Wireless => write!(f, "WiFi"),
        }
    }
}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Copy)]
pub enum InterfaceDesc {
    Wireless,
    Ethernet,
    Hardware,
    Device,
}

#[cfg(target_os = "macos")]
impl fmt::Display for InterfaceDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterfaceDesc::Ethernet => write!(f, "Ethernet"),
            InterfaceDesc::Wireless => write!(f, "Wi-Fi"),
            InterfaceDesc::Hardware => write!(f, "Hardware Port:"),
            InterfaceDesc::Device => write!(f, "Device:"),
        }
    }
}