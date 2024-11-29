use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseStruct<T> {
    pub status: u32,
    pub message: String,
    pub desc: Option<String>,
    pub data: Option<T>,
}
