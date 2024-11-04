use serde::{ Deserialize, Serialize };

use crate::shared::types::mouse_type::Mouse;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MouseEvent {
    pub event: i32,
    pub mouse: Mouse,
}
