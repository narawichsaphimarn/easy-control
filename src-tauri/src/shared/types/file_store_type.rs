use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScreenMappingMatrix {
    pub mac_source: String,
    pub mac_target: String,
    pub edge: String,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScreenSelector {
    pub ip: String,
    pub mac: String,
    pub hostname: String,
    pub width: String,
    pub height: String,
    pub screen_no: i8,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Setting {
    pub parameter_key: String,
    pub parameter_group: String,
    pub parameter_value: String,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScreenMappingRefer {
    pub parameter_key: String,
    pub parameter_group: String,
    pub parameter_value: String,
}
