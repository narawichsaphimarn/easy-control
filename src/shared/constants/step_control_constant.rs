use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy)]
pub enum StepControl {
    ServerLocal,
    ServerRemote,
}

impl fmt::Display for StepControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StepControl::ServerLocal => write!(f, "local"),
            StepControl::ServerRemote => write!(f, "remote"),
        }
    }
}

impl StepControl {
    pub fn from_string(step: &String) -> StepControl {
        match step.to_lowercase().as_str() {
            "local" => StepControl::ServerLocal,
            "remote" => StepControl::ServerRemote,
            _ => StepControl::ServerLocal,
        }
    }
}
