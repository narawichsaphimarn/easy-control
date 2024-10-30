pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

use crate::application::services::control_service::ControlServiceApplication;
use dotenvy::dotenv;
use infrastructure::api::axum_config::start;
use std::thread;

use crate::infrastructure::logs::log_custom::SimpleLogger;
use log::{LevelFilter, SetLoggerError};

static LOGGER: SimpleLogger = SimpleLogger;

fn main() {
    dotenv().ok();
    init().expect("Log initial error");
    let mut threads = Vec::new();
    let net_thd = thread::Builder::new().name("net".to_string()).spawn(move || {
        start();
    }).unwrap();
    let main_thd = thread::Builder::new().name("control".to_string()).spawn(move || {
        ControlServiceApplication::main();
    }).unwrap();
    threads.push(main_thd);
    threads.push(net_thd);
    for thread in threads {
        thread.join().unwrap();
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
}
