use super::smov_method;
use axum::{routing::get, Router};

pub fn system_api() -> Router {
  Router::new()
    .nest("/app", data_api())
    .route("/foo", get(|| async { "Hi from /foo" }))
}

pub fn data_api() -> Router {
  Router::new()
    .route("/all", get(smov_method::get_data_all))
    .route("/pagination", get(smov_method::get_data_pagination))
}
