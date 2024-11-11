use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::shared::stores::stores::Stores;
use crate::shared::utils::protocol_util::ProtocolUtil;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BlockEventControlServiceApplication {
    pub stores: Arc<Stores>,
}

impl BlockEventControlServiceApplication {
    pub fn new(stores: Arc<Stores>) -> Arc<Self> {
        Arc::new(BlockEventControlServiceApplication { stores })
    }

    pub async fn run(self: Arc<Self>) {
        let ips: (String, String) = ProtocolUtil::get_addrs();
        let (select_ip, _) = ProtocolServiceApplication::select_ip(ips);
        loop {
            tokio::select! {
                _ = async {}, if self.stores.role_event.get_is_server().await
                .clone() && !select_ip.eq_ignore_ascii_case(&self.stores.mouse_event.get_protocol_event().await.ip) => {
                    self.stores.lib_event.clone().run();
                }
                else => {}
            }
        }
    }
}
