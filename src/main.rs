pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use dotenvy::dotenv;
use infrastructure::api::axum_config::start;

use crate::application::services::control_service::ControlServiceApplication;
use crate::infrastructure::database::sqlite_database::{SqliteDBInfra, SqliteDBInfraInit};
use crate::infrastructure::log::log_custom::SimpleLogger;
use crate::shared::types::mouse_type::MouseEvent;
use crate::shared::types::protocol_type::ProtocolEvent;
use log::LevelFilter;
use std::sync::{Arc, Mutex};

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main]
async fn main() {
    let data_mouse_event = Arc::new(Mutex::new(MouseEvent { x: 0.0, y: 0.0, edge: String::new() }));
    let data_protocol_event = Arc::new(Mutex::new(ProtocolEvent { mac: String::new(), ip: String::new(), edge: String::new() }));
    init();
    tokio::spawn(start());
    tokio::spawn(ControlServiceApplication::mouse_event(Arc::clone(&data_mouse_event)));
    tokio::spawn(ControlServiceApplication::mouse_control(data_mouse_event));
    tokio::signal::ctrl_c().await.unwrap();
}

pub fn init() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace)).expect("Log initial error");
    SqliteDBInfra::init().expect("Database initial error");
}
