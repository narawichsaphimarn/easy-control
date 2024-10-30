use crate::shared::utils::mouse_util::{check_position_at_edge, get_cursor_point};
use crate::shared::utils::screen_util::get_screen_metrics;

pub struct ControlServiceApplication;

impl ControlServiceApplication {
    pub fn main() {
        loop {
            let current_point = get_cursor_point();
            // log::debug!("X {} Y {}", current_point.x, current_point.y);
            let current_screen = get_screen_metrics();
            let current_edge = check_position_at_edge(current_point, current_screen);
            // log::debug!("Edge {}", current_edge.unwrap());
        }
    }
}