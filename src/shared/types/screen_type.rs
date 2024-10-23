use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
}
