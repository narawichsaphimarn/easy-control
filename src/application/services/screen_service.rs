use crate::domain::services::screen_screen::ScreenServiceDomain;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use futures::TryFutureExt;

pub struct ScreenServiceApplication;
impl ScreenServiceApplication {
    pub async fn screen_mapping_process(request: Vec<ScreenMappingRequest>) -> Result<(), String> {
        ScreenServiceDomain::screen_mapping_metric(request).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}