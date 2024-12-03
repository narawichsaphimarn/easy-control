use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ClientStepServiceApplication {
    pub is_shutdown: Arc<Mutex<bool>>,
}

impl ClientStepServiceApplication {
    pub fn new(is_shutdown: Arc<Mutex<bool>>) -> Self {
        ClientStepServiceApplication { is_shutdown }
    }

    pub async fn run(self: Arc<Self>) {}
}
