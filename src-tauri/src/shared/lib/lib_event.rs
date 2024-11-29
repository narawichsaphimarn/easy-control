use crate::shared::types::file_store_type::{ScreenMappingMatrix, ScreenSelector};
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use std::cell::RefCell;
use std::ptr;
use std::sync::Arc;
use tokio::sync::Mutex;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::{HHOOK, HWND, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, ClipCursor, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    EnumWindows, GetClientRect, GetKeyboardState, GetMessageW, GetSystemMetrics, IsWindow,
    LoadCursorW, LoadIconW, RegisterClassExW, SetWindowsHookExW, ShowCursor, ShowWindow, ToUnicode,
    TranslateMessage, UnhookWindowsHookEx, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW,
    IDI_APPLICATION, KBDLLHOOKSTRUCT, MSG, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, VK_LMENU,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_MOUSEMOVE, WM_SYSKEYDOWN, WNDCLASSEXW, WS_EX_LAYERED,
    WS_EX_TOOLWINDOW, WS_POPUP,
};

#[derive(Debug, Clone)]
pub struct Window {
    pub off: Arc<Mutex<bool>>,
    pub hook: Arc<RefCell<Option<HHOOK>>>,
}

impl Window {
    pub fn new() -> Arc<Self> {
        Arc::new(Window {
            off: Arc::new(Mutex::new(true)),
            hook: Arc::new(RefCell::new(None)),
        })
    }
}

thread_local! {
    static HOOK: RefCell<Option<HHOOK>> = RefCell::new(None);
}

