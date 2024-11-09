use axum::{
    extract,
    response::{IntoResponse, Json},
};
use reqwest::StatusCode;

use crate::{
    application::services::mouse_event_service::MouseEventServiceApplication,
    presentation::models::mouse_event_model::MouseEvent,
};

pub async fn mouse_event(extract::Json(request): extract::Json<MouseEvent>) -> impl IntoResponse {
    match MouseEventServiceApplication::mouse_event_process(request).await {
        Ok(_) => (StatusCode::OK, Json("").into_response()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(()).into_response()),
    }
}
