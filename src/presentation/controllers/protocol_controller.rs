use crate::shared::utils::{general::protocol_util::scan_network, win::protocol_util::get_addrs};

pub async fn get_machine() {
    let ips = get_addrs();
    let select_ip = select_ip(ips);
    log::debug!("Select ip : {:?}", select_ip);
    let ip = slic_ip(select_ip);
    let ips_active = scan_network(&ip);
    log::debug!("IPS Active : {:?}", ips_active);
}

fn select_ip(ips: (Vec<String>, Vec<String>)) -> String {
    if ips.1.len() > 0 {
        return ips.1.get(0).cloned().unwrap();
    } else {
        return ips.0.get(0).cloned().unwrap();
    }
}

fn slic_ip(ip: String) -> String {
    let mut split_ip: std::str::Split<&str> = ip.split(".");
    let first_part = split_ip.next();
    let second_part = split_ip.next();
    let third_part = split_ip.next();
    format!(
        "{}.{}.{}",
        first_part.unwrap(),
        second_part.unwrap(),
        third_part.unwrap()
    )
}
