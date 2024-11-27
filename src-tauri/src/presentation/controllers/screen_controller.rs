use std::sync::Arc;

use crate::application::services::screen_service::ScreenServiceApplication;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::event_process_constant::EventProcess;
use crate::shared::constants::rest_status_constant::ResponseMessage;
use crate::shared::stores::screen_event_store::ScreenEventControl;
use crate::shared::stores::step_control_store::StepControlStore;
use crate::shared::types::response_type::ResponseStruct;
use crate::shared::types::system_type::System;
use crate::shared::utils::mapping::response_mapping::map_response;
use axum::extract;
use axum::http::StatusCode;
use axum::response;
use axum::response::IntoResponse;
use tower_http::follow_redirect::policy::PolicyExt;

pub async fn screen_mapping(
    extract::Json(request): extract::Json<Vec<ScreenMappingRequest>>,
    step_control: Arc<StepControlStore>,
) -> impl IntoResponse {
    match ScreenServiceApplication::screen_mapping_process(request).await {
        Ok(_) => {
            let resp: ResponseStruct<Vec<System>> = map_response(
                ResponseMessage::Ok as u32,
                ResponseMessage::Ok.to_string(),
                None,
                None,
            );
            step_control.send(EventProcess::Restart);
            (StatusCode::OK, response::Json(resp).into_response())
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
                response::Json(resp).into_response(),
            )
        }
    }
}

pub async fn screen_mapping_update(
    extract::Json(request): extract::Json<Vec<ScreenMappingRequest>>,
) -> impl IntoResponse {
    match ScreenServiceApplication::screen_mapping_update(request).await {
        Ok(_) => {
            let resp: ResponseStruct<Vec<System>> = map_response(
                ResponseMessage::Ok as u32,
                ResponseMessage::Ok.to_string(),
                None,
                None,
            );
            (StatusCode::OK, response::Json(resp).into_response())
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
                response::Json(resp).into_response(),
            )
        }
    }
}