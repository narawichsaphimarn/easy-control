use axum;
use log;

use crate::presentation::routers::axum_route::route;

#[tokio::main]
pub async fn start() {
    let app = route();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::debug!(
        "Listener start {}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );
    axum::serve(listener, app).await.unwrap();
}
