use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event: EventStep,
    pub step: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStep {
    Mouse,
    Keyboard,
}

#[derive(Debug, Clone)]
pub struct SocketUdp {
    pub socket: Arc<Mutex<UdpSocket>>,
}

impl SocketUdp {
    pub async fn new() -> Self {
        SocketUdp {
            socket: Arc::new(Mutex::new(
                UdpSocket::bind("0.0.0.0:9876")
                    .await
                    .expect("Failed to bind socket"),
            )),
        }
    }

    pub async fn send(&self, addr: &str, msg: String) {
        match self.socket.try_lock() {
            Ok(data) => match data
                .send_to(msg.as_bytes(), addr.to_owned() + ":9876")
                .await
            {
                Ok(_) => {}
                Err(e) => panic!("Failed to send: {:?}", e),
            },
            Err(e) => panic!("Failed to lock update: {:?}", e),
        }
    }

    pub async fn receive(&self) -> Event {
        let mut buf = [0; 1024];
        let (len, _) = self.socket.lock().await.recv_from(&mut buf).await.unwrap();
        let json_str = String::from_utf8(buf[..len].to_vec()).unwrap();
        let data: Event = serde_json::from_str(&json_str).unwrap();
        data
    }

    pub async fn get_socket(&self) -> MutexGuard<'_, UdpSocket> {
        self.socket.lock().await
    }
}
