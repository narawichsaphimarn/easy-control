use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use crate::application::services::protocol_service::ProtocolServiceApplication;
use crate::infrastructure::database::store_file::file_store::FileStore;
use crate::infrastructure::thread_async::sync_barrier::SyncBarrier;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::screen_constant::ScreenMapperController;
use crate::shared::rest_client::screen_mapping_matrix_rest_client::update_screen_matrix;
use crate::shared::types::file_store_type::{
    ScreenMappingMatrix, ScreenMappingRefer, ScreenSelector,
};
use crate::shared::utils::protocol_util::ProtocolUtil;

#[derive(Debug, Clone)]
pub struct ScreenServiceApplication;

impl ScreenServiceApplication {
    pub async fn screen_mapping_update(
        request: Vec<ScreenMappingRequest>,
        filestore: Arc<Mutex<FileStore>>,
    ) -> Result<Vec<ScreenMappingRequest>, String> {
        let screen_selects = Self::screen_select(request.clone())
            .await
            .map_err(|e| e.to_string())?;
        let mut mappings = filestore.lock().await;
        let screen_matrixs =
            Self::screen_mapping_metric(request.clone(), mappings.screen_mapping_refer.clone())
                .await
                .map_err(|e| e.to_string())?;
        mappings.screen_selector = screen_selects;
        mappings.screen_mapping_matrix = screen_matrixs;
        let result = FileStore::write_file(mappings.clone()).await;
        match result {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
        Ok(request)
    }

    pub async fn screen_mapping_process(
        request: Vec<ScreenMappingRequest>,
        filestore: Arc<Mutex<FileStore>>,
    ) -> Result<Vec<ScreenMappingRequest>, String> {
        let screen_selects = Self::screen_select(request.clone())
            .await
            .map_err(|e| e.to_string())?;
        let mut mappings = filestore.lock().await;
        let screen_matrixs =
            Self::screen_mapping_metric(request.clone(), mappings.screen_mapping_refer.clone())
                .await
                .map_err(|e| e.to_string())?;
        mappings.screen_selector = screen_selects;
        mappings.screen_mapping_matrix = screen_matrixs;
        let result = FileStore::write_file(mappings.clone()).await;
        match result {
            Ok(_) => {
                // let _ = Self::update_matrix_inside_network(request).await;
            }
            Err(e) => panic!("{}", e),
        }
        Ok(request)
    }

    pub async fn update_matrix_inside_network(
        request: Vec<ScreenMappingRequest>,
    ) -> Result<(), String> {
        let ips: (String, String) = ProtocolUtil::get_addrs();
        // log::debug!("ips  wlan : {}, lan: {}", ips.0, ips.1);
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
                panic!("{}", e);
                // log::error!("Task failed with error: {}", e);
            }
        }
        Ok(())
    }

    pub async fn screen_select(
        screens: Vec<ScreenMappingRequest>,
    ) -> Result<Vec<ScreenSelector>, String> {
        let screen_select: Vec<ScreenSelector> = screens
            .iter()
            .map(|item| {
                return ScreenSelector {
                    ip: item.machine.ip.clone(),
                    mac: item.machine.mac.clone(),
                    hostname: item.machine.host_name.clone(),
                    width: item.machine.screen.width.to_string(),
                    height: item.machine.screen.height.to_string(),
                };
            })
            .collect();
        Ok(screen_select)
    }

    pub async fn screen_mapping_metric(
        screen: Vec<ScreenMappingRequest>,
        mapping_refer: Vec<ScreenMappingRefer>,
    ) -> Result<Vec<ScreenMappingMatrix>, String> {
        let mut result = Vec::<ScreenMappingMatrix>::new();
        for system in screen.clone() {
            let no = system.screen_no;
            let mut machines_filter = screen.clone();
            machines_filter.retain(|x| x.screen_no != no);
            // log::debug!("no {}", no.to_string());
            if Self::find_screen_mapping_refer(
                &mapping_refer,
                ScreenMapperController::ScreenNumber.to_string(),
                no.to_string(),
            )
            .is_some()
            {
                // log::debug!("row_matrix {}", row_matrix.parameter_value);
                let machines_filter_mapping = &machines_filter
                    .clone()
                    .iter()
                    .map(|item| {
                        (
                            format!("{},{}", no, item.screen_no),
                            item.clone().machine.mac,
                        )
                    })
                    .collect::<HashMap<String, String>>();
                machines_filter_mapping.iter().for_each(|item| {
                    // log::debug!("position_group {:?}", item);
                    let result_matrix = Self::find_screen_mapping_refer(
                        &mapping_refer,
                        ScreenMapperController::ScreenNumber.to_string(),
                        item.0.to_string(),
                    );
                    if let Some(matrix) = result_matrix {
                        result.push(ScreenMappingMatrix {
                            mac_source: system.machine.mac.to_string(),
                            mac_target: item.1.to_string(),
                            edge: matrix.parameter_value.to_string(),
                        });
                    }
                });
            }
        }
        Ok(result)
    }

    fn find_screen_mapping_refer(
        mapping_refer: &Vec<ScreenMappingRefer>,
        key: String,
        group: String,
    ) -> Option<ScreenMappingRefer> {
        mapping_refer
            .clone()
            .iter()
            .find(|x| {
                x.parameter_key.eq_ignore_ascii_case(&key)
                    && x.parameter_group.eq_ignore_ascii_case(&group)
            })
            .cloned()
    }
}
