use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event: EventEnum,
    pub step: StepEnum,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventEnum {
    Mouse,
    Keyboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepEnum {
    MouseMove,
}

#[derive(Debug, Clone)]
pub struct SocketUdp {
    pub socket: Arc<UdpSocket>,
}

impl SocketUdp {
    pub async fn new() -> Self {
        SocketUdp {
            socket: Arc::new(
                UdpSocket::bind("0.0.0.0:9876")
                    .await
                    .expect("Failed to bind socket"),
            ),
        }
    }

    pub async fn send(&self, addr: &str, msg: String) {
        match self
            .socket
            .send_to(msg.as_bytes(), addr.to_owned() + ":8080")
            .await
        {
            Ok(_) => {}
            Err(e) => panic!("Failed to send: {:?}", e),
        }
    }

    pub async fn receive(&self) -> Event {
        let mut buf = [0; 1024];
        let (len, _) = self.socket.recv_from(&mut buf).await.unwrap();
        let json_str = String::from_utf8(buf[..len].to_vec()).unwrap();
        let data: Event = serde_json::from_str(&json_str).unwrap();
        data
    }

    pub fn destroy(&self) {
        // Explicitly drop the Arc
        std::mem::drop(self.socket.clone());
    }
}
