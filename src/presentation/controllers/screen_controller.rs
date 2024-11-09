use std::sync::Arc;

use crate::application::services::control_service::ScreenEventControlServiceApplication;
use crate::application::services::screen_service::ScreenServiceApplication;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::rest_status_constant::ResponseMessage;
use crate::shared::types::response_type::ResponseStruct;
use crate::shared::types::system_type::System;
use crate::shared::utils::mapping::response_mapping::map_response;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract, Json};

pub async fn screen_mapping(
    extract::Json(request): extract::Json<Vec<ScreenMappingRequest>>,
    screen_event: Arc<ScreenEventControlServiceApplication>,
) -> impl IntoResponse {
    match ScreenServiceApplication::screen_mapping_process(request).await {
        Ok(_) => {
            let resp: ResponseStruct<Vec<System>> = map_response(
                ResponseMessage::Ok as u32,
                ResponseMessage::Ok.to_string(),
                None,
                None,
            );
            screen_event.update_data(true).await;
            (StatusCode::OK, Json(resp).into_response())
        }
        Err(s) => {
            let resp: ResponseStruct<String> = map_response(
                ResponseMessage::Err as u32,
                ResponseMessage::Err.to_string(),
                Some(s),
                None,
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(resp).into_response(),
            )
        }
    }
}

pub async fn screen_mapping_update(
    extract::Json(request): extract::Json<Vec<ScreenMappingRequest>>,
) -> impl IntoResponse {
    match ScreenServiceApplication::screen_mapping_update(request).await {
        Ok(data) => {
            let resp: ResponseStruct<Vec<System>> = map_response(
                ResponseMessage::Ok as u32,
                ResponseMessage::Ok.to_string(),
                None,
                None,
            );
            (StatusCode::OK, Json(resp).into_response())
        }
        Err(s) => {
            let resp: ResponseStruct<String> = map_response(
                ResponseMessage::Err as u32,
                ResponseMessage::Err.to_string(),
                Some(s),
                None,
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(resp).into_response(),
            )
        }
    }
}
