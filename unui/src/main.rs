pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use dotenvy::dotenv;
use std::sync::Arc;

use crate::application::services::step_control_service::StepControlServiceApplication;
use crate::infrastructure::api::axum_config::AxumInit;
use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use crate::infrastructure::log::log_custom::SimpleLogger;
// use crate::shared::stores::stores::Stores;
use crate::shared::stores::stores_v2::StoresV2;
use log::LevelFilter;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    init();
    // let store = Stores::new().await;
    let storev2 = StoresV2::new().await;
    tokio::task::spawn(AxumInit::new(Arc::clone(&storev2)).start());
    tokio::task::spawn(StepControlServiceApplication::new(Arc::clone(&storev2)).start());
    tokio::signal::ctrl_c().await.unwrap();
}

pub fn init() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .expect("Log initial error");
    SqliteDBInfra::init().expect("Database initial error");
}
