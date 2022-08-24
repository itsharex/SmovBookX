use std::io;

use axum::{
  http::StatusCode,
  response::{Html, IntoResponse, Response},
};

pub async fn file_handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

// pub fn render_not_found() -> impl IntoResponse {
//   tower::service_fn(
//     |request: axum::http::Request<_>| async move {
//       std::io::Result::Ok(format!("{} not found", request.uri()).into_response())
//     },
//   )
// }


