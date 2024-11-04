use crate::domain::pojo::screen_mapping_matrix_pojo::ScreenMappingMatrix;
use crate::domain::repositories::screen_mapping_matrix_repository::ScreenMappingMetricRepository;
use crate::domain::repositories::screen_selector_repository::ScreenSelectorRepository;
use crate::presentation::models;
use crate::shared::constants::screen_constant::map_from_string;
use crate::shared::rest_client::mouse_event_rest_client::sent_event;
use crate::shared::types::mouse_type::{ Mouse, MouseEvent };
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use crate::shared::utils::mouse_util::{
    check_position_at_edge,
    get_cursor_point,
    mouse_different_pointer,
    revere_mouse_position,
};
use crate::shared::utils::screen_util::get_screen_metrics;
use std::sync::{ Arc, Mutex };
use std::thread::sleep;
use std::time::Duration;

pub struct ControlServiceApplication;

impl ControlServiceApplication {
    pub async fn mouse_event(data_mouse_event: Arc<Mutex<MouseEvent>>) {
        loop {
            let current_point = get_cursor_point();
            let current_screen = get_screen_metrics();
            let current_edge = check_position_at_edge(current_point, current_screen);
            let mut data = data_mouse_event.lock().unwrap();
            let new_data = MouseEvent {
                x: current_point.x,
                y: current_point.y,
                edge: current_edge.unwrap().to_string(),
            };
            *data = new_data;
        }
    }

    pub async fn mouse_control(
        data_mouse_event: Arc<Mutex<MouseEvent>>,
        data_protocol_event: Arc<Mutex<ProtocolEvent>>
    ) -> Result<(), String> {
        let current_mac = data_protocol_event.lock().unwrap().mac.clone();
        loop {
            let _data_protocol_event = data_protocol_event.lock().unwrap();
            if current_mac != _data_protocol_event.mac {
                log::debug!(
                    "current_mac {} | _data_protocol_event.mac {}",
                    current_mac,
                    _data_protocol_event.mac
                );
                let _data_mouse_event = data_mouse_event.lock().unwrap();
                let mouse = mouse_different_pointer(
                    &(Mouse {
                        x: _data_mouse_event.x,
                        y: _data_mouse_event.y,
                    }),
                    Screen {
                        width: _data_protocol_event.source_width,
                        height: _data_protocol_event.source_height,
                    },
                    Screen {
                        width: _data_protocol_event.target_width,
                        height: _data_protocol_event.target_height,
                    }
                );
                let _ = sent_event(
                    _data_protocol_event.ip.clone(),
                    models::mouse_event_model::MouseEvent {
                        event: 1,
                        mouse,
                    }
                );
            }
        }
    }

    pub async fn screen_event(
        data_mouse_event: Arc<Mutex<MouseEvent>>,
        data_protocol_event: Arc<Mutex<ProtocolEvent>>,
        mouse_switch: Arc<Mutex<bool>>
    ) -> Result<(), String> {
        let s_matrix: Vec<ScreenMappingMatrix> = ScreenMappingMetricRepository::find_all().map_err(
            |e| e.to_string()
        )?;
        let s_select = ScreenSelectorRepository::find_all().map_err(|e| e.to_string())?;
        let screen = get_screen_metrics();
        let mut protocol_event: Vec<ProtocolEvent> = Vec::new();
        loop {
            let mut _mouse_switch = mouse_switch.lock().unwrap();
            loop {
                if *_mouse_switch == false {
                    let _data_mouse_event = data_mouse_event.lock().unwrap();
                    let mut _data_protocol_event = data_protocol_event.lock().unwrap();
                    if
                        !_data_mouse_event.edge.eq_ignore_ascii_case("NONE") &&
                        !_data_mouse_event.edge.is_empty()
                    {
                        let s_matrix_match = s_matrix
                            .iter()
                            .find(|x| {
                                x.mac_source.eq_ignore_ascii_case(&_data_protocol_event.mac) &&
                                    x.edge.eq_ignore_ascii_case(&_data_mouse_event.edge)
                            });
                        if let Some(s_matrix_match) = s_matrix_match {
                            let s_select_match = s_select
                                .iter()
                                .find(|x| x.mac.eq_ignore_ascii_case(&s_matrix_match.mac_target));
                            if let Some(s_select_match) = s_select_match {
                                log::debug!("Before {:?}", _data_protocol_event);
                                let x = ProtocolEvent {
                                    mac: s_select_match.mac.clone(),
                                    ip: s_select_match.ip.to_owned(),
                                    edge: s_matrix_match.edge.to_string(),
                                    source_width: screen.width,
                                    source_height: screen.height,
                                    target_width: s_select_match.width.parse::<i32>().unwrap(),
                                    target_height: s_select_match.height.parse::<i32>().unwrap(),
                                };
                                protocol_event.push(x);
                                *_mouse_switch = true;
                                break;
                            }
                        }
                    }
                } else {
                    sleep(Duration::from_millis(3));
                    *_mouse_switch = false;
                }
            }
            if *_mouse_switch == true {
                let _data_mouse_event = data_mouse_event.lock().unwrap();
                let mut _data_protocol_event = data_protocol_event.lock().unwrap();
                revere_mouse_position(
                    map_from_string(_data_mouse_event.edge.to_string()),
                    Screen {
                        width: screen.width,
                        height: screen.height,
                    },
                    Mouse {
                        x: _data_mouse_event.x,
                        y: _data_mouse_event.y,
                    }
                );
                log::debug!("protocol_event {:?} _mouse_switch {}", protocol_event, _mouse_switch);
                *_data_protocol_event = protocol_event.get(0).unwrap().clone();
                protocol_event.clear();
                log::debug!("After {:?}", _data_protocol_event);
            } else {
                protocol_event.clear();
            }
        }
    }
}
