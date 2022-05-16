use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, get_service},
  Router,
};
use tauri::command;
use tokio::signal;
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[command]
pub async fn run_hfs() {
  let app: _ = Router::new()
    .route("/foo", get(|| async { "Hi from /foo" }))
    .fallback(get_service(ServeDir::new(".")).handle_error(handle_error))
    .layer(TraceLayer::new_for_http());

  let addr = SocketAddr::from(([127, 0, 0, 1], 3255));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[command]
pub async fn shutdown_signal() {
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
