use crate::shared::{
    constants::protocol_constant::InterfaceWinDesc,
    utils::general::convert::byte_convert::convert_option_byte_to_string,
};
use ipconfig;

pub fn get_addrs() -> (Vec<String>, Vec<String>) {
    let adapters = ipconfig::get_adapters().unwrap();
    let mut wlan_addrs: Vec<String> = Vec::new();
    let mut lan_addrs: Vec<String> = Vec::new();

    for adapter in adapters {
        for ip in adapter.ip_addresses() {
            if ip.is_ipv4() {
                if adapter
                    .friendly_name()
                    .contains(&InterfaceWinDesc::Wireless.to_string())
                    && adapter.if_type() == ipconfig::IfType::Ieee80211
                {
                    log::debug!("Wi-Fi adapter {} and IPv4 {}", adapter.description(), ip);
                    wlan_addrs.push(ip.to_string());
                } else if adapter.if_type() == ipconfig::IfType::EthernetCsmacd
                    && adapter
                        .description()
                        .contains(&InterfaceWinDesc::Ethernet.to_string())
                {
                    log::debug!("LAN adapter {} and IPv4 {}", adapter.description(), ip);
                    lan_addrs.push(ip.to_string());
                }
            }
        }
    }

    (wlan_addrs, lan_addrs)
}

pub fn get_mac_addr(ip_addr: String) -> String {
    let adapters = ipconfig::get_adapters().unwrap();
    let mut mac: String = String::new();
    for adapter in adapters {
        for ip in adapter.ip_addresses() {
            if ip.is_ipv4() && ip.to_string().eq(&ip_addr) {
                mac = convert_option_byte_to_string(adapter.physical_address(), &"-".to_string());
            }
        }
    }
    mac
}
