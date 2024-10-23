use axum::{http::StatusCode, routing::get, Router};

use crate::presentation::controllers::{
    actuator_controller::actuator, protocol_controller::get_machine,
    system_controller::get_system_detail,
};

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

pub fn route() -> Router {
    let app: Router<_> = Router::new()
        .route("/api/status", get(actuator))
        .route("/api/v1/check-machine", get(get_machine))
        .route("/api/v1/system-detail", get(get_system_detail))
        .fallback(fallback);
    app
}
