use ipconfig;

pub fn get_addrs() -> (Vec<String>, Vec<String>) {
    let adapters = ipconfig::get_adapters().unwrap();
    let mut wlan_addrs: Vec<String> = Vec::new();
    let mut lan_addrs: Vec<String> = Vec::new();

    for adapter in adapters {
        for ip in adapter.ip_addresses() {
            if ip.is_ipv4() {
                if adapter.description().contains("Wireless LAN")
                    && adapter.if_type() == ipconfig::IfType::Ieee80211
                {
                    log::debug!("Wi-Fi adapter {} and IPv4 {}", adapter.description(), ip);
                    wlan_addrs.push(ip.to_string());
                } else if adapter.if_type() == ipconfig::IfType::EthernetCsmacd
                    && adapter.description().contains("Ethernet")
                {
                    log::debug!("LAN adapter {} and IPv4 {}", adapter.description(), ip);
                    lan_addrs.push(ip.to_string());
                }
            }
        }
    }

    (wlan_addrs, lan_addrs)
}
