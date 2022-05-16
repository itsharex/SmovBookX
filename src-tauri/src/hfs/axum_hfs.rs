use crate::response::response::Response;
use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, get_service},
  Router,
};
use std::{io, net::SocketAddr};
use tauri::{command, Window};
use tokio::signal;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[command]
pub async fn run_hfs(window: Window) {
  let app: _ = Router::new()
    .route("/foo", get(|| async { "Hi from /foo" }))
    .fallback(get_service(ServeDir::new(".")).handle_error(handle_error))
    .layer(TraceLayer::new_for_http());

  let addr = SocketAddr::from(([127, 0, 0, 1], 3255));
  tracing::debug!("listening on {}", addr);

  let server = axum::Server::bind(&addr).serve(app.into_make_service());

  server
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

  window
    .emit(
      "HFS://OperatingStatus",
      Response::ok(Some(true), "hfs服务器开启"),
    )
    .unwrap();
}

//这里是否能将他定义为一个 tauri的监听 然后传输数据？ 明天实验
async fn handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }

  println!("signal received, starting graceful shutdown");
}
