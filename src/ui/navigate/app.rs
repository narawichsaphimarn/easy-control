use super::route::{build_root_widget, AppState, PageData};
use crate::ui::page::detail::HomeData;
use druid::{AppLauncher, WindowDesc};

pub fn main_app() {
    let main_window = WindowDesc::new(build_root_widget())
        .title("Navigation with Separate Data")
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        current_page: PageData::Home(HomeData {
            message: "Welcome to the Home Page!".into(),
        }),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
