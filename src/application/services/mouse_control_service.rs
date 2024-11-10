use crate::application::services::mouse_event_service::MouseEventControlServiceApplication;
use crate::application::services::role_control_service::RoleControlServiceApplication;
use crate::application::services::screen_event_service::ScreenEventControlServiceApplication;
use std::sync::Arc;
use tokio::net::UdpSocket;

#[derive(Debug, Clone)]
pub struct MouseControlServiceApplication {
    pub mouse_event: Arc<MouseEventControlServiceApplication>,
    pub role: Arc<RoleControlServiceApplication>,
    pub screen_event: Arc<ScreenEventControlServiceApplication>,
}

impl MouseControlServiceApplication {
    pub fn new(
        mouse_event: Arc<MouseEventControlServiceApplication>,
        role: Arc<RoleControlServiceApplication>,
        screen_event: Arc<ScreenEventControlServiceApplication>,
    ) -> Self {
        MouseControlServiceApplication {
            mouse_event,
            role,
            screen_event,
        }
    }

    pub async fn run(self: Arc<Self>) {
        loop {
            tokio::select! {
                Some(socket) = async {
                    if let Ok(socket) = UdpSocket::bind("127.0.0.1:8080").await {
                        Some(socket)
                    } else {None}
                    }, if !self.role.get_is_server().await.clone() => {
                        log::debug!("Server listening on {:?}", socket.local_addr());
                        let mut buf = [0; 1024];
                        loop {
                            if let Ok((_, addr)) = socket.recv_from(&mut buf).await {
                                // log::debug!("Received from server: {}", String::from_utf8_lossy(&buf[..len]));
                                socket.send_to(b"Hello from server!", addr).await.expect("TODO: panic message");
                            }
                        }
                }
                Some(socket) = async {
                    if let Ok(socket) = UdpSocket::bind("127.0.0.1:0").await {
                        Some(socket)
                    } else {None}
                    }, if self.role.get_is_server().await.clone() => {
                        log::debug!("Client listening on {:?}", socket.local_addr());
                        // let server_addr = self.mouse_event.get_protocol_event().await.clone().ip;
                        // loop {
                        //     let m = get_cursor_point();
                        //     let s= format!("X {} Y {}", m.x, m.y);
                        //     socket.send_to(s.as_bytes(), server_addr.clone()).await.expect("TODO: panic \
                        //     message");
                        //     println!("Message sent to {}", server_addr);
                        //
                        //     let mut buf = [0; 1024];
                        //     if let Ok((len, _)) = socket.recv_from(&mut buf).await {
                        //         println!("Received from server: {}", String::from_utf8_lossy(&buf[..len]));
                        //     }
                        // }
                }
            }
        }
    }
}

//     pub async fn mouse_control(
//         data_mouse_event: Arc<Mutex<MouseEvent>>,
//         data_protocol_event: Arc<Mutex<ProtocolEvent>>
//     ) -> Result<(), String> {
//         let current_mac = data_protocol_event.lock().unwrap().mac.clone();
//         loop {
//             let _data_protocol_event = data_protocol_event.lock().unwrap();
//             if current_mac != _data_protocol_event.mac {
//                 log::debug!(
//                     "current_mac {} | _data_protocol_event.mac {}",
//                     current_mac,
//                     _data_protocol_event.mac
//                 );
//                 let _data_mouse_event = data_mouse_event.lock().unwrap();
//                 let mouse = mouse_different_pointer(
//                     &(Mouse {
//                         x: _data_mouse_event.x,
//                         y: _data_mouse_event.y,
//                     }),
//                     Screen {
//                         width: _data_protocol_event.source_width,
//                         height: _data_protocol_event.source_height,
//                     },
//                     Screen {
//                         width: _data_protocol_event.target_width,
//                         height: _data_protocol_event.target_height,
//                     }
//                 );
//                 let _ = sent_event(
//                     _data_protocol_event.ip.clone(),
//                     models::mouse_event_model::MouseEvent {
//                         event: 1,
//                         mouse,
//                     }
//                 );
//             }
//         }
//     }
