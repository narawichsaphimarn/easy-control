use crate::shared::constants::step_control_constant::StepControl;
use crate::shared::types::mouse_type::MouseEvent;
use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};

#[derive(Clone, Debug)]
pub struct StepControlStore {
    pub step_tx: Sender<String>,
    pub step_rx: Receiver<String>,
}

impl StepControlStore {
    pub fn new() -> Self {
        let (step_tx, step_rx) = watch::channel::<String>(String::new());
        Self { step_tx, step_rx }
    }

    pub fn send(&self, step: String) {
        match self.step_tx.send(step) {
            Ok(_) => {
                log::debug!("sending step success");
            }
            Err(err) => {
                log::debug!("sending step failed: {}", err);
            }
        }
    }

    pub async fn wait(&mut self) -> bool {
        self.step_rx.changed().await.is_ok()
    }

    pub async fn receive(&mut self) -> String {
        self.step_rx.borrow().clone()
    }

    pub async fn get_rx(&mut self) -> Receiver<String> {
        self.step_rx.clone()
    }
}