#[cfg(target_os = "windows")]
impl Window {
    unsafe extern "system" fn keyboard_hook(
        code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        // println!("Keyboard hook triggered! 00000"); // ดูว่าฟังก์ชันทำงานหรือไม่
        if code >= 0 {
            // println!("Keyboard hook triggered!"); // ดูว่าฟังก์ชันทำงานหรือไม่
            let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);
            if (w_param as u32) == WM_SYSKEYDOWN {
                // println!("Key down detected: vkCode = {}", kb_struct.vkCode);
                if kb_struct.vkCode == (VK_LMENU as u32) && Self::is_alt_pressed() {
                    // println!("Alt + Tab pressed!");
                    // Block Alt + Tab by not calling CallNextHookEx
                    return 1;
                }
            }
        }
        CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
    }

    fn is_alt_pressed() -> bool {
        unsafe {
            let alt_state = winapi::um::winuser::GetAsyncKeyState(VK_LMENU as i32) as i32;
            let is_pressed = (alt_state & 0x0009) != 0;
            println!("Alt state: {}, is_pressed: {}", alt_state, is_pressed);
            is_pressed
        }
    }

    pub fn set_keyboard_hook() -> HHOOK {
        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(Self::keyboard_hook),
                GetModuleHandleW(ptr::null()),
                0,
            );

            if hook.is_null() {
                panic!("Failed to set hook");
            }
            // HOOK.with(|hook_cell| {
            //     *hook_cell.borrow_mut() = Some(hook); // เปลี่ยนค่าใน RefCell
            // });
            hook
        }
    }

    pub fn unset_keyboard_hook(hook: HHOOK) {
        unsafe {
            // HOOK.with(|hook_cell| {
            //     if let Some(hook) = *hook_cell.borrow() {
            //         UnhookWindowsHookEx(hook as _);
            //         *hook_cell.borrow_mut() = None; // ล้างค่า
            //     }
            // });
            UnhookWindowsHookEx(hook as _);
        }
    }

    // unsafe extern "system" fn window_proc(
    //     hwnd: HWND,
    //     msg: UINT,
    //     w_param: WPARAM,
    //     l_param: LPARAM,
    // ) -> LRESULT {
    //     match msg {
    //         WM_DESTROY => {
    //             ClipCursor(ptr::null());
    //             ShowCursor(BOOL::from(true)); // แสดงเคอร์เซอร์อีกครั้ง
    //             PostQuitMessage(0);
    //             0
    //         }
    //         WM_SYSKEYDOWN | WM_DISPLAYCHANGE => {
    //             println!("Alt + Tab pressed!");
    //             0
    //         }
    //         _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    //     }
    // }

    fn to_string(value: &str) -> Vec<u16> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new(value)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect()
    }

    pub unsafe fn create_window_class(name: String) -> Vec<u16> {
        let class_name = Self::to_string(name.as_str());
        let h_instance = GetModuleHandleW(ptr::null());
        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcW),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(h_instance, IDI_APPLICATION),
            hCursor: LoadCursorW(h_instance, IDC_ARROW),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: LoadIconW(h_instance, IDI_APPLICATION),
        };

        if RegisterClassExW(&wnd_class) == 0 {
            Self::destroy();
        }
        class_name
    }

    pub unsafe fn create_window(class_name: Vec<u16>) -> HWND {
        let h_instance = GetModuleHandleW(ptr::null());
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW,
            class_name.as_ptr(),
            Self::to_string("SHARE_MOUSE").as_ptr(),
            WS_POPUP,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            GetSystemMetrics(SM_CXSCREEN),
            GetSystemMetrics(SM_CYSCREEN),
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );
        // let hwnd = CreateWindowExW(
        //     WS_EX_LAYERED | WS_EX_TOOLWINDOW,
        //     class_name.as_ptr(),
        //     Self::to_string("MyClass").as_ptr(),
        //     WS_OVERLAPPEDWINDOW & !WS_VISIBLE,
        //     CW_USEDEFAULT,
        //     CW_USEDEFAULT,
        //     GetSystemMetrics(SM_CXSCREEN),
        //     GetSystemMetrics(SM_CYSCREEN),
        //     ptr::null_mut(),
        //     ptr::null_mut(),
        //     GetModuleHandleW(std::ptr::null()),
        //     ptr::null_mut(),
        // );
        hwnd
    }

    pub unsafe fn show_window(hwnd: &HWND) -> BOOL {
        ShowWindow(*hwnd, SW_SHOW)
    }

    pub unsafe fn show_cursor(active: bool) -> c_int {
        ShowCursor(BOOL::from(active))
    }

    pub unsafe fn get_rect(hwnd: &HWND) -> RECT {
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(*hwnd, &mut rect);
        rect
    }

    pub unsafe fn lock_cursor(rect: &RECT) -> BOOL {
        ClipCursor(rect)
    }

    pub unsafe fn get_message(msg: &mut MSG) -> BOOL {
        GetMessageW(msg, ptr::null_mut(), 0, 0)
    }

    pub fn event(
        f: fn(
            Mouse,
            &mut ProtocolEvent,
            Screen,
            String,
            Vec<ScreenMappingMatrix>,
            Vec<ScreenSelector>,
        ) -> bool,
        event: &mut ProtocolEvent,
        screen: Screen,
        target_mac: String,
        s_screen_mapping: Vec<ScreenMappingMatrix>,
        s_screen_selector: Vec<ScreenSelector>,
    ) {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while Self::get_message(&mut msg) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
                // println!("Mouse msg: {}", msg.message);
                match msg.message {
                    WM_MOUSEMOVE => {
                        // println!("Mouse moved to position: ({}, {})", msg.pt.x, msg.pt.y);
                        if f(
                            Mouse {
                                x: msg.pt.x as f64,
                                y: msg.pt.y as f64,
                            },
                            event,
                            screen,
                            target_mac.clone(),
                            s_screen_mapping.clone(),
                            s_screen_selector.clone(),
                        ) {
                            break;
                        }
                    }
                    // WM_LBUTTONDOWN => {
                    //     println!("Mouse left button pressed");
                    // }
                    // WM_LBUTTONUP => {
                    //     println!("Mouse left button released");
                    // }
                    // WM_RBUTTONDOWN => {
                    //     println!("Mouse right button pressed");
                    // }
                    // WM_RBUTTONUP => {
                    //     println!("Mouse right button released");
                    // }
                    // WM_MOUSEWHEEL => {
                    //     let delta = GET_WHEEL_DELTA_WPARAM(msg.wParam) as i16;
                    //     if delta > 0 {
                    //         println!("Mouse wheel scrolled up: {}", delta);
                    //     } else {
                    //         println!("Mouse wheel scrolled down: {}", delta);
                    //     }
                    // }
                    // WM_XBUTTONUP => {
                    //     let xbutton = GET_XBUTTON_WPARAM(msg.wParam);
                    //     match xbutton {
                    //         XBUTTON1 => println!("XButton1 (Back) pressed"),
                    //         XBUTTON2 => println!("XButton2 (Forward) pressed"),
                    //         _ => println!("Unknown XButton pressed"),
                    //     }
                    // }
                    WM_KEYDOWN => {
                        // println!("Key pressed: {}", msg.wParam);
                        if let Some(k) = Self::handle_key_event(&msg) {
                            if k.eq_ignore_ascii_case("s") {
                                break;
                            }
                        }
                    }
                    WM_KEYUP => {
                        // println!("Key released: {}", msg.wParam);
                        Self::handle_key_event(&msg);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_key_event(msg: &MSG) -> Option<String> {
        let vk_code = msg.wParam as u32; // Virtual key code
        let mut buffer = [0u16; 4]; // Buffer for Unicode characters
        let mut key_state = [0u8; 256]; // Keyboard state array

        unsafe {
            // Get the current keyboard state
            if GetKeyboardState(key_state.as_mut_ptr()) != 0 {
                // Translate the virtual key code into a Unicode character
                let chars_copied = ToUnicode(
                    vk_code,
                    ((msg.lParam >> 16) as u32) & 0xff, // Scan code from lParam
                    key_state.as_ptr(),
                    buffer.as_mut_ptr(),
                    buffer.len() as i32,
                    0,
                );

                if chars_copied > 0 {
                    // Convert the UTF-16 result to a Rust String
                    let result = String::from_utf16_lossy(&buffer[..chars_copied as usize]);
                    // println!("Key pressed: {}", result);
                    Some(result)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    unsafe extern "system" fn destroy_window_callback(hwnd: HWND, _: isize) -> i32 {
        // Check if the window is valid
        if IsWindow(hwnd).is_positive() {
            DestroyWindow(hwnd);
        }
        // Continue enumeration
        1 // TRUE
    }

    fn destroy_all_windows() {
        unsafe {
            EnumWindows(Some(Self::destroy_window_callback), 0);
        }
    }

    pub fn destroy() {
        // unsafe {
        //     EnumWindows
        //     if DestroyAllWindows(*hwnd) == 0 {
        //         println!("Failed to destroy window: {}", GetLastError());
        //     }
        // }
        Self::destroy_all_windows();
    }
}
