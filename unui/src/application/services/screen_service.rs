use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::domain::services::screen_service::ScreenServiceDomain;
use crate::infrastructure::thread_async::sync_barrier::SyncBarrier;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::rest_client::screen_mapping_matrix_rest_client::update_screen_matrix;
use crate::shared::utils::protocol_util::ProtocolUtil;
use sqlite::Error;

#[derive(Debug, Clone)]
pub struct ScreenServiceApplication;

impl ScreenServiceApplication {
    pub async fn screen_mapping_update(request: Vec<ScreenMappingRequest>) -> Result<(), String> {
        let barrier = SyncBarrier::new(1);
        let barrier_clone = barrier.barrier.clone();
        let task = tokio::spawn(async move {
            let _ = ScreenServiceDomain::screen_select(request.clone()).await;
            barrier_clone.wait().await;
            let _ = ScreenServiceDomain::screen_mapping_metric(request.clone()).await;
        });
        let _ = tokio::join!(task);
        Ok(())
    }

    pub async fn screen_mapping_process(request: Vec<ScreenMappingRequest>) -> Result<(), String> {
        let barrier = SyncBarrier::new(1);
        let barrier_clone = barrier.barrier.clone();
        let task = tokio::spawn(async move {
            let _ = ScreenServiceDomain::screen_select(request.clone()).await;
            let _ = ScreenServiceDomain::screen_mapping_metric(request.clone()).await;
            barrier_clone.wait().await;
            let _ = Self::update_matrix_inside_network(request).await;
        });
        let _ = tokio::join!(task);
        Ok(())
    }

    pub async fn update_matrix_inside_network(
        request: Vec<ScreenMappingRequest>,
    ) -> Result<(), Error> {
        let ips: (String, String) = ProtocolUtil::get_addrs();
        log::debug!("ips  wlan : {}, lan: {}", ips.0, ips.1);
        let (select_ip, _) = ProtocolServiceApplication::select_ip(ips);
        let mut request_cp = request.clone();
        request_cp.retain(|x| !x.machine.ip.eq_ignore_ascii_case(&select_ip));
        let mut join_handlers = Vec::new();
        for x in request_cp {
            let y = tokio::task::spawn(update_screen_matrix(x.machine.ip, request.clone()));
            join_handlers.push(y);
        }
        for handle in join_handlers {
            if let Err(e) = handle.await {
                log::error!("Task failed with error: {}", e);
            }
        }
        Ok(())
    }
}
