use std::time::Duration;

use crate::{
    infrastructure::client::rest_client::RestClientInfrastructure, presentation::models,
    shared::constants,
};

pub async fn sent_event(
    ip: String,
    request: models::mouse_event_model::MouseEvent,
) -> Result<(), String> {
    let url = format!(
        "{}{}:{}{}",
        constants::rest_client_constant::MouseEvent::Prefix.to_string(),
        ip,
        constants::rest_client_constant::MouseEvent::Port as u64,
        constants::rest_client_constant::MouseEvent::Path.to_string()
    );
    // log::debug!("Create screen matrix request url : {}", url);
    let resp: Result<String, String> = RestClientInfrastructure::post(
        url,
        request,
        Duration::from_millis(constants::rest_client_constant::MouseEvent::Timeout as u64),
    )
    .await;
    match resp {
        Ok(s) => {
            // log::debug!("Create screen matrix response: {:?}", s);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
