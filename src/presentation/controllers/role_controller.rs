use crate::domain::services::setting_service::SettingServiceDomain;
use crate::presentation::models::role_model::Pagination;
use crate::shared::constants::event_process_constant::EventProcess;
use crate::shared::stores::role_event_store::RoleControl;
use crate::shared::stores::step_control_store::StepControlStore;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use std::sync::Arc;

pub async fn update_role(
    query: Query<Pagination>,
    step_control: Arc<StepControlStore>,
) -> impl IntoResponse {
    match SettingServiceDomain::update_value(
        String::from("NETWORK_ROLE"),
        String::from("NETWORK"),
        query.0.role,
    ) {
        Ok(value) => {
            step_control.send(EventProcess::Restart);
            (StatusCode::OK, Json(value).into_response())
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(StatusCode::INTERNAL_SERVER_ERROR.as_str()).into_response(),
        ),
    }
}
