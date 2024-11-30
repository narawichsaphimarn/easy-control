use crate::shared::constants::protocol_constant::InterfaceDesc;
#[cfg(target_os = "windows")]
use crate::shared::utils::convert::byte_convert::convert_option_byte_to_string_for_mac;
#[cfg(target_os = "windows")]
use ipconfig;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use ping;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use pnet::datalink;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task;

pub struct ProtocolUtil;

#[cfg(target_os = "windows")]
impl ProtocolUtil {
    pub fn get_mac_addr(ip_addr: String) -> String {
        let adapters = ipconfig::get_adapters().unwrap();
        let mut mac: String = String::new();
        for adapter in adapters {
            for ip in adapter.ip_addresses() {
                if ip.is_ipv4() && ip.to_string().eq(&ip_addr) {
                    mac = convert_option_byte_to_string_for_mac(
                        adapter.physical_address(),
                        &":".to_string(),
                    );
                    break;
                }
            }
        }
        mac
    }

    pub async fn ping_ip(ip: &str) -> bool {
        ping::ping(
            ip.parse().unwrap(),
            Some(Duration::from_secs(1)),
            None,
            None,
            None,
            None,
        )
        .is_ok()
    }

    pub fn get_addrs() -> (String, String) {
        let adapters = ipconfig::get_adapters().unwrap();
        let mut wlan_addrs: String = String::new();
        let mut lan_addrs: String = String::new();

        for adapter in adapters {
            for ip in adapter.ip_addresses() {
                if ip.is_ipv4() {
                    if adapter
                        .friendly_name()
                        .eq_ignore_ascii_case(&InterfaceDesc::Wireless.to_string())
                        && adapter.if_type() == ipconfig::IfType::Ieee80211
                        && wlan_addrs.is_empty()
                    {
                        // println!("Wi-Fi adapter {} and IPv4 {}", adapter.friendly_name(), ip);
                        wlan_addrs = ip.to_string();
                    } else if adapter.if_type() == ipconfig::IfType::EthernetCsmacd
                        && adapter
                            .friendly_name()
                            .eq_ignore_ascii_case(&InterfaceDesc::Ethernet.to_string())
                        && lan_addrs.is_empty()
                    {
                        // println!("LAN adapter {} and IPv4 {}", adapter.friendly_name(), ip);
                        lan_addrs = ip.to_string();
                    }
                }
            }
        }

        (wlan_addrs, lan_addrs)
    }
}

#[cfg(target_os = "macos")]
impl ProtocolUtil {
    pub fn ping_ip(ip: &str) -> bool {
        let mut output = Command::new("ping")
            .arg("-c 1")
            .arg(ip)
            .spawn()
            .expect("Failed to execute ping");
        match output.wait() {
            Ok(r) => r.success(),
            Err(_) => {
                // log::error!("Failed to ping: {}", e);
                false
            }
        }
    }

    pub fn get_addrs() -> (String, String) {
        // log::debug!("Start mapping address");
        let mut wlan_addrs: String = String::new();
        let mut lan_addrs: String = String::new();
        let interfaces = datalink::interfaces();
        for i_face in interfaces {
            for ip in i_face.clone().ips {
                if ip.is_ipv4() {
                    let (wl, et) = Self::map_wifi_or_lan();
                    if wl.eq_ignore_ascii_case(&i_face.clone().name) && wlan_addrs.is_empty() {
                        // log::debug!("Wi-Fi adapter {} and IPv4 {}", i_face.clone().name, ip);
                        wlan_addrs = ip.ip().to_string();
                    } else if et.eq_ignore_ascii_case(&i_face.clone().name) && lan_addrs.is_empty()
                    {
                        // log::debug!("LAN adapter {} and IPv4 {}", i_face.clone().name, ip);
                        lan_addrs = ip.ip().to_string();
                    }
                }
            }
        }
        (wlan_addrs, lan_addrs)
    }

    fn map_wifi_or_lan() -> (String, String) {
        let output = Command::new("networksetup")
            .arg("-listallhardwareports")
            .output()
            .expect("Failed to execute command");
        let result = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = result.lines().collect();
        let mut hardware_port = "";
        let mut wlan_iface: String = String::new();
        let mut lan_iface: String = String::new();
        for line in lines {
            if line.starts_with(InterfaceDesc::Hardware.to_string().as_str()) {
                hardware_port = line.split(':').nth(1).unwrap().trim();
            } else if line.starts_with(InterfaceDesc::Device.to_string().as_str()) {
                let device = line.split(':').nth(1).unwrap().trim();
                if hardware_port.eq_ignore_ascii_case(InterfaceDesc::Wireless.to_string().as_str())
                {
                    // log::debug!("{} is a WLAN (Wi-Fi) interface", device);
                    wlan_iface = device.to_string();
                } else if hardware_port
                    .eq_ignore_ascii_case(InterfaceDesc::Ethernet.to_string().as_str())
                {
                    // log::debug!("{} is a LAN (Ethernet) interface", device);
                    lan_iface = device.to_string();
                }
            }
        }
        (wlan_iface, lan_iface)
    }

