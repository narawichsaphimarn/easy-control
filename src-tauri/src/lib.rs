use crate::presentation::routers::tauri_command::{get_role, scan_machine};
use infrastructure::{api::axum_config::AxumInit, database::store_file::file_store::FileStore};
use std::{env, sync::Arc};
use tokio::sync::Mutex;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod shared;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let store = FileStore::init().await;
    let store_act = Arc::new(Mutex::new(store));
    tokio::task::spawn(AxumInit::new(Arc::clone(&store_act)).start());
    tauri::Builder::default()
        .manage(Arc::clone(&store_act))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![scan_machine, get_role])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
