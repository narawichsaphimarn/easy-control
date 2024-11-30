use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::shared::constants::step_control_constant::StepControl;
use crate::shared::lib::lib_event::Window;
use crate::shared::stores::store_json::Stores;
use crate::shared::types::file_store_type::{ScreenMappingMatrix, ScreenSelector};
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use crate::shared::utils::mouse_util::MouseUtil;
use crate::shared::utils::protocol_util::ProtocolUtil;
use crate::shared::utils::screen_util::ScreenUtil;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::{watch, Mutex};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND__;

#[derive(Debug, Clone)]
pub struct ServerStepServiceApplication {
    pub step_tx: Sender<StepControl>,
    pub step_rx: Receiver<StepControl>,
    pub cancel_flag: Arc<AtomicBool>,
    pub store: Arc<Mutex<Stores>>,
}

impl ServerStepServiceApplication {
    pub fn new(store: Arc<Mutex<Stores>>) -> Arc<Self> {
        let (step_tx, step_rx) = watch::channel(StepControl::ServerLocal);
        Arc::new(ServerStepServiceApplication {
            step_tx,
            step_rx,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            store,
        })
    }

    pub async fn run(self: Arc<Self>) {
        let mut step_rx = self.step_rx.clone();
        let mut event = ProtocolEvent::new();
        let _ = self.step_tx.send(StepControl::ServerLocal);
        let class = unsafe { Window::create_window_class("SHARE_MOUSE".to_string()) };
        while step_rx.changed().await.is_ok() {
            tokio::select! {
                _ = async {}, if self.step_rx.borrow().clone().to_string().eq_ignore_ascii_case
                ("LOCAL") => {
                    Window::destroy();
                    self.local(&mut event).await;
                }
                _ = async {}, if self.step_rx.borrow().clone().to_string().eq_ignore_ascii_case
                ("REMOTE") => {
                    self.remote(&mut event, class.clone()).await;
                }
                _ = async {}, if self.step_rx.borrow().clone().to_string().eq_ignore_ascii_case
                ("AGAIN") => {
                    self.again(&mut event).await;
                }
            }
        }
    }

    pub async fn local(&self, mut event: &mut ProtocolEvent) {
        // log::debug!("Start LOCAL");
        let screen = ScreenUtil::get_screen_metrics();
        let (my_mc, ip) = Self::get_mac();
        event.source_width = screen.width;
        event.source_height = screen.height;
        event.source_mac = my_mc.clone().to_string();
        event.source_ip = ip.clone().to_string();
        let store = self.store.lock().await;
        Self::handle_loop_switch_screen(
            &mut event,
            screen,
            my_mc,
            store.screen_selector.clone(),
            store.screen_mapping_matrix.clone(),
        );
        // log::debug!("End LOCAL | Event: {:?}", event);
        let _ = self.step_tx.send(StepControl::ServerRemote);
    }

    pub async fn remote(&self, mut event: &mut ProtocolEvent, class: Vec<u16>) {
        // log::debug!("Start REMOTE");
        let screen = Screen {
            width: event.source_width,
            height: event.source_height,
        };
        let mac = event.target_mac.clone();
        let store = self.store.lock().await;
        #[cfg(target_os = "windows")]
        unsafe {
            let raw_hwnd: *mut HWND__ = Window::create_window(class.clone());
            Window::show_window(&raw_hwnd);
            let rect = Window::get_rect(&raw_hwnd);
            Window::show_cursor(false);
            Window::lock_cursor(&rect);
            Window::event(
                Self::handle_loop_switch_screen_for_event,
                &mut event,
                screen,
                mac,
                store.screen_mapping_matrix.clone(),
                store.screen_selector.clone(),
            );
        }
        // log::debug!("End REMOTE | Event: {:?}", event);
        self.switch_screen(&mut event);
    }

    pub async fn again(&self, mut event: &mut ProtocolEvent) {
        // log::debug!("Start REMOTE AGAIN");
        let screen = Screen {
            width: event.source_width,
            height: event.source_height,
        };
        let mac = event.target_mac.clone();
        let store = self.store.lock().await;
        #[cfg(target_os = "windows")]
        unsafe {
            Window::event(
                Self::handle_loop_switch_screen_for_event,
                &mut event,
                screen,
                mac,
                store.screen_mapping_matrix.clone(),
                store.screen_selector.clone(),
            );
        }
        // log::debug!("End  REMOTE AGAIN | Event: {:?}", event);
        self.switch_screen(&mut event);
    }

