pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use std::sync::Arc;
use application::services::control_service::{
    MouseEventControlServiceApplication,
    ScreenEventControlServiceApplication,
};
use dotenvy::dotenv;
use infrastructure::api::axum_config::start;

use crate::infrastructure::database::sqlite_database::{ SqliteDBInfra, SqliteDBInfraInit };
use crate::infrastructure::log::log_custom::SimpleLogger;
use log::LevelFilter;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    init();
    tokio::spawn(start());

    let mouse_event = Arc::new(MouseEventControlServiceApplication::new());
    tokio::task::spawn(ScreenEventControlServiceApplication::run(Arc::clone(&mouse_event)));
    // tokio::task::spawn(mouse_event.clone().wait_switch_cursor());
    // tokio::task::spawn(mouse_event.clone().wait_update_protocol_event());
    tokio::task::spawn(mouse_event.run());
    tokio::signal::ctrl_c().await.unwrap();
}

pub fn init() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Log initial error");
    SqliteDBInfra::init().expect("Database initial error");
}
