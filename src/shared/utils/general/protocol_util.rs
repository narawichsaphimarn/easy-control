use get_if_addrs::get_if_addrs;
use log;
use ping;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use telnet::Telnet;

pub fn get_port() {
    let mut connection = Telnet::connect(("127.0.0.1", 23), 256).unwrap();
    connection.write(b"Hello, Telnet!\n").unwrap();

    let event = connection.read().unwrap();
    log::debug!("{:?}", event);
}

pub fn get_addrs() -> Vec<String> {
    let ifaces = get_if_addrs().unwrap();
    let mut arr: Vec<String> = Vec::new();
    for iface in ifaces {
        if iface.addr.ip().is_ipv4() {
            log::debug!(
                "Interface name {} and ip {}",
                iface.name.to_string(),
                iface.addr.ip()
            );
            arr.push(iface.addr.ip().to_string());
        }
    }
    arr
}

pub fn check_ip(ip: &str) -> bool {
    TcpStream::connect_timeout(&(ip.to_string()).parse().unwrap(), Duration::from_secs(5)).is_ok()
}

pub fn ping_ip(ip: &str) -> bool {
    return ping::ping(
        ip.parse().unwrap(),
        Some(Duration::from_secs(5)),
        None,
        None,
        None,
        None,
    )
    .is_ok();
}

pub fn scan_network(base_ip: &str) -> Vec<String> {
    let mut ips_act: Vec<String> = Vec::new();
    let handles: Vec<thread::JoinHandle<Option<String>>> = (1..=255)
        .map(|i| {
            let base_ip = base_ip.to_string();
            thread::spawn(move || {
                let ip: String = format!("{}.{}", base_ip, i);
                log::debug!("Start ping IP: {}", ip);
                if ping_ip(&ip) {
                    log::debug!("Active IP: {}", ip);
                    return Some(ip);
                }
                None
            })
        })
        .collect();
    for handle in handles {
        if let Ok(result) = handle.join() {
            if let Some(active_ip) = result {
                ips_act.push(active_ip);
            }
        }
    }
    ips_act
}
