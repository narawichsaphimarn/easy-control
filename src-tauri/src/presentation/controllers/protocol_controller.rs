use crate::presentation::models::system_model::Pagination;
use crate::shared::utils::protocol_util::ProtocolUtil;
use axum::extract::Query;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn ping(pagination: Query<Pagination>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ProtocolUtil::ping_ip(&*pagination.0.ip_addr).await).into_response(),
    )
}
