use std::sync::Arc;

use crate::application::services::role_control_service::RoleControlServiceApplication;
use crate::application::services::screen_event_service::ScreenEventControlServiceApplication;
use crate::presentation::controllers::protocol_controller::ping;
use crate::presentation::controllers::role_controller::update_role;
use crate::presentation::controllers::screen_controller::{screen_mapping, screen_mapping_update};
use crate::presentation::controllers::{
    actuator_controller::actuator, protocol_controller::get_machine,
    system_controller::get_system_detail,
};
use axum::{
    http::StatusCode,
    routing::{get, post, put},
    Router,
};

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub fn route(
    screen_event: Arc<ScreenEventControlServiceApplication>,
    role: Arc<RoleControlServiceApplication>,
) -> Router {
    let app: Router<_> = Router::new()
        .route("/api/status", get(actuator))
        .route("/api/v1/check-machine", get(get_machine))
        .route("/api/v1/system-detail", get(get_system_detail))
        .route("/api/v1/ping", get(ping))
        .route(
            "/api/v1/screen-matrix",
            post({
                let screen_event = Arc::clone(&screen_event);
                move |body| screen_mapping(body, screen_event)
            }),
        )
        .route("/api/v1/screen-matrix", put(screen_mapping_update))
        .route(
            "/api/v1/update-role",
            get({
                let role = Arc::clone(&role);
                move |query| update_role(query, role)
            }),
        )
        .fallback(fallback);
    app
}
