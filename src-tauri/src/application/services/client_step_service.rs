use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    infrastructure::udp::socket_udp::{EventEnum, SocketUdp, StepEnum},
    shared::{types::mouse_type::Mouse, utils::mouse_util::MouseUtil},
};

#[derive(Debug, Clone)]
pub struct ClientStepServiceApplication {
    pub is_shutdown: Arc<Mutex<bool>>,
    pub socket: SocketUdp,
}

impl ClientStepServiceApplication {
    pub async fn new(is_shutdown: Arc<Mutex<bool>>) -> Arc<Self> {
        let socket = SocketUdp::new().await;
        Arc::new(ClientStepServiceApplication {
            is_shutdown,
            socket,
        })
    }

    pub async fn run(self: Arc<Self>) {
        while !*self.is_shutdown.lock().await {
            let event = self.socket.receive().await;
            match event.event {
                EventEnum::Mouse => match event.step {
                    StepEnum::MouseMove => {
                        let msg: Mouse = serde_json::from_str(&event.message).unwrap();
                        MouseUtil::move_cursor(msg.x as i32, msg.y as i32);
                    }
                },
                EventEnum::Keyboard => {}
            }
        }
        println!("Stop client");
    }
}
