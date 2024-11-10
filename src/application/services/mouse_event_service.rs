use crate::shared::stores::stores::Stores;
use crate::shared::types::mouse_type::MouseEvent;
use crate::shared::utils::mouse_util::{check_position_at_edge, get_cursor_point};
use crate::shared::utils::screen_util::get_screen_metrics;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MouseEventControlServiceApplication {
    pub store: Arc<Stores>,
}

impl MouseEventControlServiceApplication {
    pub fn new(store: Arc<Stores>) -> Arc<MouseEventControlServiceApplication> {
        Arc::new(MouseEventControlServiceApplication { store })
    }

    pub async fn run(self: Arc<Self>) {
        let mut protocol_event_rx = self.store.mouse_event.protocol_event_rx.clone();
        let current_screen = get_screen_metrics();
        loop {
            tokio::select! {
                _ = protocol_event_rx.changed() => {
                    self.store.mouse_event.update_switch(true).await;
                    let value = protocol_event_rx.borrow().clone();
                    log::debug!("Now screen IP:{}, MAC:{}", value.ip, value.mac);
                    self.store.mouse_event.update_protocol_event(value).await;
                    sleep(Duration::from_millis(100));
                    self.store.mouse_event.update_switch(false).await;
                }
                _ = tokio::time::sleep(Duration::from_millis(1)), if !self.store.mouse_event
                .get_switch().await.clone() && self.store.role_event.get_is_server().await.clone() => {
                    let current_point = get_cursor_point();
                    let current_edge = check_position_at_edge(current_point, current_screen);
                    self.store.mouse_event.send_mouse_event(MouseEvent { x: current_point.x, y: current_point.y, edge: current_edge.unwrap().to_string() });
                }
                _ = tokio::time::sleep(Duration::from_millis(500)), if !self.store.role_event.get_is_server().await
                .clone() => {}
            }
        }
    }
}
