use crate::shared::types::mouse_type::Screen;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub fn get_screen_metrics() -> Screen {
    unsafe {
        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);
        let screen = Screen {
            width: screen_width,
            height: screen_height,
        };
        screen
    }
}
