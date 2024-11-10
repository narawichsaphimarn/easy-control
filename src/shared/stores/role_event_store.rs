use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

use crate::domain::pojo::setting_pojo::Setting;
use crate::domain::repositories::setting_repository::SettingRepository;
use crate::domain::services::setting_service::SettingServiceDomain;

#[derive(Debug, Clone)]
pub struct RoleControl {
    pub is_server: Arc<Mutex<bool>>,
}

impl RoleControl {
    pub fn check_role() -> bool {
        if let Ok(value) = SettingRepository::find_by_key_and_group(
            String::from("NETWORK_ROLE"),
            String::from("NETWORK"),
        ) {
            let setting = if value.len() == 0 {
                if let Ok(value) = SettingServiceDomain::save(
                    String::from("NETWORK_ROLE"),
                    String::from("NETWORK"),
                    String::from("CLIENT"),
                ) {
                    value
                } else {
                    vec![Setting {
                        id: 1,
                        parameter_key: String::from("NETWORK_ROLE"),
                        parameter_group: String::from("NETWORK"),
                        parameter_value: String::from("CLIENT"),
                    }]
                }
            } else {
                value
            };
            let mut status = false;
            for value in setting {
                if value.parameter_value == "SERVER" {
                    status = true;
                } else {
                    status = false;
                }
            }
            status
        } else {
            false
        }
    }

    pub fn new() -> Self {
        RoleControl {
            is_server: Arc::new(Mutex::new(Self::check_role())),
        }
    }

    pub async fn get_is_server(&self) -> MutexGuard<'_, bool> {
        let value = self.is_server.lock().await;
        value
    }

    pub async fn update_is_server(&self) {
        match self.is_server.try_lock() {
            Ok(mut data) => {
                *data = Self::check_role();
            }
            Err(e) => log::error!("Failed to lock update: {:?}", e),
        }
    }
}
