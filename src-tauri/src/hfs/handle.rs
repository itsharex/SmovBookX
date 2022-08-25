use std::io;

use axum::{
  http::StatusCode,
  response::{ IntoResponse, Response},
};


pub async fn file_handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn render_not_found<T>(request: axum::http::Request<T>) -> std::io::Result<Response> {
  std::io::Result::Ok(format!("{} not found", request.uri()).into_response())
}
