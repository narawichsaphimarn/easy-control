use crate::shared::stores::stores_v2::StoresV2;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct StepControlServiceApplication {
    pub stores_v2: Arc<StoresV2>,
}

impl StepControlServiceApplication {
    pub fn new(stores_v2: Arc<StoresV2>) -> Arc<Self> {
        Arc::new(StepControlServiceApplication { stores_v2 })
    }

    pub async fn start(self: Arc<Self>) {

    }
}
