pub mod mouse_control_store;
pub mod mouse_event_store;
pub mod role_event_store;
pub mod screen_event_store;

pub mod stores {
    use crate::shared::lib::lib_event::LibEvent;
    use crate::shared::stores::mouse_control_store::MouseControl;
    use crate::shared::stores::mouse_event_store::MouseEventControl;
    use crate::shared::stores::role_event_store::RoleControl;
    use crate::shared::stores::screen_event_store::ScreenEventControl;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    pub struct Stores {
        pub mouse_event: Arc<MouseEventControl>,
        pub screen_event: Arc<ScreenEventControl>,
        pub role_event: Arc<RoleControl>,
        pub mouse_control: Arc<MouseControl>,
        pub lib_event: Arc<LibEvent>,
    }

    impl Stores {
        pub async fn new() -> Arc<Stores> {
            Arc::new(Stores {
                mouse_event: Arc::new(MouseEventControl::new()),
                screen_event: Arc::new(ScreenEventControl::new()),
                role_event: Arc::new(RoleControl::new()),
                mouse_control: Arc::new(MouseControl::new().await),
                lib_event: Arc::new(LibEvent::new()),
            })
        }
    }
}
