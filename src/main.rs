pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use dotenvy::dotenv;
use infrastructure::api::axum_config::start;

use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use crate::infrastructure::logs::log_custom::SimpleLogger;
use log::LevelFilter;
use crate::application::services::control_service::ControlServiceApplication;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main]
async fn main() {
    init();
    tokio::spawn(start());
    tokio::spawn(ControlServiceApplication::main());
    tokio::signal::ctrl_c().await.unwrap();
}

pub fn init() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace)).expect("Log initial error");
    SqliteDBInfra::init().expect("Database initial error");
}
