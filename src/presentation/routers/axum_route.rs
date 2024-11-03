use crate::presentation::controllers::protocol_controller::ping;
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

pub fn route() -> Router {
    let app: Router<_> = Router::new()
        .route("/api/status", get(actuator))
        .route("/api/v1/check-machine", get(get_machine))
        .route("/api/v1/system-detail", get(get_system_detail))
        .route("/api/v1/ping", get(ping))
        .route("/api/v1/screen-matrix", post(screen_mapping))
        .route("/api/v1/screen-matrix", put(screen_mapping_update))
        .fallback(fallback);
    app
}
