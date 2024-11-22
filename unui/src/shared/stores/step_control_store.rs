use crate::domain::repositories::setting_repository::SettingRepository;
use crate::shared::constants::event_process_constant::EventProcess;
use crate::shared::constants::step_control_constant::StepControl;
use crate::shared::stores::role_event_store::RoleControl;
use crate::shared::types::mouse_type::MouseEvent;
use std::fmt::{Display, Formatter};
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};

#[derive(Clone, Debug)]
pub struct StepControlStore {
    pub step_tx: Sender<EventProcess>,
    pub step_rx: Receiver<EventProcess>,
}

impl StepControlStore {
    pub fn new() -> Self {
        let (step_tx, step_rx) = watch::channel(EventProcess::Restart);
        Self { step_tx, step_rx }
    }

    pub fn send(&self, step: EventProcess) {
        match self.step_tx.send(step) {
            Ok(_) => {
                log::debug!("sending step success");
            }
            Err(err) => {
                log::debug!("sending step failed: {}", err);
            }
        }
    }

    pub async fn receive(&self) -> EventProcess {
        self.step_rx.borrow().clone()
    }

    pub fn get_rx(&self) -> Receiver<EventProcess> {
        self.step_rx.clone()
    }
}
