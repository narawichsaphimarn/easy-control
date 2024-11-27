use crate::domain::repositories::screen_mapping_matrix_repository::ScreenMappingMetricRepository;
use crate::domain::repositories::screen_mapping_refer_repository::ScreenMappingReferRepository;
use crate::domain::repositories::screen_selector_repository::ScreenSelectorRepository;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::screen_constant::ScreenMapperController;
use sqlite::Error;
use std::collections::HashMap;

pub struct ScreenServiceDomain;
impl ScreenServiceDomain {
    pub async fn screen_select(screen: Vec<ScreenMappingRequest>) -> Result<(), Error> {
        ScreenSelectorRepository::truncate()?;
        for system in screen {
            let machine = system.machine;
            ScreenSelectorRepository::save(
                machine.ip,
                machine.mac,
                machine.host_name,
                machine.screen.width.to_string(),
                machine.screen.height.to_string(),
            )?;
        }
        Ok(())
    }

    pub async fn screen_mapping_metric(screen: Vec<ScreenMappingRequest>) -> Result<(), Error> {
        ScreenMappingMetricRepository::truncate()?;
        for system in screen.clone() {
            let no = system.screen_no;
            let mut machines_filter = screen.clone();
            machines_filter.retain(|x| x.screen_no != no);
            log::debug!("no {}", no.to_string());
            for row_matrix in ScreenMappingReferRepository::find_by_key_and_group(
                ScreenMapperController::ScreenNumber.to_string(),
                no.to_string(),
            )? {
                log::debug!("row_matrix {}", row_matrix.parameter_value);
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
                    log::debug!("position_group {:?}", item);
                    match ScreenMappingReferRepository::find_by_key_and_group(
                        ScreenMapperController::ScreenNumber.to_string(),
                        item.0.to_string(),
                    ) {
                        Ok(row) => {
                            log::debug!("position_group value {:?}", row.get(0));
                            if let Some(row) = row.get(0) {
                                let _ = ScreenMappingMetricRepository::save(
                                    system.machine.mac.to_string(),
                                    item.1.to_string(),
                                    row.parameter_value.to_string(),
                                )
                                .unwrap();
                            }
                        }
                        Err(error) => {
                            log::error!(
                                "Error ScreenMappingReferRepository::find_by_key_and_group {}",
                                error
                            );
                        }
                    }
                });
            }
        }
        Ok(())
    }
}
