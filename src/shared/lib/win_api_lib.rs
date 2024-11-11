use std::ptr;
use winapi::shared::minwindef::{BOOL, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HWND, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::wingdi::RGB;
use winapi::um::winuser::{
    ClipCursor, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect,
    GetMessageW, LoadCursorW, PostQuitMessage, RegisterClassW, SetLayeredWindowAttributes,
    SetWindowLongPtrW, ShowCursor, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, GWL_EXSTYLE, IDC_ARROW, LWA_COLORKEY, MSG, SW_HIDE, SW_MAX, SW_SHOW,
    SW_SHOWMAXIMIZED, WM_DESTROY, WM_MOUSEMOVE, WNDCLASSW, WS_EX_LAYERED, WS_OVERLAPPEDWINDOW,
};

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            ClipCursor(ptr::null());
            ShowCursor(BOOL::from(true)); // แสดงเคอร์เซอร์อีกครั้ง
            PostQuitMessage(0);
            0
        }
        WM_MOUSEMOVE => {
            // คุณสามารถเพิ่มการกระทำอื่น ๆ ที่จะทำเมื่อเคอร์เซอร์เคลื่อนที่
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

fn to_string(value: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(value)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

pub fn create_window() {
    unsafe {
        let h_instance = GetModuleHandleW(ptr::null());

        let class_name = to_string("my_window_class");

        let wnd_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
            lpszClassName: class_name.as_ptr(),
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            ..std::mem::zeroed()
        };

        RegisterClassW(&wnd_class);

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            to_string("My Window").as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1920,
            1080,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );

        // ใช้ SetLayeredWindowAttributes เพื่อกำหนดให้หน้าต่างมีพื้นหลังโปร่งใส
        SetLayeredWindowAttributes(hwnd, RGB(0, 0, 0), 0, LWA_COLORKEY);

        // ตั้งค่า GWL_EXSTYLE เพื่อให้หน้าต่างมีลักษณะโปร่งใส
        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, WS_EX_LAYERED.try_into().unwrap());

        ShowWindow(hwnd, SW_SHOWMAXIMIZED);

        // ซ่อนเคอร์เซอร์
        ShowCursor(BOOL::from(false));

        // กำหนดขอบเขตการเคลื่อนไหวของเคอร์เซอร์ให้อยู่ในหน้าต่าง
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(hwnd, &mut rect);
        ClipCursor(&rect);

        let mut msg: MSG = std::mem::zeroed();

        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        DestroyWindow(hwnd);
    }
}
