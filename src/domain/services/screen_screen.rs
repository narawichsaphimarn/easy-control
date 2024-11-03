use crate::domain::repositories::screen_mapping_matrix_repository::ScreenMappingMetricRepository;
use crate::domain::repositories::screen_mapping_refer_repository::ScreenMappingReferRepository;
use crate::domain::repositories::screen_selector_repository::ScreenSelectorRepository;
use crate::presentation::models::screen_model::ScreenMappingRequest;
use crate::shared::constants::screen_constant::ScreenMapperController;
use sqlite::Error;

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
                let numbers_matrix = row_matrix.parameter_value.split(",");
                for number_matrix in numbers_matrix {
                    for machine_filter in &machines_filter {
                        if number_matrix
                            .eq_ignore_ascii_case(machine_filter.screen_no.to_string().as_ref())
                        {
                            let position_group = format!("{},{}", no, machine_filter.screen_no);
                            log::debug!("position_group {}", position_group);
                            for position_value_row in
                                ScreenMappingReferRepository::find_by_key_and_group(
                                    ScreenMapperController::ScreenNumber.to_string(),
                                    position_group,
                                )?
                            {
                                log::debug!(
                                    "position_group value {}",
                                    position_value_row.parameter_value
                                );
                                let _ = ScreenMappingMetricRepository::save(
                                    system.machine.mac.to_string(),
                                    machine_filter.machine.mac.to_string(),
                                    position_value_row.parameter_value,
                                )?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
