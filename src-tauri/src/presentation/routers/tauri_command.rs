use std::sync::Arc;

use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::application::services::screen_service::ScreenServiceApplication;
use crate::infrastructure::database::store_file::file_store::FileStore;
use crate::presentation::models::screen_model::ScreenMappingRequest;
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
pub async fn get_role(state: State<'_, Arc<Mutex<FileStore>>>) -> Result<String, ()> {
    let store = state.lock().await;
    Ok(store
        .clone()
        .setting
        .iter()
        .find(|x| x.parameter_key == "NETWORK_ROLE")
        .unwrap()
        .parameter_value
        .clone())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn set_machine(
    machine_select: Vec<ScreenMappingRequest>,
    state: State<'_, Arc<Mutex<FileStore>>>,
) -> Result<serde_json::Value, String> {
    match ScreenServiceApplication::screen_mapping_process(machine_select, Arc::clone(&state)).await
    {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

#[tauri::command]
pub fn start_server() {}
