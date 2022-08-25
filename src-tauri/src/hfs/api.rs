use super::{
  handle::{file_handle_error, render_not_found},
  smov_method,
};
use axum::{
  routing::{get, get_service},
  Router,
};
use tower_http::services::ServeDir;

pub fn system_api() -> Router {
  Router::new()
    .nest("/data", data_api())
    .nest("/file", file_api())
}

pub fn file_api() -> Router {
  let tidy_folder = &crate::app::APP.lock().clone().conf.tidy_folder;
  //通过阅读代码 解决办法在 https://github.com/search?l=Rust&p=2&q=not_found_service&type=Code 应该要在 tower_http 设置not_found_fallback 即ServeDir::new(tidy_folder) 部分
  let svc =
    get_service(ServeDir::new(tidy_folder).not_found_service(tower::service_fn(render_not_found)))
      .handle_error(file_handle_error);
  Router::new().nest("/", svc)
}

pub fn data_api() -> Router {
  Router::new()
    .route("/all", get(smov_method::get_data_all))
    .route("/pagination", get(smov_method::get_data_pagination))
    .route("/error_test", get(smov_method::error_test))
}
