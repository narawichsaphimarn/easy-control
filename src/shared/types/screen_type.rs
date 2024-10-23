use serde::Serialize;

#[derive(Serialize)]
pub struct Screen {
    pub width: i32,
    pub height: i32,
}
