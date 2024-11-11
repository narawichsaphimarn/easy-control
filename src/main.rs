pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use application::services::mouse_event_service::MouseEventControlServiceApplication;
use application::services::screen_event_service::ScreenEventControlServiceApplication;
use dotenvy::dotenv;
use std::sync::Arc;

use crate::application::services::mouse_control_service::MouseControlServiceApplication;
use crate::infrastructure::api::axum_config::AxumInit;
use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use crate::infrastructure::log::log_custom::SimpleLogger;
use crate::shared::stores::stores::Stores;
use log::LevelFilter;
use crate::application::services::block_event_control_service::BlockEventControlServiceApplication;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init();
    let store = Stores::new().await;
    tokio::task::spawn(AxumInit::new(Arc::clone(&store)).start());
    tokio::task::spawn(ScreenEventControlServiceApplication::new(Arc::clone(&store)).run());
    tokio::task::spawn(MouseEventControlServiceApplication::new(Arc::clone(&store)).run());
    tokio::task::spawn(MouseControlServiceApplication::new(Arc::clone(&store)).run());
    tokio::task::spawn(BlockEventControlServiceApplication::new(Arc::clone(&store)).run());
    tokio::signal::ctrl_c().await.unwrap();
}

pub fn init() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Log initial error");
    SqliteDBInfra::init().expect("Database initial error");
}
