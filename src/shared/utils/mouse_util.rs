use crate::shared::constants::screen_constant::PositionAtEdge;
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::screen_type::Screen;
#[cfg(target_os = "windows")]
use winapi::{
    shared::windef::{ POINT, RECT },
    um::winuser::{ ClipCursor, GetCursorPos, SetCursorPos, ShowCursor },
};

#[cfg(target_os = "windows")]
pub fn get_cursor_point() -> Mouse {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    Mouse {
        x: cursor_pos.x as f64,
        y: cursor_pos.y as f64,
    }
}

#[cfg(target_os = "windows")]
pub fn change_icon() {}

#[cfg(target_os = "macos")]
pub fn get_cursor_point() -> Mouse {
    Mouse { x: 0.0, y: 0.0 }
}

#[cfg(target_os = "windows")]
pub fn lock_cursor(cursor_pos: Mouse) {
    unsafe {
        let rect = RECT {
            left: cursor_pos.x as i32,
            top: cursor_pos.y as i32,
            right: (cursor_pos.x + 1.0) as i32,
            bottom: (cursor_pos.y + 1.0) as i32,
        };
        ClipCursor(&rect);
    }
}

#[cfg(target_os = "windows")]
pub fn unlock_cursor() {
    unsafe {
        ClipCursor(std::ptr::null());
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn check_position_at_edge(cursor_pos: Mouse, screen: Screen) -> Option<PositionAtEdge> {
    if cursor_pos.x <= 0.0 {
        Some(PositionAtEdge::Left)
    } else if cursor_pos.x >= (screen.width as f64) - 1.0 {
        Some(PositionAtEdge::Right)
    } else if cursor_pos.y <= 0.0 {
        Some(PositionAtEdge::Top)
    } else if cursor_pos.y >= (screen.height as f64) - 1.0 {
        Some(PositionAtEdge::Bottom)
    } else {
        Some(PositionAtEdge::None)
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn revere_mouse_position(edge: PositionAtEdge, screen: Screen, cursor_pos: Mouse) {
    match edge {
        PositionAtEdge::Top =>
            move_cursor(cursor_pos.x as i32, screen.height - (cursor_pos.y as i32) - 1),
        PositionAtEdge::Bottom =>
            move_cursor(cursor_pos.x as i32, (cursor_pos.y as i32) - screen.height + 1),
        PositionAtEdge::Left =>
            move_cursor(screen.width - (cursor_pos.x as i32) - 1, cursor_pos.y as i32),
        PositionAtEdge::Right =>
            move_cursor(screen.width - (cursor_pos.x as i32) + 1, cursor_pos.y as i32),
        PositionAtEdge::None => (),
    }
}

#[cfg(target_os = "windows")]
pub fn hidden_cursor() {
    unsafe {
        while ShowCursor(0) >= 0 {}
    }
}

#[cfg(target_os = "windows")]
pub fn show_cursor() {
    unsafe {
        ShowCursor(1);
    }
}

#[cfg(target_os = "windows")]
pub fn move_cursor(x: i32, y: i32) {
    unsafe {
        SetCursorPos(x, y);
    }
}

pub fn mouse_different_pointer(
    current_point: &Mouse,
    source_screen: Screen,
    target_screen: Screen
) -> Mouse {
    Mouse {
        x: (current_point.x * (source_screen.width as f64)) / (target_screen.width as f64),
        y: (current_point.y * (source_screen.height as f64)) / (target_screen.height as f64),
    }
}
