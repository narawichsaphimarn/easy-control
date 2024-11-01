use crate::domain::services::screen_screen::ScreenServiceDomain;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use futures::TryFutureExt;

pub struct ScreenServiceApplication;
impl ScreenServiceApplication {
    pub async fn screen_mapping_process(request: Vec<ScreenMappingRequest>) -> Result<(), String> {
        let screen_select = tokio::task::spawn(ScreenServiceDomain::screen_select(request.clone()));
        let screen_mapping_metric = tokio::task::spawn(ScreenServiceDomain::screen_mapping_metric(request));
        for x in [screen_select, screen_mapping_metric] {
            let join = x.await;
            let _ = join.map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}