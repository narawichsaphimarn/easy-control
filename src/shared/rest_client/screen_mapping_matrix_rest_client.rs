use crate::infrastructure::client::rest_client::RestClientInfrastructure;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::rest_client_constant::SystemDetail;
use crate::shared::types::response_type::ResponseStruct;
use crate::shared::types::system_type::System;
use std::time::Duration;

pub async fn post_screen_matrix(ip: String, request: Vec<ScreenMappingRequest>) -> Result<(), String> {
    let url = format!(
        "{}{}:{}{}",
        SystemDetail::Prefix.to_string(),
        ip,
        SystemDetail::Port as u64,
        SystemDetail::Path.to_string()
    );
    log::debug!("Create screen matrix request url : {}", url);
    let resp: Result<ResponseStruct<Vec<ScreenMappingRequest>>, String> =
        RestClientInfrastructure::post(url, request, Duration::from_millis(SystemDetail::Timeout as u64)).await;
    match resp {
        Ok(s) => {
            log::debug!("Create screen matrix response: {:?}", s);
            Ok(())
        }
        Err(e) => Err(e),
    }
}