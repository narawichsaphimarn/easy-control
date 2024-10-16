use crate::ui::page::{
    detail::{build_detail_page, HomeData},
    home::{build_home_page, DetailData},
};
use druid::widget::ViewSwitcher;
use druid::{Data, Lens, Widget, WidgetExt};

#[derive(Clone, Data)]
pub enum PageData {
    Home(HomeData),
    Detail(DetailData),
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub current_page: PageData,
}

// สร้าง root widget
pub fn build_root_widget() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_page.clone(),
        |page_data, _data, _env| match page_data {
            PageData::Home(_) => build_home_page().boxed(),
            PageData::Detail(_) => build_detail_page().boxed(),
        },
    )
}
