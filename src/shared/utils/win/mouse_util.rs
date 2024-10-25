use std::ptr;
#[cfg(target_os = "windows")]
use winapi::{
    shared::windef::{POINT, RECT},
    um::winuser::{
        CallNextHookEx, ClipCursor, DispatchMessageW, GetCursorPos, GetMessageW, SetWindowsHookExW,
        ShowCursor, TranslateMessage, MSG, WH_MOUSE_LL, WM_MOUSEMOVE,
    },
};

use crate::shared::{constants::screen_constant::PositionAtEdge, types::screen_type::Screen};

#[cfg(target_os = "windows")]
pub fn get_cursor_point() -> POINT {
    let mut cursor_pos = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut cursor_pos);
    }
    cursor_pos
}

#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
pub fn unlock_cursor() {
    unsafe {
        ClipCursor(std::ptr::null());
    }
}

#[cfg(target_os = "windows")]
pub fn check_position_at_edge(cursor_pos: POINT, screen: Screen) -> Option<PositionAtEdge> {
    if cursor_pos.x <= 0 {
        return Some(PositionAtEdge::Left);
    } else if cursor_pos.x >= screen.width - 1 {
        return Some(PositionAtEdge::Right);
    } else if cursor_pos.y <= 0 {
        return Some(PositionAtEdge::Top);
    } else if cursor_pos.y >= screen.height - 1 {
        return Some(PositionAtEdge::Bottom);
    } else {
        return None;
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
