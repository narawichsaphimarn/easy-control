use super::screen_type::Screen;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct System {
    pub host_name: String,
    pub mac: String,
    pub screen: Screen,
}