    fn switch_screen(&self, event: &mut ProtocolEvent) {
        if event.source_mac.eq_ignore_ascii_case(&event.target_mac) {
            let _ = self.step_tx.send(StepControl::ServerLocal);
        } else {
            let _ = self.step_tx.send(StepControl::ServerRemoteAgain);
        }
    }

    fn handle_loop_switch_screen(
        mut event: &mut ProtocolEvent,
        screen: Screen,
        target_mac: String,
        s_screen_selector: Vec<ScreenSelector>,
        s_screen_mapping: Vec<ScreenMappingMatrix>,
    ) {
        loop {
            let point = MouseUtil::get_cursor_point();
            if let Ok(result) = Self::check_switch_screen(
                &s_screen_mapping,
                &s_screen_selector,
                &mut event,
                screen,
                point,
                target_mac.clone(),
            ) {
                if result {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    pub fn handle_loop_switch_screen_for_event(
        point: Mouse,
        mut event: &mut ProtocolEvent,
        screen: Screen,
        target_mac: String,
        s_screen_mapping: Vec<ScreenMappingMatrix>,
        s_screen_selector: Vec<ScreenSelector>,
    ) -> bool {
        if let Ok(result) = Self::check_switch_screen(
            &s_screen_mapping,
            &s_screen_selector,
            &mut event,
            screen,
            point,
            target_mac.clone(),
        ) {
            result
        } else {
            true
        }
    }

    fn check_switch_screen(
        s_screen_mapping: &Vec<ScreenMappingMatrix>,
        s_screen_selector: &Vec<ScreenSelector>,
        event: &mut ProtocolEvent,
        screen: Screen,
        point: Mouse,
        target_mac: String,
    ) -> Result<bool, ()> {
        let current_edge = MouseUtil::check_position_at_edge(point, screen).unwrap();
        let s_matrix_match =
            Self::filter_screen_matrix(&s_screen_mapping, &target_mac, &current_edge.to_string());
        if let Some(s_matrix_match) = s_matrix_match {
            let s_select_match =
                Self::filter_screen_selector(&s_screen_selector, &s_matrix_match.mac_target);
            if let Some(s_select_match) = s_select_match {
                event.target_ip = s_select_match.ip.clone();
                event.target_mac = s_select_match.mac.clone();
                event.target_height = s_select_match.height.parse::<i32>().unwrap();
                event.target_width = s_select_match.width.parse::<i32>().unwrap();
                event.edge = current_edge.to_string();
                MouseUtil::revere_mouse_position(current_edge, screen, point);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn filter_screen_matrix<'a>(
        s_screen_mapping: &'a Vec<ScreenMappingMatrix>,
        mac: &str,
        edge: &str,
    ) -> Option<&'a ScreenMappingMatrix> {
        let s_matrix_match = s_screen_mapping
            .iter()
            .find(|x| x.mac_source.eq_ignore_ascii_case(mac) && x.edge.eq_ignore_ascii_case(edge));
        s_matrix_match
    }

    fn filter_screen_selector<'a>(
        s_screen_selector: &'a Vec<ScreenSelector>,
        mac_target: &str,
    ) -> Option<&'a ScreenSelector> {
        let s_select_match = s_screen_selector
            .iter()
            .find(|x| x.mac.eq_ignore_ascii_case(mac_target));
        s_select_match
    }

    fn get_mac() -> (String, String) {
        let ips: (String, String) = ProtocolUtil::get_addrs();
        let (select_ip, _) = ProtocolServiceApplication::select_ip(ips);
        (ProtocolUtil::get_mac_addr(select_ip.clone()), select_ip)
    }

    // fn get_screen_metrics() -> Vec<ScreenMappingMatrix> {
    //     let s_screen_mapping = if let Ok(result) = ScreenMappingMetricRepository::find_all() {
    //         result
    //     } else {
    //         Vec::new()
    //     };
    //     s_screen_mapping
    // }

    // fn get_screen_selector() -> Vec<ScreenSelector> {
    //     let s_screen_selector = if let Ok(result) = ScreenSelectorRepository::find_all() {
    //         result
    //     } else {
    //         Vec::new()
    //     };
    //     s_screen_selector
    // }
}
