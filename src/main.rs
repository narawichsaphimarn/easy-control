pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use application::services::mouse_event_service::MouseEventControlServiceApplication;
use application::services::role_control_service::RoleControlServiceApplication;
use application::services::screen_event_service::ScreenEventControlServiceApplication;
use dotenvy::dotenv;
use infrastructure::api::axum_config::start;
use std::sync::Arc;

use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use crate::infrastructure::log::log_custom::SimpleLogger;
use log::LevelFilter;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    init();
    let mouse_event = Arc::new(MouseEventControlServiceApplication::new());
    let screen_event = Arc::new(ScreenEventControlServiceApplication::new());
    let role_event = Arc::new(RoleControlServiceApplication::new());
    tokio::task::spawn(start(Arc::clone(&screen_event), Arc::clone(&role_event)));
    tokio::task::spawn(screen_event.run(Arc::clone(&mouse_event)));
    tokio::task::spawn(mouse_event.run(Arc::clone(&role_event)));
    tokio::signal::ctrl_c().await.unwrap();
}

pub fn init() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Log initial error");
    SqliteDBInfra::init().expect("Database initial error");
}
