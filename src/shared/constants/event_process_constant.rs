use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum EventProcess {
    Server,
    Client,
    Restart,
}

impl Display for EventProcess {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventProcess::Client => write!(f, "client"),
            EventProcess::Restart => write!(f, "restart"),
            EventProcess::Server => write!(f, "server"),
        }
    }
}

impl EventProcess {
    pub fn from_string(step: &str) -> EventProcess {
        match step.to_lowercase().as_str() {
            "client" => EventProcess::Client,
            "restart" => EventProcess::Restart,
            "server" => EventProcess::Server,
            _ => EventProcess::Restart,
        }
    }
}
