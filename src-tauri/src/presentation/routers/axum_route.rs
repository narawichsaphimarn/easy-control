use std::sync::Arc;

use crate::presentation::controllers::actuator_controller::actuator;
use crate::presentation::controllers::protocol_controller::ping;
use crate::presentation::controllers::screen_controller::screen_mapping_update;
use crate::presentation::controllers::system_controller::get_system_detail;
use crate::shared::stores::store_json::Stores;
use axum::{
    http::StatusCode,
    routing::{get, put},
    Router,
};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AxumRouter {
    pub store: Arc<Mutex<Stores>>,
}

impl AxumRouter {
    pub fn new(store: Arc<Mutex<Stores>>) -> Self {
        AxumRouter { store }
    }

    async fn fallback() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not Found")
    }

    pub fn route(&self) -> Router {
        let app: Router<_> = Router::new()
            .route("/api/status", get(actuator))
            .route("/api/v1/system-detail", get(get_system_detail))
            .route("/api/v1/ping", get(ping))
            .route("/api/v1/screen-matrix", put(screen_mapping_update))
            .fallback(Self::fallback);
        app
    }
}
