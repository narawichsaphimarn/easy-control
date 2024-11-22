use crate::domain::{
    pojo::setting_pojo::Setting, repositories::setting_repository::SettingRepository,
};

pub struct SettingServiceDomain;

impl SettingServiceDomain {
    pub fn save(key: String, group: String, value: String) -> Result<Vec<Setting>, ()> {
        match SettingRepository::save(key, group, value) {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error step save {}", e);
                Ok(Vec::new())
            }
        }
    }

    pub fn update_value(key: String, group: String, value: String) -> Result<Vec<Setting>, ()> {
        match SettingRepository::update_value(key, group, value) {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error step update {}", e);
                Ok(Vec::new())
            }
        }
    }
}
