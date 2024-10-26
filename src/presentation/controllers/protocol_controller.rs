use crate::presentation::models::system_model::Pagination;
use crate::shared::utils::protocol_util::ping_ip;
use crate::{
    application::services::protocol_service::ProtocolServiceApplication,
    shared::{
        constants::rest_status_constant::ResponseMessage,
        types::{response_type::ResponseStruct, system_type::System},
        utils::mapping::response_mapping::map_response,
    },
};
use axum::extract::Query;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn get_machine() -> impl IntoResponse {
    match ProtocolServiceApplication::check_machine() {
        Ok(data) => {
            let resp: ResponseStruct<Vec<System>> = map_response(
                ResponseMessage::Ok as u32,
                ResponseMessage::Ok.to_string(),
                None,
                Some(data),
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

pub async fn ping(pagination: Query<Pagination>) -> impl IntoResponse {
    (StatusCode::OK, Json(ping_ip(&*pagination.0.ip_addr)).into_response())
}