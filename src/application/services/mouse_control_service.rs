use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::shared::stores::mouse_control_store::Mouse;
use crate::shared::stores::stores::Stores;
use crate::shared::utils::mouse_util::move_cursor;
use crate::shared::utils::protocol_util::get_addrs;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MouseControlServiceApplication {
    pub stores: Arc<Stores>,
}

impl MouseControlServiceApplication {
    pub fn new(stores: Arc<Stores>) -> Arc<Self> {
        Arc::new(MouseControlServiceApplication {
            stores: Arc::clone(&stores),
        })
    }

    pub async fn run(self: Arc<Self>) {
        let ips: (String, String) = get_addrs();
        let (select_ip, _) = ProtocolServiceApplication::select_ip(ips);
        let mut mouse_event_rx = self.stores.mouse_event.get_mouse_event_rx();
        let mut mouse_event_rx_wait = self.stores.mouse_event.get_mouse_event_rx();
        loop {
            tokio::select! {
                _ = async {}, if !self.stores.role_event.get_is_server().await
                .clone() => {
                    let receive = self.stores.mouse_control.receive().await;
                    move_cursor(receive.x, receive.y);
                }
                _ = mouse_event_rx.changed(), if self.stores.role_event.get_is_server().await
                .clone() && !select_ip.eq_ignore_ascii_case(&self.stores.mouse_event
                    .get_protocol_event().await.ip) => {
                    let data_mouse_event = mouse_event_rx.borrow().clone();
                    let json = Mouse {x: data_mouse_event.x as i32,y: data_mouse_event.y as i32,};
                    if let Ok(json) = serde_json::to_string(&json) {
                        self.stores.mouse_control.send(self.stores.mouse_event
                            .get_protocol_event().await.ip.clone().as_str(), json).await;
                    }
                }
                _ = mouse_event_rx_wait.changed(), if self.stores.role_event.get_is_server().await
                .clone() && select_ip.eq_ignore_ascii_case(&self.stores.mouse_event
                    .get_protocol_event().await.ip) => {}
            }
        }
    }
}
