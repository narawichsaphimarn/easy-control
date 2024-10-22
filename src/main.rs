pub mod infrastructure;
pub mod presentation;
pub mod shared;

use dotenvy::dotenv;
use env_logger;
use infrastructure::api::axum_config::start;

fn main() {
    dotenv().ok();
    env_logger::init();
    start();
}
