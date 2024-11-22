use crate::shared::constants::event_process_constant::EventProcess;
use crate::shared::stores::role_event_store::RoleControl;
// use crate::shared::stores::stores::Stores;
use crate::application::services::server_step_service::ServerStepServiceApplication;
use crate::shared::stores::stores_v2::StoresV2;
use crate::shared::utils::mouse_util::MouseUtil;
use std::sync::Arc;
use tokio::task::JoinHandle;
use crate::shared::constants::step_control_constant::StepControl;

#[derive(Debug, Clone)]
pub struct StepControlServiceApplication {
    pub stores: Arc<StoresV2>,
}

impl StepControlServiceApplication {
    pub fn new(stores: Arc<StoresV2>) -> Arc<Self> {
        Arc::new(StepControlServiceApplication { stores })
    }

    pub async fn process_step(&self) {
        let mut rx = self.stores.step_control.get_rx();
        let mut jhs: Vec<JoinHandle<_>> = Vec::<JoinHandle<_>>::new();
        let server_step = ServerStepServiceApplication::new();
        while rx.changed().await.is_ok() {
            tokio::select! {
                _ = async {}, if self.stores.step_control.receive().await.to_string().eq_ignore_ascii_case
                    (&"RESTART")  => {
                    log::debug!("RESTART");
                    if !jhs.is_empty() {
                        jhs.iter().for_each(|jh: &JoinHandle<()>| jh.abort());
                    }
                    server_step.stop_tasks();
                    let role = RoleControl::check_role();
                    self.stores.step_control.send(EventProcess::from_string(role.as_str()));
                }
                _ = async {}, if self.stores.step_control.receive().await.to_string().eq_ignore_ascii_case(&"CLIENT")  => {
                    log::debug!("Client");
                }
                _ = async {}, if self.stores.step_control.receive().await.to_string().eq_ignore_ascii_case(&"SERVER")  => {
                    log::debug!("Server");
                    let jh: JoinHandle<_> = tokio::task::spawn(server_step.clone().run());
                    jhs.push(jh);
                }
            }
        }
    }

    pub async fn start(self: Arc<Self>) {
        self.stores.step_control.send(EventProcess::Restart);
        self.process_step().await;
    }
}
