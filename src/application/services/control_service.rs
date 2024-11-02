use crate::shared::types::mouse_type::MouseEvent;
use crate::shared::utils::mouse_util::{check_position_at_edge, get_cursor_point};
use crate::shared::utils::screen_util::get_screen_metrics;
use std::sync::{Arc, Mutex};

pub struct ControlServiceApplication;

impl ControlServiceApplication {
    pub async fn mouse_event(data_mouse_event: Arc<Mutex<MouseEvent>>) {
        loop {
            let current_point = get_cursor_point();
            let current_screen = get_screen_metrics();
            let current_edge = check_position_at_edge(current_point, current_screen);
            let mut data = data_mouse_event.lock().unwrap();
            let new_data = MouseEvent {
                x: current_point.x,
                y: current_point.y,
                edge: current_edge.unwrap().to_string(),
            };
            *data = new_data;
        }
    }

    pub async fn mouse_control(data_mouse_event: Arc<Mutex<MouseEvent>>) {
        loop {
            let current_point = data_mouse_event.lock().unwrap();
            // log::debug!("X {} Y {} E {}", current_point.x, current_point.y, current_point.edge);
            // log::debug!("Edge {}", current_edge.unwrap());
        }
    }

    pub async fn screen_event(data_mouse_event: Arc<Mutex<MouseEvent>>) {}
}