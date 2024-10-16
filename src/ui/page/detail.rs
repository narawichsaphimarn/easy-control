use crate::ui::navigate::route::{AppState, PageData};
use druid::widget::{Button, Flex, Label};
use druid::{Data, Lens, Widget, WidgetExt};

#[derive(Clone, Data, Lens)]
pub struct HomeData {
    pub message: String,
}

// สร้างหน้า detail
pub fn build_detail_page() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Label::new(|data: &AppState, _env: &_| {
                if let PageData::Detail(detail_data) = &data.current_page {
                    detail_data.detail_info.clone()
                } else {
                    String::new()
                }
            })
            .padding(10.0),
        )
        .with_child(
            Button::new("Back to Home")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    data.current_page = PageData::Home(HomeData {
                        message: "You are back at the Home Page.".into(),
                    });
                })
                .padding(10.0),
        )
}
