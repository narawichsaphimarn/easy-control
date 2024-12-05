use crate::infrastructure::udp::socket_udp::{Event, EventEnum, SocketUdp, StepEnum};
use crate::shared::types::file_store_type::{ScreenMappingMatrix, ScreenSelector};
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use std::ptr;
use std::sync::Arc;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{BOOL, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::{HHOOK, HWND, POINT, RECT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, ClipCursor, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    EnumWindows, GetClientRect, GetKeyboardState, GetMessageW, GetSystemMetrics, IsWindow,
    LoadCursorW, LoadIconW, RegisterClassExW, SetWindowsHookExW, ShowCursor, ShowWindow, ToUnicode,
    TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, IDI_APPLICATION,
    KBDLLHOOKSTRUCT, MSG, SM_CXSCREEN, SM_CYSCREEN, SW_SHOW, VK_LMENU, WH_KEYBOARD_LL, WM_KEYDOWN,
    WM_KEYUP, WM_MOUSEMOVE, WM_SYSKEYDOWN, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_POPUP,
};
// TODO
/*
4) POC How to cancel all HOOK
*/

#[derive(Debug, Clone)]
pub struct HandleEventServiceApplication {
    pub class: Vec<u16>,
    pub socket: Arc<SocketUdp>,
}

impl HandleEventServiceApplication {
    pub async fn new() -> Self {
        let class = unsafe { Self::create_window_class("SHARE_MOUSE".to_string()) };
        let socket = Arc::new(SocketUdp::new().await);
        HandleEventServiceApplication { class, socket }
    }

    fn is_alt_pressed() -> bool {
        unsafe {
            let alt_state = winapi::um::winuser::GetAsyncKeyState(VK_LMENU as i32) as i32;
            let is_pressed = (alt_state & 0x0009) != 0;
            println!("Alt state: {}, is_pressed: {}", alt_state, is_pressed);
            is_pressed
        }
    }

    pub fn set_keyboard_hook(&self) -> HHOOK {
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
            hook
        }
    }

    pub fn unset_keyboard_hook(&self) {
        // unsafe {
        //     UnhookWindowsHookEx(hook as _);
        // }
    }

    fn convert_to_string(value: &str) -> Vec<u16> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        OsStr::new(value)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect()
    }

    pub unsafe fn create_window_class(name: String) -> Vec<u16> {
        let class_name = Self::convert_to_string(name.as_str());
        let h_instance = GetModuleHandleW(ptr::null());
        let wnd_class = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcW),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(h_instance, IDI_APPLICATION),
            hCursor: LoadCursorW(h_instance, IDC_ARROW),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: LoadIconW(h_instance, IDI_APPLICATION),
        };
        if RegisterClassExW(&wnd_class) == 0 {
            Self::destroy_all();
        }
        class_name
    }

    pub unsafe fn create_window(&self) {
        let h_instance = GetModuleHandleW(ptr::null());
        let hwmd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOOLWINDOW,
            self.class.clone().as_ptr(),
            Self::convert_to_string("SHARE_MOUSE").as_ptr(),
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
        Self::show_window(&hwmd);
        Self::lock_cursor(&hwmd);
        Self::show_cursor(false);
    }

    pub unsafe fn show_window(hwnd: &HWND) -> BOOL {
        ShowWindow(*hwnd, SW_SHOW)
    }

    pub unsafe fn show_cursor(active: bool) -> c_int {
        ShowCursor(BOOL::from(active))
    }

    unsafe fn get_rect(hwnd: &HWND) -> RECT {
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(*hwnd, &mut rect);
        rect
    }

    pub unsafe fn lock_cursor(hwnd: &HWND) -> BOOL {
        let rect = Self::get_rect(&hwnd);
        ClipCursor(&rect)
    }

    pub unsafe fn get_message(msg: &mut MSG) -> BOOL {
        GetMessageW(msg, ptr::null_mut(), 0, 0)
    }

    fn get_ip_target(s_screen_selector: Vec<ScreenSelector>, target_mac: String) -> String {
        let machine_target = s_screen_selector
            .iter()
            .find(|item| item.mac == target_mac)
            .cloned();
        match machine_target {
            Some(machine) => machine.ip,
            None => String::new(),
        }
    }

    fn map_event_to_string(event: EventEnum, step: StepEnum, message: String) -> String {
        let event = Event {
            event,
            step,
            message,
        };
        serde_json::to_string(&event).unwrap()
    }

    pub fn send_mouse_move(&self, pt: POINT, ip: String) {
        let socket = Arc::clone(&self.socket);
        let ip_arc = Arc::new(ip.clone());
        tokio::task::spawn(async move {
            let event = Self::map_event_to_string(
                EventEnum::Mouse,
                StepEnum::MouseMove,
                serde_json::to_string(
                    &(Mouse {
                        x: pt.x as f64,
                        y: pt.y as f64,
                    }),
                )
                .unwrap(),
            );
            socket.send(ip_arc.as_str(), event).await;
        });
    }

    pub fn event(
        &self,
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
            let ip = Self::get_ip_target(s_screen_selector.clone(), target_mac.clone());
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
                        } else {
                            self.send_mouse_move(msg.pt, ip.clone());
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

    fn get_hwnd_all_windows(f: unsafe extern "system" fn(HWND, isize) -> i32) -> BOOL {
        unsafe { EnumWindows(Some(f), 0) }
    }

    pub fn destroy(&self) {
        Self::get_hwnd_all_windows(Self::destroy_window_callback);
    }
    fn destroy_all() {
        Self::get_hwnd_all_windows(Self::destroy_window_callback);
    }

    pub fn stop_potocol(&self) {
        self.socket.destroy();
    }

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
}