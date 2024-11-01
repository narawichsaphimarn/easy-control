use axum::http::StatusCode;
use axum::{extract, response};
use crate::presentation::models::screen_model::ScreenMappingRequest;
use axum::response::IntoResponse;

pub async fn screen_mapping(extract::Json(request): extract::Json<Vec<ScreenMappingRequest>>) -> impl IntoResponse {
    log::debug!("screen_mapping: {:#?}", request);
    (
        StatusCode::OK,
        response::Json(request).into_response(),
    )
}