use crate::shared::{
    constants::rest_status_constant::ResponseMessage, types::response_type::ResponseStruct,
    utils::mapping::response_mapping::map_response,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn actuator() -> impl IntoResponse {
    let resp: ResponseStruct<String> = map_response(
        ResponseMessage::Ok as u32,
        ResponseMessage::Ok.to_string(),
        None,
        None,
    );
    return (StatusCode::OK, Json(resp).into_response());
}
