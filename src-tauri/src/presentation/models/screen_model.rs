use crate::shared::types::system_type::System;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ScreenMappingRequest {
    pub screen_no: i8,
    pub machine: System,
}
