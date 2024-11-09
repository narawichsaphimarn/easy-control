use crate::shared::constants::screen_constant::map_from_string;
use crate::shared::types::mouse_type::{Mouse, MouseEvent};
use crate::shared::types::protocol_type::ProtocolEvent;
use crate::shared::types::screen_type::Screen;
use crate::shared::utils::mouse_util::{
    check_position_at_edge, get_cursor_point, revere_mouse_position,
};
use crate::shared::utils::protocol_util::{get_addrs, get_mac_addr};
use crate::shared::utils::screen_util::get_screen_metrics;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::{watch, Mutex, MutexGuard};

use super::protocol_service::ProtocolServiceApplication;

#[derive(Debug, Clone)]
pub struct MouseEventControlServiceApplication {
    mouse_event_tx: Sender<MouseEvent>,
    mouse_event_rx: Receiver<MouseEvent>,
    protocol_event_tx: Sender<ProtocolEvent>,
    protocol_event_rx: Receiver<ProtocolEvent>,
    protocol_mutex: Arc<Mutex<ProtocolEvent>>,
    switch: Arc<Mutex<bool>>,
}

impl MouseEventControlServiceApplication {
    pub fn new_protocol_event() -> ProtocolEvent {
        let ips: (String, String) = get_addrs();
        let (select_ip, _) = ProtocolServiceApplication::select_ip(ips);
        let screen = get_screen_metrics();
        let protocol_event = ProtocolEvent {
            mac: get_mac_addr(select_ip.clone()),
            ip: select_ip,
            edge: String::new(),
            source_width: screen.width,
            source_height: screen.height,
            target_width: 0,
            target_height: 0,
            x: 0.0,
            y: 0.0,
        };
        protocol_event
    }

    pub fn new() -> Self {
        let (mouse_event_tx, mouse_event_rx) = watch::channel(MouseEvent {
            x: 0.0,
            y: 0.0,
            edge: String::new(),
        });
        let protocol_event = Self::new_protocol_event();
        let (protocol_event_tx, protocol_event_rx) = watch::channel(protocol_event.clone());
        let protocol_mutex = Arc::new(Mutex::new(protocol_event));
        MouseEventControlServiceApplication {
            mouse_event_tx,
            mouse_event_rx,
            protocol_event_tx,
            protocol_event_rx,
            protocol_mutex,
            switch: Arc::new(Mutex::new(false)),
        }
    }

    pub fn get_mouse_event_rx(&self) -> Receiver<MouseEvent> {
        self.mouse_event_rx.clone()
    }

    pub fn get_protocol_event_rx(&self) -> Receiver<ProtocolEvent> {
        self.protocol_event_rx.clone()
    }

    pub fn get_protocol_event_tx(&self) -> Sender<ProtocolEvent> {
        self.protocol_event_tx.clone()
    }

    pub fn get_protocol_mutex(&self) -> Arc<Mutex<ProtocolEvent>> {
        Arc::clone(&self.protocol_mutex)
    }

    pub async fn get_protocol_event(&self) -> MutexGuard<ProtocolEvent> {
        let value = self.protocol_mutex.lock().await;
        value
    }

    pub async fn update_protocol_event(&self, protocol_event: ProtocolEvent) {
        let mut value = self.protocol_mutex.lock().await;
        *value = protocol_event;
    }

    pub fn send_protocol_event(&self, event: ProtocolEvent) {
        let _ = self.protocol_event_tx.send(event);
    }

    pub async fn wait_switch_cursor(self: Arc<Self>) {
        let mut rx = self.get_protocol_event_rx();
        tokio::task::spawn(async move {
            while rx.changed().await.is_ok() {
                let value = rx.borrow().clone();
                revere_mouse_position(
                    map_from_string(value.edge),
                    Screen {
                        width: value.source_width,
                        height: value.source_height,
                    },
                    Mouse {
                        x: value.x,
                        y: value.y,
                    },
                );
            }
        });
    }

    pub async fn wait_update_protocol_event(self: Arc<Self>) {
        let mut rx = self.get_protocol_event_rx();
        let protocol_mutex = Arc::clone(&self.protocol_mutex);
        tokio::task::spawn(async move {
            while rx.changed().await.is_ok() {
                let mut protocol_guard = protocol_mutex.lock().await;
                let value = rx.borrow().clone();
                *protocol_guard = value.clone();
                log::debug!("Now screen IP:{}, MAC:{}", value.ip, value.mac);
            }
        });
    }

    pub async fn run(self: Arc<Self>) {
        let mut protocol_event_rx = self.protocol_event_rx.clone();
        let current_screen = get_screen_metrics();
        let switch = Arc::clone(&self.switch);
        let protocol_mutex = Arc::clone(&self.protocol_mutex);
        loop {
            let mut switch = switch.lock().await;
            tokio::select! {
                _ = protocol_event_rx.changed() => {
                    *switch = true;
                    let mut protocol_guard = protocol_mutex.lock().await;
                    let value = protocol_event_rx.borrow().clone();
                    *protocol_guard = value.clone();
                    sleep(Duration::from_millis(100));
                    *switch = false;
                    log::debug!("Now screen IP:{}, MAC:{}", value.ip, value.mac);
                }
                _ = tokio::time::sleep(Duration::from_millis(1)), if !switch.clone() => {
                    let current_point = get_cursor_point();
                    let current_edge = check_position_at_edge(current_point, current_screen);
                    let _ = self.mouse_event_tx.send(MouseEvent { x: current_point.x, y: current_point.y, edge: current_edge.unwrap().to_string() });
                }
            }
        }
    }
}
