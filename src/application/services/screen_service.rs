use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::domain::services::screen_screen::ScreenServiceDomain;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::rest_client::screen_mapping_matrix_rest_client::update_screen_matrix;
use crate::shared::utils::protocol_util::get_addrs;
use sqlite::Error;

pub struct ScreenServiceApplication;
impl ScreenServiceApplication {
    pub async fn screen_mapping_update(request: Vec<ScreenMappingRequest>) -> Result<(), String> {
        let screen_select = tokio::task::spawn(ScreenServiceDomain::screen_select(request.clone()));
        let screen_mapping_metric = tokio::task::spawn(ScreenServiceDomain::screen_mapping_metric(request));
        let _ = tokio::join!(screen_select, screen_mapping_metric);
        Ok(())
    }

    pub async fn screen_mapping_process(request: Vec<ScreenMappingRequest>) -> Result<(), String> {
        let screen_select = tokio::task::spawn(ScreenServiceDomain::screen_select(request.clone()));
        let screen_mapping_metric = tokio::task::spawn(
            ScreenServiceDomain::screen_mapping_metric(request.clone())
        );
        let update_screen_matrix = tokio::task::spawn(Self::update_matrix_inside_network(request));
        let _ = tokio::join!(screen_select, screen_mapping_metric, update_screen_matrix);
        Ok(())
    }

    pub async fn update_matrix_inside_network(
        request: Vec<ScreenMappingRequest>
    ) -> Result<(), Error> {
        let ips: (String, String) = get_addrs();
        log::debug!("ips  wlan : {}, lan: {}", ips.0, ips.1);
        let (select_ip, _) = ProtocolServiceApplication::select_ip(ips);
        let mut request_cp = request.clone();
        request_cp.retain(|x| !x.machine.ip.eq_ignore_ascii_case(&select_ip));
        let mut join_handlers = Vec::new();
        for x in request_cp {
            let y = tokio::task::spawn(update_screen_matrix(x.machine.ip, request.clone()));
            join_handlers.push(y);
        }
        for join_handler in join_handlers {
            let _ = join_handler.await;
        }
        Ok(())
    }
}
