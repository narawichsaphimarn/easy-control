use std::ptr;
use crate::shared::constants::screen_constant::PositionAtEdge;
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::screen_type::Screen;
#[cfg(target_os = "windows")]
use winapi::{
    shared::windef::{POINT, RECT},
    um::winuser::{
        CallNextHookEx, ClipCursor, DispatchMessageW, GetCursorPos, GetMessageW, SetWindowsHookExW,
        ShowCursor, TranslateMessage, MSG, WH_MOUSE_LL, WM_MOUSEMOVE,
    },
};

#[cfg(target_os = "windows")]
pub fn get_cursor_point() -> Mouse {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    Mouse { x: cursor_pos.x as f64, y: cursor_pos.y as f64 }
}

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
    } else if cursor_pos.x >= screen.width as f64 - 1.0 {
        Some(PositionAtEdge::Right)
    } else if cursor_pos.y <= 0.0 {
        Some(PositionAtEdge::Top)
    } else if cursor_pos.y >= screen.height as f64 - 1.0 {
        Some(PositionAtEdge::Bottom)
    } else {
        Some(PositionAtEdge::None)
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn mouse_hook_proc(n_code: i32, w_param: usize, l_param: isize) -> isize {
    if n_code >= 0 && w_param == WM_MOUSEMOVE as usize {
        let mouse_data = *(l_param as *const winapi::shared::windef::POINT);
        log::debug!(
            "Mouse moved to position: ({}, {})",
            mouse_data.x,
            mouse_data.y
        );
    }
    CallNextHookEx(ptr::null_mut(), n_code, w_param, l_param)
}

#[cfg(target_os = "windows")]
pub fn check_mouse_position() {
    unsafe {
        SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), ptr::null_mut(), 0);

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn hidden_cursor() {
    unsafe {
        ShowCursor(0);
    }
}

#[cfg(target_os = "windows")]
pub fn show_cursor() {
    unsafe {
        ShowCursor(1);
    }
}
