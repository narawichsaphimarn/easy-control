use crate::{
    presentation::models::mouse_event_model::MouseEvent,
    shared::utils::mouse_util::move_cursor,
};

pub struct MouseEventServiceApplication;

impl MouseEventServiceApplication {
    pub async fn mouse_event_process(request: MouseEvent) -> Result<(), ()> {
        move_cursor(request.mouse.x as i32, request.mouse.y as i32);
        Ok(())
    }
}
