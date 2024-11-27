use std::sync::Arc;

use crate::presentation::controllers::protocol_controller::ping;
use crate::presentation::controllers::role_controller::update_role;
use crate::presentation::controllers::screen_controller::{screen_mapping, screen_mapping_update};
use crate::presentation::controllers::{
    actuator_controller::actuator, protocol_controller::get_machine,
    system_controller::get_system_detail,
};
// use crate::shared::stores::stores::Stores;
use crate::shared::stores::stores_v2::StoresV2;
use axum::{
    http::StatusCode,
    routing::{get, post, put},
    Router,
};

#[derive(Debug, Clone)]
pub struct AxumRouter {
    pub stores: Arc<StoresV2>,
}

impl AxumRouter {
    pub fn new(stores: Arc<StoresV2>) -> Self {
        AxumRouter { stores }
    }

    async fn fallback() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not Found")
    }

    pub fn route(&self) -> Router {
        let app: Router<_> = Router::new()
            .route("/api/status", get(actuator))
            .route("/api/v1/check-machine", get(get_machine))
            .route("/api/v1/system-detail", get(get_system_detail))
            .route("/api/v1/ping", get(ping))
            .route(
                "/api/v1/screen-matrix",
                post({
                    let step_control = Arc::clone(&self.stores.step_control);
                    move |body| screen_mapping(body, step_control)
                }),
            )
            .route("/api/v1/screen-matrix", put(screen_mapping_update))
            .route(
                "/api/v1/update-role",
                get({
                    let step_control = Arc::clone(&self.stores.step_control);
                    move |query| update_role(query, step_control)
                }),
            )
            .fallback(Self::fallback);
        app
    }
}
