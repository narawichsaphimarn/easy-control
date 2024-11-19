use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::domain::pojo::screen_mapping_matrix_pojo;
use crate::domain::pojo::screen_mapping_matrix_pojo::ScreenMappingMatrix;
use crate::domain::pojo::screen_selector_pojo::ScreenSelector;
use crate::domain::repositories::screen_mapping_matrix_repository::ScreenMappingMetricRepository;
use crate::domain::repositories::screen_selector_repository::ScreenSelectorRepository;
use crate::shared::constants::screen_constant::PositionAtEdge;
use crate::shared::constants::step_control_constant::StepControl;
use crate::shared::types::mouse_type::Mouse;
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use crate::shared::utils::mouse_util::MouseUtil;
use crate::shared::utils::protocol_util::ProtocolUtil;
use crate::shared::utils::screen_util::ScreenUtil;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct ServerStepServiceApplication {
    pub step_tx: Sender<StepControl>,
    pub step_rx: Receiver<StepControl>,
}

impl ServerStepServiceApplication {
    pub fn new() -> Arc<Self> {
        let (step_tx, step_rx) = watch::channel(StepControl::ServerLocal);
        Arc::new(ServerStepServiceApplication { step_tx, step_rx })
    }

    pub async fn run(self: Arc<Self>) {
        let mut step_rx = self.step_rx.clone();
        let mut event = ProtocolEvent::new();
        let _ = self.step_tx.send(StepControl::ServerLocal);
        while step_rx.changed().await.is_ok() {
            tokio::select! {
                _ = async {}, if self.step_rx.borrow().clone().to_string().eq_ignore_ascii_case
                ("LOCAL") => {
                    self.local(&mut event).await;
                }
                _ = async {}, if self.step_rx.borrow().clone().to_string().eq_ignore_ascii_case
                ("REMOTE") => {
                    log::debug!("REMOTE");
                    self.remote(&mut event).await;
                }
            }
        }
    }

    pub async fn local(&self, mut event: &mut ProtocolEvent) {
        log::debug!("Start LOCAL");
        let screen = ScreenUtil::get_screen_metrics();
        let (my_mc, ip) = Self::get_mac();
        event.source_width = screen.width;
        event.source_height = screen.height;
        event.source_mac = my_mc.clone().to_string();
        event.source_ip = ip.clone().to_string();
        let s_screen_mapping = Self::get_screen_metrics();
        let s_screen_selector = Self::get_screen_selector();
        loop {
            let point = MouseUtil::get_cursor_point();
            let current_edge = MouseUtil::check_position_at_edge(point, screen).unwrap();
            let s_matrix_match =
                Self::filter_screen_matrix(&s_screen_mapping, &my_mc, &current_edge.to_string());
            if let Ok(result) = Self::check_switch_screen(
                &s_matrix_match,
                &s_screen_selector,
                &mut event,
                current_edge,
                screen,
                point,
            ) {
                if result {
                    break;
                }
            }
        }
        log::debug!("End LOCAL | Event: {:?}", event);
        let _ = self.step_tx.send(StepControl::ServerRemote);
    }

    pub async fn remote(&self, mut event: &mut ProtocolEvent) {
        log::debug!("Start REMOTE");
        let screen = Screen {
            width: event.source_width,
            height: event.source_height,
        };
        let s_screen_mapping = Self::get_screen_metrics();
        let s_screen_selector = Self::get_screen_selector();
        loop {
            let point = MouseUtil::get_cursor_point();
            let current_edge = MouseUtil::check_position_at_edge(point, screen).unwrap();
            let s_matrix_match = Self::filter_screen_matrix(
                &s_screen_mapping,
                &event.target_mac,
                &current_edge.to_string(),
            );
            if let Ok(result) = Self::check_switch_screen(
                &s_matrix_match,
                &s_screen_selector,
                &mut event,
                current_edge,
                screen,
                point,
            ) {
                if result {
                    break;
                }
            }
        }
        log::debug!("End REMOTE | Event: {:?}", event);
        if (event.source_mac.eq_ignore_ascii_case(&event.target_mac)) {
            let _ = self.step_tx.send(StepControl::ServerLocal);
        } else {
            let _ = self.step_tx.send(StepControl::ServerRemote);
        }
    }

    fn check_switch_screen(
        s_matrix_match: &Option<&ScreenMappingMatrix>,
        s_screen_selector: &Vec<ScreenSelector>,
        mut event: &mut ProtocolEvent,
        current_edge: PositionAtEdge,
        screen: Screen,
        point: Mouse,
    ) -> Result<bool, ()> {
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
        s_screen_mapping: &'a Vec<screen_mapping_matrix_pojo::ScreenMappingMatrix>,
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

    fn get_screen_metrics() -> Vec<ScreenMappingMatrix> {
        let s_screen_mapping = if let Ok(result) = ScreenMappingMetricRepository::find_all() {
            result
        } else {
            Vec::new()
        };
        s_screen_mapping
    }

    fn get_screen_selector() -> Vec<ScreenSelector> {
        let s_screen_selector = if let Ok(result) = ScreenSelectorRepository::find_all() {
            result
        } else {
            Vec::new()
        };
        s_screen_selector
    }
}