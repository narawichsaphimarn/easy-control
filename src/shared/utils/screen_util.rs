use crate::shared::types::screen_type::Screen;
use log;
#[cfg(target_os = "macos")]
use rdev::display_size;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "macos")]
pub fn get_screen_metrics() -> Screen {
    let screen = display_size().unwrap();
    let screen = Screen {
        width: screen.0 as i32,
        height: screen.1 as i32,
    };
    screen
}

#[cfg(target_os = "windows")]
pub fn scale_coordinates(
    x: i32,
    y: i32,
    src_width: i32,
    src_height: i32,
    dest_width: i32,
    dest_height: i32,
) -> (i32, i32) {
    let scaled_x = (x * dest_width) / src_width;
    let scaled_y = (y * dest_height) / src_height;
    (scaled_x, scaled_y)
}
