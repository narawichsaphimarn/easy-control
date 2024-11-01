use crate::domain::repositories::screen_mapping_metric_repository::ScreenMappingMetricRepository;
use crate::domain::repositories::screen_selector_repository::ScreenSelectorRepository;
use crate::presentation::models::screen_model::ScreenMappingRequest;

pub struct ScreenServiceDomain;
impl ScreenServiceDomain {
    pub async fn screen_select(screen: Vec<ScreenMappingRequest>) {
        ScreenSelectorRepository::truncate();
        for system in screen {
            let machine = system.machine;
            ScreenSelectorRepository::save(machine.ip, machine.mac, machine.host_name, machine.screen.width.to_string(), machine.screen.height.to_string());
        }
    }

    pub async fn screen_mapping_metric(screen: Vec<ScreenMappingRequest>) {
        ScreenMappingMetricRepository::truncate();
        for system in screen {
            let no = system.screen_no;
            let machine = system.machine;

        }
    }
}