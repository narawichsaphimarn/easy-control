use crate::shared::constants::event_process_constant::EventProcess;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::{watch, Mutex};

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
                // log::debug!("sending step success");
            }
            Err(err) => {
                // log::debug!("sending step failed: {}", err);
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

#[derive(Clone, Debug)]
pub struct StepControlStoreV2 {
    pub step: Arc<Mutex<EventProcess>>,
}

impl StepControlStoreV2 {
    pub fn new() -> Self {
        Self {
            step: Arc::new(Mutex::new(EventProcess::Restart)),
        }
    }

    pub async fn get_event(&self) -> EventProcess {
        let event = self.step.lock().await;
        event.clone()
    }

    pub async fn set_event(&self, event: EventProcess) {
        let mut event_lock = self.step.lock().await;
        *event_lock = event;
    }
}
