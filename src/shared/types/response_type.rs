use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ResponseStruct<T> {
    pub status: u32,
    pub message: String,
    pub desc: Option<String>,
    pub data: Option<T>,
}
