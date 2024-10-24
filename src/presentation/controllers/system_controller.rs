use crate::{
    application::services::system_service::SystemServiceApplication,
    presentation::models::system_model::Pagination,
    shared::{
        constants::rest_status_constant::ResponseMessage,
        types::{response_type::ResponseStruct, system_type::System},
        utils::general::mapping::response_mapping::map_response,
    },
};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn get_system_detail(pagination: Query<Pagination>) -> impl IntoResponse {
    match SystemServiceApplication::get_system_detail(pagination.0.ip_addr) {
        Ok(system) => {
            let resp: ResponseStruct<System> = map_response(
                ResponseMessage::Ok as u32,
                ResponseMessage::Ok.to_string(),
                None,
                Some(system),
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
