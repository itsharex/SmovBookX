use std::io;

use axum::response::IntoResponse;
use reqwest::StatusCode;

pub async fn file_handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}
