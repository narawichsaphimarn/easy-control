use crate::shared::{
    constants::rest_status_constant::ResponseMessage, types::response_type::ResponseStruct,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn actuator() -> impl IntoResponse {
    let resp: ResponseStruct<String> = ResponseStruct {
        status: ResponseMessage::Ok as u32,
        message: ResponseMessage::Ok.to_string(),
        desc: None,
        data: None,
    };
    return (StatusCode::OK, Json(resp).into_response());
}
