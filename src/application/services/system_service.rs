use crate::shared::{
    types::system_type::System,
    utils::{
        protocol_util::get_mac_addr, screen_util::get_screen_metrics, system_util::get_hostname,
    },
};

pub struct SystemServiceApplication;

impl SystemServiceApplication {
    pub fn get_system_detail(ip_addr: String) -> Result<System, String> {
        let hostname = get_hostname();
        if hostname.is_none() {
            return Err(String::from("Failed to get hostname"));
        }

        Ok(System {
            host_name: hostname.unwrap(),
            screen: get_screen_metrics(),
            mac: get_mac_addr(ip_addr),
        })
    }
}
