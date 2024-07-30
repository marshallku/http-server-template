use axum::{routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/", get(super::index::get))
}
