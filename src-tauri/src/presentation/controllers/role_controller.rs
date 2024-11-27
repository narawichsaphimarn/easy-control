use crate::domain::services::setting_service::SettingServiceDomain;
use crate::presentation::models::role_model::Pagination;
use crate::shared::constants::event_process_constant::EventProcess;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{ IntoResponse, Json };

pub async fn update_role(query: Query<Pagination>) -> impl IntoResponse {
    match
        SettingServiceDomain::update_value(
            String::from("NETWORK_ROLE"),
            String::from("NETWORK"),
            query.0.role
        )
    {
        Ok(value) => {
            (StatusCode::OK, Json(value).into_response())
        }
        Err(_) =>
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(StatusCode::INTERNAL_SERVER_ERROR.as_str()).into_response(),
            ),
    }
}
