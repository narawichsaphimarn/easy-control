use super::screen_type::Screen;
use serde::Serialize;

#[derive(Serialize)]
pub struct System {
    pub host_name: String,
    pub mac: String,
    pub screen: Screen,
}
