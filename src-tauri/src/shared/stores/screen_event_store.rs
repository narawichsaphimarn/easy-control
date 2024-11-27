use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct ScreenEventControl {
    pub update: Arc<Mutex<bool>>,
}

impl ScreenEventControl {
    pub fn new() -> Self {
        ScreenEventControl {
            update: Arc::new(Mutex::new(true)),
        }
    }

    pub async fn get_update(&self) -> MutexGuard<'_, bool> {
        let data = self.update.lock().await;
        data
    }

    pub async fn update_data(&self, status: bool) {
        match self.update.try_lock() {
            Ok(mut data) => {
                *data = status;
            }
            Err(e) => {
                // log::error!("Failed to lock update: {:?}", e)
            }
        }
    }
}
