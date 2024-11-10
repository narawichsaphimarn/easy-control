use std::sync::Arc;

use axum;
use log;

use crate::{
    application::services::{
        role_control_service::RoleControlServiceApplication,
        screen_event_service::ScreenEventControlServiceApplication,
    },
    presentation::routers::axum_route::route,
};

pub async fn start(
    screen_event: Arc<ScreenEventControlServiceApplication>,
    role: Arc<RoleControlServiceApplication>,
) {
    let app = route(Arc::clone(&screen_event), Arc::clone(&role));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::debug!(
        "Listener start {}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );
    axum::serve(listener, app).await.unwrap();
}
