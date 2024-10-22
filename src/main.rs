pub mod shared;
use dotenvy::dotenv;
use env_logger;
use log::debug;

fn main() {
    dotenv().ok();
    env_logger::init();

    loop {
        let screen = shared::utils::win::screen_util::get_screen_metrics();
        // debug!("width: {} heigth: {}", screen.width, screen.height);

        let cursor = shared::utils::win::mouse_util::get_cursor_point();
        debug!("x: {} y: {}", cursor.x, cursor.y);

        let check_edge = shared::utils::win::mouse_util::check_position_at_edge(cursor, screen);
        // debug!("edge : {}", check_edge.clone().unwrap());

        let edge_result = check_edge.clone().unwrap();

        if check_edge_eq(&edge_result) {
            shared::utils::win::mouse_util::lock_cursor(cursor);
        }
    }
}

fn check_edge_eq(s: &String) -> bool {
    return !s.eq_ignore_ascii_case("else");
}
