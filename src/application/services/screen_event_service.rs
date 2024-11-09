use crate::domain::repositories::screen_mapping_matrix_repository::ScreenMappingMetricRepository;
use crate::domain::repositories::screen_selector_repository::ScreenSelectorRepository;
use crate::shared::constants::screen_constant::map_from_string;
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use crate::shared::utils::mouse_util::{
    get_revere_mouse_position, lock_cursor, revere_mouse_position, unlock_cursor,
};
use crate::shared::utils::screen_util::get_screen_metrics;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard};

use super::mouse_event_service::MouseEventControlServiceApplication;

pub struct ScreenEventControlServiceApplication {
    update: Arc<Mutex<bool>>,
}

impl ScreenEventControlServiceApplication {
    pub fn new() -> Self {
        ScreenEventControlServiceApplication {
            update: Arc::new(Mutex::new(true)),
        }
    }

    pub async fn get_update(&self) -> MutexGuard<'_, bool> {
        let data = self.update.lock().await;
        data
    }

    pub async fn update_data(&self, status: bool) {
        let mut data = self.update.lock().await;
        *data = status;
    }

    pub async fn run(self: Arc<Self>, mouse_event: Arc<MouseEventControlServiceApplication>) {
        let mut s_matrix = Vec::new();
        let mut s_select = Vec::new();
        let screen = get_screen_metrics();
        let mut mouse_event_rx_status = mouse_event.get_mouse_event_rx();
        let mut mouse_event_rx = mouse_event.get_mouse_event_rx();
        while mouse_event_rx.changed().await.is_ok() {
            let mut update = self.get_update().await;
            tokio::select! {
                _ = mouse_event_rx_status.changed(), if update.clone() => {
                    if let Ok(result) = ScreenMappingMetricRepository::find_all() {
                        s_matrix = result;
                    }
                    if let Ok(result) = ScreenSelectorRepository::find_all() {
                        s_select = result;
                    }
                    let protocol_event = MouseEventControlServiceApplication::new_protocol_event();
                    mouse_event.update_protocol_event(protocol_event).await;
                    *update = false;
                }
                _ = mouse_event_rx.changed(), if !update.clone() => {
                    let data_mouse_event = mouse_event_rx.borrow().clone();
                    let data_protocol_event = mouse_event.get_protocol_event().await;
                    if
                        !data_mouse_event.edge.eq_ignore_ascii_case("NONE") &&
                        !data_mouse_event.edge.is_empty()
                    {
                        let s_matrix_match = s_matrix
                            .iter()
                            .find(|x| {
                                x.mac_source.eq_ignore_ascii_case(&data_protocol_event.mac) &&
                                    x.edge.eq_ignore_ascii_case(&data_mouse_event.edge)
                            });
                        if let Some(s_matrix_match) = s_matrix_match {
                            let s_select_match = s_select
                                .iter()
                                .find(|x| x.mac.eq_ignore_ascii_case(&s_matrix_match.mac_target));
                            if let Some(s_select_match) = s_select_match {
                                let protocol_event_map = ProtocolEvent {
                                    mac: s_select_match.mac.clone(),
                                    ip: s_select_match.ip.to_owned(),
                                    edge: s_matrix_match.edge.to_string(),
                                    source_width: screen.width,
                                    source_height: screen.height,
                                    target_width: s_select_match.width.parse::<i32>().unwrap(),
                                    target_height: s_select_match.height.parse::<i32>().unwrap(),
                                    x: data_mouse_event.x,
                                    y: data_mouse_event.y,
                                };
                                mouse_event.send_protocol_event(protocol_event_map);
                                revere_mouse_position(
                                    map_from_string(s_matrix_match.edge.to_string()),
                                    Screen { width: screen.width, height: screen.height },
                                    Mouse { x: data_mouse_event.x, y: data_mouse_event.y }
                                );
                                let reverse_point = get_revere_mouse_position(map_from_string(s_matrix_match.edge.to_string()),
                                    Screen { width: screen.width, height: screen.height },
                                    Mouse { x: data_mouse_event.x, y: data_mouse_event.y });
                                lock_cursor(reverse_point);
                                sleep(Duration::from_millis(50));
                                unlock_cursor()
                            }
                        }
                    }
                }
            }
        }
    }
}
