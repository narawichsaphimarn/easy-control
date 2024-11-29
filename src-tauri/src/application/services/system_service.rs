use crate::shared::types::system_type::System;
use crate::shared::utils::protocol_util::ProtocolUtil;
use crate::shared::utils::screen_util::ScreenUtil;
use crate::shared::utils::system_util::SystemUtil;

#[derive(Debug, Clone)]
pub struct SystemServiceApplication;

impl SystemServiceApplication {
    pub fn get_system_detail(ip_addr: String) -> Result<System, String> {
        let hostname = SystemUtil::get_hostname();
        if hostname.is_none() {
            return Err(String::from("Failed to get hostname"));
        }

        Ok(System {
            host_name: hostname.unwrap(),
            screen: ScreenUtil::get_screen_metrics(),
            mac: ProtocolUtil::get_mac_addr(ip_addr.clone()),
            ip: ip_addr,
        })
    }
}
