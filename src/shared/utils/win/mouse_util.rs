use crate::shared::types::mouse_type::Screen;
use winapi::shared::windef::{POINT, RECT};
use winapi::um::winuser::{ClipCursor, GetCursorPos};

pub fn get_cursor_point() -> POINT {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    cursor_pos
}

pub fn lock_cursor(cursor_pos: POINT) {
    unsafe {
        let rect = RECT {
            left: cursor_pos.x,
            top: cursor_pos.y,
            right: cursor_pos.x + 1,
            bottom: cursor_pos.y + 1,
        };
        ClipCursor(&rect);
    }
}

pub fn unlock_cursor() {
    unsafe {
        ClipCursor(std::ptr::null());
    }
}

pub fn check_position_at_edge(cursor_pos: POINT, screen: Screen) -> Option<String> {
    if cursor_pos.x <= 0 {
        return Some("left".to_string());
    } else if cursor_pos.x >= screen.width - 1 {
        return Some("right".to_string());
    } else if cursor_pos.y <= 0 {
        return Some("top".to_string());
    } else if cursor_pos.y >= screen.height - 1 {
        return Some("bottom".to_string());
    } else {
        return Some("else".to_string());
    }
}