    pub fn get_mac_addr(ip_addr: String) -> String {
        let mut mac: String = String::new();
        let interfaces = datalink::interfaces();
        for i_face in interfaces {
            for ip in i_face.ips {
                if ip.is_ipv4() && ip.ip().to_string().eq_ignore_ascii_case(&ip_addr) {
                    if let Some(s) = i_face.mac {
                        mac = s.to_string();
                    }
                    break;
                }
            }
        }
        mac
    }
}

#[cfg(target_os = "linux")]
impl ProtocolUtil {
    pub async fn ping_ip(ip: &str) -> bool {
        ping::ping(
            ip.parse().unwrap(),
            Some(Duration::from_secs(1)),
            None,
            None,
            None,
            None,
        )
        .is_ok()
    }

    pub fn get_addrs() -> (String, String) {
        log::debug!("Start mapping address");
        let mut wlan_addrs: String = String::new();
        let mut lan_addrs: String = String::new();
        let interfaces = datalink::interfaces();
        for i_face in interfaces {
            for ip in i_face.clone().ips {
                if ip.is_ipv4() {
                    let (wl, et) = map_wifi_or_lan();
                    if wl.eq_ignore_ascii_case(&i_face.clone().name) && wlan_addrs.is_empty() {
                        log::debug!("Wi-Fi adapter {} and IPv4 {}", i_face.clone().name, ip);
                        wlan_addrs = ip.ip().to_string();
                    } else if et.eq_ignore_ascii_case(&i_face.clone().name) && lan_addrs.is_empty()
                    {
                        log::debug!("LAN adapter {} and IPv4 {}", i_face.clone().name, ip);
                        lan_addrs = ip.ip().to_string();
                    }
                }
            }
        }
        (wlan_addrs, lan_addrs)
    }

    fn map_wifi_or_lan() -> (String, String) {
        let path = "/sys/class/net/";
        let mut wlan_iface: String = String::new();
        let mut lan_iface: String = String::new();
        if let Ok(interfaces) = fs::read_dir(path) {
            for interface in interfaces {
                if let Ok(interface) = interface {
                    let iface_name = interface.file_name().into_string().unwrap();
                    let iface_type_path = format!("{}/type", interface.path().display());
                    if Path::new(&iface_type_path).exists() {
                        let iface_type = fs::read_to_string(iface_type_path)
                            .unwrap()
                            .trim()
                            .to_string();
                        match iface_type.as_str() {
                            "1" => {
                                log::debug!("{} is a LAN (Ethernet) interface", iface_name);
                                lan_iface = iface_name;
                            }
                            "801" | "802" => {
                                log::debug!("{} is a WLAN (Wi-Fi) interface", iface_name);
                                wlan_iface = iface_name;
                            }
                            _ => log::debug!("{} is an unknown interface type", iface_name),
                        }
                    }
                }
            }
        } else {
            println!("Could not read network interfaces.");
        }
        (wlan_iface, lan_iface)
    }

    pub fn get_mac_addr(ip_addr: String) -> String {
        let mut mac: String = String::new();
        let interfaces = datalink::interfaces();
        for i_face in interfaces {
            for ip in i_face.ips {
                if ip.is_ipv4() && ip.ip().to_string().eq_ignore_ascii_case(&ip_addr) {
                    if let Some(s) = i_face.mac {
                        mac = s.to_string();
                    }
                    break;
                }
            }
        }
        mac
    }
}

impl ProtocolUtil {
    pub async fn scan_network(base_ip: &str) -> Vec<String> {
        let semaphore = Arc::new(Semaphore::new(100));
        let base_ip = base_ip.to_string();
        let mut jhs = Vec::with_capacity(255);
        for i in 2..=255 {
            let ip = format!("{}.{}", base_ip, i);
            // log::debug!("Start ping IP: {}", ip);
            let _semaphore = semaphore.clone();
            let jh = task::spawn(async move {
                let permit = _semaphore.acquire_owned().await.unwrap();
                let status = Self::ping_ip(&ip);
                drop(permit);
                if status {
                    Some(ip)
                } else {
                    None
                }
            });
            jhs.push(jh);
        }
        let mut ips_act: Vec<String> = Vec::new();
        for jh in jhs {
            if let Some(result) = jh.await.unwrap() {
                ips_act.push(result);
            }
        }
        ips_act
    }
}
