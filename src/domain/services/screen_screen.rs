use crate::domain::repositories::screen_mapping_metric_repository::ScreenMappingMetricRepository;
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
            ScreenSelectorRepository::save(machine.ip, machine.mac, machine.host_name, machine.screen.width.to_string(), machine.screen.height.to_string())?;
        }
        Ok(())
    }

    pub async fn screen_mapping_metric(screen: Vec<ScreenMappingRequest>) -> Result<(), Error> {
        ScreenMappingMetricRepository::truncate()?;
        for system in screen {
            let no = system.screen_no;
            let machine = system.machine;
            for row in ScreenMappingReferRepository::find_by_key_and_group(ScreenMapperController::ScreenNumber.to_string(), no.to_string())? {
                log::debug!("{}", row.read::<&str, _>("mac"));
            }
        }
        Ok(())
    }
}