use std::sync::Arc;

use crate::presentation::controllers::protocol_controller::ping;
use crate::presentation::controllers::screen_controller::screen_mapping_update;
use crate::presentation::controllers::system_controller::get_system_detail;
use crate::{
    infrastructure::database::store_file::file_store::FileStore,
    presentation::controllers::actuator_controller::actuator,
};
use axum::{
    http::StatusCode,
    routing::{get, post, put},
    Router,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AxumRouter {
    pub filestore: Arc<Mutex<FileStore>>,
}

impl AxumRouter {
    pub fn new(filestore: Arc<Mutex<FileStore>>) -> Self {
        AxumRouter { filestore }
    }

    async fn fallback() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not Found")
    }

    pub fn route(&self) -> Router {
        let app: Router<_> = Router::new()
            .route("/api/status", get(actuator))
            .route("/api/v1/system-detail", get(get_system_detail))
            .route("/api/v1/ping", get(ping))
            .route(
                "/api/v1/screen-matrix",
                put({
                    let store = Arc::clone(&self.filestore);
                    move |body| screen_mapping_update(body, store)
                }),
            )
            .fallback(Self::fallback);
        app
    }
}
