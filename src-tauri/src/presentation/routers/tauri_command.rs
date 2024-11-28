use std::sync::Arc;

use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::application::services::screen_service::ScreenServiceApplication;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::stores::setting_json::Settings;
use crate::shared::stores::store_json::Stores;
use crate::shared::utils::system_util::SystemUtil;
use serde_json::json;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command(rename_all = "snake_case")]
pub async fn scan_machine() -> Result<serde_json::Value, String> {
    let result = ProtocolServiceApplication::check_machine()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json!(result))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_role() -> Result<String, ()> {
    let setting =
        Settings::get_setting(String::from("NETWORK_ROLE"), String::from("NETWORK")).await;
    Ok(setting.parameter_value)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_machine(
    machine_select: Vec<ScreenMappingRequest>,
    state: State<'_, Arc<Mutex<Stores>>>,
) -> Result<serde_json::Value, String> {
    match ScreenServiceApplication::screen_mapping_process(machine_select, Arc::clone(&state)).await
    {
        Ok(result) => Ok(json!(result)),
        Err(e) => Err(e),
    }
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_screen_selector(
    state: State<'_, Arc<Mutex<Stores>>>,
) -> Result<serde_json::Value, String> {
    let store = state.lock().await;
    Ok(json!(store.screen_selector))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_system_detail() -> Result<serde_json::Value, String> {
    let result = ProtocolServiceApplication::get_machine_detail()
        .await
        .map_err(|e| e.to_string())?;
    Ok(json!(result))
}

#[tauri::command(rename_all = "snake_case")]
pub async fn switch_row() {
    let mut settings = Settings::default();
    for mut setting in Settings::read_file().await.setting {
        if setting.parameter_key.eq_ignore_ascii_case(&"NETWORK_ROLE")
            && setting.parameter_group.eq_ignore_ascii_case(&"NETWORK")
        {
            if setting.parameter_value.eq_ignore_ascii_case("CLIENT") {
                setting.parameter_value = String::from("SERVER");
            } else {
                setting.parameter_value = String::from("CLIENT");
            }
        }
        settings.setting.push(setting);
    }
    let result = Settings::write_file(settings).await;
    match result {
        Ok(_) => {
            print!("Switch role `SUCCESS`");
        }
        Err(e) => panic!("Error: {}", e),
    }
    SystemUtil::restart_app();
}

#[tauri::command]
pub fn start_server() {}
