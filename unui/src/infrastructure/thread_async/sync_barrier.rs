use std::sync::Arc;
use tokio::sync::Barrier;

pub struct SyncBarrier {
    pub barrier: Arc<Barrier>,
}

impl SyncBarrier {
    pub fn new(count: usize) -> Self {
        SyncBarrier {
            barrier: Arc::new(Barrier::new(count)),
        }
    }

    pub async fn wait(&self) {
        self.barrier.wait().await;
    }
}
