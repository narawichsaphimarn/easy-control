use super::screen_type::Screen;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct System {
    pub host_name: String,
    pub mac: String,
    pub screen: Screen,
}
