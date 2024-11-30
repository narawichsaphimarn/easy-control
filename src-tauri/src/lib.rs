use crate::presentation::routers::tauri_command::{
    get_role, get_system_detail, scan_machine, switch_role,
};
use crate::shared::stores::setting_json::Settings;
use crate::shared::stores::setting_mapping_refer_json::SettingMappingRef;
use crate::shared::stores::store_json::Stores;
use infrastructure::api::axum_config::AxumInit;
use presentation::routers::tauri_command::{get_screen_selector, set_machine, start_server};
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
    Settings::init().await;
    SettingMappingRef::init().await;
    let store = Arc::new(Mutex::new(Stores::init().await));
    tokio::task::spawn(AxumInit::new(Arc::clone(&store)).start());
    tauri::Builder::default()
        .manage(Arc::clone(&store))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            scan_machine,
            get_role,
            get_system_detail,
            switch_role,
            set_machine,
            get_screen_selector,
            start_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
