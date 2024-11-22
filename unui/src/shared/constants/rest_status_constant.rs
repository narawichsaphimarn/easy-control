use serde::Serialize;
use std::fmt;
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ResponseMessage {
    Ok = 200,
    Err = 999,
}

impl fmt::Display for ResponseMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseMessage::Ok => write!(f, "SUCCESS"),
            ResponseMessage::Err => write!(f, "ERROR"),
        }
    }
}
