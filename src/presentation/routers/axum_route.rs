use axum::{routing::get, Router};

pub fn route() -> Router {
    let app: Router<_> = Router::new().route("/status", get({}));

    app
}
