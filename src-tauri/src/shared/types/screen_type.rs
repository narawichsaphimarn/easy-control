use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
}
