use crate::ui::navigate::route::{AppState, PageData};
use druid::widget::{Button, Flex, Label};
use druid::{Data, Lens, Widget, WidgetExt};

#[derive(Clone, Data, Lens)]
pub struct DetailData {
    pub detail_info: String,
}

// สร้างหน้า home
pub fn build_home_page() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Label::new(|data: &AppState, _env: &_| {
                if let PageData::Home(home_data) = &data.current_page {
                    home_data.message.clone()
                } else {
                    String::new()
                }
            })
            .padding(10.0),
        )
        .with_child(
            Button::new("Go to Detail Page")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    data.current_page = PageData::Detail(DetailData {
                        detail_info: "This is some detailed information.".into(),
                    });
                })
                .padding(10.0),
        )
}
