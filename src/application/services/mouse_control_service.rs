use crate::application::services::protocol_service::ProtocolServiceApplication;
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
                    let socket = self.stores.mouse_control.get_socket().await;
                    log::debug!("Server listening on {:?}", socket.local_addr().unwrap());
                    // let server_addr = self.mouse_event.get_protocol_event().await.clone().ip;
                    // loop {
                    //     let m = get_cursor_point();
                    //     let s= format!("X {} Y {}", m.x, m.y);
                    //     socket.send_to(s.as_bytes(), server_addr.clone()).await.expect("TODO: panic \
                    //     message");
                    //     println!("Message sent to {}", server_addr);
                    //
                    //     let mut buf = [0; 1024];
                    //     if let Ok((len, _)) = socket.recv_from(&mut buf).await {
                    //         println!("Received from server: {}", String::from_utf8_lossy(&buf[..len]));
                    //     }
                    // }
                }
                _ = mouse_event_rx_wait.changed(), if self.stores.role_event.get_is_server().await
                .clone() && select_ip.eq_ignore_ascii_case(&self.stores.mouse_event
                    .get_protocol_event().await.ip) => {}
            }
        }
    }
}
