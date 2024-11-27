pub mod mouse_control_store;
pub mod mouse_event_store;
pub mod role_event_store;
pub mod screen_event_store;
pub mod step_control_store;

pub mod stores {

    // #[derive(Debug, Clone)]
    // pub struct Stores {
    //     pub mouse_event: Arc<MouseEventControl>,
    //     pub screen_event: Arc<ScreenEventControl>,
    //     pub role_event: Arc<RoleControl>,
    //     pub mouse_control: Arc<MouseControl>,
    //     pub lib_event: Arc<LibEvent>,
    // }

    // impl Stores {
    //     pub async fn new() -> Arc<Stores> {
    //         Arc::new(Stores {
    //             mouse_event: Arc::new(MouseEventControl::new()),
    //             screen_event: Arc::new(ScreenEventControl::new()),
    //             role_event: Arc::new(RoleControl::new()),
    //             mouse_control: Arc::new(MouseControl::new().await),
    //             lib_event: Arc::new(LibEvent::new()),
    //         })
    //     }
    // }
}

pub mod stores_v2 {
    use crate::shared::stores::step_control_store::StepControlStore;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    pub struct StoresV2 {
        pub step_control: Arc<StepControlStore>,
    }

    impl StoresV2 {
        pub fn new() -> Arc<StoresV2> {
            Arc::new(StoresV2 {
                step_control: Arc::new(StepControlStore::new()),
            })
        }
    }
}

pub mod stores_v3 {
    use crate::shared::stores::step_control_store::StepControlStoreV2;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    pub struct StoresV3 {
        pub step_control: Arc<StepControlStoreV2>,
    }

    impl StoresV3 {
        pub fn new() -> Arc<StoresV3> {
            Arc::new(StoresV3 {
                step_control: Arc::new(StepControlStoreV2::new()),
            })
        }
    }
}
