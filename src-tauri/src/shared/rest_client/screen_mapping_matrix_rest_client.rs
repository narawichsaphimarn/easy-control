use crate::infrastructure::client::rest_client::RestClientInfrastructure;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::rest_client_constant::ScreenMappingMatrix;
use crate::shared::types::response_type::ResponseStruct;
use std::time::Duration;

pub async fn update_screen_matrix(
    ip: String,
    request: Vec<ScreenMappingRequest>,
) -> Result<(), String> {
    let url = format!(
        "{}{}:{}{}",
        ScreenMappingMatrix::Prefix.to_string(),
        ip,
        ScreenMappingMatrix::Port as u64,
        ScreenMappingMatrix::Path.to_string()
    );
    // log::debug!("Create screen matrix request url : {}", url);
    let resp: Result<ResponseStruct<Vec<ScreenMappingRequest>>, String> =
        RestClientInfrastructure::put(
            url,
            request,
            Duration::from_millis(ScreenMappingMatrix::Timeout as u64),
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
