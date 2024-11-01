use serde::{Deserialize, Serialize};
use crate::shared::types::system_type::System;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ScreenMappingRequest {
    pub screen_no: i8,
    pub machine: System,
}