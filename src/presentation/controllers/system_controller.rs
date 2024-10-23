use crate::{
    application::services::system_domain::SystemServiceApplication,
    presentation::models::system_model::Pagination,
};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
};

pub async fn get_system_detail(pagination: Query<Pagination>) -> impl IntoResponse {
    match SystemServiceApplication::get_system_detail(pagination.0.ip_addr) {
        Ok(system) => (StatusCode::OK, Json(system).into_response()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json({}).into_response()),
    }
}
